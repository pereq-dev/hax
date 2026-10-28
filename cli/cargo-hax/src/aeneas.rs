//! Lean backend: runs the charon + aeneas pipeline to translate Rust to Lean.
//! This bypasses the hax frontend exporter and engine entirely.
//!
//! The pipeline is:
//!   1. Run charon on the crate to produce an LLBC file
//!   2. Run aeneas (`-split-files -specs hax -subdir <LibName>/Extraction`) on
//!      the LLBC file to produce the Lean extraction under `<LibName>/Extraction/`
//!   3. Scaffold the Lean package around the extraction (see [`package`])

use hax_types::cli_options::*;
use hax_types::diagnostics::message::HaxMessage;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, process};

use super::tools;

pub(crate) mod package;

const BACKEND_DIR: &str = "lean";

// Charon flags that are rejected when passed by the user: aeneas is always
// run on the LLBC file hax chooses, so redirecting charon's output severs
// the charon→aeneas handoff.
const CHARON_RESERVED_FLAGS: &[&str] = &["--dest-file"];
// Aeneas flags that trigger a warning when passed by the user.
const AENEAS_WARN_FLAGS: &[&str] = &["-backend", "-dest", "-subdir", "-split-files"];

/// The provided methods the `CoreModels` structures declare as fields. Charon
/// drops a provided method the crate never calls, and aeneas then emits the
/// trait impl without that field, which Lean rejects (#2172); naming them as
/// translation roots keeps them. Must match the structures' fields exactly:
/// these are the only eleven methods the model crates give a default body.
/// (`Drop::drop` also has one, but an `impl Drop` cannot omit its only
/// method, so no instance is ever emitted without the field.)
const CHARON_DEFAULT_METHOD_ROOTS: &[&str] = &[
    // Reached by every `#[derive(Clone)]`, which never calls `clone_from`.
    "core::clone::Clone::clone_from",
    "core::cmp::PartialEq::ne",
    "core::cmp::PartialOrd::lt",
    "core::cmp::PartialOrd::le",
    "core::cmp::PartialOrd::gt",
    "core::cmp::PartialOrd::ge",
    // Only a crate with its own `impl Step` reaches these.
    "core::iter::range::Step::forward",
    "core::iter::range::Step::forward_unchecked",
    "core::iter::range::Step::backward",
    "core::iter::range::Step::backward_unchecked",
    // Only a crate with its own `impl RangeBounds` reaches this.
    "core::ops::range::RangeBounds::contains",
];

fn picks_roots(args: &[String]) -> bool {
    args.iter()
        .any(|arg| arg == "--start-from" || arg.starts_with("--start-from="))
}

/// Shell-split a user-supplied extra-args string, reporting a fatal error on
/// unmatched quotes. Returns an empty vector if `s` is `None`.
fn shell_split(s: Option<&str>, who: &str, message_format: MessageFormat) -> Vec<String> {
    let Some(s) = s else { return Vec::new() };
    match shlex::split(s) {
        Some(v) => v,
        None => {
            HaxMessage::GenericError {
                message: format!("could not parse --{who}-args (unmatched quote?): {s}"),
            }
            .report(message_format, None);
            std::process::exit(1);
        }
    }
}

/// The subset of `flags` that `user_args` re-specifies, matching both
/// `-flag` and `-flag=value`.
fn overridden_flags<'a>(user_args: &[String], flags: &[&'a str]) -> Vec<&'a str> {
    flags
        .iter()
        .copied()
        .filter(|&flag| {
            let prefix = format!("{flag}=");
            user_args
                .iter()
                .any(|arg| arg == flag || arg.starts_with(&prefix))
        })
        .collect()
}

/// Name the source(s) whose extra tool arguments carry the given flags:
/// the scenario's `<tool>-args` key, the `--<tool>-args` flag, or both.
fn args_source(
    scenario_args: &[String],
    cli_args: &[String],
    flags: &[&str],
    tool: &str,
) -> String {
    let in_scenario = !overridden_flags(scenario_args, flags).is_empty();
    let in_cli = !overridden_flags(cli_args, flags).is_empty();
    match (in_scenario, in_cli) {
        (true, false) => format!("the scenario's `{tool}-args`"),
        (false, true) => format!("--{tool}-args"),
        _ => format!("--{tool}-args and the scenario's `{tool}-args`"),
    }
}

