use crate::prelude::*;

use clap::{Parser, Subcommand, ValueEnum};
use std::fmt;

pub use hax_frontend_exporter_options::*;
pub mod extension;
use extension::Extension;

#[derive_group(Serializers)]
#[derive(JsonSchema, Debug, Clone, Eq, PartialEq)]
pub enum DebugEngineMode {
    File(PathOrDash),
    Interactive,
}

impl std::convert::From<&str> for DebugEngineMode {
    fn from(s: &str) -> Self {
        match s {
            "i" | "interactively" => DebugEngineMode::Interactive,
            s => DebugEngineMode::File(s.strip_prefix("file:").unwrap_or(s).into()),
        }
    }
}

#[derive_group(Serializers)]
#[derive(JsonSchema, Debug, Clone, Default)]
pub struct ForceCargoBuild {
    pub data: u64,
}

impl std::convert::From<&str> for ForceCargoBuild {
    fn from(s: &str) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        if s == "false" {
            let data = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|r| r.as_millis())
                .unwrap_or(0);
            ForceCargoBuild { data: data as u64 }
        } else {
            ForceCargoBuild::default()
        }
    }
}

#[derive_group(Serializers)]
#[derive(Debug, Clone, JsonSchema, Eq, PartialEq)]
pub enum PathOrDash {
    Dash,
    Path(PathBuf),
}

impl std::convert::From<&str> for PathOrDash {
    fn from(s: &str) -> Self {
        match s {
            "-" => PathOrDash::Dash,
            _ => PathOrDash::Path(PathBuf::from(s)),
        }
    }
}

impl PathOrDash {
    pub fn open_or_stdout(&self) -> Box<dyn std::io::Write> {
        use std::io::BufWriter;
        match self {
            PathOrDash::Dash => Box::new(BufWriter::new(std::io::stdout())),
            PathOrDash::Path(path) => {
                Box::new(BufWriter::new(std::fs::File::create(&path).unwrap()))
            }
        }
    }
    pub fn map_path<F: FnOnce(&Path) -> PathBuf>(&self, f: F) -> Self {
        match self {
            PathOrDash::Path(path) => PathOrDash::Path(f(path)),
            PathOrDash::Dash => PathOrDash::Dash,
        }
    }
}

fn absolute_path(path: impl AsRef<std::path::Path>) -> std::io::Result<std::path::PathBuf> {
    use path_clean::PathClean;
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    }
    .clean();

    Ok(absolute_path)
}

pub trait NormalizePaths {
    fn normalize_paths(&mut self);
}

impl NormalizePaths for PathBuf {
    fn normalize_paths(&mut self) {
        *self = absolute_path(&self).unwrap();
    }
}
impl NormalizePaths for PathOrDash {
    fn normalize_paths(&mut self) {
        match self {
            PathOrDash::Path(p) => p.normalize_paths(),
            PathOrDash::Dash => (),
        }
    }
}

#[derive_group(Serializers)]
#[derive(JsonSchema, Parser, Debug, Hash, Clone, Eq, PartialEq)]
pub struct ProVerifOptions {
    /// Items for which hax should extract a default-valued process
    /// macro with a corresponding type signature. This flag expects a
    /// space-separated list of inclusion clauses. An inclusion clause
    /// is a Rust path prefixed with `+`, `+!` or `-`. `-` means
    /// implementation only, `+!` means interface only and `+` means
    /// implementation and interface. Rust path chunks can be either a
    /// concrete string, or a glob (just like bash globs, but with
    /// Rust paths).
    #[arg(
        long,
        value_parser = parse_inclusion_clause,
        value_delimiter = ' ',
        allow_hyphen_values(true)
    )]
    pub assume_items: Vec<InclusionClause>,
}

impl ProVerifOptions {
    /// The flag rendering of these options, as `cargo hax extract
    /// --dry-run` prints it. Lives next to the fields so a new or renamed
    /// option is reflected here.
    pub fn flags(&self) -> Vec<String> {
        if self.assume_items.is_empty() {
            return Vec::new();
        }
        std::iter::once("--assume-items".to_string())
            .chain(self.assume_items.iter().map(ToString::to_string))
            .collect()
    }
}

/// The defaults of the F* flags below, shared with
/// [`FStarOptions::defaults`] so the two cannot diverge.
const FSTAR_DEFAULT_Z3RLIMIT: u32 = 15;
const FSTAR_DEFAULT_FUEL: u32 = 0;
const FSTAR_DEFAULT_IFUEL: u32 = 1;
const FSTAR_DEFAULT_LINE_WIDTH: u16 = 100;

