//! `cargo hax extract`: run the proof scenarios declared in `hax.toml`.
//!
//! A scenario is resolved into the same `Options` value a flag-driven
//! `cargo hax into` run parses, and executed by re-entering `cargo-hax`
//! through the `__json`/`DRIVER_HAX_FRONTEND_FULL_OPTS` mechanism: no argv
//! strings are synthesized, and the scenario's `env` entries are set on the
//! re-entered process, so every process a scenario run spawns (cargo
//! included) sees them. Scenarios run sequentially: cargo's target-directory
//! lock would serialize concurrent builds anyway, and interleaved tool
//! output would be unreadable.

use std::path::PathBuf;
use std::process;

use hax_types::cli_options::*;
use hax_types::diagnostics::message::HaxMessage;
use path_clean::PathClean;

use super::aeneas::{self, quoted};
use super::tools::config::{ScenarioBackend, ScenarioEntry};
use super::tools::defaults;
use super::tools::project::{ProjectContext, ScopedScenario};

/// A scenario resolved to everything a run needs.
struct ResolvedScenario {
    scoped: ScopedScenario,
    /// The scenario's output directory, absolute.
    output_dir: PathBuf,
    /// The effective opaque set: the inherited defaults (built-in,
    /// workspace, member; unless opted out) followed by the scenario's own
    /// patterns.
    opaque: Vec<String>,
}

impl ResolvedScenario {
    fn entry(&self) -> &ScenarioEntry {
        &self.scoped.entry
    }
    fn name(&self) -> &str {
        &self.scoped.entry.name
    }
}

/// Entry point for `cargo hax extract`. Returns the process exit code.
pub fn run(
    names: &[String],
    packages: &[String],
    cargo_flags: &[String],
    hermeticity: &CargoHermeticityOptions,
    dry_run: bool,
    verbose: u8,
    message_format: MessageFormat,
) -> i32 {
    match extract(
        names,
        packages,
        cargo_flags,
        hermeticity,
        dry_run,
        verbose,
        message_format,
    ) {
        Ok(failed) => {
            if failed {
                1
            } else {
                0
            }
        }
        Err(message) => {
            HaxMessage::GenericError { message }.report(message_format, None);
            1
        }
    }
}