/// Warn if the extra aeneas args (the scenario's `<tool>-args` key or the
/// `--<tool>-args` flag) re-specify a flag that the pipeline already sets
/// and relies on (e.g. those controlling where output is written). Such
/// flags are still forwarded (the tools keep the last occurrence), but
/// overriding them can break the layout the generated proof project
/// assumes.
fn warn_on_overridden_pipeline_flags(
    scenario_args: &[String],
    cli_args: &[String],
    pipeline_flags: &[&str],
    tool: &str,
    message_format: MessageFormat,
) {
    let combined: Vec<String> = scenario_args.iter().chain(cli_args).cloned().collect();
    let overridden = overridden_flags(&combined, pipeline_flags);
    if !overridden.is_empty() {
        HaxMessage::GenericWarning {
            message: format!(
                "{} re-specifies {tool} flag(s) the pipeline sets and relies on: {}. \
                 They are still forwarded, but this may break \
                 the extraction or the generated proof project.",
                args_source(scenario_args, cli_args, &overridden, tool),
                overridden.join(", ")
            ),
        }
        .report(message_format, None);
    }
}

/// Shell-quote arguments for display. Args containing spaces or shell
/// metacharacters (e.g. `{impl X for _}`, `register_tool(_hax)`,
/// `host.rustflags=["--cfg","hax"]`) are quoted so the printed line can be
/// pasted into a shell verbatim. Display-only: the real commands are
/// executed without a shell, so quoting never affects execution.
pub(crate) fn quoted(args: &[String]) -> String {
    shlex::try_join(args.iter().map(String::as_str)).unwrap_or_else(|_| args.join(" "))
}

/// Shell-quote one value for display, leaving a value that needs no
/// quoting untouched.
pub(crate) fn quoted_value(value: &str) -> String {
    shlex::try_quote(value)
        .map(|quoted| quoted.into_owned())
        .unwrap_or_else(|_| value.to_string())
}

/// Format a `Command` as a copy-pasteable, shell-quoted invocation for display.
fn format_command(cmd: &process::Command) -> String {
    let parts: Vec<String> = std::iter::once(cmd.get_program())
        .chain(cmd.get_args())
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    quoted(&parts)
}

/// Forward all aeneas output lines.
fn report_output(lines: &[String], message_format: MessageFormat) {
    for line in lines {
        HaxMessage::SubprocessOutput {
            prefix: "aeneas".into(),
            line: line.clone(),
        }
        .report(message_format, None);
    }
}

/// Forward aeneas error output, truncating if longer than 10 lines
/// unless verbose mode is on. Always writes the full log to a file.
fn report_error_output(
    lines: &[String],
    lean_dir: &Path,
    verbose: u8,
    message_format: MessageFormat,
) {
    const MAX_LINES: usize = 10;

    let show = if verbose > 0 {
        lines.len()
    } else {
        lines.len().min(MAX_LINES)
    };
    report_output(&lines[..show], message_format);

    if lines.len() > MAX_LINES {
        let log_path = lean_dir.join("aeneas-error.log");
        let _ = fs::write(&log_path, lines.join("\n"));
        if verbose == 0 {
            HaxMessage::OutputTruncated {
                prefix: "aeneas".into(),
                remaining: lines.len() - MAX_LINES,
                log_path,
            }
            .report(message_format, None);
        }
    }
}

/// Collect aeneas output lines from stdout and stderr, filtering out
/// progress bar escape sequences from stderr.
fn collect_output_lines(output: &process::Output) -> Vec<String> {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut lines: Vec<String> = Vec::new();

    for line in stdout.lines() {
        if !line.is_empty() {
            lines.push(line.to_string());
        }
    }
    for line in stderr.lines() {
        let stripped = line.trim();
        if stripped.is_empty() || stripped.contains("[?25l") || stripped.contains("[?25h") {
            continue;
        }
        lines.push(line.to_string());
    }

    lines
}
/// The directory of the crate being processed: the root package of the
/// current invocation.
fn crate_dir(project: &tools::project::ProjectContext) -> PathBuf {
    project
        .root_package
        .as_ref()
        .map(|package| package.dir.clone())
        .unwrap_or_else(|| std::env::current_dir().expect("Could not get current directory"))
}