#[derive_group(Serializers)]
#[derive(JsonSchema, Parser, Debug, Hash, Clone, Eq, PartialEq)]
pub struct FStarOptions {
    /// Set the Z3 per-query resource limit
    #[arg(long, default_value_t = FSTAR_DEFAULT_Z3RLIMIT)]
    pub z3rlimit: u32,
    /// Number of unrolling of recursive functions to try
    #[arg(long, default_value_t = FSTAR_DEFAULT_FUEL)]
    pub fuel: u32,
    /// Number of unrolling of inductive datatypes to try
    #[arg(long, default_value_t = FSTAR_DEFAULT_IFUEL)]
    pub ifuel: u32,
    /// Modules for which Hax should extract interfaces (`*.fsti`
    /// files) in supplement to implementations (`*.fst` files). By
    /// default we extract no interface, only implementations. If a
    /// item is signature only (see the `+:` prefix of the
    /// `--include_namespaces` flag of the `into` subcommand), then
    /// its namespace is extracted with an interface. This flag
    /// expects a space-separated list of inclusion clauses. An
    /// inclusion clause is a Rust path prefixed with `+`, `+!` or
    /// `-`. `-` means implementation only, `+!` means interface only
    /// and `+` means implementation and interface. Rust path chunks
    /// can be either a concrete string, or a glob (just like bash
    /// globs, but with Rust paths).
    #[arg(
        long,
        value_parser = parse_inclusion_clause,
        value_delimiter = ' ',
        allow_hyphen_values(true)
    )]
    pub interfaces: Vec<InclusionClause>,

    #[arg(long, default_value_t = FSTAR_DEFAULT_LINE_WIDTH, env = "HAX_FSTAR_LINE_WIDTH")]
    pub line_width: u16,
}

impl FStarOptions {
    /// The flags' defaults. Proof scenarios resolve absent keys to these;
    /// environment-supplied flag defaults (`HAX_FSTAR_LINE_WIDTH`)
    /// deliberately do not apply to scenario runs.
    pub fn defaults() -> Self {
        Self {
            z3rlimit: FSTAR_DEFAULT_Z3RLIMIT,
            fuel: FSTAR_DEFAULT_FUEL,
            ifuel: FSTAR_DEFAULT_IFUEL,
            interfaces: Vec::new(),
            line_width: FSTAR_DEFAULT_LINE_WIDTH,
        }
    }

    /// The flag rendering of these options, as `cargo hax extract
    /// --dry-run` prints it. Lives next to the fields so a new or renamed
    /// option is reflected here.
    pub fn flags(&self) -> Vec<String> {
        let mut flags = vec![
            format!("--z3rlimit={}", self.z3rlimit),
            format!("--fuel={}", self.fuel),
            format!("--ifuel={}", self.ifuel),
            format!("--line-width={}", self.line_width),
        ];
        if !self.interfaces.is_empty() {
            flags.push("--interfaces".to_string());
            flags.extend(self.interfaces.iter().map(ToString::to_string));
        }
        flags
    }
}

/// The inputs a proof scenario resolves for the Lean backend, carried
/// through the `__json` re-entry rather than argv: verbatim argument
/// arrays (no shell splitting), the compiled item selection, and the
/// package-layout overrides. Empty on flag-driven `into` invocations.
#[derive_group(Serializers)]
#[derive(JsonSchema, Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct LeanScenarioOptions {
    /// The Lean package name, overriding the crate-name derivation.
    pub package_name: Option<String>,
    /// The scenario's `project-files` key, overriding the top-level key.
    pub project_files: Option<bool>,
    /// Charon name patterns compiled to `--start-from`.
    pub include: Vec<String>,
    /// Charon name patterns compiled to `--exclude`.
    pub exclude: Vec<String>,
    /// Charon name patterns compiled to `--opaque`, the default opaque
    /// set already merged in.
    pub opaque: Vec<String>,
    /// Verbatim extra charon arguments, one element per process argument.
    pub charon_args: Vec<String>,
    /// Verbatim extra aeneas arguments, one element per process argument.
    pub aeneas_args: Vec<String>,
    /// Cargo arguments (feature selection) for the cargo invocation
    /// charon drives.
    pub cargo_args: Vec<String>,
}

impl LeanScenarioOptions {
    /// The charon flags compiled from the unified item-selection keys.
    /// Both the real charon invocation and the `--dry-run` display use
    /// this compilation, so the two cannot diverge.
    pub fn selection_flags(&self) -> Vec<String> {
        [
            ("start-from", &self.include),
            ("exclude", &self.exclude),
            ("opaque", &self.opaque),
        ]
        .into_iter()
        .flat_map(|(flag, patterns)| {
            patterns
                .iter()
                .map(move |pattern| format!("--{flag}={pattern}"))
        })
        .collect()
    }
}

