//! The exclusion file `tests/fls-excluded.toml`: the chapters not yet
//! triaged and, per chapter and category, the paragraphs no test covers.

use std::{collections::HashMap, fs, path::Path};

use anyhow::{Context as _, Result, bail};
use toml::Value;

/// The manual exclusion categories. `informational` and `pending` are
/// computed by the coverage check and rejected in the file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Category {
    Lexical,
    Legality,
    OutOfScope,
    Unsafe,
    Ffi,
    InlineAssembly,
    Concurrency,
    MacroExpanded,
}

impl Category {
    pub const ALL: [Category; 8] = [
        Category::Lexical,
        Category::Legality,
        Category::OutOfScope,
        Category::Unsafe,
        Category::Ffi,
        Category::InlineAssembly,
        Category::Concurrency,
        Category::MacroExpanded,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Category::Lexical => "lexical",
            Category::Legality => "legality",
            Category::OutOfScope => "out-of-scope",
            Category::Unsafe => "unsafe",
            Category::Ffi => "ffi",
            Category::InlineAssembly => "inline-assembly",
            Category::Concurrency => "concurrency",
            Category::MacroExpanded => "macro-expanded",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.as_str() == name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exclusion {
    pub id: String,
    pub chapter: String,
    pub category: Category,
    pub reason: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Exclusions {
    pub pending_chapters: Vec<String>,
    pub entries: Vec<Exclusion>,
}

/// Reads the file; `chapters` are those of the merged identifier list.
pub fn read(path: &Path, chapters: &[String]) -> Result<Exclusions> {
    let text =
        fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;
    parse(&text, chapters).with_context(|| format!("in {}", path.display()))
}

pub fn parse(text: &str, chapters: &[String]) -> Result<Exclusions> {
    let table: toml::Table = text.parse().context("not valid TOML")?;
    let mut exclusions = Exclusions::default();
    let mut seen: HashMap<String, String> = HashMap::new();
    for (key, value) in &table {
        if key == "pending-chapters" {
            for chapter in strings_of(value, "pending-chapters")? {
                if !chapters.contains(&chapter) {
                    bail!("pending-chapters: `{chapter}` is not a chapter of the identifier lists");
                }
                if exclusions.pending_chapters.contains(&chapter) {
                    bail!("pending-chapters: `{chapter}` is listed twice");
                }
                exclusions.pending_chapters.push(chapter);
            }
            continue;
        }
        if !chapters.contains(key) {
            bail!("`{key}` is neither `pending-chapters` nor a chapter of the identifier lists");
        }
        let Value::Table(categories) = value else {
            bail!("[{key}]: expected a table of categories");
        };
        for (name, entry) in categories {
            let table_name = format!("{key}.{name}");
            let category = match name.as_str() {
                "informational" | "pending" => bail!(
                    "[{table_name}]: `{name}` is computed by the coverage check and not written in the exclusion file"
                ),
                _ => Category::parse(name).with_context(|| {
                    format!(
                        "[{table_name}]: unknown category `{name}`; the categories are {}",
                        Category::ALL.map(Category::as_str).join(", ")
                    )
                })?,
            };
            let Value::Table(entry) = entry else {
                bail!("[{table_name}]: expected a table with `reason` and `ids`");
            };
            for field in entry.keys() {
                if !["reason", "ids", "overrides"].contains(&field.as_str()) {
                    bail!("[{table_name}]: unknown field `{field}`");
                }
            }
            let reason = entry
                .get("reason")
                .with_context(|| format!("[{table_name}]: missing `reason`"))?;
            let reason = non_empty_string(reason, &format!("[{table_name}].reason"))?;
            let ids = entry
                .get("ids")
                .with_context(|| format!("[{table_name}]: missing `ids`"))?;
            let ids = strings_of(ids, &format!("[{table_name}].ids"))?;
            if ids.is_empty() {
                bail!("[{table_name}].ids: empty; remove the table instead");
            }
            let mut overrides: HashMap<String, String> = HashMap::new();
            if let Some(value) = entry.get("overrides") {
                let Value::Table(table) = value else {
                    bail!("[{table_name}.overrides]: expected a table of identifier to reason");
                };
                for (id, reason) in table {
                    if !ids.contains(id) {
                        bail!("[{table_name}.overrides]: `{id}` is not in `ids`");
                    }
                    let reason =
                        non_empty_string(reason, &format!("[{table_name}.overrides].{id}"))?;
                    overrides.insert(id.clone(), reason);
                }
            }
            for id in ids {
                if let Some(other) = seen.insert(id.clone(), table_name.clone()) {
                    bail!("`{id}` is listed in both [{other}] and [{table_name}]");
                }
                let reason = overrides
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| reason.clone());
                exclusions.entries.push(Exclusion {
                    id,
                    chapter: key.clone(),
                    category,
                    reason,
                });
            }
        }
    }
    Ok(exclusions)
}

fn non_empty_string(value: &Value, what: &str) -> Result<String> {
    match value.as_str() {
        Some(s) if !s.trim().is_empty() => Ok(s.to_string()),
        Some(_) => bail!("{what}: empty string"),
        None => bail!("{what}: expected a string"),
    }
}

fn strings_of(value: &Value, what: &str) -> Result<Vec<String>> {
    let Value::Array(items) = value else {
        bail!("{what}: expected an array of strings");
    };
    items
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_string)
                .with_context(|| format!("{what}: expected an array of strings"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chapters() -> Vec<String> {
        ["general", "expressions", "hax"].map(String::from).to_vec()
    }

    #[test]
    fn valid_file() {
        let text = r#"
pending-chapters = ["general", "hax"]

[expressions.legality]
reason = "shared"
ids = ["fls_a", "fls_b"]

[expressions.legality.overrides]
fls_b = "special"

[expressions.unsafe]
reason = "raw pointers"
ids = ["fls_c"]
"#;
        let parsed = parse(text, &chapters()).unwrap();
        assert_eq!(parsed.pending_chapters, ["general", "hax"]);
        let reasons: Vec<(&str, Category, &str)> = parsed
            .entries
            .iter()
            .map(|e| (e.id.as_str(), e.category, e.reason.as_str()))
            .collect();
        assert_eq!(
            reasons,
            [
                ("fls_a", Category::Legality, "shared"),
                ("fls_b", Category::Legality, "special"),
                ("fls_c", Category::Unsafe, "raw pointers"),
            ]
        );
        assert!(parsed.entries.iter().all(|e| e.chapter == "expressions"));
        assert_eq!(parse("", &chapters()).unwrap(), Exclusions::default());
    }

    #[test]
    fn error_cases() {
        let cases = [
            ("pending-chapters = [\"patterns\"]", "not a chapter"),
            ("pending-chapters = [\"hax\", \"hax\"]", "twice"),
            (
                "[patterns.legality]\nreason = \"r\"\nids = [\"fls_a\"]",
                "neither `pending-chapters` nor a chapter",
            ),
            (
                "[expressions.informational]\nreason = \"r\"\nids = [\"fls_a\"]",
                "computed by the coverage check",
            ),
            (
                "[expressions.pending]\nreason = \"r\"\nids = [\"fls_a\"]",
                "computed by the coverage check",
            ),
            (
                "[expressions.typo]\nreason = \"r\"\nids = [\"fls_a\"]",
                "unknown category `typo`",
            ),
            (
                "[expressions.legality]\nids = [\"fls_a\"]",
                "missing `reason`",
            ),
            (
                "[expressions.legality]\nreason = \"\"\nids = [\"fls_a\"]",
                "empty string",
            ),
            ("[expressions.legality]\nreason = \"r\"", "missing `ids`"),
            (
                "[expressions.legality]\nreason = \"r\"\nids = []",
                "empty; remove the table",
            ),
            (
                "[expressions.legality]\nreason = \"r\"\nids = [1]",
                "array of strings",
            ),
            (
                "[expressions.legality]\nreason = \"r\"\nids = [\"fls_a\"]\nextra = 1",
                "unknown field `extra`",
            ),
            (
                "[expressions.legality]\nreason = \"r\"\nids = [\"fls_a\"]\n[expressions.legality.overrides]\nfls_b = \"x\"",
                "not in `ids`",
            ),
            (
                "[expressions.legality]\nreason = \"r\"\nids = [\"fls_a\"]\n[expressions.legality.overrides]\nfls_a = \"\"",
                "empty string",
            ),
            (
                "[expressions.legality]\nreason = \"r\"\nids = [\"fls_a\"]\n[expressions.unsafe]\nreason = \"r\"\nids = [\"fls_a\"]",
                "listed in both",
            ),
            ("expressions = 1", "expected a table of categories"),
            ("[expressions]\nlegality = 1", "expected a table with"),
            ("not toml", "not valid TOML"),
        ];
        for (text, expected) in cases {
            let err = format!("{:#}", parse(text, &chapters()).unwrap_err());
            assert!(err.contains(expected), "{text:?}: {err}");
        }
    }
}
