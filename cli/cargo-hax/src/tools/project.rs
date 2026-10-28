//! Project discovery: which `hax.toml` files apply to the current project.
//!
//! Discovery is bound to the Rust project, not the invocation directory:
//! hax asks Cargo for the workspace root and the member-crate roots and
//! reads `hax.toml` from exactly these directories. A `hax.toml` anywhere
//! else has no effect (and is warned about when it sits between the
//! invocation directory and the workspace root).

use std::path::{Path, PathBuf};

use hax_types::cli_options::MessageFormat;
use hax_types::diagnostics::message::HaxMessage;

use super::config::{self, HaxToml, ScenarioEntry};

/// The `-C <cargo-args> ;` arguments discovery must honor: they go to the
/// `cargo check` invocation that drives the frontend, so they decide which
/// manifest and which crates a run processes. Everything else in
/// `-C ... ;` is `cargo check`'s business alone.
#[derive(Debug, Default)]
pub(crate) struct CargoArgs {
    /// `--manifest-path`, verbatim (Cargo resolves a relative value
    /// against the invocation directory, and so does `cargo metadata`).
    manifest_path: Option<String>,
    /// `--target-dir`, verbatim. It beats the `CARGO_TARGET_DIR`
    /// environment variable, so a run that passes it builds there.
    pub(crate) target_dir: Option<String>,
    /// The `-p`/`--package` values, verbatim. A value that names a
    /// workspace member lets the `hax-lib` gate check exactly the selected
    /// crates; any other spec form (paths, globs, `name@version`) is
    /// Cargo's to interpret and only marks the selection.
    package_specs: Vec<String>,
    /// Whether the invocation selects packages beyond plain `-p` values
    /// (`--workspace`, `--all`, `--exclude`). *Which* ones is Cargo's
    /// answer to give: `cargo metadata` takes no selection, and
    /// reconstructing Cargo's semantics here would only approximate them.
    broad_selection: bool,
    /// Arguments `cargo metadata` accepts too, forwarded so that discovery
    /// cannot contradict the build: `--offline` must not reach the network,
    /// `--locked`/`--frozen` must not rewrite `Cargo.lock`, and the feature
    /// selection can add or drop the `hax-lib` dependency.
    metadata_args: Vec<String>,
}

impl CargoArgs {
    pub(crate) fn parse(flags: &[String]) -> Self {
        const FORWARDED: &[&str] = &[
            "--offline",
            "--locked",
            "--frozen",
            "--all-features",
            "--no-default-features",
        ];
        const BROAD_SELECTION: &[&str] = &["--exclude", "--workspace", "--all"];
        let mut args = Self::default();
        let mut flags = flags.iter();
        while let Some(flag) = flags.next() {
            // Everything after a bare `--` goes to rustc, not to Cargo.
            if flag == "--" {
                break;
            }
            // A value is attached (`--flag=value`) or is the next argument.
            let name = flag.split('=').next().unwrap_or(flag);
            if flag == "--manifest-path" {
                args.manifest_path = flags.next().cloned();
            } else if let Some(path) = flag.strip_prefix("--manifest-path=") {
                args.manifest_path = Some(path.to_string());
            } else if flag == "--target-dir" {
                args.target_dir = flags.next().cloned();
            } else if let Some(path) = flag.strip_prefix("--target-dir=") {
                args.target_dir = Some(path.to_string());
            } else if flag == "-p" || flag == "--package" {
                if let Some(spec) = flags.next() {
                    args.package_specs.push(spec.clone());
                }
            } else if let Some(spec) = flag.strip_prefix("--package=") {
                args.package_specs.push(spec.to_string());
            } else if let Some(spec) = flag.strip_prefix("-p") {
                args.package_specs
                    .push(spec.strip_prefix('=').unwrap_or(spec).to_string());
            } else if BROAD_SELECTION.contains(&name) {
                args.broad_selection = true;
            } else if flag == "--features" || flag == "-F" {
                if let Some(value) = flags.next() {
                    args.metadata_args.extend([flag.clone(), value.clone()]);
                }
            } else if name == "--features"
                || (flag.starts_with("-F") && flag.len() > 2)
                || FORWARDED.contains(&name)
            {
                args.metadata_args.push(flag.clone());
            }
        }
        args
    }

    fn selects_packages(&self) -> bool {
        self.broad_selection || !self.package_specs.is_empty()
    }
}

