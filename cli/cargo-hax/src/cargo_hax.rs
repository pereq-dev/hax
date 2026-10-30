use clap::Parser;
use colored::Colorize;
use hax_types::cli_options::*;
use hax_types::diagnostics::message::HaxMessage;
use hax_types::driver_api::*;
use hax_types::engine_api::*;
use is_terminal::IsTerminal;
use serde_jsonlines::BufReadExt;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::BufRead;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;

mod aeneas;
mod engine_debug_webapp;
mod scenario;
mod tools;
use hax_frontend_exporter::id_table;

/// Return a toolchain argument to pass to `cargo`: when the correct nightly is
/// already present, this is None, otherwise we (1) ensure `rustup` is available
/// (2) install the nightly (3) return the toolchain
fn toolchain() -> Option<&'static str> {
    let current_rustc_version = version_check::triple()
        .map(|(_, channel, date)| format!("{channel}-{date}"))
        .unwrap_or("unknown".into());
    if env!("HAX_RUSTC_VERSION") != current_rustc_version {
        const TOOLCHAIN: &str = env!("HAX_TOOLCHAIN");
        // ensure rustup is available
        which::which("rustup").ok().unwrap_or_else(|| {
            println!("Error: {} was not found, but toolchain {} is required while the current toolchain is {}\n\nExiting.", "rustup".bold(), TOOLCHAIN.bold(), current_rustc_version.bold());
            std::process::exit(1)
        });
        // make sure the toolchain is installed
        rustup_toolchain::install(TOOLCHAIN).unwrap();
        // return the correct toolchain
        Some(TOOLCHAIN)
    } else {
        None
    }
}

/// [`get_args`] is a wrapper of `std::env::args` that strips a possible
/// cargo subcommand. This allows for a binary `BINARY` to be called
/// both with `cargo BINARY args...` and `cargo-BINARY args...`.
pub fn get_args(subcommand: &str) -> Vec<String> {
    let mut args: Vec<_> = std::env::args().collect();
    if args.get(1) == Some(&subcommand.to_string()) {
        // we face a call `cargo [subcommand]`: we need to get rid of the first argument
        args = args.into_iter().skip(1).collect();
    }
    args
}

/// Our custom rustc driver will *not* be run in an proper terminal,
/// thus logs would appear uncolored. When no `RUST_LOG_STYLE` env. var.
/// is set, [`rust_log_style`] checks wether the `cargo hax` command was
/// run inside a terminal. If it was inside a terminal,
/// [`rust_log_style`] returns `"always"`, which is the usual default
/// behavior. Otherwise we return `"never"`. When [`RUST_LOG_STYLE`] is
/// set, we just return its value.
const RUST_LOG_STYLE: &str = "RUST_LOG_STYLE";
fn rust_log_style() -> String {
    std::env::var(RUST_LOG_STYLE).unwrap_or_else(|_| {
        if std::io::stderr().is_terminal() {
            "always".to_string()
        } else {
            "never".to_string()
        }
    })
}

/// The `cfg` names that hax uses: `hax`, `hax_backend_<name>`, the hax_lib-internal
/// `hax_compilation`, and the ones hax sets for `anodized`.
pub fn hax_cfg_names() -> impl Iterator<Item = String> {
    ["hax".to_string(), "hax_compilation".to_string()]
        .into_iter()
        .chain(
            BackendName::iter()
                .map(|backend| format!("hax_backend_{}", backend.to_string().replace('-', "_"))),
        )
        .chain(ANODIZED_CFG_NAMES.iter().map(|name| name.to_string()))
}

