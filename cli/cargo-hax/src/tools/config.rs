//! Parsing of `hax.toml` files.
//!
//! A `hax.toml` carries a `[tools]` table (managed tools, pinned by version
//! or pointed at a local binary by path), a `[versions]` table
//! (declared-only versions), a top-level `project-files` key
//! (proof-project file generation), `[scenario.<name>]` tables (named
//! extraction configurations, run by `cargo hax extract`), and
//! `[scenario-defaults.<backend>]` tables (project-level extensions of the
//! built-in default opaque set). Unknown top-level keys, unknown tools, and
//! unknown keys inside a `[tools]` entry are warned about and skipped, so
//! files written for a newer hax remain readable by an older one. Malformed
//! entries are hard errors, as are unknown keys inside a scenario or
//! scenario-defaults table: silently ignoring a misspelled selection key
//! would change extraction semantics.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use path_clean::PathClean;

use hax_types::cli_options::{
    BackendName, ENV_VAR_OPTIONS_FRONTEND, ENV_VAR_OPTIONS_FULL, InclusionClause,
    parse_inclusion_clause,
};

use super::{DECLARED_VERSION_KEYS, MANAGED_TOOLS};

/// A `[tools]` entry, mirroring Cargo's dependency syntax: a plain version
/// string, or a table with exactly one of `version` and `path`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolEntry {
    Version(String),
    /// Path to an existing executable, used as-is. Resolved against the
    /// directory of the defining `hax.toml` if relative.
    Path(PathBuf),
}

/// The backends a `[scenario.<name>]` table can declare.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScenarioBackend {
    #[default]
    Lean,
    Fstar,
    Coq,
    Ssprove,
    Easycrypt,
    Proverif,
}

/// The corresponding backend of the full `BackendName` list. The scenario
/// backend names are derived from this mapping, so they cannot drift from
/// the names the rest of the CLI uses.
impl From<ScenarioBackend> for BackendName {
    fn from(backend: ScenarioBackend) -> Self {
        match backend {
            ScenarioBackend::Lean => BackendName::Lean,
            ScenarioBackend::Fstar => BackendName::Fstar,
            ScenarioBackend::Coq => BackendName::Coq,
            ScenarioBackend::Ssprove => BackendName::Ssprove,
            ScenarioBackend::Easycrypt => BackendName::Easycrypt,
            ScenarioBackend::Proverif => BackendName::ProVerif,
        }
    }
}

impl ScenarioBackend {
    pub const ALL: [Self; 6] = [
        Self::Lean,
        Self::Fstar,
        Self::Coq,
        Self::Ssprove,
        Self::Easycrypt,
        Self::Proverif,
    ];

    pub fn name(self) -> String {
        BackendName::from(self).to_string()
    }

    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|backend| backend.name() == name)
    }

    /// Parse a backend name, with the error message every caller shares;
    /// callers prepend their own context.
    fn parse_or_err(name: &str) -> Result<Self, String> {
        Self::parse(name).ok_or_else(|| {
            format!(
                "`{name}` is not a backend that scenarios support; expected one of {}",
                Self::names()
            )
        })
    }

    /// The backend names, joined for error messages.
    fn names() -> String {
        Self::ALL.map(Self::name).join(", ")
    }
}

impl std::fmt::Display for ScenarioBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}

/// Deserialized through [`ScenarioBackend::parse`], so the embedded
/// defaults are keyed by the same names a `hax.toml` uses.
impl<'de> serde::Deserialize<'de> for ScenarioBackend {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let name = String::deserialize(deserializer)?;
        Self::parse_or_err(&name).map_err(serde::de::Error::custom)
    }
}

/// One `[scenario.<name>]` table: a complete, named extraction
/// configuration.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScenarioEntry {
    pub name: String,
    pub backend: ScenarioBackend,
    /// The package the scenario extracts; optional where the invocation
    /// determines it (a member-level scenario extracts its own member, and
    /// a single-package project has only one candidate).
    pub package: Option<String>,
    /// Overrides the default `proofs/<name>/<backend>` layout; resolved
    /// relative to the root of the extracted package.
    pub output_dir: Option<PathBuf>,
    pub features: Vec<String>,
    pub all_features: bool,
    pub no_default_features: bool,
    /// Environment entries set for every process a scenario run spawns.
    pub env: BTreeMap<String, String>,
    /// Unified item selection (Lean only in this version): root patterns,
    /// patterns to drop, and patterns to extract signature-only, in
    /// Charon's name-pattern language.
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub opaque: Vec<String>,
    /// Whether the combined default opaque set applies.
    pub default_opaques: bool,
    /// `-i` clauses, for the engine backends.
    pub select_clauses: Vec<InclusionClause>,
    pub z3rlimit: Option<u32>,
    pub fuel: Option<u32>,
    pub ifuel: Option<u32>,
    /// F* `--interfaces` clauses.
    pub interfaces: Vec<InclusionClause>,
    pub line_width: Option<u16>,
    /// Verbatim extra arguments, one array element per process argument.
    pub charon_args: Vec<String>,
    pub aeneas_args: Vec<String>,
    /// Overrides the top-level `project-files` key.
    pub project_files: Option<bool>,
    /// ProVerif `--assume-items` clauses.
    pub assume_items: Vec<InclusionClause>,
}

