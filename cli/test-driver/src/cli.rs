//! Command-line interface helpers.

use anyhow::{Result, bail};
use clap::Parser;
use hax_types::cli_options::BackendName;
use std::{
    env::current_dir,
    fs::create_dir_all,
    path::{Path, PathBuf},
    sync::OnceLock,
};
use tokio::sync::{AcquireError, Semaphore, SemaphorePermit};

/// The test suites the driver can run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Suite {
    /// Module-granular tests under `src/legacy` and `src/rustc-coverage`, run
    /// through the hax frontend once per backend, with `@fail` directives.
    Legacy,
    /// Crate-granular tests under `fls/src`, one generated package per test,
    /// with support tiers and recorded pipeline stages.
    Fls,
}

#[derive(Debug, Clone, Parser)]
/// Test driver for hax.
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Root path of the crate that contains the integration tests.
    tests_crate_dir: PathBuf,
    /// Folder that stores intermediate artifacts produced by the
    /// driver.
    #[clap(long, short, default_value = current_dir().unwrap().join(".hax-tests-cache").into_os_string())]
    cache_dir: PathBuf,
    /// Maximum number of jobs that are allowed to execute concurrently.
    #[clap(long, short, default_value_t = num_cpus::get() * 2)]
    jobs: usize,
    /// The test suite to run.
    #[clap(long, value_enum, default_value_t = Suite::Legacy)]
    suite: Suite,
    /// Enabling this flag will add directives to the Rust source files so that
    /// the errors collected are marked as expected.
    /// This can be useful when adding a new backend to hax.
    /// Applies to the `legacy` suite only.
    #[clap(long)]
    promote_directives: bool,
    /// Rewrite the results files of the FLS suite from the observed stages
    /// instead of failing when they differ from the committed files.
    #[clap(long)]
    update_results: bool,
    /// Run tests only for the specified backend.
    #[clap(long, short)]
    backend: Option<BackendName>,
    /// Only run tests whose name contains the given string; in the FLS
    /// suite, the test at the given path or the tests under it.
    #[clap(long, short)]
    matching: Option<String>,
    /// Skip verification (F* / Lean type-checking) and only run extraction.
    #[clap(long)]
    no_verify: bool,
}

impl Cli {
    pub fn canonicalize(&mut self) -> Result<()> {
        match self.suite {
            Suite::Legacy if self.update_results => {
                bail!("`--update-results` applies to `--suite fls` only")
            }
            Suite::Fls if self.promote_directives => {
                bail!("`--promote-directives` applies to `--suite legacy` only")
            }
            Suite::Fls if self.update_results && self.no_verify => {
                bail!("`--update-results` needs the verified stages and excludes `--no-verify`")
            }
            _ => {}
        }
        self.prepare_folders()?;
        self.tests_crate_dir = self.tests_crate_dir.canonicalize()?;
        self.cache_dir = self.cache_dir.canonicalize()?;
        Ok(())
    }

    /// Returns the root of the crate that contains the integration tests.
    pub fn tests_crate_dir(&self) -> PathBuf {
        self.tests_crate_dir.canonicalize().unwrap()
    }
    /// Returns the folder that stores intermediate artifacts produced by the
    /// driver.
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }
    /// Maximum number of jobs that are allowed to execute concurrently.
    pub fn jobs(&self) -> usize {
        self.jobs
    }

    pub fn suite(&self) -> Suite {
        self.suite
    }

    pub fn promote_directives(&self) -> bool {
        self.promote_directives
    }

    pub fn update_results(&self) -> bool {
        self.update_results
    }

    pub fn backend(&self) -> Option<BackendName> {
        self.backend
    }

    pub fn matching(&self) -> Option<&str> {
        self.matching.as_deref()
    }

    /// Whether `--matching` selects the test at this path: no value given, or
    /// the value the path itself or one of its leading directories.
    pub fn selects_path(&self, path: &str) -> bool {
        self.matching
            .as_deref()
            .is_none_or(|m| path_matches(m, path))
    }

    pub fn no_verify(&self) -> bool {
        self.no_verify
    }

    /// Acquires a permit from the shared semaphore that limits concurrency.
    pub async fn semaphore_permit_for_test<'a>(
        &'a self,
    ) -> Result<SemaphorePermit<'a>, AcquireError> {
        static SEMAPHORE: OnceLock<Semaphore> = OnceLock::new();
        SEMAPHORE
            .get_or_init(|| Semaphore::new(self.jobs()))
            .acquire()
            .await
    }

    /// Creates the folder structure that the driver relies upon before running
    /// any work.
    pub fn prepare_folders(&self) -> Result<()> {
        if !self.tests_crate_dir().exists() {
            bail!("The test crate provided doesn't exist")
        }
        create_dir_all(self.cache_dir())?;
        Ok(())
    }
}

/// Whether `m` is the path itself or one of its leading directories.
fn path_matches(m: &str, path: &str) -> bool {
    path.strip_prefix(m.trim_end_matches('/'))
        .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_path_matches_whole_segments() {
        let cli = Cli::parse_from(["test-driver", "tests", "-m", "values/statics"]);
        assert!(cli.selects_path("values/statics/accepted"));
        assert!(!cli.selects_path("values/statics-and-more/accepted"));
        let cli = Cli::parse_from(["test-driver", "tests", "-m", "expressions/"]);
        assert!(cli.selects_path("expressions/loop-expressions/loop"));
    }
}