/// Run the selected scenarios. `Err` is a configuration or resolution
/// error before anything ran; `Ok(true)` means at least one scenario
/// failed.
fn extract(
    names: &[String],
    packages: &[String],
    cargo_flags: &[String],
    hermeticity: &CargoHermeticityOptions,
    dry_run: bool,
    verbose: u8,
    message_format: MessageFormat,
) -> Result<bool, String> {
    // `-C ... ;` drives a `cargo check` that `extract` never runs: each
    // scenario derives its cargo invocation from its own keys. Honoring the
    // flags only for discovery would silently apply them to half of the run.
    if !cargo_flags.is_empty() {
        return Err(
            "`-C ... ;` does not apply to `extract`: a scenario's cargo \
             invocation is derived from its `hax.toml` keys (`features`, \
             `env`, ...), and `--locked`/`--frozen`/`--offline` are flags of \
             `extract` itself; run `extract` from the project's directory"
                .to_string(),
        );
    }
    // The hermeticity flags start applying with the discovery this very
    // run performs.
    let project = ProjectContext::load_for(&hermeticity.flags(), message_format)?;
    // Output directories are checked across every scenario of the
    // workspace, before `-p` or the invocation directory narrows the
    // scope: a narrowed run must not silently clobber the output of a
    // scenario the narrowing filtered out.
    let all = resolve(&project, project.all_scenarios(message_format)?)?;
    check_output_dirs(&project, &all)?;
    let mut scope: Vec<&ResolvedScenario> = all
        .iter()
        .filter(|s| project.in_invocation_scope(&s.scoped.package))
        .collect();
    let full_scope = names.is_empty() && packages.is_empty();

    // `-p` narrows the scope; a package that does not exist is a typo.
    for package in packages {
        if !project.members.iter().any(|m| &m.name == package) {
            return Err(format!(
                "`-p {package}` names no workspace member; the members are: {}",
                project.member_names()
            ));
        }
    }

    if scope.is_empty() {
        return Err(
            "no scenarios in scope. Declare extraction configurations as \
             `[scenario.<name>]` tables in a `hax.toml` next to Cargo.toml \
             (workspace root or member crate); see the hax manual \
             (docs/manual/tools.md)"
                .to_string(),
        );
    }

    if !packages.is_empty() {
        scope.retain(|s| packages.contains(&s.scoped.package));
    }
    warn_scenario_less_layouts(&scope, message_format);

    if scope.is_empty() {
        return Err(format!(
            "no scenarios in scope extract the selected package(s) ({})",
            packages.join(", ")
        ));
    }

    // Select the scenarios to run. A name selects every scenario with that
    // name in scope; an unmatched name is an error and nothing runs. A
    // repeated name (or two names matching the same scenario) runs it once.
    let selected: Vec<&ResolvedScenario> = if names.is_empty() {
        scope.to_vec()
    } else {
        let mut selected: Vec<&ResolvedScenario> = Vec::new();
        let mut taken: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();
        for name in names {
            let matching: Vec<_> = scope
                .iter()
                .enumerate()
                .filter(|(_, s)| s.name() == name)
                .collect();
            if matching.is_empty() {
                let in_scope: std::collections::BTreeSet<_> =
                    scope.iter().map(|s| s.name()).collect();
                return Err(format!(
                    "no scenario named `{name}` in scope; the scenarios in \
                     scope are: {}",
                    in_scope.into_iter().collect::<Vec<_>>().join(", ")
                ));
            }
            // Keyed by position in `scope`, so a repeated name runs the
            // scenario once while the order stays the one the names give.
            for (index, scenario) in matching {
                if taken.insert(index) {
                    selected.push(*scenario);
                }
            }
        }
        selected
    };

    // Orphaned scenario directories (e.g. after a rename) are detected
    // only on full-scope runs: a narrowed scope proves nothing about a
    // directory no selected scenario owns. Checked against every scenario
    // of the workspace: a shared tree may be owned by a scenario of
    // another member, outside the invocation scope.
    if full_scope {
        warn_orphan_directories(&project, &all, message_format);
    }

    if dry_run {
        for scenario in &selected {
            HaxMessage::ScenarioDryRun {
                name: scenario.name().to_string(),
                package: scenario.scoped.package.clone(),
                lines: describe(scenario, hermeticity, verbose, message_format),
            }
            .report(message_format, None);
        }
        return Ok(false);
    }

    // Failures are collected rather than aborting the run, and reported in
    // a summary.
    let mut failed = Vec::new();
    for scenario in &selected {
        HaxMessage::Step {
            verb: "Extracting".to_string(),
            target: format!(
                "scenario `{}` (package `{}`)",
                scenario.name(),
                scenario.scoped.package
            ),
        }
        .report(message_format, None);
        if !execute(scenario, hermeticity, verbose, message_format) {
            failed.push(format!(
                "{} (package `{}`)",
                scenario.name(),
                scenario.scoped.package
            ));
        }
    }
    let any_failed = !failed.is_empty();
    HaxMessage::ScenarioSummary {
        total: selected.len(),
        failed,
    }
    .report(message_format, None);
    Ok(any_failed)
}

/// Resolve each scenario's output directory and effective opaque set.
fn resolve(
    project: &ProjectContext,
    scope: Vec<ScopedScenario>,
) -> Result<Vec<ResolvedScenario>, String> {
    scope
        .into_iter()
        .map(|scoped| {
            let entry = &scoped.entry;
            let relative = entry.output_dir.clone().unwrap_or_else(|| {
                ["proofs", &entry.name, &entry.backend.name()]
                    .iter()
                    .collect()
            });
            let output_dir = scoped.package_root.join(relative).clean();
            let opaque = effective_opaques(project, &scoped);
            Ok(ResolvedScenario {
                scoped,
                output_dir,
                opaque,
            })
        })
        .collect()
}