impl ScenarioEntry {
    fn new(name: &str, backend: ScenarioBackend) -> Self {
        ScenarioEntry {
            name: name.to_string(),
            backend,
            // The combined default opaque set applies unless the scenario
            // opts out.
            default_opaques: true,
            ..ScenarioEntry::default()
        }
    }
}

/// The parsed contents of one `hax.toml`.
#[derive(Debug, Clone, Default)]
pub struct HaxToml {
    /// The file this was parsed from.
    pub path: PathBuf,
    pub tools: BTreeMap<String, ToolEntry>,
    pub versions: BTreeMap<String, String>,
    /// The top-level `project-files` key: whether hax generates and checks
    /// the proof-project files around an extraction. Backend-neutral; a
    /// member-level value overrides a workspace-level one.
    pub project_files: Option<bool>,
    /// The `[scenario.<name>]` tables, by name (names are unique per file:
    /// TOML rejects a duplicate table).
    pub scenarios: BTreeMap<String, ScenarioEntry>,
    /// The `[scenario-defaults.<backend>]` tables: extra opaque patterns
    /// extending the built-in default opaque set.
    pub scenario_defaults: BTreeMap<ScenarioBackend, Vec<String>>,
}

impl HaxToml {
    /// Whether this file carries any entry that affects resolution.
    pub fn has_entries(&self) -> bool {
        !self.tools.is_empty() || !self.versions.is_empty()
    }

    /// The names of all entries, for the member-override warning.
    pub fn entry_names(&self) -> Vec<String> {
        self.tools
            .keys()
            .chain(self.versions.keys())
            .cloned()
            .collect()
    }
}

/// Parse a `hax.toml`. Returns the parsed file and a list of non-fatal
/// warnings; a malformed entry is a hard error.
pub fn parse(path: &Path, contents: &str) -> Result<(HaxToml, Vec<String>), String> {
    let table: toml::Table = contents.parse().map_err(|e| format!("invalid TOML: {e}"))?;
    let dir = path.parent().unwrap_or(Path::new("."));
    let mut warnings = Vec::new();
    let mut result = HaxToml {
        path: path.to_path_buf(),
        ..HaxToml::default()
    };

    for (key, value) in table {
        match key.as_str() {
            "tools" => {
                let Some(tools) = value.as_table() else {
                    return Err("`[tools]` must be a table".into());
                };
                for (tool, entry) in tools {
                    if !MANAGED_TOOLS.contains(&tool.as_str()) {
                        warnings.push(format!(
                            "tool `{tool}` is not managed by this version of hax; \
                             its entry is ignored"
                        ));
                        continue;
                    }
                    let entry = parse_tool_entry(tool, entry, dir, &mut warnings)?;
                    result.tools.insert(tool.clone(), entry);
                }
            }
            "versions" => {
                let Some(versions) = value.as_table() else {
                    return Err("`[versions]` must be a table".into());
                };
                for (name, value) in versions {
                    if !DECLARED_VERSION_KEYS.contains(&name.as_str()) {
                        warnings.push(format!(
                            "`[versions]` key `{name}` is not known to this version of hax; \
                             it is ignored"
                        ));
                        continue;
                    }
                    let Some(version) = value.as_str() else {
                        return Err(format!(
                            "`[versions]` entry `{name}` must be a version string"
                        ));
                    };
                    validate_declared_version(name, version)?;
                    result.versions.insert(name.clone(), version.to_string());
                }
            }
            "project-files" => {
                let Some(enabled) = value.as_bool() else {
                    return Err("`project-files` must be a boolean".into());
                };
                result.project_files = Some(enabled);
            }
            "scenario" => {
                let Some(scenarios) = value.as_table() else {
                    return Err("`[scenario]` entries must be tables (`[scenario.<name>]`)".into());
                };
                for (name, entry) in scenarios {
                    let entry = parse_scenario(name, entry)?;
                    result.scenarios.insert(name.clone(), entry);
                }
            }
            "scenario-defaults" => {
                let Some(defaults) = value.as_table() else {
                    return Err("`[scenario-defaults]` entries must be tables \
                         (`[scenario-defaults.<backend>]`)"
                        .into());
                };
                for (backend, entry) in defaults {
                    let (backend, opaque) = parse_scenario_defaults(backend, entry)?;
                    result.scenario_defaults.insert(backend, opaque);
                }
            }
            other => {
                warnings.push(format!(
                    "unknown top-level key `{other}` is ignored by this version of hax"
                ));
            }
        }
    }

    Ok((result, warnings))
}