/// Resolve the `project-files` key for the crate being processed: the
/// member-level value overrides the workspace-level one, consistent with
/// the tool version resolution order; the default is enabled.
pub fn project_files_enabled(project: &tools::project::ProjectContext) -> bool {
    let crate_dir = crate_dir(project);
    project
        .member_config(&crate_dir)
        .and_then(|config| config.project_files)
        .or_else(|| {
            project
                .workspace_config
                .as_ref()
                .and_then(|config| config.project_files)
        })
        .unwrap_or(true)
}

/// Runs the charon + aeneas pipeline for the `lean` backend.
/// Returns `true` if an error occurred.
///
/// A scenario run carries the package name and the `project-files` override
/// in `options`; both fall back to the flag-driven behavior.
pub fn run(
    options: &LeanOptions,
    output_dir: Option<PathBuf>,
    verbose: u8,
    message_format: MessageFormat,
    project: &tools::project::ProjectContext,
) -> bool {
    let package_name = options.scenario.package_name.clone();
    let project_files = options
        .scenario
        .project_files
        .unwrap_or_else(|| project_files_enabled(project));
    // Per-crate tool resolution: the crate being processed is the root
    // package of the current invocation.
    let crate_dir = crate_dir(project);
    let crate_name = project
        .root_package
        .as_ref()
        .map(|package| package.name.clone());

    let member = project.member_config(&crate_dir);
    let workspace = project.workspace_config.as_ref();
    let provide = |tool: &str| match tools::provide_tool(tool, member, workspace, message_format) {
        Ok(provided) => Some(provided),
        Err(message) => {
            HaxMessage::GenericError { message }.report(message_format, None);
            None
        }
    };
    // Charon finds `charon-driver` next to its own executable; providing
    // the tool guarantees the sibling is there. One tool at a time, so a
    // failure to provide the first does not download the second for a run
    // that is over.
    let Some(charon) = provide("charon") else {
        return true;
    };
    let Some(aeneas) = provide("aeneas") else {
        return true;
    };
    let charon = charon.executables["charon"].clone();
    // Keep aeneas's resolution: the generated lakefile pins the aeneas Lean
    // library to the matching version, reusing this resolution rather than
    // resolving aeneas a second time.
    let tools::Provided {
        executables: aeneas_executables,
        resolution: aeneas_resolution,
    } = aeneas;
    let aeneas = aeneas_executables["aeneas"].clone();

    // The Lean package is named after the scenario (a scenario run passes
    // the name in `options`) or the crate, and its library, module root
    // and directory carry the UpperCamelCase form of that name. An
    // unusable name is a configuration error, caught before any tool runs.
    let (pkg_name, lib_name, origin) = match (&package_name, &crate_name) {
        (Some(name), _) => (
            name.clone(),
            name.clone(),
            "the requested package name".to_string(),
        ),
        (None, Some(crate_name)) => (
            crate_name.clone(),
            hax_types::lean::to_camel_case(crate_name),
            format!("derived from the crate name `{crate_name}`"),
        ),
        (None, None) => {
            HaxMessage::GenericError {
                message: "the Lean package is named after the crate, and this \
                          invocation has no root package (a virtual workspace has \
                          none); run from the package's directory, select its manifest \
                          with `-C --manifest-path <path> ;`, or extract through a \
                          proof scenario, whose name determines the package name"
                    .to_string(),
            }
            .report(message_format, None);
            return true;
        }
    };
    if let Err(message) =
        package::validate_lib_name(&lib_name, &origin, package::core_models_extraction_mode())
    {
        HaxMessage::GenericError { message }.report(message_format, None);
        return true;
    }

    // Parse the user's aeneas flags up front: an overridden `-dest` or
    // `-subdir` means hax does not know the package layout, so everything
    // keyed to that location is skipped, that is the clearing of the
    // extraction directory, the `Assumptions/` wiring, the generation and
    // the checks of the package files, and the reporting of the extracted
    // files. For both tools, scenario-resolved arguments come first and are
    // taken verbatim (no shell splitting).
    let scenario_aeneas_args = &options.scenario.aeneas_args;
    let cli_aeneas_args = shell_split(options.aeneas_args.as_deref(), "aeneas", message_format);
    let mut user_aeneas_args = scenario_aeneas_args.clone();
    user_aeneas_args.extend(cli_aeneas_args.iter().cloned());
    let overridden_layout_flags = overridden_flags(&user_aeneas_args, &["-dest", "-subdir"]);
    let layout_overridden = !overridden_layout_flags.is_empty();
    let layout_warned = layout_overridden && project_files;
    if layout_warned {
        HaxMessage::GenericWarning {
            message: format!(
                "{} overrides {}, so hax does not know the Lean \
                 package layout: the package files are neither generated nor \
                 checked, and stale extraction files are not removed",
                args_source(
                    scenario_aeneas_args,
                    &cli_aeneas_args,
                    &overridden_layout_flags,
                    "aeneas"
                ),
                overridden_layout_flags.join(" and ")
            ),
        }
        .report(message_format, None);
    }
    let project_files = project_files && !layout_overridden;

    // Output directory layout:
    //   <lean_dir>/
    //     <LibName>.lean          <- root module (imports the extraction and the proofs)
    //     <LibName>/Extraction.lean <- imports the extraction modules (hax-owned,
    //                                rewritten on every extraction)
    //     <LibName>/Extraction/   <- Lean files produced by aeneas
    //     <LibName>/Assumptions/  <- models of external definitions, seeded
    //                                from the aeneas templates, then the user's
    //     <LibName>/Verification/ <- handwritten proofs, never touched by hax
    //     llbc/                   <- LLBC file produced by charon
    //     lakefile.toml           <- Lean project file
    //     lean-toolchain
    //     .gitignore
    let lean_dir = output_dir.unwrap_or_else(|| crate_dir.join("proofs").join(BACKEND_DIR));
    let out_dir = lean_dir.join(&lib_name).join("Extraction");
    let llbc_dir = lean_dir.join("llbc");

    // An overridden layout moves the output away from `out_dir`: creating
    // the default tree would leave an unused directory behind.
    if !layout_overridden && let Err(e) = fs::create_dir_all(&out_dir) {
        HaxMessage::GenericError {
            message: format!("failed to create output directory: {}", e),
        }
        .report(message_format, None);
        return true;
    }

    if let Err(e) = fs::create_dir_all(&llbc_dir) {
        HaxMessage::GenericError {
            message: format!("failed to create llbc directory: {}", e),
        }
        .report(message_format, None);
        return true;
    }
    // Named like the rustc crate, whose name a dashed cargo name yields
    // with the dashes replaced.
    let llbc_file = llbc_dir.join(format!(
        "{}.llbc",
        crate_name.as_deref().unwrap_or("output").replace('-', "_")
    ));

    // Parse once so we can both inspect and forward the user's charon flags.
    // Rejected before anything is deleted or run, so a pure usage error
    // leaves the previous run's artifacts intact.
    let scenario_charon_args = &options.scenario.charon_args;
    let cli_charon_args = shell_split(options.charon_args.as_deref(), "charon", message_format);
    let mut user_charon_args = scenario_charon_args.clone();
    user_charon_args.extend(cli_charon_args.iter().cloned());
    let reserved_charon_flags = overridden_flags(&user_charon_args, CHARON_RESERVED_FLAGS);
    if !reserved_charon_flags.is_empty() {
        HaxMessage::GenericError {
            message: format!(
                "{} overrides {}, which hax reserves: aeneas is always run on the \
                 LLBC file hax chooses, so redirecting charon's output would make \
                 the extraction read a stale or missing file",
                args_source(
                    scenario_charon_args,
                    &cli_charon_args,
                    &reserved_charon_flags,
                    "charon"
                ),
                reserved_charon_flags.join(" and ")
            ),
        }
        .report(message_format, None);
        return true;
    }

    // The charon→aeneas handoff is this file: remove any LLBC a previous run
    // left, so a charon run that produces none is caught by the existence
    // check below instead of feeding aeneas stale code.
    if let Err(e) = fs::remove_file(&llbc_file)
        && e.kind() != std::io::ErrorKind::NotFound
    {
        HaxMessage::GenericError {
            message: format!(
                "failed to remove the previous LLBC file {}: {}",
                llbc_file.display(),
                e
            ),
        }
        .report(message_format, None);
        return true;
    }

    // Running charon

    HaxMessage::Step {
        verb: "Running".to_string(),
        target: "charon".to_string(),
    }
    .report(message_format, None);
    // The unified item-selection keys, compiled to charon's selection
    // flags; the verbatim extra arguments follow them.
    let selection_flags = options.scenario.selection_flags();

    let mut charon_cmd = process::Command::new(&charon);
    charon_cmd.args([
        "cargo",
        "--preset=aeneas",
        "--dest-file",
        llbc_file.to_str().expect("non-UTF8 path"),
        // Compile the crate as hax does: `--cfg=hax_compilation` makes hax-lib proc
        // macros emit their verification artifacts, and `hax_backend_lean` follows the
        // engine's `hax_backend_<name>` convention, so a crate can scope a marker to
        // this backend with `#[cfg_attr(hax_backend_lean, hax_lib::opaque)]`.
        "--rustc-arg=--cfg=hax_compilation",
        "--rustc-arg=--cfg=hax_backend_lean",
    ]);
    // Naming a root suppresses charon's implicit `crate` one, so restore it
    // unless the caller picked its own roots.
    if !picks_roots(&selection_flags) && !picks_roots(&user_charon_args) {
        charon_cmd.args(["--start-from", "crate"]);
    }
    for root in CHARON_DEFAULT_METHOD_ROOTS {
        charon_cmd.args(["--start-from", root]);
    }
    // User-supplied charon flags go before the `--` cargo separator.
    charon_cmd.args(&selection_flags);
    charon_cmd.args(&user_charon_args);
    // Extra flags that charon will forward to cargo. Meant mostly for `hax_lib`.
    charon_cmd.args([
        "--",
        "-Zhost-config",
        "-Ztarget-applies-to-host",
        "--config",
        // Proc-macro crates are host artifacts: they take `host.rustflags`, not
        // `RUSTFLAGS`, so `anodized`'s flags have to be repeated here.
        r#"host.rustflags=["--cfg","hax","--cfg","hax_backend_lean","--cfg","anodized_hax","--cfg","anodized_discard_specs"]"#,
    ]);
    // Register the tool-attribute namespaces through `RUSTFLAGS`, which cargo applies
    // to every target crate. `--rustc-arg` only reaches the crate charon instruments,
    // so markers in dependencies would fail to resolve. `RUSTFLAGS` replaces (never
    // merges with) `build.rustflags` from `.cargo/config.toml`, so it must carry
    // `--cfg hax` itself: without it `hax-lib` builds its no-op `dummy` half and the
    // specifications the macros just emitted no longer resolve.
    charon_cmd.env(
        "RUSTFLAGS",
        format!(
            "{} -Zcrate-attr=feature(register_tool) \
             -Zcrate-attr=register_tool(_hax) -Zcrate-attr=register_tool(charon)",
            super::rustflags()
        ),
    );
    // Scenario feature selection goes to the cargo invocation charon drives.
    charon_cmd.args(&options.scenario.cargo_args);
    if verbose > 0 {
        HaxMessage::SubprocessOutput {
            prefix: "cmd".into(),
            line: format_command(&charon_cmd),
        }
        .report(message_format, None);
    }
    let charon_status = charon_cmd
        .current_dir(&crate_dir)
        .stderr(process::Stdio::inherit())
        .status();

    match charon_status {
        Ok(status) if status.success() => {}
        Ok(status) => {
            HaxMessage::GenericError {
                message: format!(
                    "charon exited with non-zero code {}",
                    status.code().unwrap_or(-1)
                ),
            }
            .report(message_format, None);
            return true;
        }
        Err(e) => {
            HaxMessage::GenericError {
                message: format!("failed to run charon: {}", e),
            }
            .report(message_format, None);
            return true;
        }
    }

    // The file was removed above, so its presence proves this charon run
    // wrote it.
    if !llbc_file.is_file() {
        HaxMessage::GenericError {
            message: format!(
                "charon exited successfully but did not produce {}",
                llbc_file.display()
            ),
        }
        .report(message_format, None);
        return true;
    }

    // Running Aeneas

    // The output-layout flags are reserved: overriding them moves the output
    // away from where the per-file report and the package scaffolding expect
    // it. `-dest` and `-subdir` are left out when the layout warning above
    // already named them, which it does more precisely.
    let aeneas_warn_flags: Vec<&str> = AENEAS_WARN_FLAGS
        .iter()
        .copied()
        .filter(|flag| !(layout_warned && matches!(*flag, "-dest" | "-subdir")))
        .collect();
    warn_on_overridden_pipeline_flags(
        scenario_aeneas_args,
        &cli_aeneas_args,
        &aeneas_warn_flags,
        "aeneas",
        message_format,
    );

    // Runs before the clearing below, which would destroy the files it
    // rescues. Like the clearing, the `Assumptions/` wiring is extraction
    // behavior, not scaffolding, so it stays active under
    // `project-files = false`.
    if !layout_overridden && package::rescue_external_files(&lean_dir, &lib_name, message_format) {
        return true;
    }

    // Snapshot the contents of the .lean files before aeneas runs, to report
    // each regenerated file as wrote or unchanged.
    let contents_before: HashMap<PathBuf, Vec<u8>> = if layout_overridden {
        HashMap::new()
    } else {
        fs::read_dir(&out_dir)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "lean"))
            .filter_map(|e| {
                let path = e.path();
                fs::read(&path).ok().map(|contents| (path, contents))
            })
            .collect()
    };

    // `Extraction/` is fully hax-owned: clear it before aeneas regenerates
    // it, so files the extraction no longer produces do not linger and
    // silently feed stale definitions into the build. A failed aeneas run
    // thus leaves the directory empty until the next successful one, losing
    // only regenerated artifacts: user content lives outside `Extraction/`
    // or was just rescued. A partial clearing would let stale definitions
    // feed the build, the very thing the clearing prevents, so a failure is
    // fatal.
    if !layout_overridden {
        let entries = match fs::read_dir(&out_dir) {
            Ok(entries) => entries,
            Err(e) => {
                HaxMessage::GenericError {
                    message: format!("failed to read {}: {}", out_dir.display(), e),
                }
                .report(message_format, None);
                return true;
            }
        };
        let mut clearing_failed = false;
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    HaxMessage::GenericError {
                        message: format!("failed to read an entry of {}: {}", out_dir.display(), e),
                    }
                    .report(message_format, None);
                    clearing_failed = true;
                    continue;
                }
            };
            let path = entry.path();
            let removed = if path.is_dir() {
                fs::remove_dir_all(&path)
            } else {
                fs::remove_file(&path)
            };
            if let Err(e) = removed {
                HaxMessage::GenericError {
                    message: format!("failed to remove {}: {}", path.display(), e),
                }
                .report(message_format, None);
                clearing_failed = true;
            }
        }
        if clearing_failed {
            return true;
        }
    }

    HaxMessage::Step {
        verb: "Running".to_string(),
        target: format!("aeneas on {}", llbc_file.display()),
    }
    .report(message_format, None);

    // We run aeneas with `-core-models-lib` so it uses hax's core models library
    // for the translation, `-split-files` so it emits the function and type files
    // (`Funs.lean`/`Types.lean`, and any proof-obligation / external-template
    // files) separately, and `-subdir <LibName>/Extraction` so they land in
    // `<lean_dir>/<LibName>/Extraction/` with import paths prefixed by
    // `<LibName>.Extraction.`. The files are Lean modules (aeneas's default),
    // like the hax Lean library they import.
    let subdir = format!("{lib_name}/Extraction");
    let mut aeneas_cmd = process::Command::new(&aeneas);
    aeneas_cmd.args([
        "-backend",
        "lean",
        "-core-models-lib",
        "-split-files",
        "-specs",
        "hax",
        llbc_file.to_str().expect("non-UTF8 path"),
        "-dest",
        lean_dir.to_str().expect("non-UTF8 path"),
        "-subdir",
        &subdir,
    ]);
    aeneas_cmd.args(&user_aeneas_args);
    if verbose > 0 {
        HaxMessage::SubprocessOutput {
            prefix: "cmd".into(),
            line: format_command(&aeneas_cmd),
        }
        .report(message_format, None);
    }
    let aeneas_output = aeneas_cmd.current_dir(&crate_dir).output();

    let output = match aeneas_output {
        Ok(output) => output,
        Err(e) => {
            HaxMessage::GenericError {
                message: format!("failed to run aeneas: {}", e),
            }
            .report(message_format, None);
            return true;
        }
    };

    let all_lines = collect_output_lines(&output);

    // Forward aeneas output (always on error, only in verbose mode on success)
    if !output.status.success() {
        report_error_output(&all_lines, &lean_dir, verbose, message_format);
    } else if verbose > 0 {
        report_output(&all_lines, message_format);
    }

    // Failures to write package or `Assumptions/` files are reported where
    // they occur and collected here: the run must not exit successfully
    // with an incomplete package.
    let mut package_error = false;

    // Wire the external definitions the extraction needs to the user's
    // models in `Assumptions/`, and rewrite the aggregate extraction module.
    // Both are extraction behavior like `Extraction/` itself, so they stay
    // active under `project-files = false`.
    if !layout_overridden && output.status.success() {
        package_error |= package::process_external_templates(&lean_dir, &lib_name, message_format);
        package_error |= package::write_extraction_module(&lean_dir, &lib_name, message_format);
    }

    // Report results

    // Report .lean files: "wrote" if new or its contents changed,
    // "unchanged" otherwise.
    if !layout_overridden && let Ok(entries) = fs::read_dir(&out_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "lean") {
                continue;
            }
            let Ok(contents) = fs::read(&path) else {
                continue;
            };
            let wrote = contents_before.get(&path) != Some(&contents);
            HaxMessage::ProducedFile {
                path: path.clone(),
                wrote,
            }
            .report(message_format, None);
        }
    }

    // The Lean package around the extraction: check an existing package's
    // pins on every run, create every missing package file, and check the
    // root module against the files on disk.
    if project_files {
        use tools::resolve::Resolved;
        let lean_toolchain = tools::provide_version("lean", member, workspace, message_format);
        let hax_lean_lib_rev =
            tools::provide_version("hax-lean-lib", member, workspace, message_format);
        // The aeneas Lean proof library must match the aeneas binary, so its
        // rev is the resolved aeneas version, reused from the resolution
        // that selected the binary above. A path-resolved aeneas has no
        // version hax can name: existing pins are not checked against it.
        let check_aeneas_rev = match &aeneas_resolution.kind {
            Resolved::Version(version) => Some(version.clone()),
            Resolved::Path(_) => None,
        };
        package::check_existing(
            &lean_dir,
            check_aeneas_rev.as_deref(),
            &lean_toolchain,
            &hax_lean_lib_rev,
            message_format,
        );
        // With no aeneas version to pin, the default is pinned instead;
        // `generate` warns about the substitution when it actually writes
        // a lakefile.
        let (aeneas_rev, aeneas_local_path) = match &aeneas_resolution.kind {
            Resolved::Version(version) => (version.clone(), None),
            Resolved::Path(path) => (
                tools::defaults::defaults().tools["aeneas"].clone(),
                Some(path.as_path()),
            ),
        };
        let pins = package::LakefilePins {
            aeneas_rev,
            lean_toolchain,
            hax_lean_lib_rev,
        };
        package_error |= package::generate(
            &lean_dir,
            &pkg_name,
            &lib_name,
            &pins,
            output.status.success(),
            aeneas_local_path,
            message_format,
        );
        // A failed extraction leaves `Extraction/` in a partial state the
        // root module cannot meaningfully be compared against.
        if output.status.success() {
            package::check_root_module(&lean_dir, &lib_name, message_format);
        }
    }

    if !output.status.success() {
        HaxMessage::GenericError {
            message: format!(
                "aeneas exited with non-zero code {}",
                output.status.code().unwrap_or(-1)
            ),
        }
        .report(message_format, None);
        return true;
    }

    package_error
}