/// The effective opaque set of a scenario: the built-in default set,
/// extended by the workspace-level and member-level
/// `[scenario-defaults.<backend>]` tables (unless the scenario opts out
/// with `default-opaques = false`), followed by the scenario's own
/// patterns.
fn effective_opaques(project: &ProjectContext, scoped: &ScopedScenario) -> Vec<String> {
    let entry = &scoped.entry;
    let mut opaque = Vec::new();
    if entry.default_opaques {
        if let Some(defaults) = defaults::defaults().scenario_defaults.get(&entry.backend) {
            opaque.extend(defaults.opaque.iter().cloned());
        }
        for config in [
            project.workspace_config.as_ref(),
            project.member_config(&scoped.package_root),
        ] {
            if let Some(patterns) =
                config.and_then(|config| config.scenario_defaults.get(&entry.backend))
            {
                opaque.extend(patterns.iter().cloned());
            }
        }
    }
    opaque.extend(entry.opaque.iter().cloned());
    opaque
}

/// Verify that the workspace's scenarios resolve to distinct output
/// directories, none of which contains another: silent mutual clobbering
/// is the problem scenarios exist to solve, and a package nested inside
/// another package's tree would break the outer one's scaffolding checks.
/// No output directory may be a crate's root either: `output-dir` allows a
/// `..` component so several members can extract into one shared tree, and
/// a path that walks out of the declaring member and onto another one's
/// root would scaffold the generated Lean package over that crate's source
/// directory. For the same reason, every output directory must lie strictly
/// inside the workspace: the workspace root and anything outside it are not
/// hax's to scaffold over.
fn check_output_dirs(project: &ProjectContext, scope: &[ResolvedScenario]) -> Result<(), String> {
    for scenario in scope {
        if scenario.output_dir == project.workspace_root {
            return Err(format!(
                "the output directory of scenario `{}` (package `{}`) is the \
                 workspace root ({}); it would scaffold the generated Lean \
                 package over the workspace itself",
                scenario.name(),
                scenario.scoped.package,
                scenario.output_dir.display()
            ));
        }
        if !scenario.output_dir.starts_with(&project.workspace_root) {
            return Err(format!(
                "the output directory of scenario `{}` (package `{}`) ({}) \
                 lies outside the workspace ({}); a `..` component may only \
                 reach a shared tree within the workspace",
                scenario.name(),
                scenario.scoped.package,
                scenario.output_dir.display(),
                project.workspace_root.display()
            ));
        }
        if let Some(member) = project
            .members
            .iter()
            .find(|member| member.root == scenario.output_dir)
        {
            return Err(format!(
                "the output directory of scenario `{}` (package `{}`) is the \
                 root of package `{}` ({}); it would scaffold the generated \
                 Lean package over that crate's source directory",
                scenario.name(),
                scenario.scoped.package,
                member.name,
                member.root.display()
            ));
        }
    }
    for (i, a) in scope.iter().enumerate() {
        for b in &scope[i + 1..] {
            let describe =
                |s: &ResolvedScenario| format!("`{}` (package `{}`)", s.name(), s.scoped.package);
            if a.output_dir == b.output_dir {
                return Err(format!(
                    "scenarios {} and {} resolve to the same output directory \
                     {}; they would clobber each other",
                    describe(a),
                    describe(b),
                    a.output_dir.display()
                ));
            }
            let (outer, inner) = if b.output_dir.starts_with(&a.output_dir) {
                (a, b)
            } else if a.output_dir.starts_with(&b.output_dir) {
                (b, a)
            } else {
                continue;
            };
            return Err(format!(
                "the output directory of scenario {} ({}) lies inside the \
                 output directory of scenario {} ({})",
                describe(inner),
                inner.output_dir.display(),
                describe(outer),
                outer.output_dir.display()
            ));
        }
    }
    Ok(())
}