#[derive_group(Serializers)]
#[derive(JsonSchema, Parser, Debug, Clone, Hash, Eq, PartialEq)]
#[command(after_help = "\
TOOLS:
  This backend runs `charon`, then `aeneas`, and generates a Lean proof project.
  Tool versions are managed by hax: each tool resolves through, in order, the
  project's `hax.toml` (member crate, then workspace root) and the built-in
  default version this release was tested with, and is downloaded into the
  tool cache on demand with checksum verification.

  Inspect the active versions and their sources with `cargo hax tools show`;
  pre-install them with `cargo hax tools install`.

INVOCATION:
  The tools are run with some fixed flags (to which any --charon-args/--aeneas-args
  are appended). Pass `-v` (`cargo hax into -v lean`) to print the exact command
  before each tool runs.

  Overriding a flag that controls where output is written (aeneas's -backend,
  -dest, -subdir, or -split-files) may break the extraction or the generated
  proof project. Charon's --dest-file is reserved and rejected: aeneas is always
  run on the LLBC file hax chooses. Overriding -dest or -subdir additionally disables
  the generation and checking of the Lean package files and the clearing of stale
  extraction files, since hax no longer knows the package layout. Committing
  `project-files = false` in `hax.toml` disables the package files for every
  invocation.

  To use a binary you built yourself, commit a `path` entry for the tool in
  `hax.toml` instead of a version.")]
pub struct LeanOptions {
    /// Extra arguments forwarded to charon. Parsed with shell-style quoting,
    /// so values containing spaces can be single- or double-quoted.
    /// Example: --charon-args="--opaque '{impl Serialize for _}'"
    #[arg(long)]
    pub charon_args: Option<String>,

    /// Extra arguments forwarded to aeneas. Parsed with shell-style quoting.
    /// Example: --aeneas-args="-split-files"
    #[arg(long)]
    pub aeneas_args: Option<String>,

    /// The scenario-resolved inputs; not settable from the command line.
    #[clap(skip)]
    pub scenario: LeanScenarioOptions,
}

#[derive_group(Serializers)]
#[derive(JsonSchema, Subcommand, Debug, Clone, Hash, Eq, PartialEq)]
pub enum Backend {
    /// Use the F* backend
    Fstar(FStarOptions),
    /// Use the legacy Lean backend (warning: experimental)
    LegacyLean,
    /// Use the Lean backend (charon + aeneas pipeline)
    Lean(LeanOptions),
    /// Use the Coq backend
    Coq,
    /// Use the SSProve backend
    Ssprove,
    /// Use the EasyCrypt backend (warning: work in progress!)
    Easycrypt,
    /// Use the ProVerif backend (warning: work in progress!)
    #[clap(alias("proverif"))]
    ProVerif(ProVerifOptions),
    /// Use the Rust backend (warning: work in progress!)
    #[clap(hide = true)]
    Rust,
    /// Extract `DefId`s of the crate as a Rust module tree.
    /// This is a command that regenerates code for the rust engine.
    #[clap(hide = true)]
    GenerateRustEngineNames,
    /// A debugger for the Rust engine
    Debugger {
        #[arg(long, short)]
        interactive: bool,
    },
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        BackendName::from(self).fmt(f)
    }
}

#[derive_group(Serializers)]
#[derive(JsonSchema, Debug, Hash, Clone, Eq, PartialEq)]
pub enum DepsKind {
    Transitive,
    Shallow,
    None,
}

#[derive_group(Serializers)]
#[derive(JsonSchema, Debug, Hash, Clone, Eq, PartialEq)]
pub enum InclusionKind {
    /// `+query` include the items selected by `query`
    Included(DepsKind),
    SignatureOnly,
    Excluded,
}

#[derive_group(Serializers)]
#[derive(JsonSchema, Debug, Hash, Clone, Eq, PartialEq)]
pub struct InclusionClause {
    pub kind: InclusionKind,
    pub namespace: Namespace,
}

const PREFIX_INCLUDED_TRANSITIVE: &str = "+";
const PREFIX_INCLUDED_SHALLOW: &str = "+~";
const PREFIX_INCLUDED_NONE: &str = "+!";
const PREFIX_SIGNATURE_ONLY: &str = "+:";
const PREFIX_EXCLUDED: &str = "-";

