//! Orchestrates the execution of Hax backend integration tests.
//!
//! The binary discovers test modules, spawns extraction jobs for each enabled
//! backend, and reconciles the diagnostics that were observed with the
//! directives encoded in the test sources.

#![feature(rustc_private)]
#![warn(
    rustdoc::broken_intra_doc_links,
    unused_qualifications,
    unused_crate_dependencies
)]

use std::{
    collections::HashSet,
    env::set_current_dir,
    fs::{self},
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex},
};

use anyhow::{Context as _, Result, bail};
use clap::Parser;
use futures::future::TryJoinAll;
use hax_frontend_exporter::DefId;
use hax_types::cli_options::BackendName;

use crate::{
    directives::{ErrorCode, FailureKind},
    test_module::{TestModule, compute_test_modules},
};

mod cli;
mod commands;
mod directives;
mod fls;
mod helpers;
mod log;
mod promote_directives;
mod span_hint;
mod test_module;

use commands::*;
use helpers::BackendNameExt;
use log::*;

#[derive(Clone, Debug)]
/// Snapshot of the context necessary to run all tests for one backend.
struct BackendTestsContext {
    tests: Vec<TestModule>,
    backend: BackendName,
    options: cli::Cli,
}

#[derive(Clone, Debug)]
/// Per-test execution context.
struct BackendTestContext {
    test: TestModule,
    backend: BackendName,
    options: cli::Cli,
    haxmeta: PathBuf,
}

impl BackendTestContext {
    /// Executes the full backend workflow for a single test module.
    async fn run(self) -> Result<()> {
        let _permit = self.options.semaphore_permit_for_test().await?;
        let test = self.test.module_name.to_string();
        let out_dir = self
            .backend
            .job_kind(BackendJobKind::CargoHaxInto { test: test.clone() })
            .run(async |job| self.run_inner(job).await)
            .await?;
        self.move_to_snapshots_directory(&out_dir).await?;
        if self.test.needs_verification(self.backend, &self.options) {
            self.backend
                .job_kind(BackendJobKind::Verification { test })
                .run(async |job| self.run_verification(job).await)
                .await?;
        }
        Ok(())
    }

    /// Invokes `cargo hax into` for the current backend.
    async fn call_engine(&self, job: JobKind, out_dir: &Path) -> Result<HaxEngineOutput> {
        let (into_flags, backend_flags) = self.test.cli(self.backend)?;
        let output = hax_engine(
            &self.haxmeta,
            &self.test.module_name,
            out_dir,
            self.backend,
            &into_flags[..],
            &backend_flags[..],
        )
        .await?;
        output.report_stderr_and_stdout(job);
        Ok(output)
    }

    /// Writes diagnostics for the job and converts unexpected/missing
    /// diagnostics into user-facing errors.
    async fn report_results(
        &self,
        job: JobKind,
        exit_status: i32,
        unexpected_diagnostics: Option<Diagnostics>,
        missing_expected_diagnostics: Vec<(DefId, ErrorCode)>,
    ) -> Result<()> {
        let unexpected_diagnostics = unexpected_diagnostics
            .iter()
            .flat_map(|d| d.iter())
            .flat_map(OutMsg::render)
            .collect::<Vec<_>>();

        for rendered in &unexpected_diagnostics {
            job.report_message(rendered);
        }
        for (did, error_code) in &missing_expected_diagnostics {
            // TODO: add precise directive span, display nice error message
            job.report_message(format!(
                "expected error {error_code} on item `{did}`, got none",
                did = helpers::string_of_def_id(did)
            ));
        }

        const ERROR: bool = false;
        match (
            unexpected_diagnostics.is_empty(),
            missing_expected_diagnostics.is_empty(),
            exit_status == 0,
        ) {
            (ERROR, ERROR, _) => {
                bail!(
                    "unexpected extraction failure(s), and expected extraction failure(s) not found"
                )
            }
            (ERROR, _, _) => bail!("unexpected extraction failure(s)"),
            (_, ERROR, _) => bail!("expected extraction failure(s) not found"),
            _ => Ok(()),
        }
    }