fn parse_tool_entry(
    tool: &str,
    entry: &toml::Value,
    dir: &Path,
    warnings: &mut Vec<String>,
) -> Result<ToolEntry, String> {
    // A pinned version is checked here rather than only where it is
    // installed, so an unusable pin is reported as the configuration error
    // it is, by every command that reads the file.
    let version_entry = |version: &str| {
        super::manifest::validate_version_id(version)
            .map_err(|e| format!("the entry for tool `{tool}` is invalid: {e}"))?;
        Ok(ToolEntry::Version(version.to_string()))
    };
    match entry {
        toml::Value::String(version) => version_entry(version),
        toml::Value::Table(table) => {
            for key in table.keys() {
                if key != "version" && key != "path" {
                    warnings.push(format!(
                        "unknown key `{key}` in the entry for tool `{tool}` is ignored"
                    ));
                }
            }
            let version = table.get("version");
            let path = table.get("path");
            match (version, path) {
                (Some(_), Some(_)) => Err(format!(
                    "the entry for tool `{tool}` declares both `version` and `path`; \
                     declare exactly one"
                )),
                (Some(version), None) => {
                    let Some(version) = version.as_str() else {
                        return Err(format!("`version` of tool `{tool}` must be a string"));
                    };
                    version_entry(version)
                }
                (None, Some(path)) => {
                    let Some(path) = path.as_str() else {
                        return Err(format!("`path` of tool `{tool}` must be a string"));
                    };
                    let path = Path::new(path);
                    let resolved = if path.is_absolute() {
                        path.to_path_buf()
                    } else {
                        dir.join(path)
                    };
                    Ok(ToolEntry::Path(resolved))
                }
                (None, None) => Err(format!(
                    "the entry for tool `{tool}` must declare exactly one of \
                     `version` and `path`"
                )),
            }
        }
        _ => Err(format!(
            "the entry for tool `{tool}` must be a version string or a table \
             with exactly one of `version` and `path`"
        )),
    }
}

/// Parse a TOML value as an array of strings. `context` names the
/// enclosing table for the error message.
fn parse_string_list(context: &str, key: &str, value: &toml::Value) -> Result<Vec<String>, String> {
    value
        .as_array()
        .and_then(|entries| {
            entries
                .iter()
                .map(|entry| entry.as_str().map(str::to_string))
                .collect()
        })
        .ok_or_else(|| format!("{context}: `{key}` must be an array of strings"))
}

/// Check a scenario name against the grammar
/// `[a-z][a-z0-9]*(-[a-z][a-z0-9]*)*`: lowercase alphanumeric segments
/// starting with a letter, separated by single hyphens. Names double as
/// directory names and, CamelCased, as Lean package names; the grammar
/// makes the CamelCase conversion injective, so two scenarios can never
/// yield the same package name.
fn valid_scenario_name(name: &str) -> bool {
    !name.is_empty()
        && name.split('-').all(|segment| {
            segment
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_lowercase())
                && segment
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
}

/// Validate a scenario name: the grammar above, no collision with a
/// backend name (a scenario named like a backend would nest its default
/// output directory inside the scenario-less layout and make the two
/// indistinguishable), and no CamelCase collision with a Lean module root
/// present in every generated package.
pub fn validate_scenario_name(name: &str) -> Result<(), String> {
    if !valid_scenario_name(name) {
        return Err(format!(
            "`{name}` is not a valid scenario name: expected lowercase \
             alphanumeric segments starting with a letter, separated by \
             single hyphens (e.g. `treemath` or `book-fstar`)"
        ));
    }
    if BackendName::iter().any(|backend| backend.to_string() == name) {
        return Err(format!(
            "`{name}` cannot name a scenario: it is a backend name, and the \
             scenario's default output directory would be indistinguishable \
             from the scenario-less `proofs/{name}/` layout; rename the \
             scenario"
        ));
    }
    let camel = hax_types::lean::to_camel_case(name);
    if crate::aeneas::package::RESERVED_MODULE_ROOTS.contains(&camel.as_str()) {
        return Err(format!(
            "`{name}` cannot name a scenario: as a Lean package name it \
             becomes `{camel}`, a module root present in every generated \
             Lean package; rename the scenario"
        ));
    }
    Ok(())
}

/// Validate a scenario's `output-dir`: a relative path that names a
/// directory of its own. A `..` component is allowed, so several members
/// can extract into one shared tree, but an absolute path would discard the
/// package root the key is documented to be relative to, a path resolving
/// to the package root itself would scaffold the generated Lean package
/// over the crate's own source directory, and a path made of `..`
/// components only names an ancestor directory that exists for other
/// reasons. Where the resolved directory lands relative to the workspace is
/// checked later, against the loaded project.
fn validate_output_dir(context: &str, value: String) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);
    let cleaned = path.clean();
    let reason = if path.is_absolute() {
        "it is absolute"
    } else if path.as_os_str().is_empty() || cleaned == Path::new(".") {
        "it resolves to the root of the extracted package"
    } else if cleaned
        .components()
        .all(|c| c == std::path::Component::ParentDir)
    {
        "it resolves to an ancestor of the extracted package's root"
    } else {
        return Ok(path);
    };
    Err(format!(
        "{context}: `output-dir` must be a relative path naming a directory \
         of its own, but {reason}"
    ))
}