impl ToString for InclusionClause {
    fn to_string(&self) -> String {
        let kind = match self.kind {
            InclusionKind::Included(DepsKind::Transitive) => PREFIX_INCLUDED_TRANSITIVE,
            InclusionKind::Included(DepsKind::Shallow) => PREFIX_INCLUDED_SHALLOW,
            InclusionKind::Included(DepsKind::None) => PREFIX_INCLUDED_NONE,
            InclusionKind::SignatureOnly => PREFIX_SIGNATURE_ONLY,
            InclusionKind::Excluded => PREFIX_EXCLUDED,
        };
        format!("{kind}{}", self.namespace.to_string())
    }
}

pub fn parse_inclusion_clause(
    s: &str,
) -> Result<InclusionClause, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let s = s.trim();
    if s.is_empty() {
        Err("Expected `-` or `+`, got an empty string")?
    }
    let (prefix, namespace) = {
        let f = |&c: &char| matches!(c, '+' | '-' | '~' | '!' | ':');
        (
            s.chars().take_while(f).into_iter().collect::<String>(),
            s.chars().skip_while(f).into_iter().collect::<String>(),
        )
    };
    let kind = match &prefix[..] {
        PREFIX_INCLUDED_TRANSITIVE => InclusionKind::Included(DepsKind::Transitive),
        PREFIX_INCLUDED_SHALLOW => InclusionKind::Included(DepsKind::Shallow),
        PREFIX_INCLUDED_NONE => InclusionKind::Included(DepsKind::None),
        PREFIX_SIGNATURE_ONLY => InclusionKind::SignatureOnly,
        PREFIX_EXCLUDED => InclusionKind::Excluded,
        prefix => Err(format!(
            "Expected `+`, `+~`, `+!`, `+:` or `-`, got an `{prefix}`"
        ))?,
    };
    Ok(InclusionClause {
        kind,
        namespace: namespace.to_string().into(),
    })
}

#[derive_group(Serializers)]
#[derive(JsonSchema, Parser, Debug, Clone, Eq, PartialEq)]
pub struct TranslationOptions {
    /// Controls which Rust item should be extracted or not.
    ///
    /// This is a space-separated list of patterns prefixed with a
    /// modifier, read from the left to the right.
    ///
    /// A pattern is a Rust path (say `mycrate::mymod::myfn`) where
    /// globs are allowed: `*` matches any name
    /// (e.g. `mycrate::mymod::myfn` is matched by
    /// `mycrate::*::myfn`), while `**` matches any subpath, empty
    /// included (e.g. `mycrate::mymod::myfn` is matched by
    /// `**::myfn`).

    /// By default, hax includes all items. Then, the patterns
    /// prefixed by modifiers are processed from left to right,
    /// excluding or including items. Each pattern selects a number of
    /// item. The modifiers are:

    /// {n}{n} - `+`: includes the selected items with their
    /// dependencies, transitively (e.g. if function `f` calls `g`
    /// which in turn calls `h`, then `+k::f` includes `f`, `g` and
    /// `h`)

    /// {n} - `+~`: includes the selected items with their direct
    /// dependencies only (following the previous example, `+~k::f`
    /// would select `f` and `g`, but not `h`)

    /// {n} - `+!`: includes the selected items, without their
    /// dependencies (`+!k::f` would only select `f`)

    /// {n} - `+:`: only includes the type of the selected items (no
    /// dependencies). This includes full struct and enums, but only
    /// the type signature of functions and trait impls (except when
    /// they contain associated types), dropping their bodies.
    #[arg(
        value_parser = parse_inclusion_clause,
        value_delimiter = ' ',
    )]
    #[arg(short, allow_hyphen_values(true))]
    pub include_namespaces: Vec<InclusionClause>,
}

#[derive_group(Serializers)]
#[derive(JsonSchema, Parser, Debug, Clone, Eq, PartialEq)]
pub struct BackendOptions<E: Extension> {
    #[command(subcommand)]
    pub backend: Backend,

    /// Don't write anything on disk. Output everything as JSON to stdout
    /// instead.
    #[arg(long = "dry-run")]
    pub dry_run: bool,

    /// Verbose mode for the Hax engine. Set `-vv` for maximal verbosity.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Prints statistics about how many items have been translated
    /// successfully by the engine.
    #[arg(long)]
    pub stats: bool,

    /// Enables profiling for the engine: for each phase of the
    /// engine, time and memory usage are recorded and reported.
    #[arg(long)]
    pub profile: bool,

