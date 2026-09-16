//! The cargo package generated for each test, and the paths shared by every
//! backend of the suite.

use std::{fs, path::PathBuf};

use anyhow::{Context as _, Result, bail};

use super::test::FlsTest;
use crate::cli::Cli;

/// Paths shared by every backend of the suite.
pub struct FlsContext {
    pub options: Cli,
    /// Root of the repository, the parent of the tests crate.
    pub repo_root: PathBuf,
    /// `<cache-dir>/fls`, where generated packages and workspaces live.
    pub fls_cache: PathBuf,
    /// The `hax-lib` crate, a path dependency of every generated package.
    pub hax_lib: PathBuf,
    /// The `hax-test-macros` crate, a path dependency of every generated package.
    pub hax_test_macros: PathBuf,
}

const CARGO_TOML_TEMPLATE: &str = include_str!("../../templates/fls-package/Cargo.toml");

impl FlsContext {
    pub fn new(options: Cli) -> Result<Self> {
        let repo_root = options
            .tests_crate_dir()
            .parent()
            .context("the tests crate has no parent directory")?
            .to_path_buf();
        let hax_lib = repo_root.join("hax-lib");
        if !hax_lib.is_dir() {
            bail!(
                "expected `hax-lib` next to the tests crate, in {}",
                repo_root.display()
            );
        }
        let hax_test_macros = options.tests_crate_dir().join("hax-test-macros");
        if !hax_test_macros.is_dir() {
            bail!(
                "expected `hax-test-macros` in {}",
                options.tests_crate_dir().display()
            );
        }
        let fls_cache = options.cache_dir().join("fls");
        fs::create_dir_all(&fls_cache)?;
        Ok(Self {
            options,
            repo_root,
            fls_cache,
            hax_lib,
            hax_test_macros,
        })
    }

    pub fn package_dir(&self, test: &FlsTest) -> PathBuf {
        self.fls_cache.join(&test.path)
    }

    /// Generates the cargo package of a test from the template: the test file
    /// as `src/lib.rs` and the repository's `Cargo.lock`, so that every
    /// package resolves the dependencies of `hax-lib` identically.
    pub fn generate_package(&self, test: &FlsTest) -> Result<PathBuf> {
        let dir = self.package_dir(test);
        let src = dir.join("src");
        if src.exists() {
            fs::remove_dir_all(&src)?;
        }
        fs::create_dir_all(&src)?;
        let cargo_toml = CARGO_TOML_TEMPLATE
            .replace("{{name}}", &test.crate_name())
            .replace(
                "{{hax_lib}}",
                &hax_types::lean::toml_basic_string(&self.hax_lib.display().to_string()),
            )
            .replace(
                "{{hax_test_macros}}",
                &hax_types::lean::toml_basic_string(&self.hax_test_macros.display().to_string()),
            );
        fs::write(dir.join("Cargo.toml"), cargo_toml)?;
        fs::copy(&test.source, src.join("lib.rs"))?;
        fs::copy(self.repo_root.join("Cargo.lock"), dir.join("Cargo.lock"))?;
        Ok(dir)
    }
}