/// Warn about a scenario in scope whose output directory equals the
/// scenario-less `proofs/<backend>/` layout. Only a warning: the two never
/// run within one command, and pointing a single scenario at the old path
/// is a reasonable migration step.
fn warn_scenario_less_layouts(scope: &[&ResolvedScenario], message_format: MessageFormat) {
    for scenario in scope {
        let scenario_less = scenario
            .scoped
            .package_root
            .join("proofs")
            .join(scenario.entry().backend.name());
        if scenario.output_dir == scenario_less {
            HaxMessage::GenericWarning {
                message: format!(
                    "scenario `{}` writes to {}, the layout of scenario-less \
                     `cargo hax into` invocations of package `{}`",
                    scenario.name(),
                    scenario.output_dir.display(),
                    scenario.scoped.package
                ),
            }
            .report(message_format, None);
        }
    }
}

/// Warn about directories directly below a member's `proofs/` that no
/// resolved output directory of the workspace accounts for and that are
/// not a backend directory of the scenario-less layout: typically the
/// leftovers of a renamed scenario, including `Verification/` content.
/// `all` must hold every scenario of the workspace, not the invocation
/// scope: a directory of one member may be the shared tree of a scenario
/// declared by another member.
fn warn_orphan_directories(
    project: &ProjectContext,
    all: &[ResolvedScenario],
    message_format: MessageFormat,
) {
    let members: Vec<_> = if project.workspace_scope() {
        project.members.iter().collect()
    } else {
        // A member-scope run still checks against all scenarios of the
        // workspace, so its `proofs/` directory can be checked soundly.
        let member = &project
            .root_package
            .as_ref()
            .expect("a non-workspace scope has a root package")
            .dir;
        project
            .members
            .iter()
            .filter(|m| &m.root == member)
            .collect()
    };
    for member in members {
        let proofs = member.root.join("proofs");
        let entries = match std::fs::read_dir(&proofs) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let dir = entry.path();
            if !dir.is_dir() {
                continue;
            }
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if BackendName::iter().any(|backend| backend.to_string() == name) {
                continue;
            }
            let accounted = all
                .iter()
                .any(|s| s.output_dir.starts_with(&dir) || dir.starts_with(&s.output_dir));
            if !accounted {
                HaxMessage::GenericWarning {
                    message: format!(
                        "{} lies under no output directory of a scenario in \
                         scope; if it belongs to a renamed or removed scenario, \
                         move its handwritten content and delete it",
                        dir.display()
                    ),
                }
                .report(message_format, None);
            }
        }
    }
}

/// The cargo feature-selection arguments of a scenario.
fn feature_args(entry: &ScenarioEntry) -> Vec<String> {
    let mut args = Vec::new();
    if entry.all_features {
        args.push("--all-features".to_string());
    }
    if entry.no_default_features {
        args.push("--no-default-features".to_string());
    }
    if !entry.features.is_empty() {
        args.push("--features".to_string());
        args.push(entry.features.join(","));
    }
    args
}