    /// Prune Rust items that are not under the provided top-level module name.
    /// This will effectively remove all items that don't match `*::<prune_haxmetadata>::**`.
    /// This prunning occurs directly on the `haxmeta` file, in the frontend.
    /// This is independent from any engine options.
    #[arg(long)]
    #[clap(hide = true)]
    pub prune_haxmeta: Option<String>,

    /// Enable engine debugging: dumps the AST at each phase.
    ///
    /// The value of `<DEBUG_ENGINE>` can be either:

    /// {n}{n} - `interactive` (or `i`): enables debugging of the engine,
    /// and visualize interactively in a webapp how a crate was
    /// transformed by each phase, both in Rust-like syntax and
    /// browsing directly the internal AST. By default, the webapp is
    /// hosted on `http://localhost:8000`, the port can be override by
    /// setting the `HAX_DEBUGGER_PORT` environment variable.

    /// {n} - `<FILE>` or `file:<FILE>`: outputs the different AST as JSON
    /// to `<FILE>`. `<FILE>` can be either [-] or a path.
    #[arg(short, long = "debug-engine")]
    pub debug_engine: Option<DebugEngineMode>,

    /// Extract type aliases. This is disabled by default, since
    /// extracted terms depends on expanded types rather than on type
    /// aliases. Turning this option on is discouraged: Rust type
    /// synonyms can ommit generic bounds, which are ususally
    /// necessary in the hax backends, leading to typechecking
    /// errors. For more details see
    /// https://github.com/pereq-dev/hax/issues/708.
    #[arg(long)]
    pub extract_type_aliases: bool,

    #[command(flatten)]
    pub translation_options: TranslationOptions,

    /// Where to put the output files resulting from the translation.
    /// Defaults to "<crate folder>/proofs/<backend>/extraction".
    #[arg(long)]
    pub output_dir: Option<PathBuf>,

    #[group(flatten)]
    pub cli_extension: E::BackendOptions,
}

/// Cargo's hermeticity flags, applied to every cargo invocation a
/// `cargo hax extract` run drives: project discovery, the frontend's
/// `cargo check`, and the build charon drives.
#[derive_group(Serializers)]
#[derive(JsonSchema, Parser, Debug, Clone, Default, Eq, PartialEq)]
pub struct CargoHermeticityOptions {
    /// Assert that `Cargo.lock` will remain unchanged
    #[arg(long)]
    pub locked: bool,

    /// Run without accessing the network
    #[arg(long)]
    pub offline: bool,

    /// Equivalent to specifying both --locked and --offline
    #[arg(long)]
    pub frozen: bool,
}

impl CargoHermeticityOptions {
    /// The cargo flags these options stand for, verbatim.
    pub fn flags(&self) -> Vec<String> {
        [
            ("--locked", self.locked),
            ("--offline", self.offline),
            ("--frozen", self.frozen),
        ]
        .into_iter()
        .filter(|(_, set)| *set)
        .map(|(flag, _)| flag.to_string())
        .collect()
    }
}

#[derive_group(Serializers)]
#[derive(JsonSchema, Subcommand, Debug, Clone, Eq, PartialEq)]
pub enum Command<E: Extension> {
    /// Translate to a backend. The translated modules will be written
    /// under the directory `<PKG>/proofs/<BACKEND>/extraction`, where
    /// `<PKG>` is the translated cargo package name and `<BACKEND>`
    /// the name of the backend.
    #[clap(name = "into")]
    Backend(BackendOptions<E>),

    /// Run the proof scenarios declared in `hax.toml` (`[scenario.<name>]`
    /// tables): named, complete extraction configurations. Without names,
    /// every scenario in scope runs. Scenarios run sequentially; a failing
    /// scenario does not abort the run, failures are reported in a summary
    /// and produce a non-zero exit code.
    Extract {
        /// The scenarios to run. Each name selects every scenario with
        /// that name in scope. Absent, every scenario in scope runs.
        names: Vec<String>,

        /// Restrict the scope to the scenarios extracting the given
        /// package. May be repeated.
        #[arg(short = 'p', long = "package")]
        packages: Vec<String>,

        /// Print the resolved invocations, including the arguments
        /// compiled from the scenarios, without running them.
        #[arg(long)]
        dry_run: bool,

        /// Verbose mode, forwarded to each scenario run.
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,

        #[command(flatten)]
        hermeticity: CargoHermeticityOptions,
    },

