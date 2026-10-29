//! The FLS suite: one test file per language construct under `fls/src/`, run
//! through each backend as a whole crate. The driver generates a cargo
//! package per test, observes how far the backend's pipeline gets, and
//! compares the observed stages with the results file committed for the
//! backend.

mod coverage;
mod exclusions;
mod ids;
mod lean;
mod package;
pub mod refresh;
mod results;
mod runner;
pub mod stage;
mod test;
mod triage;

use anyhow::{Result, ensure};
use hax_types::cli_options::BackendName;

use self::{
    package::FlsContext,
    results::{Committed, ResultsFile},
    runner::runner_for,
    stage::scale_of,
    test::FlsTest,
};
use crate::{
    cli::{Cli, FlsCommand},
    helpers::BackendNameExt,
    log::{BackendJobKind, JobKind},
};

/// Entry point of the `fls` subcommands.
pub async fn subcommand(options: &Cli, command: FlsCommand) -> Result<()> {
    match command {
        FlsCommand::Refresh { rev, check } => refresh::run(options, &rev, check),
        FlsCommand::Coverage => coverage::run(options).await,
        FlsCommand::Triage { chapter, all } => triage::run(options, &chapter, all).await,
    }
}

/// Entry point of the FLS suite.
pub async fn run(options: Cli) -> Result<()> {
    let ctx = FlsContext::new(options)?;
    let fls_dir = ctx.options.fls_dir();
    let tests = JobKind::ResolveTests
        .run(async |_| test::discover(&fls_dir))
        .await?;
    // A `--matching` value that names no test is most likely a typo.
    let paths: Vec<&str> = tests.iter().map(|test| test.path.as_str()).collect();
    let unmatched = ctx.options.unmatched_paths(&paths);
    ensure!(
        unmatched.is_empty(),
        "--matching selects no FLS test: {}",
        unmatched.join(", ")
    );

    // Without `--backend`, the suite runs the Lean backend only until an
    // engine runner exists.
    let backend = ctx.options.backend().unwrap_or(BackendName::Lean);
    // Every test with a tier for the backend has a table in the results
    // file; `--matching` narrows which of them are run.
    let with_tier: Vec<&FlsTest> = tests
        .iter()
        .filter(|test| test.tiers.contains_key(&backend))
        .collect();
    let selected: Vec<&FlsTest> = with_tier
        .iter()
        .filter(|test| ctx.options.selects_path(&test.path))
        .copied()
        .collect();
    backend
        .job_kind(BackendJobKind::NumberBackendJobs(selected.len()))
        .report_no_message();
    let results_job = backend.job_kind(BackendJobKind::Phase {
        name: "results".into(),
    });

    let runner = runner_for(&ctx, backend)?;
    // Read before the run, so that a malformed file fails before any tool
    // runs; `--no-verify` never compares with it.
    let committed = if ctx.options.no_verify() {
        None
    } else {
        Committed::read(&ctx, backend, scale_of(backend)?)?
    };
    let tools = runner.tool_versions(&ctx).await?;
    Committed::check_partial_update(
        committed.as_ref(),
        &ctx,
        &tools,
        selected.len() < with_tier.len(),
    )?;
    let stages = runner.run(&ctx, &selected).await?;
    results_job
        .run(async |job| {
            if ctx.options.no_verify() {
                job.report_message("results comparison skipped (--no-verify)");
                return Ok(());
            }
            let file = ResultsFile::merged(
                backend,
                committed.as_ref(),
                tools.clone(),
                &with_tier,
                &selected,
                &stages,
            )?;
            results::check(&ctx, &file, job).await
        })
        .await
}