/// Build the complete `Options` value a scenario run re-enters `cargo-hax`
/// with: the same value `cargo hax into` would have parsed, so the run
/// takes exactly the same code paths.
fn scenario_options(
    scenario: &ResolvedScenario,
    hermeticity: &CargoHermeticityOptions,
    verbose: u8,
    message_format: MessageFormat,
) -> Options {
    let entry = scenario.entry();
    // The arguments every cargo invocation of the run receives: the
    // scenario's feature selection and the invocation's hermeticity flags.
    let mut cargo_args = feature_args(entry);
    cargo_args.extend(hermeticity.flags());
    let backend = match entry.backend {
        ScenarioBackend::Lean => Backend::Lean(LeanOptions {
            charon_args: None,
            aeneas_args: None,
            scenario: LeanScenarioOptions {
                package_name: Some(aeneas::to_camel_case(&entry.name)),
                project_files: entry.project_files,
                include: entry.include.clone(),
                exclude: entry.exclude.clone(),
                opaque: scenario.opaque.clone(),
                charon_args: entry.charon_args.clone(),
                aeneas_args: entry.aeneas_args.clone(),
                cargo_args: cargo_args.clone(),
            },
        }),
        ScenarioBackend::Fstar => {
            // An absent key falls back to the flag's default.
            let defaults = FStarOptions::defaults();
            Backend::Fstar(FStarOptions {
                z3rlimit: entry.z3rlimit.unwrap_or(defaults.z3rlimit),
                fuel: entry.fuel.unwrap_or(defaults.fuel),
                ifuel: entry.ifuel.unwrap_or(defaults.ifuel),
                interfaces: entry.interfaces.clone(),
                line_width: entry.line_width.unwrap_or(defaults.line_width),
            })
        }
        ScenarioBackend::Coq => Backend::Coq,
        ScenarioBackend::Ssprove => Backend::Ssprove,
        ScenarioBackend::Easycrypt => Backend::Easycrypt,
        ScenarioBackend::Proverif => Backend::ProVerif(ProVerifOptions {
            assume_items: entry.assume_items.clone(),
        }),
    };
    let output_dir = match BackendName::from(entry.backend).output_subdir() {
        Some(subdir) => scenario.output_dir.join(subdir),
        None => scenario.output_dir.clone(),
    };
    // `cargo_flags` drive the frontend's `cargo check` and, on every
    // backend, the re-entered run's project discovery: naming the package
    // keeps the `hax-lib` gate on exactly the extracted crate, and the
    // feature selection and hermeticity flags keep the discovered
    // dependency graph in sync with the build. The lean pipeline
    // additionally passes them to the cargo invocation charon drives, via
    // `scenario.cargo_args` above.
    let mut cargo_flags = vec!["-p".to_string(), scenario.scoped.package.clone()];
    cargo_flags.extend(cargo_args);
    Options {
        cargo_flags,
        command: Command::Backend(BackendOptions {
            backend,
            dry_run: false,
            verbose,
            stats: false,
            profile: false,
            prune_haxmeta: None,
            debug_engine: None,
            extract_type_aliases: false,
            translation_options: TranslationOptions {
                include_namespaces: entry.select_clauses.clone(),
            },
            output_dir: Some(output_dir),
            cli_extension: extension::EmptyArgsExtension {},
        }),
        force_cargo_build: ForceCargoBuild::default(),
        deps: false,
        haxmeta: None,
        no_custom_target_directory: false,
        message_format,
        experimental_full_def: false,
        extension: extension::EmptyArgsExtension {},
    }
}