    /// Export directly as a JSON file
    JSON {
        /// Path to the output JSON file, "-" denotes stdout.
        #[arg(
            short,
            long = "output-file",
            default_value = "hax_frontend_export.json"
        )]
        output_file: PathOrDash,
        /// Whether the bodies are exported as THIR, built MIR, const
        /// MIR, or a combination. Repeat this option to extract a
        /// combination (e.g. `-k thir -k mir-built`). Pass `--kind`
        /// alone with no value to disable body extraction.
        #[arg(
            value_enum,
            short,
            long = "kind",
            num_args = 0..=3,
            default_values_t = [ExportBodyKind::Thir]
        )]
        kind: Vec<ExportBodyKind>,

        /// By default, `cargo hax json` outputs a JSON where every
        /// piece of information is inlined. This however creates very
        /// large JSON files. This flag enables the use of unique IDs
        /// and outputs a map from IDs to actual objects.
        #[arg(long)]
        use_ids: bool,

        /// Whether to include extra informations about `DefId`s.
        #[arg(short = 'E', long = "include-extra", default_value = "false")]
        include_extra: bool,
    },

    /// Serialize to a `haxmeta` file, the internal binary format used by hax to
    /// store the ASTs produced by the hax exporter.
    #[clap(hide = true)]
    Serialize {
        /// Whether the bodies are exported as THIR, built MIR, const
        /// MIR, or a combination. Repeat this option to extract a
        /// combination (e.g. `-k thir -k mir-built`). Pass `--kind`
        /// alone with no value to disable body extraction.
        #[arg(
            value_enum,
            short,
            long = "kind",
            num_args = 0..=3,
            default_values_t = [ExportBodyKind::Thir]
        )]
        kind: Vec<ExportBodyKind>,

        /// When extracting to a given backend, the exporter is called with different `cfg` options.
        /// This option allows to set the same flags as `cargo hax into` would pick.
        #[arg(short)]
        backend: Option<BackendName>,
    },

    /// Manage the external tools hax depends on (e.g. charon and aeneas).
    #[command(subcommand)]
    Tools(ToolsCommand),

    #[command(flatten)]
    CliExtension(E::Command),
}

/// Subcommands of `cargo hax tools`.
#[derive_group(Serializers)]
#[derive(JsonSchema, Subcommand, Debug, Clone, Eq, PartialEq)]
pub enum ToolsCommand {
    /// Download and cache the tool versions the current project
    /// resolves to, or a specific `<tool>@<version>`.
    Install {
        /// A `<tool>@<version>` specification (e.g.
        /// `charon@nightly-2026.07.01`) to install into the
        /// machine-wide cache. When absent, installs what the current
        /// project's configuration resolves to.
        spec: Option<String>,
        /// Re-download and verify even if the version is already cached
        /// (e.g. to verify a copy installed before its checksum shipped).
        #[arg(long)]
        force: bool,
    },
    /// List the tool versions this release of hax can install with
    /// checksum verification.
    List {
        /// Restrict the listing to one tool.
        tool: Option<String>,
        /// Only show versions present in the local cache.
        #[arg(long)]
        installed: bool,
        /// Show every version instead of only the most recent ones.
        #[arg(long)]
        all: bool,
    },
    /// Show which tool versions are active in the current project,
    /// and where each one comes from.
    Show,
    /// Remove a tool version from the machine-wide cache.
    Remove {
        /// The `<tool>@<version>` specification (e.g.
        /// `charon@nightly-2026.07.01`) to remove from the cache.
        spec: String,
    },
    /// Delete the entire tool cache. Later runs download what they need
    /// again.
    Clean,
    /// Write version pins into the project's `hax.toml`, creating the
    /// file when missing.
    Pin {
        /// A `<name>@<version>` specification to write as one entry,
        /// accepting managed tools (e.g. `charon@nightly-2026.07.01`)
        /// and declared versions (e.g. `lean@leanprover/lean4:v4.31.0`).
        /// When absent, pins the built-in defaults of this release.
        spec: Option<String>,
    },
}

impl<E: Extension> Command<E> {
    pub fn body_kinds(&self) -> Vec<ExportBodyKind> {
        match self {
            Command::JSON { kind, .. } => kind.clone(),
            Command::Serialize { kind, .. } => kind.clone(),
            Command::Backend { .. }
            | Command::Extract { .. }
            | Command::Tools { .. }
            | Command::CliExtension { .. } => {
                vec![ExportBodyKind::Thir]
            }
        }
    }
    pub fn backend_name(&self) -> Option<BackendName> {
        match self {
            Command::Backend(backend_options) => Some((&backend_options.backend).into()),
            Command::JSON { .. } => None,
            Command::Serialize { backend, .. } => backend.clone(),
            Command::Extract { .. } => None,
            Command::Tools(_) => None,
            Command::CliExtension(_) => None,
        }
    }
}