/// Parse one `[scenario.<name>]` table. Every key is validated against the
/// declared backend; an inapplicable or unknown key is a hard error.
fn parse_scenario(name: &str, value: &toml::Value) -> Result<ScenarioEntry, String> {
    let context = format!("`[scenario.{name}]`");
    validate_scenario_name(name)?;
    let Some(table) = value.as_table() else {
        return Err(format!("{context} must be a table"));
    };

    let backend = match table.get("backend") {
        Some(toml::Value::String(backend)) => backend.as_str(),
        Some(_) => return Err(format!("{context}: `backend` must be a string")),
        None => return Err(format!("{context}: the `backend` key is required")),
    };
    let backend = ScenarioBackend::parse_or_err(backend).map_err(|e| format!("{context}: {e}"))?;

    let string = |key: &str, value: &toml::Value| {
        value
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| format!("{context}: `{key}` must be a string"))
    };
    let boolean = |key: &str, value: &toml::Value| {
        value
            .as_bool()
            .ok_or_else(|| format!("{context}: `{key}` must be a boolean"))
    };
    let integer = |key: &str, value: &toml::Value, max: i64| {
        value
            .as_integer()
            .filter(|n| (0..=max).contains(n))
            .ok_or_else(|| format!("{context}: `{key}` must be an integer between 0 and {max}"))
    };
    let string_list = |key: &str, value: &toml::Value| -> Result<Vec<String>, String> {
        parse_string_list(&context, key, value)
    };
    // `-i`-style clause lists are parsed here so a malformed clause fails
    // where it is written, not in the middle of a run.
    let clause_list = |key: &str, value: &toml::Value| -> Result<Vec<InclusionClause>, String> {
        string_list(key, value)?
            .iter()
            .map(|clause| {
                parse_inclusion_clause(clause)
                    .map_err(|e| format!("{context}: `{key}` entry `{clause}` is invalid: {e}"))
            })
            .collect()
    };
    // A structured key that does not apply to the declared backend is a
    // hard error: it would silently not do what it says.
    let only = |applies: bool, key: &str, note: &str| {
        if applies {
            Ok(())
        } else {
            Err(format!(
                "{context}: `{key}` does not apply to the `{backend}` backend{note}"
            ))
        }
    };
    let lean = backend == ScenarioBackend::Lean;
    let fstar = backend == ScenarioBackend::Fstar;
    // The unified item selection is implemented for the Lean backend only;
    // using it elsewhere errors rather than being ignored, reserving its
    // semantics for the later extension.
    let unified = |key: &str| {
        only(
            lean,
            key,
            ": the unified item selection is not supported yet there; \
             use `select-clauses`",
        )
    };

    let mut entry = ScenarioEntry::new(name, backend);
    for (key, value) in table {
        match key.as_str() {
            "backend" => {}
            "package" => entry.package = Some(string(key, value)?),
            "output-dir" => {
                entry.output_dir = Some(validate_output_dir(&context, string(key, value)?)?)
            }
            "features" => entry.features = string_list(key, value)?,
            "all-features" => entry.all_features = boolean(key, value)?,
            "no-default-features" => entry.no_default_features = boolean(key, value)?,
            "env" => {
                let Some(env) = value.as_table() else {
                    return Err(format!("{context}: `env` must be a table"));
                };
                for (var, value) in env {
                    // The variables hax communicates through internally are
                    // reserved: an `env` entry would silently replace the
                    // serialized options of the re-entered run.
                    if var == ENV_VAR_OPTIONS_FULL || var == ENV_VAR_OPTIONS_FRONTEND {
                        return Err(format!(
                            "{context}: `env` must not set `{var}`: hax uses \
                             it to carry its own options"
                        ));
                    }
                    entry.env.insert(var.clone(), string("env", value)?);
                }
            }
            "include" => {
                unified(key)?;
                entry.include = string_list(key, value)?;
            }
            "exclude" => {
                unified(key)?;
                entry.exclude = string_list(key, value)?;
            }
            "opaque" => {
                unified(key)?;
                entry.opaque = string_list(key, value)?;
            }
            "default-opaques" => {
                unified(key)?;
                entry.default_opaques = boolean(key, value)?;
            }
            "select-clauses" => {
                only(
                    !lean,
                    key,
                    "; use the unified `include`/`exclude`/`opaque` keys",
                )?;
                entry.select_clauses = clause_list(key, value)?;
            }
            "z3rlimit" => {
                only(fstar, key, "")?;
                entry.z3rlimit = Some(integer(key, value, u32::MAX as i64)? as u32);
            }
            "fuel" => {
                only(fstar, key, "")?;
                entry.fuel = Some(integer(key, value, u32::MAX as i64)? as u32);
            }
            "ifuel" => {
                only(fstar, key, "")?;
                entry.ifuel = Some(integer(key, value, u32::MAX as i64)? as u32);
            }
            "interfaces" => {
                only(fstar, key, "")?;
                entry.interfaces = clause_list(key, value)?;
            }
            "line-width" => {
                only(fstar, key, "")?;
                entry.line_width = Some(integer(key, value, u16::MAX as i64)? as u16);
            }
            "charon-args" => {
                only(lean, key, "")?;
                entry.charon_args = string_list(key, value)?;
            }
            "aeneas-args" => {
                only(lean, key, "")?;
                entry.aeneas_args = string_list(key, value)?;
            }
            "project-files" => {
                only(lean, key, "")?;
                entry.project_files = Some(boolean(key, value)?);
            }
            "assume-items" => {
                only(backend == ScenarioBackend::Proverif, key, "")?;
                entry.assume_items = clause_list(key, value)?;
            }
            other => {
                return Err(format!(
                    "{context}: unknown key `{other}`; this version of hax \
                     rejects it because silently ignoring it could change \
                     what is extracted"
                ));
            }
        }
    }
    Ok(entry)
}