    /// Path to snapshots directory
    async fn path_to_snapshots(&self) -> Result<PathBuf> {
        Ok(self
            .options
            .tests_crate_dir()
            .join("snapshots")
            .join(self.test.snapshot_dir(&self.options.tests_crate_dir())?)
            .join(self.test.snapshot_backend_dir(self.backend)))
    }

    /// Move the given output extraction directory to the canonical path the snapshots belongs to.
    async fn move_to_snapshots_directory(&self, out_dir: &Path) -> Result<()> {
        static DIRS: LazyLock<Mutex<HashSet<PathBuf>>> =
            LazyLock::new(|| Mutex::new(HashSet::new()));

        let path_to_snapshots = self.path_to_snapshots().await?;

        if !DIRS.lock().unwrap().insert(path_to_snapshots.clone()) {
            panic!(
                "Internal error: we are trying to output twice to {}",
                path_to_snapshots.display(),
            )
        }

        let parent = path_to_snapshots.parent().unwrap();
        for dir in TestModule::snapshot_backend_dirs(self.backend) {
            let previous = parent.join(dir);
            if previous.exists() {
                fs::remove_dir_all(&previous)?
            }
        }
        fs::create_dir_all(parent)?;
        fs::rename(out_dir, &path_to_snapshots)?;
        helpers::delete_sourcemaps(&path_to_snapshots)?;
        Ok(())
    }

    async fn run_inner(&self, job: JobKind) -> Result<PathBuf> {
        let out_dir = self
            .options
            .cache_dir()
            .join(&self.test.module_name)
            .join(self.backend.to_string());
        if out_dir.exists() {
            fs::remove_dir_all(&out_dir)?;
        }
        fs::create_dir_all(&out_dir)?;

        let engine_out = self.call_engine(job.clone(), &out_dir).await?;

        let mut diagnostics = engine_out.diagnostics();
        if engine_out.error_code != 0 && diagnostics.iter().next().is_none() {
            bail!(
                "the engine ended with a non-null exit code, but no diagnostic could be found. See stdout and stderr."
            )
        }

        let (unexpected_diagnostics, missing_expected_diagnostics) = {
            let missing_expected_diagnostics: Vec<_> = self
                .test
                .expected_diagnostics(self.backend, FailureKind::Extract)
                .into_iter()
                .filter(|(diag, error_code)| diagnostics.remove(diag, error_code).is_none())
                .collect();
            (diagnostics, missing_expected_diagnostics)
        };

        if self.options.promote_directives() {
            unexpected_diagnostics
                .add_directives_in_files(self.backend, &self.test.def_id)
                .await?;
        }

        self.report_results(
            job,
            engine_out.error_code,
            Some(unexpected_diagnostics),
            missing_expected_diagnostics,
        )
        .await?;
        Ok(out_dir)
    }

    /// Runs backend verification
    async fn run_verification(&self, job: JobKind) -> Result<()> {
        let dir = self.path_to_snapshots().await?;
        let output = match self.backend {
            BackendName::Fstar => run_fstar(true, dir.clone()).await?,
            BackendName::LegacyLean => run_lean(dir.clone()).await?,
            _ => unreachable!(),
        };
        if output.error_code != 0 {
            if self.options.promote_directives() {
                use crate::promote_directives::*;
                let kind = LineKind::Directive {
                    directive: directives::Directive::Item(directives::ItemDirective::Failure {
                        kind: FailureKind::Typecheck,
                        backends: std::iter::once((
                            self.backend,
                            vec![ErrorCode::new(output.error_code.to_string())],
                        ))
                        .collect(),
                    }),
                    bang: true,
                };
                push_line(&self.test.module_path, Line { line: 0, kind })?;
                // The directive just written is what decides the directory, and
                // it was not there when the snapshot was moved.
                let xfail = dir.with_file_name(format!("{}-xfail", self.backend));
                if xfail.exists() {
                    fs::remove_dir_all(&xfail)?
                }
                fs::rename(&dir, &xfail)?;
            }
            job.report_message(output.stderr);
            bail!("Type-checking failed")
        }
        self.report_results(job, output.error_code, None, Vec::new())
            .await?;
        Ok(())
    }
}

