use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use anyhow::{Context, Result, bail};
use hax_frontend_exporter::{DefId, DefKind};
use hax_types::{cli_options::BackendName, driver_api::HaxMeta};

use crate::{
    commands::hax_serialize,
    directives::{Directive, ErrorCode, FailureKind, ItemDirective, TestDirective},
    helpers, span_hint,
};

#[derive(Clone, Debug)]
/// Representation of a test module extracted from the Hax metadata.
pub struct TestModule {
    pub module_name: String,
    pub module_path: PathBuf,
    pub def_id: DefId,
    pub test_directives: Vec<TestDirective>,
    pub items: Vec<(DefId, Vec<ItemDirective>)>,
}

impl TestModule {
    /// Returns the diagnostics that should be emitted for the provided backend
    /// and failure kind.
    pub fn expected_diagnostics(
        &self,
        backend: BackendName,
        kind: FailureKind,
    ) -> Vec<(DefId, ErrorCode)> {
        self.items
            .iter()
            .flat_map(|(def_id, directives)| {
                directives.iter().flat_map(|directive| match directive {
                    ItemDirective::Failure { kind: k, backends } if *k == kind => {
                        Some(backends.iter().filter_map(|(b, errors)| {
                            if backend == *b {
                                Some(errors.iter().map(|e| (def_id.clone(), e.clone())))
                            } else {
                                None
                            }
                        }))
                    }
                    _ => None,
                })
            })
            .flatten()
            .flatten()
            .collect()
    }

    /// Resolves the optional `@cli` directive attached to the module for a given backend name.
    pub fn cli(&self, backend: BackendName) -> Result<(Vec<String>, Vec<String>)> {
        Ok(self
            .test_directives
            .iter()
            .filter_map(|directive| match directive {
                TestDirective::SetCli {
                    backend: b,
                    into_flags,
                    backend_flags,
                } if *b == backend => Some((into_flags.clone(), backend_flags.clone())),
                _ => None,
            })
            .expect_le_one()
            .context("Multiple @cli flags for the same backend")?
            .unwrap_or_default())
    }
    /// Returns the set of backends disabled for this module.
    pub fn off(&self) -> HashSet<BackendName> {
        self.test_directives
            .iter()
            .filter_map(|directive| match directive {
                TestDirective::Off { backends } => Some(backends.iter()),
                _ => None,
            })
            .flatten()
            .copied()
            .collect()
    }

    /// Returns true if this test's output for `backend` is expected to
    /// type-check: the backend has a type-checker, and no directive says the
    /// test fails it.
    pub fn verification_expected(&self, backend: BackendName) -> bool {
        Self::has_type_checker(backend)
            && self
                .expected_diagnostics(backend, FailureKind::Typecheck)
                .is_empty()
    }

    /// Whether `backend` type-checks its output at all.
    fn has_type_checker(backend: BackendName) -> bool {
        matches!(
            backend,
            BackendName::Fstar | BackendName::LegacyLean | BackendName::ProVerif
        )
    }

    /// The subdirectory this test's `backend` snapshots go in: `<backend>`, or
    /// `<backend>-xfail` when a type-checker is expected to reject them.
    pub fn snapshot_backend_dir(&self, backend: BackendName) -> String {
        if Self::has_type_checker(backend) && !self.verification_expected(backend) {
            format!("{backend}-xfail")
        } else {
            backend.to_string()
        }
    }

    /// Every subdirectory this test's `backend` snapshots could occupy.
    pub fn snapshot_backend_dirs(backend: BackendName) -> Vec<String> {
        let name = backend.to_string();
        if Self::has_type_checker(backend) {
            vec![format!("{name}-xfail"), name]
        } else {
            vec![name]
        }
    }

    /// Returns true if verification needs to run for this test, i.e. if the backend supports verification,
    /// verification is not expected to fail, and `--no-verify` was not passed.
    pub fn needs_verification(&self, backend: BackendName, options: &crate::cli::Cli) -> bool {
        !options.no_verify() && self.verification_expected(backend)
    }

    /// The directory holding this test's snapshots, relative to
    /// `tests/snapshots` (e.g. `legacy/attributes/lib`). The per-backend
    /// snapshots live in a subdirectory named after the backend.
    pub fn snapshot_dir(&self, tests_crate_dir: &std::path::Path) -> Result<PathBuf> {
        let relative = self
            .module_path
            .strip_prefix(tests_crate_dir.join("src"))
            .context("internal error, cannot figure out relative path of test module")?;
        Ok(relative.with_file_name(
            relative
                .file_stem()
                .context("internal error, test module has no `*.rs` extension?")?,
        ))
    }
}