/// The resolved invocation of a scenario, as `--dry-run` prints it.
/// Every line is rendered from the constructed [`Options`] value the run
/// re-enters `cargo-hax` with, so the display cannot diverge from what
/// runs.
fn describe(
    scenario: &ResolvedScenario,
    hermeticity: &CargoHermeticityOptions,
    verbose: u8,
    message_format: MessageFormat,
) -> Vec<String> {
    let entry = scenario.entry();
    let options = scenario_options(scenario, hermeticity, verbose, message_format);
    let Command::Backend(backend_options) = &options.command else {
        unreachable!("scenario options always carry a backend command")
    };
    let output_dir = backend_options
        .output_dir
        .as_ref()
        .expect("scenario options always set an output directory");
    let mut lines = vec![
        format!("backend: {}", entry.backend),
        format!("directory: {}", scenario.scoped.package_root.display()),
        format!("output-dir: {}", output_dir.display()),
    ];
    if !entry.env.is_empty() {
        lines.push(format!(
            "env: {}",
            entry
                .env
                .iter()
                .map(|(var, value)| format!("{var}={}", aeneas::quoted_value(value)))
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }
    if !options.cargo_flags.is_empty() {
        lines.push(format!("cargo args: {}", quoted(&options.cargo_flags)));
    }
    match &backend_options.backend {
        Backend::Lean(lean) => {
            let scenario_opts = &lean.scenario;
            lines.push(format!(
                "lean package: {}",
                scenario_opts
                    .package_name
                    .as_deref()
                    .expect("scenario options always set the package name")
            ));
            let mut charon = scenario_opts.selection_flags();
            charon.extend(scenario_opts.charon_args.iter().cloned());
            if !charon.is_empty() {
                lines.push(format!("charon args: {}", quoted(&charon)));
            }
            if !scenario_opts.cargo_args.is_empty() {
                lines.push(format!(
                    "charon cargo args: {}",
                    quoted(&scenario_opts.cargo_args)
                ));
            }
            if !scenario_opts.aeneas_args.is_empty() {
                lines.push(format!(
                    "aeneas args: {}",
                    quoted(&scenario_opts.aeneas_args)
                ));
            }
        }
        backend => {
            let mut args = Vec::new();
            let clauses = &backend_options.translation_options.include_namespaces;
            if !clauses.is_empty() {
                args.push("-i".to_string());
                args.extend(clauses.iter().map(|clause| clause.to_string()));
            }
            match backend {
                Backend::Fstar(fstar) => args.extend(fstar.flags()),
                Backend::ProVerif(proverif) => args.extend(proverif.flags()),
                // The option-less backends, spelled out: one of them gaining
                // options must fail to compile here rather than being
                // silently dropped from the display.
                Backend::Coq | Backend::Ssprove | Backend::Easycrypt => {}
                // Lean is handled by the enclosing match; the rest are not
                // scenario backends.
                Backend::Lean(_)
                | Backend::LegacyLean
                | Backend::Rust
                | Backend::GenerateRustEngineNames
                | Backend::Debugger { .. } => {
                    unreachable!("scenarios construct no such backend")
                }
            }
            lines.push(format!(
                "backend args: {}",
                if args.is_empty() {
                    "(none)".to_string()
                } else {
                    quoted(&args)
                }
            ));
        }
    }
    lines
}

/// Run one scenario by re-entering `cargo-hax` through the `__json`
/// mechanism, in the extracted package's directory, with the scenario's
/// `env` entries set. Returns whether the run succeeded.
fn execute(
    scenario: &ResolvedScenario,
    hermeticity: &CargoHermeticityOptions,
    verbose: u8,
    message_format: MessageFormat,
) -> bool {
    let options = scenario_options(scenario, hermeticity, verbose, message_format);
    let exe = match std::env::current_exe() {
        Ok(exe) => exe,
        Err(e) => {
            HaxMessage::GenericError {
                message: format!("could not locate the cargo-hax executable: {e}"),
            }
            .report(message_format, None);
            return false;
        }
    };
    let mut command = process::Command::new(exe);
    command
        .arg("__json")
        .env(
            ENV_VAR_OPTIONS_FULL,
            serde_json::to_string(&options).expect("Options serialize to JSON"),
        )
        .envs(&scenario.entry().env)
        .current_dir(&scenario.scoped.package_root);
    match command.status() {
        Ok(status) => status.success(),
        Err(e) => {
            HaxMessage::GenericError {
                message: format!("failed to run scenario `{}`: {e}", scenario.name()),
            }
            .report(message_format, None);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::project::ScopedScenario;
    use std::path::Path;

    /// A scenario entry with every key set to a recognizable value. The
    /// struct literal is exhaustive, so a new `[scenario.<name>]` key must
    /// be added here, and the assertions below then force it to reach the
    /// `Options` a run re-enters with: a key that parses but is dropped on
    /// the way would silently change what is extracted.
    fn full_entry(backend: ScenarioBackend) -> ScenarioEntry {
        let clause = |s: &str| parse_inclusion_clause(s).unwrap();
        ScenarioEntry {
            name: "demo-scenario".to_string(),
            backend,
            package: Some("fixture".to_string()),
            output_dir: Some(PathBuf::from("proofs/custom")),
            features: vec!["fast".to_string()],
            all_features: true,
            no_default_features: true,
            env: [("K".to_string(), "V".to_string())].into_iter().collect(),
            include: vec!["fixture::included".to_string()],
            exclude: vec!["fixture::excluded".to_string()],
            opaque: vec!["fixture::opaque".to_string()],
            default_opaques: false,
            select_clauses: vec![clause("+**::selected")],
            z3rlimit: Some(11),
            fuel: Some(22),
            ifuel: Some(33),
            interfaces: vec![clause("+**::interfaced")],
            line_width: Some(44),
            charon_args: vec!["--charon-flag".to_string()],
            aeneas_args: vec!["-aeneas-flag".to_string()],
            project_files: Some(false),
            assume_items: vec![clause("+**::assumed")],
        }
    }

    /// `output_dir`, `opaque` and `default_opaques` are resolved before
    /// this point (by `resolve` and `effective_opaques`), so the resolved
    /// scenario carries their result rather than the raw keys.
    fn resolved(entry: ScenarioEntry) -> ResolvedScenario {
        let package_root = PathBuf::from("/ws/fixture");
        let opaque = entry.opaque.clone();
        ResolvedScenario {
            output_dir: package_root.join("proofs/custom"),
            opaque,
            scoped: ScopedScenario {
                package: entry.package.clone().expect("the entry names a package"),
                defined_in: package_root.join("hax.toml"),
                package_root,
                entry,
            },
        }
    }

    fn options_of(backend: ScenarioBackend) -> Options {
        scenario_options(
            &resolved(full_entry(backend)),
            &CargoHermeticityOptions {
                locked: true,
                offline: false,
                frozen: false,
            },
            0,
            MessageFormat::Human,
        )
    }

    fn backend_options(options: &Options) -> &BackendOptions<()> {
        let Command::Backend(backend) = &options.command else {
            panic!("a scenario always resolves to a backend command")
        };
        backend
    }

    /// The keys every backend shares: the package selection, the feature
    /// selection, and the invocation's hermeticity flags.
    #[test]
    fn the_shared_scenario_keys_reach_the_options() {
        let options = options_of(ScenarioBackend::Lean);
        let flags = options.cargo_flags.join(" ");
        assert!(flags.contains("-p fixture"), "{flags}");
        assert!(flags.contains("--all-features"), "{flags}");
        assert!(flags.contains("--no-default-features"), "{flags}");
        assert!(flags.contains("--features fast"), "{flags}");
        assert!(flags.contains("--locked"), "{flags}");
    }

    /// The Lean keys: the unified item selection, the verbatim tool
    /// arguments, the `project-files` override, and the package name
    /// derived from the scenario name.
    #[test]
    fn the_lean_scenario_keys_reach_the_options() {
        let options = options_of(ScenarioBackend::Lean);
        let backend = backend_options(&options);
        let Backend::Lean(lean) = &backend.backend else {
            panic!("the lean backend resolves to `Backend::Lean`")
        };
        let scenario = &lean.scenario;
        assert_eq!(scenario.package_name.as_deref(), Some("DemoScenario"));
        assert_eq!(scenario.project_files, Some(false));
        assert_eq!(scenario.include, ["fixture::included"]);
        assert_eq!(scenario.exclude, ["fixture::excluded"]);
        assert_eq!(scenario.opaque, ["fixture::opaque"]);
        assert_eq!(scenario.charon_args, ["--charon-flag"]);
        assert_eq!(scenario.aeneas_args, ["-aeneas-flag"]);
        // The Lean package sits at the top of the output directory.
        assert_eq!(
            backend.output_dir.as_deref(),
            Some(Path::new("/ws/fixture/proofs/custom"))
        );
    }

    /// The F* keys, and the `extraction/` level every engine backend keeps
    /// below the scenario's output directory.
    #[test]
    fn the_fstar_scenario_keys_reach_the_options() {
        let options = options_of(ScenarioBackend::Fstar);
        let backend = backend_options(&options);
        let Backend::Fstar(fstar) = &backend.backend else {
            panic!("the fstar backend resolves to `Backend::Fstar`")
        };
        assert_eq!(fstar.z3rlimit, 11);
        assert_eq!(fstar.fuel, 22);
        assert_eq!(fstar.ifuel, 33);
        assert_eq!(fstar.line_width, 44);
        let interfaces: Vec<_> = fstar.interfaces.iter().map(ToString::to_string).collect();
        assert_eq!(interfaces, ["+**::interfaced"]);
        let selected: Vec<_> = backend
            .translation_options
            .include_namespaces
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(selected, ["+**::selected"]);
        assert_eq!(
            backend.output_dir.as_deref(),
            Some(Path::new("/ws/fixture/proofs/custom/extraction"))
        );
    }

    /// The ProVerif key.
    #[test]
    fn the_proverif_scenario_keys_reach_the_options() {
        let options = options_of(ScenarioBackend::Proverif);
        let Backend::ProVerif(proverif) = &backend_options(&options).backend else {
            panic!("the proverif backend resolves to `Backend::ProVerif`")
        };
        let assumed: Vec<_> = proverif
            .assume_items
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(assumed, ["+**::assumed"]);
    }

    /// A scenario of `package`, extracting into `output_dir`.
    fn extracting_into(package: &str, output_dir: &str) -> ResolvedScenario {
        let package_root = Path::new("/ws").join(package);
        ResolvedScenario {
            output_dir: PathBuf::from(output_dir),
            opaque: vec![],
            scoped: ScopedScenario {
                entry: ScenarioEntry {
                    name: "demo".to_string(),
                    ..Default::default()
                },
                package: package.to_string(),
                defined_in: package_root.join("hax.toml"),
                package_root,
            },
        }
    }

    fn workspace_of(members: &[&str]) -> ProjectContext {
        ProjectContext {
            workspace_root: PathBuf::from("/ws"),
            workspace_config: None,
            members: members
                .iter()
                .map(|name| crate::tools::project::MemberCrate {
                    name: name.to_string(),
                    root: Path::new("/ws").join(name),
                    config: None,
                    hax_lib: None,
                })
                .collect(),
            root_package: None,
            selects_packages: false,
            package_specs: vec![],
        }
    }

    /// A crate root, of the declaring member or of another one, is rejected.
    #[test]
    fn an_output_directory_on_a_package_root_is_rejected() {
        let project = workspace_of(&["a", "b"]);
        for (case, output_dir) in [("a sibling member", "/ws/b"), ("its own member", "/ws/a")] {
            let err =
                check_output_dirs(&project, &[extracting_into("a", output_dir)]).expect_err(case);
            assert!(err.contains("root of package"), "{case}: {err}");
        }
        // A directory inside another member is not that member's root.
        check_output_dirs(&project, &[extracting_into("a", "/ws/b/proofs/demo")]).unwrap();
    }

    /// The workspace root, and anything outside the workspace, is rejected.
    #[test]
    fn an_output_directory_outside_the_workspace_is_rejected() {
        let project = workspace_of(&["a"]);
        let err = check_output_dirs(&project, &[extracting_into("a", "/ws")]).unwrap_err();
        assert!(err.contains("workspace root"), "{err}");
        let err =
            check_output_dirs(&project, &[extracting_into("a", "/elsewhere/proofs")]).unwrap_err();
        assert!(err.contains("outside the workspace"), "{err}");
    }
}