/// One workspace member crate.
#[derive(Debug, Clone)]
pub struct MemberCrate {
    pub name: String,
    pub root: PathBuf,
    /// The member's own `hax.toml`, if it has one. `None` for the crate
    /// whose root is the workspace root itself: the workspace file covers it.
    pub config: Option<HaxToml>,
    /// The version of `hax-lib` this crate's own *direct* dependency edge
    /// resolved to, if it has one. A transitive-only `hax-lib` is ignored:
    /// it is not what this crate's annotations compile against.
    pub hax_lib: Option<String>,
}

/// The directory roots of the current project, as `cargo metadata` reports
/// them. Discovering them reads no `hax.toml`, so a command that writes one
/// is not blocked by a file that configuration loading rejects.
#[derive(Debug, Clone)]
pub struct ProjectLayout {
    pub workspace_root: PathBuf,
    pub member_roots: Vec<PathBuf>,
}

impl ProjectLayout {
    pub fn load(message_format: MessageFormat) -> Result<Self, String> {
        // The layout needs no resolve graph, and asking for one would make
        // discovery fail on a project whose dependencies do not resolve.
        let metadata = run_cargo_metadata(&CargoArgs::default(), Deps::Skip)?;
        let layout = Self {
            workspace_root: metadata.workspace_root.clone().into(),
            member_roots: metadata
                .packages
                .iter()
                .filter(|package| metadata.workspace_members.contains(&package.id))
                .map(package_root)
                .collect(),
        };
        warn_stray_hax_tomls(&layout.workspace_root, &layout.member_roots, message_format);
        Ok(layout)
    }
}

/// The `hax.toml` configuration of the current project.
#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub workspace_root: PathBuf,
    pub workspace_config: Option<HaxToml>,
    pub members: Vec<MemberCrate>,
    /// The package `cargo metadata` reports as the root of the current
    /// invocation, if any (a virtual workspace has none).
    pub root_package: Option<RootPackage>,
    /// Whether the invocation selects the packages to process itself, so
    /// that `root_package` is not what it processes. See
    /// [`CargoArgs::broad_selection`].
    pub selects_packages: bool,
    /// The plain `-p`/`--package` values of the invocation, when they are
    /// its only form of package selection; empty otherwise.
    pub package_specs: Vec<String>,
}

/// The package the current invocation processes.
#[derive(Debug, Clone)]
pub struct RootPackage {
    pub name: String,
    pub dir: PathBuf,
}

impl ProjectContext {
    /// The `hax.toml` of the member crate rooted at `dir`, for per-crate
    /// resolution.
    pub fn member_config(&self, dir: &Path) -> Option<&HaxToml> {
        self.members
            .iter()
            .find(|member| member.root == dir)
            .and_then(|member| member.config.as_ref())
    }