/// Parse one `[scenario-defaults.<backend>]` table: `opaque` is the only
/// key accepted in this version, and only the `lean` backend consumes the
/// unified item selection; other backends are rejected rather than
/// silently ignored.
fn parse_scenario_defaults(
    backend: &str,
    value: &toml::Value,
) -> Result<(ScenarioBackend, Vec<String>), String> {
    let context = format!("`[scenario-defaults.{backend}]`");
    let backend = ScenarioBackend::parse_or_err(backend).map_err(|e| format!("{context}: {e}"))?;
    if backend != ScenarioBackend::Lean {
        return Err(format!(
            "{context}: scenario defaults apply to the `lean` backend only \
             in this version; the unified item selection is not supported \
             yet for the `{backend}` backend"
        ));
    }
    let Some(table) = value.as_table() else {
        return Err(format!("{context} must be a table"));
    };
    let mut opaque = Vec::new();
    for (key, value) in table {
        if key != "opaque" {
            return Err(format!(
                "{context}: unknown key `{key}`; `opaque` is the only key \
                 supported here"
            ));
        }
        opaque = parse_string_list(&context, "opaque", value)?;
    }
    Ok((backend, opaque))
}

/// Validate a `[versions]` value. These versions are not installed by hax,
/// but they are written into the files it generates: a `lean-toolchain`
/// holds one verbatim, and a `lakefile.toml` names one as a `rev`. A value
/// must therefore not be able to restructure those files, which the
/// accepted character set rules out (no quotes, whitespace, comment
/// markers, or control characters), and must not traverse a path.
pub fn validate_declared_version(key: &str, value: &str) -> Result<(), String> {
    let allowed =
        |c: char| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | '+' | ':' | '/');
    let valid = !value.is_empty()
        && value.len() <= 128
        && value.chars().all(allowed)
        && !value.contains("..")
        && !value.starts_with(['.', '-', '/'])
        && !value.ends_with('/');
    if valid {
        Ok(())
    } else {
        Err(format!(
            "`{value}` is not a valid value for the `[versions]` entry `{key}`: expected a \
             version, tag, or toolchain name of ASCII alphanumerics and `.-_+:/`"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_str(contents: &str) -> Result<(HaxToml, Vec<String>), String> {
        parse(Path::new("/project/hax.toml"), contents)
    }

    #[test]
    fn string_and_table_forms_are_equivalent() {
        let (a, w) = parse_str(
            r#"[tools]
aeneas = "nightly-2026.07.01""#,
        )
        .unwrap();
        let (b, _) = parse_str(
            r#"[tools]
aeneas = { version = "nightly-2026.07.01" }"#,
        )
        .unwrap();
        assert!(w.is_empty());
        assert_eq!(a.tools, b.tools);
        assert_eq!(
            a.tools["aeneas"],
            ToolEntry::Version("nightly-2026.07.01".into())
        );
    }

    #[test]
    fn relative_path_resolves_against_defining_dir() {
        let (parsed, _) = parse_str(
            r#"[tools]
charon = { path = "vendor/bin/charon" }"#,
        )
        .unwrap();
        assert_eq!(
            parsed.tools["charon"],
            ToolEntry::Path(PathBuf::from("/project/vendor/bin/charon"))
        );
    }

    #[test]
    fn absolute_path_is_kept() {
        let (parsed, _) = parse_str(
            r#"[tools]
charon = { path = "/usr/bin/charon" }"#,
        )
        .unwrap();
        assert_eq!(
            parsed.tools["charon"],
            ToolEntry::Path(PathBuf::from("/usr/bin/charon"))
        );
    }

    #[test]
    fn version_and_path_together_is_an_error() {
        let err = parse_str(
            r#"[tools]
charon = { version = "x", path = "y" }"#,
        )
        .unwrap_err();
        assert!(err.contains("both `version` and `path`"), "{err}");
    }

    #[test]
    fn neither_version_nor_path_is_an_error() {
        let err = parse_str("[tools]\ncharon = {}").unwrap_err();
        assert!(err.contains("exactly one"), "{err}");
    }

    #[test]
    fn unknown_tool_is_warned_and_skipped() {
        let (parsed, warnings) = parse_str(
            r#"[tools]
lean = "4.31.0""#,
        )
        .unwrap();
        assert!(parsed.tools.is_empty());
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("`lean`"), "{}", warnings[0]);
    }

    #[test]
    fn unknown_entry_key_is_warned() {
        let (parsed, warnings) = parse_str(
            r#"[tools]
charon = { version = "x", features = ["y"] }"#,
        )
        .unwrap();
        assert_eq!(parsed.tools["charon"], ToolEntry::Version("x".into()));
        assert!(warnings.iter().any(|w| w.contains("`features`")));
    }

    #[test]
    fn unknown_top_level_key_is_warned() {
        let (parsed, warnings) = parse_str(r#"hax = "0.3.7""#).unwrap();
        assert!(!parsed.has_entries());
        assert!(warnings.iter().any(|w| w.contains("`hax`")));
    }

    #[test]
    fn versions_table_parses_known_keys() {
        let (parsed, warnings) = parse_str(
            r#"[versions]
lean = "leanprover/lean4:v4.30.0-rc2"
hax-lean-lib = "v0.1.0"
unknown-thing = "1""#,
        )
        .unwrap();
        assert_eq!(parsed.versions.len(), 2);
        assert_eq!(parsed.versions["lean"], "leanprover/lean4:v4.30.0-rc2");
        assert!(warnings.iter().any(|w| w.contains("`unknown-thing`")));
    }

    #[test]
    fn non_string_version_is_an_error() {
        assert!(parse_str("[versions]\nlean = 4").is_err());
    }

    #[test]
    fn tool_versions_are_validated() {
        for entry in [
            r#"charon = "../escape""#,
            r#"charon = "nightly 1""#,
            r#"charon = { version = "a/b" }"#,
            r#"charon = """#,
        ] {
            let err = parse_str(&format!("[tools]\n{entry}")).unwrap_err();
            assert!(err.contains("`charon`"), "{entry}: {err}");
        }
    }

    #[test]
    fn declared_versions_are_validated() {
        let valid = [
            "leanprover/lean4:v4.31.0",
            "v0.2.0",
            "refs/tags/v1",
            "4.31.0-rc2+build_1",
        ];
        for value in valid {
            validate_declared_version("lean", value).unwrap();
        }
        let invalid = [
            // Closing a TOML string and appending to the generated file.
            "v1\"\n[[require]]\nname = \"X\"\ngit = \"https://evil.example/x\"\nrev = \"main",
            "v1 # comment",
            "v1\nsecond-line",
            "../escape",
            "a/../../escape",
            "-leading-dash",
            ".hidden",
            "trailing/",
            "",
        ];
        for value in invalid {
            assert!(
                validate_declared_version("lean", value).is_err(),
                "accepted `{value}`"
            );
        }
        assert!(validate_declared_version("lean", &"v".repeat(129)).is_err());
    }

    /// A `[versions]` value that could restructure a generated file is
    /// rejected where it enters, so no consumer has to defend against it.
    #[test]
    fn injecting_declared_version_is_an_error() {
        let err = parse_str(
            r#"[versions]
hax-lean-lib = "v1\"\ngit = \"https://evil.example/x\"\nrev = \"main""#,
        )
        .unwrap_err();
        assert!(err.contains("`hax-lean-lib`"), "{err}");
    }

    #[test]
    fn project_files_parses_both_values_and_defaults_to_none() {
        let (parsed, warnings) = parse_str("project-files = false").unwrap();
        assert_eq!(parsed.project_files, Some(false));
        assert!(warnings.is_empty());
        let (parsed, _) = parse_str("project-files = true").unwrap();
        assert_eq!(parsed.project_files, Some(true));
        let (parsed, _) = parse_str("").unwrap();
        assert_eq!(parsed.project_files, None);
    }

    #[test]
    fn non_boolean_project_files_is_an_error() {
        let err = parse_str(r#"project-files = "no""#).unwrap_err();
        assert!(err.contains("`project-files`"), "{err}");
    }

    #[test]
    fn empty_file_is_valid() {
        let (parsed, warnings) = parse_str("").unwrap();
        assert!(!parsed.has_entries());
        assert!(warnings.is_empty());
    }

    #[test]
    fn a_full_lean_scenario_parses() {
        let (parsed, warnings) = parse_str(
            r#"[scenario.treemath]
backend = "lean"
package = "openmls"
include = ["openmls::treemath"]
exclude = ["openmls::treemath::internal"]
opaque = ["{impl tls_codec::Size for _}"]
default-opaques = false
charon-args = ["--some-rare-flag"]
aeneas-args = ["-flag"]
output-dir = "proofs/treemath/lean"
features = ["fast"]
all-features = false
no-default-features = true
project-files = true

[scenario.treemath.env]
RUSTFLAGS = ""
"#,
        )
        .unwrap();
        assert!(warnings.is_empty(), "{warnings:?}");
        let entry = &parsed.scenarios["treemath"];
        assert_eq!(entry.name, "treemath");
        assert_eq!(entry.backend, ScenarioBackend::Lean);
        assert_eq!(entry.package.as_deref(), Some("openmls"));
        assert_eq!(entry.include, ["openmls::treemath"]);
        assert_eq!(entry.exclude, ["openmls::treemath::internal"]);
        assert_eq!(entry.opaque, ["{impl tls_codec::Size for _}"]);
        assert!(!entry.default_opaques);
        assert_eq!(entry.charon_args, ["--some-rare-flag"]);
        assert_eq!(entry.aeneas_args, ["-flag"]);
        assert_eq!(
            entry.output_dir,
            Some(PathBuf::from("proofs/treemath/lean"))
        );
        assert_eq!(entry.features, ["fast"]);
        assert!(entry.no_default_features);
        assert_eq!(entry.project_files, Some(true));
        assert_eq!(entry.env["RUSTFLAGS"], "");
    }

    #[test]
    fn a_full_fstar_scenario_parses() {
        let (parsed, _) = parse_str(
            r#"[scenario.book-fstar]
backend = "fstar"
select-clauses = ["-**", "+**::process_order"]
interfaces = ["+**"]
z3rlimit = 100
fuel = 2
ifuel = 1
line-width = 120
"#,
        )
        .unwrap();
        let entry = &parsed.scenarios["book-fstar"];
        assert_eq!(entry.backend, ScenarioBackend::Fstar);
        let strings = |clauses: &[InclusionClause]| {
            clauses.iter().map(ToString::to_string).collect::<Vec<_>>()
        };
        assert_eq!(
            strings(&entry.select_clauses),
            ["-**", "+**::process_order"]
        );
        assert_eq!(strings(&entry.interfaces), ["+**"]);
        assert_eq!(entry.z3rlimit, Some(100));
        assert_eq!(entry.fuel, Some(2));
        assert_eq!(entry.ifuel, Some(1));
        assert_eq!(entry.line_width, Some(120));
        // Unset keys stay unset; defaults apply at resolution.
        assert!(entry.default_opaques);
        assert!(entry.project_files.is_none());
    }

    #[test]
    fn scenario_names_follow_the_grammar() {
        for name in ["treemath", "book-fstar", "a2b-c3", "x"] {
            validate_scenario_name(name).unwrap();
        }
        for name in ["Foo", "1x", "a--b", "-a", "a-", "a_b", "", "a-1b", "ä"] {
            let err = validate_scenario_name(name).unwrap_err();
            assert!(err.contains("not a valid scenario name"), "{name}: {err}");
        }
    }

    #[test]
    fn scenario_names_cannot_collide_with_backends_or_lean_roots() {
        for name in ["lean", "fstar", "coq", "proverif", "legacy-lean"] {
            let err = validate_scenario_name(name).unwrap_err();
            assert!(err.contains("backend name"), "{name}: {err}");
        }
        for (name, camel) in [
            ("init", "Init"),
            ("std", "Std"),
            ("lake", "Lake"),
            ("aeneas", "Aeneas"),
            ("hax", "Hax"),
        ] {
            let err = validate_scenario_name(name).unwrap_err();
            assert!(err.contains(camel), "{name}: {err}");
            assert!(err.contains("rename the scenario"), "{name}: {err}");
        }
    }

    #[test]
    fn the_backend_key_is_required_and_validated() {
        let err = parse_str("[scenario.a]\ninclude = []").unwrap_err();
        assert!(err.contains("`backend` key is required"), "{err}");
        let err = parse_str("[scenario.a]\nbackend = \"agda\"").unwrap_err();
        assert!(err.contains("`agda`"), "{err}");
    }

    #[test]
    fn unknown_scenario_keys_are_hard_errors() {
        let err = parse_str("[scenario.a]\nbackend = \"lean\"\nexlude = []").unwrap_err();
        assert!(err.contains("unknown key `exlude`"), "{err}");
    }

    #[test]
    fn keys_are_validated_against_the_backend() {
        // The unified selection is Lean-only for now.
        let err = parse_str("[scenario.a]\nbackend = \"fstar\"\ninclude = []").unwrap_err();
        assert!(err.contains("not supported yet"), "{err}");
        assert!(err.contains("select-clauses"), "{err}");
        // `select-clauses` covers only the engine backends.
        let err = parse_str("[scenario.a]\nbackend = \"lean\"\nselect-clauses = []").unwrap_err();
        assert!(err.contains("does not apply"), "{err}");
        for (backend, key) in [
            ("lean", "z3rlimit = 1"),
            ("fstar", "charon-args = []"),
            ("fstar", "project-files = true"),
            ("fstar", "assume-items = []"),
            ("proverif", "interfaces = []"),
        ] {
            let err =
                parse_str(&format!("[scenario.a]\nbackend = \"{backend}\"\n{key}")).unwrap_err();
            assert!(err.contains("does not apply"), "{backend}/{key}: {err}");
        }
    }

    #[test]
    fn clause_lists_are_validated_when_parsed() {
        let err = parse_str(
            r#"[scenario.a]
backend = "fstar"
select-clauses = ["+~~x"]"#,
        )
        .unwrap_err();
        assert!(err.contains("`+~~x` is invalid"), "{err}");
    }

    #[test]
    fn reserved_env_variables_are_rejected() {
        for var in [ENV_VAR_OPTIONS_FULL, ENV_VAR_OPTIONS_FRONTEND] {
            let err = parse_str(&format!(
                "[scenario.a]\nbackend = \"lean\"\n[scenario.a.env]\n{var} = \"x\"\n"
            ))
            .unwrap_err();
            assert!(err.contains(var), "{err}");
            assert!(err.contains("must not set"), "{err}");
        }
    }

    #[test]
    fn malformed_scenario_values_are_errors() {
        for entry in [
            "backend = 1",
            "backend = \"lean\"\ninclude = \"x\"",
            "backend = \"lean\"\ninclude = [1]",
            "backend = \"lean\"\ndefault-opaques = \"no\"",
            "backend = \"lean\"\nenv = { A = 1 }",
            "backend = \"fstar\"\nz3rlimit = -1",
        ] {
            assert!(
                parse_str(&format!("[scenario.a]\n{entry}")).is_err(),
                "{entry}"
            );
        }
    }

    /// An `output-dir` that names no directory of its own would scaffold
    /// the Lean package over the crate's source root; an absolute one would
    /// discard the package root it is documented to be relative to.
    #[test]
    fn degenerate_output_dirs_are_rejected() {
        for value in [
            "",
            ".",
            "./.",
            "a/..",
            "/tmp/proofs",
            "..",
            "../..",
            "a/../..",
        ] {
            assert!(
                parse_str(&format!(
                    "[scenario.a]\nbackend = \"lean\"\noutput-dir = \"{value}\""
                ))
                .is_err(),
                "{value}"
            );
        }
        // The accepted forms, a `..` component among them.
        for value in ["../shared", "proofs/a/lean", "./proofs/a"] {
            let parsed = parse_str(&format!(
                "[scenario.a]\nbackend = \"lean\"\noutput-dir = \"{value}\""
            ));
            assert!(parsed.is_ok(), "{value}");
        }
    }

    #[test]
    fn scenario_defaults_accept_only_opaque() {
        let (parsed, warnings) = parse_str(
            r#"[scenario-defaults.lean]
opaque = ["{impl core::fmt::Debug for _}"]"#,
        )
        .unwrap();
        assert!(warnings.is_empty());
        assert_eq!(
            parsed.scenario_defaults[&ScenarioBackend::Lean],
            ["{impl core::fmt::Debug for _}"]
        );
        let err = parse_str("[scenario-defaults.lean]\nexclude = []").unwrap_err();
        assert!(err.contains("`opaque` is the only key"), "{err}");
        let err = parse_str("[scenario-defaults.agda]\nopaque = []").unwrap_err();
        assert!(err.contains("`agda`"), "{err}");
        let err = parse_str("[scenario-defaults.fstar]\nopaque = []").unwrap_err();
        assert!(err.contains("`lean` backend only"), "{err}");
        let err = parse_str("[scenario-defaults.lean]\nopaque = [1]").unwrap_err();
        assert!(err.contains("array of strings"), "{err}");
    }
}
