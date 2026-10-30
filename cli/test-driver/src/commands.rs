//! Async helpers that wrap the `cargo hax` commands and diagnostic plumbing.
//!
//! This module is intentionally lightweight: it only hides the raw command
//! invocation details and structures the diagnostics that are produced by the
//! backend.

use anyhow::{Context as _, Result};
use hax_frontend_exporter::DefId;
use hax_types::{
    cli_options::{BackendName, MessageFormat},
    diagnostics::message::HaxMessage,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use crate::{
    directives::{Directive, ErrorCode, ItemDirective},
    helpers,
    log::{JobKind, ReportMessage},
};

/// Runs `cargo hax serialize` and returns the path of the produced `.haxmeta`
/// file.
pub async fn hax_serialize(flags: &[&str]) -> Result<PathBuf> {
    static PERMITS: tokio::sync::Semaphore = tokio::sync::Semaphore::const_new(1);
    let _permit = PERMITS.acquire().await?;
    let mut command = tokio::process::Command::new("cargo");
    command.args(["hax", "--message-format", "json", "serialize"]);
    command.args(flags);
    let out = command.output().await?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let raw_stdout = stdout.clone();
    let stderr = String::from_utf8_lossy(&out.stderr);
    let path = stdout
        .lines()
        .flat_map(serde_json::from_str::<HaxMessage>)
        .find_map(|hax_message| match hax_message {
            HaxMessage::ProducedFile { path, .. } => Some(path.clone()),
            _ => None,
        })
        .with_context(|| format!("`{command:?}` produced no haxmeta file\n\n\nstderr={stderr}\n\n\nstdout={raw_stdout}"))?;
    Ok(path)
}

/// Captures the output of a `cargo hax into` invocation.
pub struct HaxEngineOutput {
    pub error_code: i32,
    pub messages: Vec<OutMsg>,
    pub stderr: String,
}

/// Captures the output of a backend invocation
pub struct BackendOutput {
    pub error_code: i32,
    pub stderr: String,
}

impl HaxEngineOutput {
    /// Parses the output of a `cargo hax --message-format json` invocation.
    pub fn from_output(out: &std::process::Output) -> Result<Self> {
        Ok(Self {
            error_code: out
                .status
                .code()
                .context("No error code: was the process terminated?")?,
            messages: String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|s| {
                    serde_json::from_str::<OutMsg>(s)
                        .unwrap_or_else(|_| OutMsg::Unknown(s.to_string()))
                })
                .collect(),
            stderr: String::from_utf8_lossy(&out.stderr).to_string(),
        })
    }

    /// Forwards the captured stderr/stdout to the logging infrastructure.
    pub fn report_stderr_and_stdout(&self, job: JobKind) {
        job.report(ReportMessage::Stderr(self.stderr.clone()));
        job.report(ReportMessage::Stdout({
            let rendered = self
                .messages
                .iter()
                .filter_map(|message| message.render())
                .collect::<Vec<_>>();
            rendered.join("\n")
        }));
    }

    /// Extracts the diagnostics that were emitted by the engine.
    pub fn diagnostics(&self) -> Diagnostics {
        let set: HashSet<_> = self
            .messages
            .iter()
            .filter_map(Diagnostic::try_from_out_msg)
            .collect();
        Diagnostics::new(set.into_iter())
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
/// A single diagnostic message emitted by the backend.
pub struct Diagnostic {
    pub def_id: Option<DefId>,
    pub error_code: ErrorCode,
    pub full_message: OutMsg,
}

impl Diagnostic {
    fn try_from_out_msg(out_msg: &OutMsg) -> Option<Self> {
        match out_msg {
            OutMsg::Hax(HaxMessage::Diagnostic { diagnostic, .. }) => Some(Self {
                def_id: diagnostic.owner_id.clone(),
                error_code: ErrorCode::new(diagnostic.kind.code()),
                full_message: out_msg.clone(),
            }),
            _ => None,
        }
    }

    /// Returns `true` if this diagnostic belongs to the provided item or one of
    /// its descendants.
    pub fn is_for_item(&self, item_id: &DefId) -> bool {
        self.def_id
            .as_ref()
            .map(|child| helpers::def_id_under(item_id, child))
            .unwrap_or(true)
    }
}

/// Diagnostic multimap keyed by [`ErrorCode`].
pub struct Diagnostics(HashMap<ErrorCode, Vec<Diagnostic>>);

impl Diagnostics {
    fn new(it: impl Iterator<Item = Diagnostic>) -> Self {
        let mut map: HashMap<_, Vec<_>> = HashMap::new();
        for diag in it {
            map.entry(diag.error_code.clone()).or_default().push(diag);
        }
        Self(map)
    }

    /// Removes a diagnostic from Self if the diagnostic is for the item
    /// `item_id` (or below), and if the diagnostic is tagged with an error that
    /// matches `error_code`.
    /// Returns `Some(_)` if a matching diagnostic was found.
    /// Removes (and returns) a diagnostic that matches the provided item and
    /// error code, if present.
    pub fn remove(&mut self, item_id: &DefId, error_code: &ErrorCode) -> Option<Diagnostic> {
        if let Some(diags) = self.0.get_mut(error_code)
            && let Some((i, diag)) = diags
                .iter()
                .enumerate()
                .find(|(_, diag)| diag.is_for_item(item_id))
        {
            let diag = diag.clone();
            diags.swap_remove(i);
            Some(diag)
        } else {
            None
        }
    }

    /// Iterates over the retained diagnostic messages.
    pub fn iter(&self) -> impl Iterator<Item = &OutMsg> {
        self.0.values().flatten().map(|diag| &diag.full_message)
    }

    /// Queues directives that should be promoted into the source code for all
    /// diagnostics that remain unexpected.
    pub async fn add_directives_in_files(
        &self,
        backend: BackendName,
        fallback_def_id: &DefId,
    ) -> Result<()> {
        for diag in self.0.values().flatten() {
            let owner_id = diag.def_id.as_ref().unwrap_or(fallback_def_id);
            let Some(span) = crate::span_hint::span_hint(owner_id).await? else {
                println!("No span for {}", helpers::string_of_def_id(owner_id));
                continue;
            };
            let (Some(span_path), line, is_mod) = span.get_file_and_line() else {
                continue;
            };

            use crate::promote_directives::*;
            let kind = LineKind::Directive {
                directive: Directive::Item(ItemDirective::Failure {
                    kind: crate::directives::FailureKind::Extract,
                    backends: std::iter::once((backend, vec![diag.error_code.clone()])).collect(),
                }),
                bang: is_mod,
            };
            push_line(span_path, Line { line, kind })?;
        }

        Ok(())
    }
}

/// Executes `cargo hax into` for the specified backend.
pub async fn hax_engine(
    haxmeta: &Path,
    test_module_name: &str,
    output_dir: &Path,
    backend: BackendName,
    into_flags: &[String],
    backend_flags: &[String],
) -> Result<HaxEngineOutput> {
    let mut command = tokio::process::Command::new("cargo");
    command.args(["hax", "--message-format", "json", "--haxmeta"]);
    command.arg(haxmeta);
    command.arg("into");
    command.arg("--output-dir");
    command.arg(output_dir);
    command.args(["--prune-haxmeta", test_module_name]);
    command.args(into_flags);
    command.arg(backend.to_string());
    command.args(backend_flags);
    HaxEngineOutput::from_output(&command.output().await?)
}

/// Executes F*
pub async fn run_fstar(lax: bool, file_path: PathBuf) -> Result<BackendOutput> {
    let root_path = std::env::current_dir()?;
    let makefile_path = file_path.join("Makefile");
    if !makefile_path.exists() {
        let template_path = root_path.join("templates").join("Makefile.fstar.template");
        if template_path.exists() {
            std::fs::copy(template_path, makefile_path)?;
        }
    }
    let mut command = tokio::process::Command::new("make");
    if lax {
        command.env("OTHERFLAGS", "--lax");
    }
    command.current_dir(file_path);
    let out = command.output().await?;
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    let mut error = stderr.lines().rev().take(10).collect::<Vec<_>>();
    error.reverse();

    Ok(BackendOutput {
        error_code: out
            .status
            .code()
            .context("No error code: was the process terminated?")?,
        stderr: error.join("\n"),
    })
}

/// Executes Lean
pub async fn run_lean(dir: PathBuf) -> Result<BackendOutput> {
    let root_path = std::env::current_dir()?;

    let lakefile_path = dir.join("lakefile.toml");
    if !lakefile_path.exists() && !dir.join("lakefile.lean").exists() {
        // Find the single .lean file to derive the crate name.
        let crate_name = std::fs::read_dir(&dir)?
            .filter_map(|e| e.ok())
            .find(|e| e.path().extension().map_or(false, |ext| ext == "lean"))
            .context("No .lean file found in output directory")?
            .path()
            .file_stem()
            .context("Lean file has no stem")?
            .to_string_lossy()
            .to_string();

        // Resolve the local path to the Hax proof library.
        let hax_lib_path = root_path
            .join("../hax-lib/proof-libs/legacy-lean")
            .canonicalize()
            .context("Could not resolve hax proof lib path")?;

        let lakefile = format!(
            r#"name = "{crate_name}"
version = "0.1.0"
defaultTargets = ["{crate_name}"]

[[lean_lib]]
name = "{crate_name}"

[[require]]
name = "Hax"
path = "{hax_path}"
"#,
            hax_path = hax_lib_path.display()
        );
        std::fs::write(&lakefile_path, lakefile)?;

        // Copy the lean-toolchain from the proof library so that elan
        // picks the matching Lean version.
        let toolchain_src = hax_lib_path.join("lean-toolchain");
        if toolchain_src.exists() {
            std::fs::copy(&toolchain_src, dir.join("lean-toolchain"))?;
        }
    }

    let mut command = tokio::process::Command::new("lake");
    command.arg("build");
    command.current_dir(&dir);
    let out = command.output().await?;
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    Ok(BackendOutput {
        error_code: out
            .status
            .code()
            .context("No error code: was the process terminated?")?,
        stderr,
    })
}

/// The JSON-encoded messages `cargo hax` can emit
#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
#[serde(untagged)]
/// JSON messages emitted by `cargo hax`.
pub enum OutMsg {
    Cargo(cargo_metadata::CompilerMessage),
    Hax(HaxMessage),
    Unknown(String),
}

impl OutMsg {
    /// Renders the message into a human-readable string, when possible.
    pub fn render(&self) -> Option<String> {
        let owner = if let OutMsg::Hax(HaxMessage::Diagnostic {
            diagnostic:
                hax_types::diagnostics::Diagnostics {
                    owner_id: Some(owner),
                    ..
                },
            ..
        }) = &self
        {
            Some(format!("owner={owner:?}"))
        } else {
            None
        };
        match self {
            OutMsg::Cargo(compiler_message) => Some(
                compiler_message
                    .message
                    .rendered
                    .clone()
                    .unwrap_or_else(|| compiler_message.message.message.clone()),
            ),
            OutMsg::Hax(hax_message) => hax_message
                .clone()
                .render(MessageFormat::Human, None)
                .map(|s| format!("{s}{}", owner.unwrap_or("".into()))),
            OutMsg::Unknown(unknown) => Some(unknown.clone()),
        }
    }
}