#[derive_group(Serializers)]
#[derive(JsonSchema, ValueEnum, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExportBodyKind {
    Thir,
    MirBuilt,
}

#[derive_group(Serializers)]
#[derive(JsonSchema, Parser, Debug, Clone)]
#[command(
    author,
    version = crate::HAX_VERSION,
    long_version = concat!("\nversion=", env!("HAX_VERSION"), "\n", "commit=", env!("HAX_GIT_COMMIT_HASH")),
    name = "hax",
    about,
    long_about = None
)]
pub struct ExtensibleOptions<E: Extension> {
    /// Semi-colon terminated list of arguments to pass to the
    /// `cargo build` invocation. For example, to apply this
    /// program on a package `foo`, use `-C -p foo ;`. (make sure
    /// to escape `;` correctly in your shell)
    #[arg(default_values = Vec::<&str>::new(), short='C', allow_hyphen_values=true, num_args=1.., long="cargo-args", value_terminator=";")]
    pub cargo_flags: Vec<String>,

    #[command(subcommand)]
    pub command: Command<E>,

    /// `cargo` caching is enable by default, this flag disables it.
    #[arg(long="disable-cargo-cache", action=clap::builder::ArgAction::SetFalse)]
    pub force_cargo_build: ForceCargoBuild,

    /// Apply the command to every local package of the dependency closure. By
    /// default, the command is only applied to the primary packages (i.e. the
    /// package(s) of the current directory, or the ones selected with cargo
    /// options like `-C -p <PKG> ;`).
    #[arg(long = "deps")]
    pub deps: bool,

    /// Provide a precomputed haxmeta file explicitly.
    /// Setting this option bypasses rustc and the exporter altogether.
    #[arg(long)]
    #[clap(hide = true)]
    pub haxmeta: Option<PathBuf>,

    /// By default, hax uses `$CARGO_TARGET_DIR/hax` as target folder,
    /// to avoid recompilation when working both with `cargo hax` and
    /// `cargo build` (or, e.g. `rust-analyzer`). This option disables
    /// this behavior.
    #[arg(long)]
    pub no_custom_target_directory: bool,

    /// Diagnostic format. Sets `cargo`'s `--message-format` as well,
    /// if not present.
    #[arg(long, default_value = "human")]
    pub message_format: MessageFormat,

    /// Enables experimental FullDef format for items exported from the frontend
    /// in the haxmeta file.
    #[arg(long, env = "HAX_EXPERIMENTAL_FULL_DEF")]
    pub experimental_full_def: bool,

    #[group(flatten)]
    pub extension: E::Options,
}

pub type Options = ExtensibleOptions<()>;

#[derive_group(Serializers)]
#[derive(JsonSchema, ValueEnum, Debug, Clone, Copy, Eq, PartialEq)]
pub enum MessageFormat {
    Human,
    Json,
}

impl<E: Extension> NormalizePaths for Command<E> {
    fn normalize_paths(&mut self) {
        use Command::*;
        match self {
            JSON { output_file, .. } => output_file.normalize_paths(),
            _ => (),
        }
    }
}

impl NormalizePaths for Options {
    fn normalize_paths(&mut self) {
        self.command.normalize_paths()
    }
}

impl From<Options> for hax_frontend_exporter_options::Options {
    fn from(_opts: Options) -> hax_frontend_exporter_options::Options {
        hax_frontend_exporter_options::Options {
            inline_anon_consts: true,
            bounds_options: hax_frontend_exporter_options::BoundsOptions {
                resolve_destruct: false,
                prune_sized: true,
            },
            item_ref_use_concrete_impl: false,
        }
    }
}

/// The subset of `Options` the frontend is sensible to.
#[derive_group(Serializers)]
#[derive(JsonSchema, Debug, Clone)]
pub struct ExporterOptions {
    pub deps: bool,
    pub force_cargo_build: ForceCargoBuild,
    /// When exporting, the driver sets `--cfg hax_backend_{backkend}`, thus we need this information.
    pub backend: Option<BackendName>,
    pub body_kinds: Vec<ExportBodyKind>,
    pub experimental_full_def: bool,
}

#[derive_group(Serializers)]
#[derive(JsonSchema, ValueEnum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendName {
    Fstar,
    Coq,
    Ssprove,
    Easycrypt,
    #[clap(alias("proverif"))]
    ProVerif,
    LegacyLean,
    Lean,
    Rust,
    GenerateRustEngineNames,
    Debugger,
}

