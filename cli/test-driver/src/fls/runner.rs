//! How a backend is run on the tests of the suite: external tool chains
//! through a pipeline of steps.

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
};

use anyhow::{Result, bail};
use futures::future::{BoxFuture, TryJoinAll};
use hax_types::cli_options::BackendName;

use super::{lean::Lean, package::FlsContext, stage::Stage, test::FlsTest};
use crate::{
    helpers::BackendNameExt,
    log::{BackendJobKind, JobKind},
};

/// Observed stage per test path.
pub type Stages = BTreeMap<String, Stage>;

/// Name and version of the tools that produced the stages.
pub type ToolVersions = Vec<(String, String)>;

/// Runs one backend on the tests of the suite.
pub trait BackendRunner: Send + Sync {
    /// Versions of the tools that produce the stages, recorded in the results file.
    fn tool_versions<'a>(&'a self, ctx: &'a FlsContext) -> BoxFuture<'a, Result<ToolVersions>>;

    /// Runs the backend on every test and returns the observed stages.
    fn run<'a>(
        &'a self,
        ctx: &'a FlsContext,
        tests: &'a [&'a FlsTest],
    ) -> BoxFuture<'a, Result<Stages>>;
}

pub fn runner_for(ctx: &FlsContext, backend: BackendName) -> Result<Box<dyn BackendRunner>> {
    Ok(match backend {
        BackendName::Lean => Box::new(ExternalPipeline {
            steps: Lean::new(ctx)?,
        }),
        other => bail!("the FLS suite does not run the hax-engine backend `{other}` yet"),
    })
}

/// The phases of an external pipeline, for naming the jobs in the log.
#[derive(Clone, Copy)]
pub enum PipelinePhase {
    Canary,
    Extract,
    Prepare,
    Batch(Stage),
}

/// The steps of an external tool chain: a per-test extraction that decides
/// the stages up to the first batch stage, then one batch command per
/// remaining stage.
pub trait PipelineSteps: Send + Sync {
    fn backend(&self) -> BackendName;
    fn phase_name(&self, phase: PipelinePhase) -> String;

    /// Per test, under the job semaphore: runs the tool chain and returns the
    /// stage reached, up to the first batch stage.
    fn extract(
        &self,
        ctx: &FlsContext,
        test: &FlsTest,
        job: JobKind,
    ) -> impl Future<Output = Result<Stage>> + Send;

    /// Called once before the batch stages with every test that reached the
    /// stage below the first batch stage.
    fn prepare(
        &self,
        ctx: &FlsContext,
        tests: &[&FlsTest],
        job: JobKind,
    ) -> impl Future<Output = Result<()>> + Send;

    /// The stages decided by one batch command each, in order; each promotes
    /// the tests at the stage below it that do not fail.
    fn batch_stages(&self) -> Vec<Stage>;

    /// Runs the batch command of a stage and returns the paths of the tests that failed.
    fn run_batch(
        &self,
        ctx: &FlsContext,
        stage: Stage,
        tests: &[&FlsTest],
        job: JobKind,
    ) -> impl Future<Output = Result<HashSet<String>>> + Send;

    fn tool_versions(&self, ctx: &FlsContext) -> impl Future<Output = Result<ToolVersions>> + Send;
}

const CANARY_SOURCE: &str = include_str!("../../templates/fls-canary/lib.rs");

/// A backend runner made of pipeline steps.
pub struct ExternalPipeline<S: PipelineSteps> {
    pub steps: S,
}

impl<S: PipelineSteps> ExternalPipeline<S> {
    fn phase(&self, phase: PipelinePhase) -> JobKind {
        self.steps.backend().job_kind(BackendJobKind::Phase {
            name: self.steps.phase_name(phase),
        })
    }

    async fn extract(&self, ctx: &FlsContext, test: &FlsTest) -> Result<(String, Stage)> {
        let _permit = ctx.options.semaphore_permit_for_test().await?;
        let job = self.steps.backend().job_kind(BackendJobKind::CargoHaxInto {
            test: test.path.clone(),
        });
        let stage = job
            .run(async |job| self.steps.extract(ctx, test, job).await)
            .await?;
        Ok((test.path.clone(), stage))
    }

    /// Extracts a trivial crate, which only fails when the environment cannot
    /// run the tool chain. Such a failure would otherwise pass for a
    /// rejection of every test.
    /// It runs under the phase `job`, so it does not count as a test.
    async fn canary(&self, ctx: &FlsContext, expected: Stage, job: JobKind) -> Result<()> {
        let source = ctx.fls_cache.join("canary.rs");
        fs::write(&source, CANARY_SOURCE)?;
        let test = FlsTest {
            path: "_canary".into(),
            source,
            fls: vec![],
            title: "canary".into(),
            no_equiv: None,
            tiers: HashMap::new(),
            reasons: HashMap::new(),
            equiv: vec![],
        };
        let stage = self.steps.extract(ctx, &test, job).await?;
        if stage != expected {
            bail!(
                "a trivial crate reached `{}` instead of `{}`; the environment cannot run the tool chain",
                stage.as_str(),
                expected.as_str()
            );
        }
        Ok(())
    }
}

impl<S: PipelineSteps> BackendRunner for ExternalPipeline<S> {
    fn tool_versions<'a>(&'a self, ctx: &'a FlsContext) -> BoxFuture<'a, Result<ToolVersions>> {
        Box::pin(self.steps.tool_versions(ctx))
    }

    fn run<'a>(
        &'a self,
        ctx: &'a FlsContext,
        tests: &'a [&'a FlsTest],
    ) -> BoxFuture<'a, Result<Stages>> {
        Box::pin(async move {
            let batches = self.steps.batch_stages();
            if let Some(expected) = batches.first().and_then(|first| first.prev())
                && !tests.is_empty()
            {
                self.phase(PipelinePhase::Canary)
                    .run(async |job| self.canary(ctx, expected, job).await)
                    .await?;
            }
            let mut stages: Stages = self
                .phase(PipelinePhase::Extract)
                .run(async |_| {
                    tests
                        .iter()
                        .map(|test| self.extract(ctx, test))
                        .collect::<TryJoinAll<_>>()
                        .await
                })
                .await?
                .into_iter()
                .collect();
            if ctx.options.no_verify() {
                return Ok(stages);
            }

            let Some(first) = batches.first() else {
                return Ok(stages);
            };
            // The tests whose next stage is `stage`: the candidates of that batch.
            let below = |stages: &Stages, stage: Stage| -> Vec<&'a FlsTest> {
                tests
                    .iter()
                    .filter(|test| stages.get(&test.path).and_then(|s| s.next()) == Some(stage))
                    .copied()
                    .collect()
            };

            let ready = below(&stages, *first);
            if ready.is_empty() {
                return Ok(stages);
            }
            self.phase(PipelinePhase::Prepare)
                .run(async |job| self.steps.prepare(ctx, &ready, job).await)
                .await?;

            for stage in batches {
                let candidates = below(&stages, stage);
                let failed = self
                    .phase(PipelinePhase::Batch(stage))
                    .run(async |job| self.steps.run_batch(ctx, stage, &candidates, job).await)
                    .await?;
                for test in candidates {
                    if !failed.contains(&test.path) {
                        stages.insert(test.path.clone(), stage);
                    }
                }
            }
            Ok(stages)
        })
    }
}