/// `--check-cfg` declarations for the cfg names that hax uses.
fn check_cfg_flags() -> String {
    hax_cfg_names()
        .map(|name| format!("--check-cfg cfg({name})"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// The `cfg` names that make `anodized`'s `#[spec(..)]` emit `hax_lib`
/// annotations. `anodized` reads them when its proc-macro crate is compiled.
const ANODIZED_CFG_NAMES: &[&str] = &["anodized_hax"];

/// Sets the `anodized` cfg names. They are declared by `check_cfg_flags`,
/// along with hax's own.
pub fn anodized_flags() -> String {
    ANODIZED_CFG_NAMES
        .iter()
        .map(|name| format!("--cfg {name}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// We set `cfg(hax)` so that client crates can include dependencies
/// or cfg-gate pieces of code. Moreover, we use `--check-cfg` to
/// suppress warnings about all cfg-names that hax uses.
const RUSTFLAGS: &str = "RUSTFLAGS";
pub fn rustflags() -> String {
    let rustflags = std::env::var(RUSTFLAGS).unwrap_or("".into());
    [
        rustflags,
        "--cfg hax".into(),
        check_cfg_flags(),
        anodized_flags(),
    ]
    .join(" ")
}

/// Find an external binary: check the given env var, then `PATH`.
fn find_binary(name: &str, env_var: &str, message_format: MessageFormat) -> PathBuf {
    std::env::var(env_var)
        .map(PathBuf::from)
        .or_else(|_| which::which(name))
        .unwrap_or_else(|_| {
            HaxMessage::BinaryNotFound {
                binary_name: name.into(),
                env_var: env_var.into(),
                hint: None,
            }
            .report(message_format, None);
            std::process::exit(2);
        })
}

const ENGINE_BINARY_NAME: &str = "hax-engine";
const ENGINE_BINARY_ENV: &str = "HAX_ENGINE_BINARY";

const RUST_ENGINE_BINARY_NAME: &str = "hax-rust-engine";
const RUST_ENGINE_BINARY_ENV: &str = "HAX_RUST_ENGINE_BINARY";

/// Dynamically looks for binary [ENGINE_BINARY_NAME].  First, we
/// check whether [HAX_ENGINE_BINARY] is set, and use that if it
/// is. Then, we try to find [ENGINE_BINARY_NAME] in PATH.
fn find_hax_engine(message_format: MessageFormat) -> process::Command {
    use which::which;

    std::env::var(ENGINE_BINARY_ENV)
        .ok()
        .map(process::Command::new)
        .or_else(|| which(ENGINE_BINARY_NAME).ok().map(process::Command::new))
        .unwrap_or_else(|| {
            let opam_ok = std::env::var("OPAM_SWITCH_PREFIX").is_ok();
            let opam_diag = if opam_ok {
                "opam seems okay ✓"
            } else {
                "opam seems not okay ❌"
            };
            HaxMessage::BinaryNotFound {
                binary_name: ENGINE_BINARY_NAME.into(),
                env_var: ENGINE_BINARY_ENV.into(),
                hint: Some(format!(
                    "With OPAM, `eval $(opam env)` is necessary for OPAM binaries to be in PATH: \
                     make sure to run `eval $(opam env)` before running `cargo hax`. \
                     (diagnostics: {})",
                    opam_diag
                )),
            }
            .report(message_format, None);
            std::process::exit(2);
        })
}

fn find_rust_hax_engine(message_format: MessageFormat) -> process::Command {
    process::Command::new(find_binary(
        RUST_ENGINE_BINARY_NAME,
        RUST_ENGINE_BINARY_ENV,
        message_format,
    ))
}

/// Runs `hax-engine`
fn run_engine(
    haxmeta: HaxMeta<hax_frontend_exporter::ThirBody>,
    id_table: id_table::Table,
    working_dir: Option<PathBuf>,
    manifest_dir: Option<PathBuf>,
    backend: &BackendOptions<()>,
    message_format: MessageFormat,
) -> bool {
    let engine_options = EngineOptions {
        hax_version: haxmeta.hax_version,
        backend: backend.clone(),
        input: haxmeta.items,
        impl_infos: haxmeta.impl_infos,
    };
    let mut hax_engine_command = match &engine_options.backend.backend {
        Backend::Coq | Backend::Ssprove | Backend::Easycrypt => find_hax_engine(message_format),
        Backend::Fstar(_) if matches!(&engine_options.input, Items::Legacy(_)) => {
            find_hax_engine(message_format)
        }
        _ => find_rust_hax_engine(message_format),
    };
    let mut engine_subprocess = hax_engine_command
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .inspect_err(|e| {
            if let std::io::ErrorKind::NotFound = e.kind() {
                panic!(
                    "The binary [{}] was not found in your [PATH].",
                    ENGINE_BINARY_NAME
                )
            }
        })
        .unwrap();

    let mut error = false;
    let mut output = Output {
        diagnostics: vec![],
        files: vec![],
        debug_json: vec![],
    };
    {
        let mut rctx = hax_types::diagnostics::report::ReportCtx::default();
        let mut stdin = std::io::BufWriter::new(
            engine_subprocess
                .stdin
                .as_mut()
                .expect("Could not write on stdin"),
        );

        macro_rules! send {
            ($value:expr) => {
                serde_json::to_writer(&mut stdin, $value).unwrap();
                stdin.write_all(b"\n").unwrap();
                stdin.flush().unwrap();
            };
        }

        id_table::WithTable::run(id_table, engine_options, |with_table| {
            send!(with_table);
        });

        let out_dir = backend.output_dir.clone().unwrap_or({
            let backend_name = BackendName::from(&backend.backend);
            let mut relative_path = PathBuf::from("proofs");
            relative_path.push(backend_name.to_string());
            relative_path.extend(backend_name.output_subdir());
            manifest_dir
                .map(|manifest_dir| manifest_dir.join(&relative_path))
                .unwrap_or(relative_path)
        });

        let stdout = std::io::BufReader::new(engine_subprocess.stdout.take().unwrap());
        let mut errors_per_item: HashMap<_, usize> = HashMap::new();
        for msg in stdout.json_lines() {
            let msg = msg.expect(
                "Hax engine sent an invalid json value. \
            This might be caused by debug messages on stdout, \
            which is reserved for JSON communication with cargo-hax",
            );
            use protocol::*;
            match msg {
                FromEngine::Exit => break,
                FromEngine::Diagnostic(diagnostic) => {
                    error = true;
                    if backend.dry_run {
                        output.diagnostics.push(diagnostic.clone())
                    }
                    if let Some(owner_id) = &diagnostic.owner_id {
                        *errors_per_item.entry(owner_id.clone()).or_default() += 1;
                    }
                    HaxMessage::Diagnostic {
                        diagnostic,
                        working_dir: working_dir.clone(),
                    }
                    .report(message_format, Some(&mut rctx));
                }
                FromEngine::File(file) => {
                    if backend.dry_run {
                        output.files.push(file)
                    } else {
                        let path = out_dir.join(&file.path);
                        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                        let mut wrote = false;
                        if fs::read_to_string(&path).as_ref().ok() != Some(&file.contents) {
                            std::fs::write(&path, file.contents).unwrap();
                            wrote = true;
                        }
                        if let Some(mut sourcemap) = file.sourcemap.clone() {
                            sourcemap.sourcesContent = sourcemap
                                .sources
                                .iter()
                                .map(PathBuf::from)
                                .map(|path| {
                                    if let Some(working_dir) = working_dir.as_ref()
                                        && path.is_relative()
                                    {
                                        working_dir.join(path).to_path_buf()
                                    } else {
                                        path
                                    }
                                })
                                .map(|path| fs::read_to_string(path).ok())
                                .collect();
                            let f = std::fs::File::create(path.with_file_name(format!(
                                "{}.map",
                                path.file_name().unwrap().to_string_lossy()
                            )))
                            .unwrap();
                            serde_json::to_writer(std::io::BufWriter::new(f), &sourcemap).unwrap()
                        }
                        HaxMessage::ProducedFile { path, wrote }.report(message_format, None)
                    }
                }
                FromEngine::DebugString(debug) => output.debug_json.push(debug),
                FromEngine::PrettyPrintDiagnostic(diag) => {
                    send!(&ToEngine::PrettyPrintedDiagnostic(format!("{}", diag)));
                }
                FromEngine::PrettyPrintRust(code) => {
                    let code = match syn::parse_file(&code) {
                        Ok(file) => match std::panic::catch_unwind(|| prettyplease::unparse(&file))
                        {
                            Ok(pp) => Ok(pp),
                            Err(err) => Err(format!("prettyplease panicked with: {:#?}", err)),
                        },
                        Err(err) => Err(format!("{}", err)),
                    };
                    send!(&ToEngine::PrettyPrintedRust(code));
                }
                FromEngine::ProfilingData(profiling_data) => {
                    HaxMessage::ProfilingData(profiling_data).report(message_format, None)
                }
                FromEngine::ItemProcessed(items) => {
                    for item in items {
                        errors_per_item.insert(item, 0);
                    }
                }
                FromEngine::Ping => {
                    send!(&ToEngine::Pong);
                }
            }
        }
        if backend.stats {
            HaxMessage::Stats {
                errors_per_item: errors_per_item.into_iter().collect(),
            }
            .report(message_format, None)
        }
        drop(stdin);
    }

    let exit_status = engine_subprocess.wait().unwrap();
    if !exit_status.success() {
        HaxMessage::HaxEngineFailure {
            exit_code: exit_status.code().unwrap_or(-1),
        }
        .report(message_format, None);
        std::process::exit(1);
    }

    if backend.dry_run {
        serde_json::to_writer(std::io::BufWriter::new(std::io::stdout()), &output).unwrap()
    }
    if !output.debug_json.is_empty() {
        use DebugEngineMode;
        let debug_json = &format!("[{}]", output.debug_json.join(","));
        match &backend.debug_engine {
            Some(DebugEngineMode::Interactive) => {
                eprintln!("----------------------------------------------");
                eprintln!("----------------------------------------------");
                eprintln!("----------------------------------------------");
                eprintln!("-- Engine debug mode. Press CTRL+C to exit. --");
                eprintln!("----------------------------------------------");
                eprintln!("----------------------------------------------");
                eprintln!("----------------------------------------------");
                engine_debug_webapp::run(|| debug_json.clone())
            }
            Some(DebugEngineMode::File(file)) if !backend.dry_run => {
                let mut file = file.open_or_stdout();
                write!(file, "{debug_json}").unwrap()
            }
            _ => (),
        }
    }

    error
}

/// Gets hax version: if hax is being compiled from a dirty git repo,
/// then this function taints the hax version with the hash of the
/// current executable. This makes sure cargo doesn't cache across
/// different versions of hax, for more information see
/// https://github.com/pereq-dev/hax/issues/801.
fn get_hax_version() -> String {
    let mut version = hax_types::HAX_VERSION.to_string();
    if env!("HAX_GIT_IS_DIRTY") == "true" {
        version += &std::env::current_exe()
            .ok()
            .and_then(|exe_path| std::fs::read(exe_path).ok())
            .map(|contents| {
                use std::hash::{DefaultHasher, Hash, Hasher};
                let mut s = DefaultHasher::new();
                contents.hash(&mut s);
                format!("hash-exe-{}", s.finish())
            })
            .expect("Expect read path")
    }

    version
}

/// Returns the path to the custom rustc driver used by cargo-hax.
///
/// This function retrieves the path of the current executable (i.e. `cargo-hax`), determines its
/// parent directory, and then appends the driver executable name `"driver-hax-frontend-exporter"` to it.
/// This path is used to locate the custom rustc driver that computes `haxmeta` files.
///
/// An installation that provides `cargo-hax` alone (e.g. `cargo install
/// cargo-hax`) has no driver: those commands that need one are unavailable, and
/// this reports so rather than letting `cargo` fail on a missing wrapper.
fn get_hax_rustc_driver_path(message_format: MessageFormat) -> PathBuf {
    let path = std::env::current_exe()
        .expect("Could not get the current executable path for `cargo-hax`.")
        .parent().expect("The executable `cargo-hax` is supposed to be a file, which is supposed to have a parent folder.")
        .join("driver-hax-frontend-exporter");
    if !path.exists() {
        HaxMessage::GenericError {
            message: "This command needs the hax frontend and engine, which a standalone \
                      `cargo-hax` installation does not provide: only `cargo hax into lean`, \
                      `cargo hax extract` for Lean scenarios, and `cargo hax tools` are \
                      available. To use the other commands, install the full toolchain as \
                      documented at https://github.com/pereq-dev/hax#for-all-backends."
                .into(),
        }
        .report(message_format, None);
        std::process::exit(2);
    }
    path
}

/// Parses and validates one `::hax-driver::`-prefixed line from cargo's
/// stderr. That stream is shared with everything the build runs (proc
/// macros, build scripts), so a report is only accepted when it names a
/// `.haxmeta` file of a workspace crate under this run's target directory
/// (`target_dir` must be canonical); anything else is an error, never a
/// trusted input or a panic.
fn accept_driver_message(
    msg: &str,
    target_dir: &Path,
    workspace_crates: &HashSet<String>,
) -> Result<EmitHaxMetaMessage, String> {
    let msg: HaxDriverMessage = serde_json::from_str(msg)
        .map_err(|e| format!("malformed hax driver message on cargo's stderr: {e}"))?;
    let HaxDriverMessage::EmitHaxMeta(data) = msg;
    let path = data.path.canonicalize().map_err(|e| {
        format!(
            "hax driver message names an unreadable haxmeta file {}: {e}",
            data.path.display()
        )
    })?;
    if !path.starts_with(target_dir) {
        return Err(format!(
            "hax driver message names a haxmeta file outside the target directory {}: {}",
            target_dir.display(),
            path.display()
        ));
    }
    // The driver names its output `<crate_name>-<cg_metadata>.haxmeta`.
    let crate_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| name.strip_suffix(".haxmeta"))
        .and_then(|stem| stem.rsplit_once('-'))
        .map(|(crate_name, _cg_metadata)| crate_name);
    match crate_name {
        Some(name) if workspace_crates.contains(name) => Ok(data),
        _ => Err(format!(
            "hax driver message names a haxmeta file that belongs to no workspace crate: {}",
            path.display()
        )),
    }
}

/// Calls `cargo` with a custom driver which computes `haxmeta` files
/// in `TARGET`. One `haxmeta` file is produced by crate. Each
/// `haxmeta` file contains the full AST of one crate.
fn compute_haxmeta_files(options: &Options) -> (Vec<EmitHaxMetaMessage>, i32) {
    let frontend_options = ExporterOptions::from(options);
    // Resolved before anything else, so that a missing driver is reported
    // instead of installing a toolchain for a run that cannot happen.
    let driver = get_hax_rustc_driver_path(options.message_format);
    // `cargo metadata` yields both the target directory the driver writes
    // into and the workspace members, the two facts the driver's reports
    // are validated against. It gets the `-C ... ;` flags it understands
    // (`--manifest-path`, feature selection, ...), so that it describes the
    // same workspace as the `cargo check` below.
    let cargo_args = tools::project::CargoArgs::parse(&options.cargo_flags);
    let report_error_and_abort = |message: String| {
        HaxMessage::GenericError { message }.report(options.message_format, None);
        (vec![], 1)
    };
    let metadata = match tools::project::run_cargo_metadata(&cargo_args, tools::project::Deps::Skip)
    {
        Ok(metadata) => metadata,
        Err(message) => return report_error_and_abort(message),
    };
    // An explicit `--target-dir` among the cargo flags beats the
    // `CARGO_TARGET_DIR` environment variable, so when one is given the
    // build writes there verbatim and no `hax` subdirectory can be imposed.
    let (target_dir, custom_target_dir) = match &cargo_args.target_dir {
        Some(dir) => (PathBuf::from(dir), false),
        None => {
            let mut dir: PathBuf = metadata.target_directory.clone().into();
            if !options.no_custom_target_directory {
                dir.push("hax");
            }
            (dir, !options.no_custom_target_directory)
        }
    };
    // The driver's reports are matched against the canonical target
    // directory, resolved once here. It is created first: on a fresh
    // project no build has made it yet, and canonicalization needs an
    // existing path.
    if let Err(e) = fs::create_dir_all(&target_dir) {
        return report_error_and_abort(format!(
            "could not create the target directory {}: {e}",
            target_dir.display()
        ));
    }
    let target_dir = match target_dir.canonicalize() {
        Ok(dir) => dir,
        Err(e) => {
            return report_error_and_abort(format!(
                "could not resolve the target directory {}: {e}",
                target_dir.display()
            ));
        }
    };
    // The driver names its haxmeta file after the rustc crate it compiled,
    // i.e. after the cargo target (lib, bins, ...), not after the package.
    let workspace_crates: HashSet<String> = metadata
        .packages
        .iter()
        .filter(|package| metadata.workspace_members.contains(&package.id))
        .flat_map(|package| &package.targets)
        .map(|target| target.name.replace('-', "_"))
        .collect();
    let mut cmd = {
        let mut cmd = process::Command::new("cargo");
        if let Some(toolchain) = toolchain() {
            cmd.env("RUSTUP_TOOLCHAIN", toolchain);
        }
        cmd.args(["check".into()].iter().chain(options.cargo_flags.iter()));
        const COLOR_FLAG: &str = "--color";
        let explicit_color_flag = options.cargo_flags.iter().any(|flag| flag == COLOR_FLAG);
        if !explicit_color_flag && std::io::stderr().is_terminal() {
            cmd.args([COLOR_FLAG, "always"]);
        }
        const MSG_FMT_FLAG: &str = "--message-format";
        let explicit_msg_fmt_flag = options.cargo_flags.iter().any(|flag| flag == MSG_FMT_FLAG);
        if !explicit_msg_fmt_flag && options.message_format == MessageFormat::Json {
            cmd.args([MSG_FMT_FLAG, "json"]);
        }
        cmd.stderr(std::process::Stdio::piped());
        if custom_target_dir {
            cmd.env("CARGO_TARGET_DIR", &target_dir);
        };
        cmd.env("RUSTC_WORKSPACE_WRAPPER", driver)
            .env(RUST_LOG_STYLE, rust_log_style())
            .env(RUSTFLAGS, rustflags())
            .env("HAX_CARGO_CACHE_KEY", get_hax_version())
            .env(
                ENV_VAR_OPTIONS_FRONTEND,
                serde_json::to_string(&frontend_options)
                    .expect("Options could not be converted to a JSON string"),
            );
        cmd
    };

    let mut child = cmd.spawn().unwrap();
    let mut channel_error = false;
    let mut haxmeta_files = {
        let mut haxmeta_files = vec![];
        let stderr = child.stderr.take().unwrap();
        let stderr = std::io::BufReader::new(stderr);
        for line in std::io::BufReader::new(stderr).lines() {
            let line = match line {
                Ok(line) => line,
                // A line the channel cannot represent (e.g. invalid UTF-8)
                // could hide a driver report: fail rather than risk a
                // silently incomplete extraction. The stream is still
                // drained, so the build is not blocked on a full pipe.
                Err(e) if e.kind() == std::io::ErrorKind::InvalidData => {
                    HaxMessage::GenericError {
                        message: format!("unreadable line on cargo's stderr: {e}"),
                    }
                    .report(options.message_format, None);
                    channel_error = true;
                    continue;
                }
                Err(e) => {
                    HaxMessage::GenericError {
                        message: format!("could not read cargo's stderr: {e}"),
                    }
                    .report(options.message_format, None);
                    channel_error = true;
                    break;
                }
            };
            if let Some(msg) = line.strip_prefix(HAX_DRIVER_STDERR_PREFIX) {
                match accept_driver_message(msg, &target_dir, &workspace_crates) {
                    Ok(data) => haxmeta_files.push(data),
                    Err(message) => {
                        HaxMessage::GenericError { message }.report(options.message_format, None);
                        channel_error = true;
                    }
                }
            } else {
                eprintln!("{}", line);
            }
        }
        haxmeta_files
    };

    let status = child
        .wait()
        .expect("`driver-hax-frontend-exporter`: could not start?");

    // A corrupt driver channel makes the collected set untrustworthy:
    // producing output from it would overwrite a previous good extraction
    // with a possibly truncated one.
    if channel_error {
        haxmeta_files.clear();
    }
    let exit_code = if !status.success() {
        HaxMessage::CargoBuildFailure.report(options.message_format, None);
        status.code().unwrap_or(254)
    } else if channel_error {
        1
    } else if haxmeta_files.is_empty() {
        HaxMessage::GenericError {
            message: "the build succeeded, but the hax driver reported no haxmeta file: \
                      the extraction would be empty"
                .into(),
        }
        .report(options.message_format, None);
        1
    } else {
        0
    };

    (haxmeta_files, exit_code)
}

/// Run the command given by the user
fn run_command(options: &Options, haxmeta_files: Vec<EmitHaxMetaMessage>) -> bool {
    match options.command.clone() {
        Command::JSON {
            output_file,
            kind,
            include_extra,
            use_ids,
            ..
        } => {
            with_kind_type!(kind, <Body>|| {
                for EmitHaxMetaMessage { path, .. } in haxmeta_files {
                    let (haxmeta, id_table): (HaxMeta<Body>, _) = HaxMeta::read(fs::File::open(&path).unwrap());
                    let dest = output_file.open_or_stdout();

                    (if include_extra {
                        let data = WithDefIds {
                            def_ids: haxmeta.def_ids,
                            impl_infos: haxmeta.impl_infos,
                            items: haxmeta.items,
                            comments: haxmeta.comments,
                        };
                        if use_ids {
                            id_table::WithTable::run(id_table, data, |with_table| {
                                serde_json::to_writer(dest, with_table)
                            })
                        } else {
                            serde_json::to_writer(dest, &data)
                        }
                    } else if use_ids {
                        id_table::WithTable::run(id_table, haxmeta.items, |with_table| {
                            serde_json::to_writer(dest, with_table)
                        })
                    } else {
                        serde_json::to_writer(dest, &haxmeta.items)
                    }
                  ).unwrap()
                }
            });
            false
        }
        Command::Backend(backend) => {
            use Backend;
            use hax_frontend_exporter::ThirBody as Body;

            if matches!(backend.backend, Backend::Easycrypt | Backend::ProVerif(..)) {
                HaxMessage::WarnExperimentalBackend {
                    backend: backend.backend.clone(),
                }
                .report(options.message_format, None);
            }

            let mut error = false;
            for EmitHaxMetaMessage {
                working_dir,
                manifest_dir,
                path,
            } in haxmeta_files
            {
                let (mut haxmeta, id_table): (HaxMeta<Body>, _) =
                    HaxMeta::read(fs::File::open(&path).unwrap());

                if let Some(root_module) = &backend.prune_haxmeta {
                    use hax_frontend_exporter::{DefPathItem, DisambiguatedDefPathItem, IsBody};

                    /// Remove every item from an `HaxMeta` whose path is not `*::<root_module>::**`, where `root_module` is a string.
                    fn prune_haxmeta<B: IsBody>(haxmeta: &mut HaxMeta<B>, root_module: &str) {
                        match &mut haxmeta.items {
                            Items::Legacy(items) => {
                                items.retain(|item| match &item.owner_id.path[..] {
                                    [] => true,
                                    [
                                        DisambiguatedDefPathItem {
                                            data: DefPathItem::TypeNs(s),
                                            disambiguator: 0,
                                        },
                                        ..,
                                    ] => s == root_module,
                                    _ => false,
                                })
                            }
                            Items::FullDef(items) => {
                                items.retain(|item| match &item.this.contents().def_id.path[..] {
                                    [] => true,
                                    [
                                        DisambiguatedDefPathItem {
                                            data: DefPathItem::TypeNs(s),
                                            disambiguator: 0,
                                        },
                                        ..,
                                    ] => s == root_module,
                                    _ => false,
                                })
                            }
                        };
                    }
                    prune_haxmeta(&mut haxmeta, root_module.as_str())
                }

                error = error
                    || run_engine(
                        haxmeta,
                        id_table,
                        working_dir,
                        manifest_dir,
                        &backend,
                        options.message_format,
                    );
            }
            error
        }
        Command::Serialize { .. } => {
            for EmitHaxMetaMessage { path, .. } in haxmeta_files {
                HaxMessage::ProducedFile { path, wrote: true }.report(options.message_format, None);
            }
            false
        }
        // Dispatched directly in `main`, before the frontend runs.
        Command::Tools(_) => unreachable!("`tools` subcommands are handled in `main`"),
        Command::Extract { .. } => unreachable!("`extract` is handled in `main`"),
    }
}

/// Exits the process, downgrading a successful code to a failure when an
/// error-severity message was reported: a reported error and a zero exit
/// status must never combine. Every exit path that can carry code 0 goes
/// through here.
fn exit(code: i32) -> ! {
    if code == 0 && hax_types::diagnostics::message::errors_reported() {
        std::process::exit(1)
    }
    std::process::exit(code)
}

fn main() {
    let args: Vec<String> = get_args("hax");
    let mut options = match &args[..] {
        [_, kw] if kw == "__json" => {
            serde_json::from_str(&std::env::var(ENV_VAR_OPTIONS_FULL).unwrap_or_else(|_| {
                panic!(
                    "Cannot find environnement variable {}",
                    ENV_VAR_OPTIONS_FULL
                )
            }))
            .unwrap_or_else(|_| {
                panic!(
                    "Invalid value for the environnement variable {}",
                    ENV_VAR_OPTIONS_FULL
                )
            })
        }
        _ => Options::parse_from(args.iter()),
    };
    options.normalize_paths();

    // The `tools` subcommands never involve the hax frontend: handle them
    // directly and exit.
    if let Command::Tools(ref command) = options.command {
        exit(tools::run(command, options.message_format));
    }

    // `extract` resolves the proof scenarios of the project and re-enters
    // this binary once per scenario; each re-entered run does its own
    // discovery and `hax-lib` gating in the extracted package's directory.
    if let Command::Extract {
        ref names,
        ref packages,
        dry_run,
        verbose,
        ref hermeticity,
    } = options.command
    {
        exit(scenario::run(
            names,
            packages,
            &options.cargo_flags,
            hermeticity,
            dry_run,
            verbose,
            options.message_format,
        ));
    }

    // Every other command processes source: discover the project once
    // (hax.toml configuration and dependency graph) and gate on `hax-lib`
    // compatibility before any tool runs. Discovery is driven by the same
    // Cargo arguments as the build it precedes, so that it finds the
    // manifest, and gates the crates, that are actually processed.
    //
    // `--haxmeta` reuses an already-extracted crate and runs no Cargo
    // command, so it needs no Cargo project around it; the lean backend
    // ignores the option and needs the project either way.
    let lean_backend = matches!(&options.command,
        Command::Backend(backend) if matches!(backend.backend, Backend::Lean(_)));
    let project = if options.haxmeta.is_some() && !lean_backend {
        None
    } else {
        match tools::project::ProjectContext::load_for(&options.cargo_flags, options.message_format)
        {
            Ok(project) => Some(project),
            Err(message) => {
                HaxMessage::GenericError { message }.report(options.message_format, None);
                std::process::exit(1);
            }
        }
    };
    if let Some(project) = &project
        && tools::haxlib::enforce(project, options.message_format)
    {
        std::process::exit(1);
    }

    // Lean bypasses the hax frontend entirely: run charon + aeneas directly
    if let Command::Backend(ref backend) = options.command
        && let Backend::Lean(ref aeneas_opts) = backend.backend
    {
        // Warn about options that are not supported by the lean backend
        for (set, name) in [
            (backend.dry_run, "--dry-run"),
            (backend.stats, "--stats"),
            (backend.profile, "--profile"),
            (backend.debug_engine.is_some(), "--debug-engine"),
            (backend.extract_type_aliases, "--extract-type-aliases"),
            (
                !backend.translation_options.include_namespaces.is_empty(),
                "-i",
            ),
        ] {
            if set {
                HaxMessage::UnsupportedOption {
                    option: name.into(),
                    backend: BackendName::Lean,
                }
                .report(options.message_format, None);
            }
        }

        let project = project
            .as_ref()
            .expect("the lean backend always discovers the project");
        let error = aeneas::run(
            aeneas_opts,
            backend.output_dir.clone(),
            backend.verbose,
            options.message_format,
            project,
        );
        exit(if error { 1 } else { 0 });
    }

    let (haxmeta_files, exit_code) = options
        .haxmeta
        .clone()
        .map(|path| {
            (
                vec![EmitHaxMetaMessage {
                    working_dir: None,
                    manifest_dir: None,
                    path,
                }],
                0,
            )
        })
        .unwrap_or_else(|| compute_haxmeta_files(&options));
    let error = run_command(&options, haxmeta_files);

    exit(if exit_code == 0 && error {
        1
    } else {
        exit_code
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace() -> HashSet<String> {
        HashSet::from(["my_crate".to_string()])
    }

    fn message_for(path: &Path) -> String {
        serde_json::to_string(&HaxDriverMessage::EmitHaxMeta(EmitHaxMetaMessage {
            working_dir: None,
            manifest_dir: None,
            path: path.to_path_buf(),
        }))
        .unwrap()
    }

    #[test]
    fn a_marker_line_from_a_compiled_crate_cannot_inject_a_haxmeta() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("target");
        std::fs::create_dir(&target).unwrap();
        // The caller hands the function a canonical target directory.
        let target = target.canonicalize().unwrap();

        // A well-formed report for a file outside the target directory.
        let outside = dir.path().join("my_crate-0123456789abcdef.haxmeta");
        std::fs::write(&outside, b"").unwrap();
        assert!(accept_driver_message(&message_for(&outside), &target, &workspace()).is_err());

        // A well-formed report under the target directory for a crate that
        // is no workspace member.
        let foreign = target.join("victim-0123456789abcdef.haxmeta");
        std::fs::write(&foreign, b"").unwrap();
        assert!(accept_driver_message(&message_for(&foreign), &target, &workspace()).is_err());

        // A marker-prefixed line that is not a driver message is an error,
        // not a panic.
        assert!(accept_driver_message("not json", &target, &workspace()).is_err());

        // The driver's own report is accepted.
        let legit = target.join("my_crate-0123456789abcdef.haxmeta");
        std::fs::write(&legit, b"").unwrap();
        assert!(accept_driver_message(&message_for(&legit), &target, &workspace()).is_ok());
    }
}