    /// The member names, joined for error messages.
    pub fn member_names(&self) -> String {
        self.members
            .iter()
            .map(|m| m.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// One scenario in scope, resolved to the member it extracts.
#[derive(Debug, Clone)]
pub struct ScopedScenario {
    pub entry: ScenarioEntry,
    /// The name of the member the scenario extracts.
    pub package: String,
    pub package_root: PathBuf,
    /// The `hax.toml` declaring the scenario, for messages.
    pub defined_in: PathBuf,
}

impl ProjectContext {
    /// Every scenario of the workspace, resolved to the member it
    /// extracts, regardless of the invocation directory: checks that must
    /// see scenarios a narrowed run filters out (output-directory
    /// collisions) run on this set. [`Self::in_invocation_scope`] narrows
    /// it to the invocation's scope.
    ///
    /// A member-level scenario extracts that member; its `package`, if
    /// given, must match. A workspace-level scenario requires `package`
    /// when the workspace has more than one member, and `package` must
    /// name a member. A member-level scenario shadows a same-named
    /// workspace-level one, with a warning; shadowed scenarios are
    /// excluded from every scope.
    pub fn all_scenarios(
        &self,
        message_format: MessageFormat,
    ) -> Result<Vec<ScopedScenario>, String> {
        let mut scope = Vec::new();
        for member in &self.members {
            let Some(config) = &member.config else {
                continue;
            };
            for entry in config.scenarios.values() {
                if let Some(package) = &entry.package
                    && package != &member.name
                {
                    return Err(format!(
                        "{}: scenario `{}` declares `package = \"{package}\"`, but a \
                         member-level scenario extracts the member declaring it \
                         (`{}`); drop the key or move the scenario to the \
                         workspace `hax.toml`",
                        config.path.display(),
                        entry.name,
                        member.name
                    ));
                }
                scope.push(ScopedScenario {
                    entry: entry.clone(),
                    package: member.name.clone(),
                    package_root: member.root.clone(),
                    defined_in: config.path.clone(),
                });
            }
        }

        if let Some(config) = &self.workspace_config {
            for entry in config.scenarios.values() {
                // A member-level scenario shadows a same-named
                // workspace-level one, consistent with the tool version
                // resolution order.
                if let Some(shadowing) = scope.iter().find(|s| s.entry.name == entry.name) {
                    HaxMessage::GenericWarning {
                        message: format!(
                            "scenario `{}` in {} is shadowed by the scenario of \
                             the same name in {}",
                            entry.name,
                            config.path.display(),
                            shadowing.defined_in.display()
                        ),
                    }
                    .report(message_format, None);
                    continue;
                }
                let member = match &entry.package {
                    Some(package) => self
                        .members
                        .iter()
                        .find(|m| &m.name == package)
                        .ok_or_else(|| {
                            format!(
                                "{}: scenario `{}` declares `package = \"{package}\"`, \
                                 which names no workspace member; the members are: {}",
                                config.path.display(),
                                entry.name,
                                self.member_names()
                            )
                        })?,
                    None => match &self.members[..] {
                        [member] => member,
                        _ => {
                            return Err(format!(
                                "{}: scenario `{}` needs a `package` key: the \
                                 workspace has more than one member ({})",
                                config.path.display(),
                                entry.name,
                                self.member_names()
                            ));
                        }
                    },
                };
                scope.push(ScopedScenario {
                    entry: entry.clone(),
                    package: member.name.clone(),
                    package_root: member.root.clone(),
                    defined_in: config.path.clone(),
                });
            }
        }

        Ok(scope)
    }

    /// Whether a scenario extracting `package` is in scope for this
    /// invocation: from the workspace root every scenario is; from inside a
    /// member crate (other than the crate rooted at the workspace root)
    /// only the scenarios extracting that member.
    pub fn in_invocation_scope(&self, package: &str) -> bool {
        self.workspace_scope()
            || self
                .root_package
                .as_ref()
                .is_some_and(|root| root.name == package)
    }

    /// Whether this invocation sees the whole workspace's scenarios, which
    /// is what the orphan-directory check needs to be meaningful.
    pub fn workspace_scope(&self) -> bool {
        self.root_package
            .as_ref()
            .is_none_or(|package| package.dir == self.workspace_root)
    }
}

impl ProjectContext {
    /// Discovery for an invocation that takes no Cargo arguments (the
    /// `tools` subcommands): the project of the invocation directory.
    pub fn load(message_format: MessageFormat) -> Result<Self, String> {
        Self::load_for(&[], message_format)
    }

    /// Discover the project with `cargo metadata` and load its `hax.toml`
    /// files. Parse warnings are reported immediately; a malformed file or
    /// a failing `cargo metadata` is an error.
    ///
    /// `cargo_flags` are the arguments given with `-C ... ;`, which is what
    /// the `cargo check` invocation is driven with: discovery must ask
    /// `cargo metadata` about the same manifest, and gate on the same
    /// crates, as that build processes.
    pub fn load_for(cargo_flags: &[String], message_format: MessageFormat) -> Result<Self, String> {
        let cargo_args = CargoArgs::parse(cargo_flags);
        let metadata = run_cargo_metadata(&cargo_args, Deps::Resolve)?;

        let workspace_root: PathBuf = metadata.workspace_root.clone().into();
        let workspace_config = load_hax_toml(&workspace_root, message_format)?;

        // The resolved version each package's direct `hax-lib` dependency
        // edge points to, from the resolve graph.
        let packages_by_id: std::collections::HashMap<_, _> = metadata
            .packages
            .iter()
            .map(|package| (&package.id, package))
            .collect();
        let direct_hax_lib = |id: &cargo_metadata::PackageId| -> Option<String> {
            let nodes = &metadata.resolve.as_ref()?.nodes;
            let node = nodes.iter().find(|node| &node.id == id)?;
            node.deps.iter().find_map(|dep| {
                // Only a normal dependency edge means this crate's own
                // annotations compile against `hax-lib`; a dev- or
                // build-dependency does not. `dep_kinds` is empty on very
                // old metadata formats, which predate the distinction, so
                // treat that as normal.
                let is_normal = dep.dep_kinds.is_empty()
                    || dep
                        .dep_kinds
                        .iter()
                        .any(|kind| kind.kind == cargo_metadata::DependencyKind::Normal);
                if !is_normal {
                    return None;
                }
                let package = packages_by_id.get(&dep.pkg)?;
                (package.name == "hax-lib").then(|| package.version.to_string())
            })
        };

        let mut members = Vec::new();
        for package in metadata
            .packages
            .iter()
            .filter(|p| metadata.workspace_members.contains(&p.id))
        {
            let root = package_root(package);
            let config = if root == workspace_root {
                None
            } else {
                load_hax_toml(&root, message_format)?
            };
            members.push(MemberCrate {
                name: package.name.clone(),
                root,
                config,
                hax_lib: direct_hax_lib(&package.id),
            });
        }

        let root_package = metadata.root_package().map(|package| RootPackage {
            name: package.name.clone(),
            dir: package_root(package),
        });
        let ctx = ProjectContext {
            workspace_root,
            workspace_config,
            members,
            root_package,
            selects_packages: cargo_args.selects_packages(),
            package_specs: if cargo_args.broad_selection {
                Vec::new()
            } else {
                cargo_args.package_specs
            },
        };
        ctx.warn_member_overrides(message_format);
        ctx.warn_stray_files(message_format);
        Ok(ctx)
    }

    /// Overriding members lose the workspace's single answer to "which tool
    /// versions does this project use": keep the divergence visible. Keyed
    /// on the entries' presence, not their effect.
    fn warn_member_overrides(&self, message_format: MessageFormat) {
        for member in &self.members {
            if let Some(config) = &member.config
                && config.has_entries()
            {
                HaxMessage::MemberToolOverrides {
                    crate_name: member.name.clone(),
                    path: config.path.clone(),
                    entries: config.entry_names(),
                }
                .report(message_format, None);
            }
        }
    }

    fn warn_stray_files(&self, message_format: MessageFormat) {
        let member_roots: Vec<PathBuf> = self.members.iter().map(|m| m.root.clone()).collect();
        warn_stray_hax_tomls(&self.workspace_root, &member_roots, message_format);
    }
}

/// Whether the resolve graph is needed. Skipping it (`--no-deps`) skips
/// dependency resolution, which needs neither the network nor a resolvable
/// dependency set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Deps {
    Resolve,
    Skip,
}

/// Run `cargo metadata` for the project of the invocation directory.
pub(crate) fn run_cargo_metadata(
    args: &CargoArgs,
    deps: Deps,
) -> Result<cargo_metadata::Metadata, String> {
    let mut command = cargo_metadata::MetadataCommand::new();
    if let Some(path) = &args.manifest_path {
        command.manifest_path(path);
    }
    if deps == Deps::Skip {
        command.no_deps();
    }
    command.other_options(args.metadata_args.clone());
    command.exec().map_err(|e| match e {
        cargo_metadata::Error::CargoMetadata { stderr } if args.manifest_path.is_none() => {
            format!(
                "`cargo metadata` failed (this command must be run inside a \
                 Cargo project):\n{}",
                stderr.trim()
            )
        }
        cargo_metadata::Error::CargoMetadata { stderr } => {
            format!("`cargo metadata` failed:\n{}", stderr.trim())
        }
        e => format!("`cargo metadata` failed: {e}"),
    })
}

fn package_root(package: &cargo_metadata::Package) -> PathBuf {
    package
        .manifest_path
        .parent()
        .expect("a Cargo.toml always has a parent directory")
        .into()
}

/// Warn about `hax.toml` files in directories between the invocation
/// directory and the workspace root that are not member-crate roots:
/// they have no effect, which is usually a misplaced file.
fn warn_stray_hax_tomls(
    workspace_root: &Path,
    member_roots: &[PathBuf],
    message_format: MessageFormat,
) {
    // `cargo metadata` reports canonical paths (symlinks resolved), so
    // canonicalize the invocation directory too; otherwise a symlinked
    // checkout (e.g. `/tmp` -> `/private/tmp` on macOS) makes the
    // `starts_with` and per-member comparisons below spuriously fail.
    let Ok(invocation_dir) = std::env::current_dir().and_then(std::fs::canonicalize) else {
        return;
    };
    if !invocation_dir.starts_with(workspace_root) {
        return;
    }
    for dir in invocation_dir.ancestors() {
        if dir == workspace_root {
            break;
        }
        let candidate = dir.join("hax.toml");
        if candidate.is_file() && !member_roots.iter().any(|root| root == dir) {
            HaxMessage::StrayHaxToml { path: candidate }.report(message_format, None);
        }
    }
}

/// Read and parse `<dir>/hax.toml`, treating absence as `None`. Parse
/// warnings are reported immediately; malformed contents are an error.
fn load_hax_toml(dir: &Path, message_format: MessageFormat) -> Result<Option<HaxToml>, String> {
    let path = dir.join("hax.toml");
    let contents = match std::fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("could not read {}: {e}", path.display())),
    };
    match config::parse(&path, &contents) {
        Ok((parsed, warnings)) => {
            for message in warnings {
                HaxMessage::HaxTomlWarning {
                    path: path.clone(),
                    message,
                }
                .report(message_format, None);
            }
            Ok(Some(parsed))
        }
        Err(message) => {
            HaxMessage::HaxTomlError {
                path: path.clone(),
                message,
            }
            .report(message_format, None);
            Err(format!("invalid {}", path.display()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(flags: &[&str]) -> CargoArgs {
        CargoArgs::parse(&flags.iter().map(|f| f.to_string()).collect::<Vec<_>>())
    }

    #[test]
    fn the_manifest_path_is_read_in_both_spellings() {
        for flags in [
            vec!["--manifest-path", "../a/Cargo.toml"],
            vec!["--manifest-path=../a/Cargo.toml"],
        ] {
            let args = parse(&flags);
            assert_eq!(args.manifest_path.as_deref(), Some("../a/Cargo.toml"));
            assert!(!args.selects_packages());
        }
    }

    #[test]
    fn the_target_dir_is_read_in_both_spellings() {
        for flags in [
            vec!["--target-dir", "../build"],
            vec!["--target-dir=../build"],
        ] {
            assert_eq!(parse(&flags).target_dir.as_deref(), Some("../build"));
        }
        // Arguments past a bare `--` are rustc's.
        assert_eq!(parse(&["--", "--target-dir", "../build"]).target_dir, None);
    }

    #[test]
    fn every_spelling_of_a_package_selection_is_noticed() {
        for flags in [
            vec!["-p", "app"],
            vec!["-papp"],
            vec!["--package=app"],
            vec!["--package", "a,b"],
            vec!["--workspace"],
            vec!["--all"],
            vec!["--workspace", "--exclude", "legacy"],
        ] {
            assert!(parse(&flags).selects_packages(), "{flags:?}");
        }
        // Arguments past a bare `--` are rustc's, and no other Cargo
        // argument selects packages.
        assert!(!parse(&["--", "-p", "app"]).selects_packages());
        assert!(!parse(&["--release", "--profile", "test"]).selects_packages());
    }

    #[test]
    fn plain_package_specs_are_recorded() {
        for flags in [
            vec!["-p", "app"],
            vec!["-papp"],
            vec!["-p=app"],
            vec!["--package=app"],
            vec!["--package", "app"],
        ] {
            assert_eq!(parse(&flags).package_specs, ["app"], "{flags:?}");
        }
        let args = parse(&["--workspace", "-p", "app"]);
        assert!(args.broad_selection);
        assert_eq!(args.package_specs, ["app"]);
    }

    #[test]
    fn only_arguments_cargo_metadata_shares_are_forwarded_to_it() {
        let args = parse(&[
            "--offline",
            "--no-default-features",
            "--release",
            "--target-dir",
            "/tmp/x",
        ]);
        assert_eq!(args.metadata_args, ["--offline", "--no-default-features"]);
    }

    #[test]
    fn the_feature_selection_is_forwarded_with_its_values() {
        let args = parse(&["--features", "fast,net", "--release"]);
        assert_eq!(args.metadata_args, ["--features", "fast,net"]);
        assert_eq!(
            parse(&["--features=fast"]).metadata_args,
            ["--features=fast"]
        );
        assert_eq!(parse(&["-F", "fast"]).metadata_args, ["-F", "fast"]);
        assert_eq!(parse(&["-Ffast"]).metadata_args, ["-Ffast"]);
    }
}