#[extension_traits::extension(trait ExtIntoIterator)]
impl<T, U: Iterator<Item = T>> U {
    /// Ensures that the iterator produces at most one element.
    fn expect_le_one(mut self) -> Result<Option<T>> {
        match (self.next(), self.next()) {
            (None, None) => Ok(None),
            (Some(first), None) => Ok(Some(first)),
            _ => bail!("expect_one_or_less: got 2 or more"),
        }
    }
}

/// Builds a map from definition identifiers to vectors of directives, out of the Rust crate in the current working directory.
/// This function also initializes span hints (see module [`span_hints`]): this function should thus be ran only once.
async fn compute_def_id_attrs() -> Result<HashMap<DefId, Vec<Directive>>> {
    let (haxmeta, _): (HaxMeta<()>, _) =
        HaxMeta::read(fs::File::open(&hax_serialize(&["--kind"]).await?)?);

    span_hint::init(&haxmeta.items)?;

    match haxmeta.items {
        hax_types::driver_api::Items::FullDef(full_defs) => full_defs
            .iter()
            .map(|item| {
                let directives = item
                    .attributes
                    .iter()
                    .map(crate::directives::directive_of_attribute)
                    .collect::<Result<Vec<Option<_>>>>()?
                    .into_iter()
                    .flatten();
                Ok((item.this.def_id.clone(), directives.collect()))
            })
            .collect(),
        hax_types::driver_api::Items::Legacy(items) => items
            .iter()
            .map(|item| {
                let directives = item
                    .attributes
                    .attributes
                    .iter()
                    .map(crate::directives::directive_of_attribute)
                    .collect::<Result<Vec<Option<_>>>>()?
                    .into_iter()
                    .flatten();
                Ok((item.owner_id.clone(), directives.collect()))
            })
            .collect(),
    }
}

/// Compute a list of test modules for the Rust crate of the current working directory.
pub async fn compute_test_modules() -> Result<Vec<TestModule>> {
    // A map from test module names to `TestModule`s.
    // A `TestModule` contains items.
    let mut tests: HashMap<String, TestModule> = HashMap::new();

    // A map from test module names to vectors of definition identifiers with their directives.
    let mut items: HashMap<String, Vec<(DefId, Vec<ItemDirective>)>> = HashMap::new();

    // For each `def_id`, we add at most an entry to `tests` or to `items`,
    // according to whether `def_id` refers to a test module or to an item.
    for (def_id, directives) in compute_def_id_attrs().await? {
        let Some(module_name) = helpers::module_name(&def_id) else {
            continue;
        };
        let (test_directives, item_directives) = Directive::partition(directives);
        let is_a_test_module = {
            let path_of_size_one = def_id.path.len() == 1;
            if path_of_size_one && def_id.kind != DefKind::Mod {
                // discard: this is from the prelude inserted by Rust
                continue;
            }
            path_of_size_one
        };
        if is_a_test_module {
            let span = span_hint::span_hint(&def_id)
                .await?
                .context("internal error: every test module item should have a span hint")?;

            let module_path = span
                .module_file
                .clone()
                .with_context(|| format!("internal error: every test module item should have a `module_file`. Found none for {:?}.", helpers::string_of_def_id(&def_id)))?.canonicalize()
                .context("internal error: cannot canonicalize module path")?;

            let test_module = TestModule {
                items: vec![(def_id.clone(), item_directives)],
                def_id,
                module_name: module_name.clone(),
                module_path,
                test_directives,
            };
            tests.insert(module_name, test_module);
        } else {
            let trying_to_add_a_test_directive_to_a_not_test = !test_directives.is_empty();
            if trying_to_add_a_test_directive_to_a_not_test {
                bail!(
                    "Item {def_id:#?} has a test-level directive, but it is not a test. A test *has* to exactly be a module whose path is of size one: e.g. `my_krate::correct` is good, `my_crate::foo::bad` is bad."
                )
            }
            // If the item `def_id` has no directive, it is useless to add an entry.
            if !item_directives.is_empty() {
                items
                    .entry(module_name)
                    .or_default()
                    .push((def_id, item_directives));
            }
        }
    }

    // Insert the non-test items in their respective `TestModule`s
    for (module_name, items) in items {
        let test_module = tests
            .get_mut(&module_name)
            .with_context(|| format!("Each item of a test crate is supposed to be contained in a top-level module in the crate. An item of name {module_name} seems to exists at top-level in the crate, but seems not to be a module."))?;
        for (def_id, directives) in items {
            test_module.items.push((def_id, directives))
        }
    }

    Ok(tests.into_values().collect())
}