impl BackendTestsContext {
    /// Compute the haxmeta for this backend.
    async fn haxmeta(&self) -> Result<PathBuf> {
        self.backend
            .job_kind(BackendJobKind::CargoHaxSerialize)
            .run(|_| async {
                let original_path =
                    hax_serialize(&["-b", self.backend.to_string().as_str()]).await?;
                let path = self
                    .options
                    .cache_dir()
                    .join(format!("{}.haxmeta", self.backend));
                tokio::fs::rename(original_path, &path).await?;
                Ok(path)
            })
            .await
    }
    /// Executes every test module for the backend.
    async fn run(self) -> Result<()> {
        let haxmeta = self.haxmeta().await?;

        self.tests
            .iter()
            .cloned()
            .map(|test| BackendTestContext {
                test,
                backend: self.backend,
                options: self.options.clone(),
                haxmeta: haxmeta.clone(),
            })
            .map(async |context| tokio::spawn(context.run()).await)
            .collect::<TryJoinAll<_>>()
            .await?
            .into_iter()
            .collect::<Result<Vec<_>>>()?;

        Ok(())
    }
}

/// Backend disabled by default.
const DISABLED_BACKENDS: &[BackendName] = &[
    BackendName::Easycrypt,
    BackendName::Lean,
    BackendName::Rust,
    BackendName::GenerateRustEngineNames,
];

#[tokio::main]
async fn main() -> Result<()> {
    // Child processes may run in their own process groups, which the
    // terminal's SIGINT does not reach; dropping the run on a signal kills
    // them.
    let result = tokio::select! {
        result = run() => result,
        signal = interrupted() => Err(anyhow::anyhow!("interrupted by {signal}")),
    };
    flush().await;
    // Drift gets its own exit code, so that a caller can tell it from a
    // failure; clap uses 2 for usage errors.
    if let Err(e) = &result
        && e.is::<fls::refresh::Drift>()
    {
        eprintln!("Error: {e:?}");
        std::process::exit(3);
    }
    result
}

/// Waits for SIGINT or, on Unix, SIGTERM and returns its name.
async fn interrupted() -> &'static str {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        let Ok(mut terminate) = signal(SignalKind::terminate()) else {
            let _ = tokio::signal::ctrl_c().await;
            return "SIGINT";
        };
        tokio::select! {
            _ = tokio::signal::ctrl_c() => "SIGINT",
            _ = terminate.recv() => "SIGTERM",
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
        "SIGINT"
    }
}

async fn run() -> Result<()> {
    let mut options = cli::Cli::parse();
    options.canonicalize()?;

    set_current_dir(options.tests_crate_dir()).with_context(|| {
        format!(
            "Could not change directory to {}",
            options.tests_crate_dir().display()
        )
    })?;

    let cache_dir = options.cache_dir();
    tokio::fs::create_dir_all(&cache_dir)
        .await
        .with_context(|| {
            format!(
                "Could not create the cache directory {}",
                cache_dir.display()
            )
        })?;

    if let Some(cli::Command::Fls(command)) = options.command() {
        return fls::subcommand(&options, command.clone()).await;
    }

    // The FLS suite reads its tests from the files and never runs the frontend.
    if options.suite() == cli::Suite::Fls {
        return fls::run(options).await;
    }

    let tests = JobKind::ResolveTests
        .run(|_| compute_test_modules())
        .await?;

    let backends: Vec<_> = if let Some(backend) = options.backend() {
        vec![backend]
    } else {
        BackendName::iter()
            .filter(|backend| !DISABLED_BACKENDS.contains(backend))
            .collect()
    };

    let result = backends
        .iter()
        .copied()
        .map(async |backend| {
            let context = BackendTestsContext {
                tests: tests
                    .iter()
                    .filter(|test| !test.off().contains(&backend))
                    .filter(|test| options.selects(&test.module_name))
                    .cloned()
                    .collect(),
                options: options.clone(),
                backend,
            };
            let verification_count = context
                .tests
                .iter()
                .filter(|test| test.needs_verification(backend, &options))
                .count();
            backend
                .job_kind(BackendJobKind::NumberBackendJobs(
                    context.tests.len() + verification_count,
                ))
                .report_no_message();
            tokio::spawn(context.run()).await
        })
        .collect::<TryJoinAll<_>>()
        .await?
        .into_iter()
        .collect::<Result<Vec<_>>>();

    promote_directives::save()?;
    result?;

    Ok(())
}