impl BackendName {
    /// The subdirectory holding the backend's extraction, below the output
    /// directory. The engine backends keep an `extraction/` level; the Lean
    /// backend writes its package at the top of the output directory.
    pub fn output_subdir(self) -> Option<&'static str> {
        match self {
            Self::Lean => None,
            _ => Some("extraction"),
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Fstar,
            Self::Coq,
            Self::Ssprove,
            Self::Easycrypt,
            Self::ProVerif,
            Self::LegacyLean,
            Self::Lean,
            Self::Rust,
            Self::GenerateRustEngineNames,
        ]
        .into_iter()
    }
}

impl fmt::Display for BackendName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            BackendName::Fstar => "fstar",
            BackendName::Coq => "coq",
            BackendName::Ssprove => "ssprove",
            BackendName::Easycrypt => "easycrypt",
            BackendName::ProVerif => "proverif",
            BackendName::LegacyLean => "legacy-lean",
            BackendName::Lean => "lean",
            BackendName::Rust => "rust",
            BackendName::GenerateRustEngineNames => "generate_rust_engine_names",
            BackendName::Debugger => "debugger",
        };
        write!(f, "{name}")
    }
}

impl From<&Options> for ExporterOptions {
    fn from(options: &Options) -> Self {
        ExporterOptions {
            deps: options.deps,
            force_cargo_build: options.force_cargo_build.clone(),
            backend: options.command.backend_name(),
            body_kinds: options.command.body_kinds(),
            experimental_full_def: options.experimental_full_def,
        }
    }
}

impl From<&Backend> for BackendName {
    fn from(backend: &Backend) -> Self {
        match backend {
            Backend::Fstar { .. } => BackendName::Fstar,
            Backend::Coq { .. } => BackendName::Coq,
            Backend::Ssprove { .. } => BackendName::Ssprove,
            Backend::Easycrypt { .. } => BackendName::Easycrypt,
            Backend::ProVerif { .. } => BackendName::ProVerif,
            Backend::LegacyLean { .. } => BackendName::LegacyLean,
            Backend::Lean { .. } => BackendName::Lean,
            Backend::Rust { .. } => BackendName::Rust,
            Backend::GenerateRustEngineNames { .. } => BackendName::GenerateRustEngineNames,
            Backend::Debugger { .. } => BackendName::Debugger,
        }
    }
}

pub const ENV_VAR_OPTIONS_FRONTEND: &str = "DRIVER_HAX_FRONTEND_OPTS";
pub const ENV_VAR_OPTIONS_FULL: &str = "DRIVER_HAX_FRONTEND_FULL_OPTS";

#[cfg(test)]
mod tests {
    use super::*;

    /// `flags()` renders the options for `extract --dry-run`. The
    /// destructuring makes a new field a compile error here, and each field
    /// is asserted to reach the rendering, which nothing else enforces.
    #[test]
    fn fstar_flags_render_every_field() {
        let options = FStarOptions {
            z3rlimit: 111,
            fuel: 222,
            ifuel: 333,
            interfaces: vec![parse_inclusion_clause("+**::foo").unwrap()],
            line_width: 444,
        };
        let FStarOptions {
            z3rlimit,
            fuel,
            ifuel,
            interfaces,
            line_width,
        } = &options;
        let flags = options.flags().join(" ");
        assert!(flags.contains(&format!("--z3rlimit={z3rlimit}")), "{flags}");
        assert!(flags.contains(&format!("--fuel={fuel}")), "{flags}");
        assert!(flags.contains(&format!("--ifuel={ifuel}")), "{flags}");
        assert!(
            flags.contains(&format!("--line-width={line_width}")),
            "{flags}"
        );
        assert!(flags.contains("--interfaces"), "{flags}");
        for clause in interfaces {
            assert!(flags.contains(&clause.to_string()), "{flags}");
        }
    }

    #[test]
    fn proverif_flags_render_every_field() {
        let options = ProVerifOptions {
            assume_items: vec![parse_inclusion_clause("+**::bar").unwrap()],
        };
        let ProVerifOptions { assume_items } = &options;
        let flags = options.flags().join(" ");
        assert!(flags.contains("--assume-items"), "{flags}");
        for clause in assume_items {
            assert!(flags.contains(&clause.to_string()), "{flags}");
        }
    }

    /// The Lean backend writes its package at the top of the output
    /// directory; every engine backend keeps an `extraction/` level.
    #[test]
    fn only_the_lean_backend_has_no_extraction_subdir() {
        for backend in BackendName::iter() {
            let subdir = backend.output_subdir();
            if backend == BackendName::Lean {
                assert_eq!(subdir, None, "{backend}");
            } else {
                assert_eq!(subdir, Some("extraction"), "{backend}");
            }
        }
    }
}
