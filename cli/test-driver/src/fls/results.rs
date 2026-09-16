//! The results file of a backend, `tests/fls-results/<backend>.toml`: the
//! observed stage and the tier of every test, written deterministically so
//! that diffs are minimal.

use std::{
    collections::{BTreeMap, HashSet},
    fs,
    path::PathBuf,
};

use anyhow::{Context as _, Result, bail};
use hax_types::cli_options::BackendName;
use serde::Deserialize;

use super::{
    package::FlsContext,
    runner::{Stages, ToolVersions},
    stage::{Stage, StageScale, Tier, classify},
    test::FlsTest,
};
use crate::log::JobKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestResult {
    pub stage: Stage,
    pub tier: Tier,
}

pub struct ResultsFile {
    pub backend: BackendName,
    pub tools: ToolVersions,
    pub tests: BTreeMap<String, TestResult>,
}

/// The committed file as read back; the writer below is the only producer.
#[derive(Deserialize)]
struct CommittedFile {
    /// Kept in file order, which is the order `cargo hax tools show` reports.
    tools: indexmap::IndexMap<String, toml::Value>,
    #[serde(default)]
    tests: BTreeMap<String, CommittedTest>,
}

#[derive(Deserialize)]
struct CommittedTest {
    stage: String,
    tier: String,
}

impl CommittedFile {
    fn read(path: &PathBuf) -> Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }
        let text = fs::read_to_string(path)?;
        let file: Self =
            toml::from_str(&text).with_context(|| format!("cannot parse {}", path.display()))?;
        Ok(Some(file))
    }

    fn tools(&self) -> ToolVersions {
        self.tools
            .iter()
            .map(|(name, version)| {
                let version = match version.as_str() {
                    Some(v) => v.to_string(),
                    None => version.to_string(),
                };
                (name.clone(), version)
            })
            .collect()
    }

    fn tests(&self, scale: &'static StageScale) -> Result<BTreeMap<String, TestResult>> {
        self.tests
            .iter()
            .map(|(path, t)| {
                Ok((
                    path.clone(),
                    TestResult {
                        stage: scale.parse(&t.stage)?,
                        tier: t.tier.parse()?,
                    },
                ))
            })
            .collect()
    }
}

/// Entries of the committed file for tests that still have a tier, with the
/// observed entries of the tests that ran replacing theirs.
fn merge_tests(
    committed: BTreeMap<String, TestResult>,
    with_tier: &HashSet<&str>,
    observed: BTreeMap<String, TestResult>,
) -> BTreeMap<String, TestResult> {
    committed
        .into_iter()
        .filter(|(path, _)| with_tier.contains(path.as_str()))
        .chain(observed)
        .collect()
}

/// The committed results file of a backend.
pub struct Committed {
    tools: ToolVersions,
    tests: BTreeMap<String, TestResult>,
}

impl Committed {
    /// Reads and validates the committed file, `None` when there is none.
    pub fn read(
        ctx: &FlsContext,
        backend: BackendName,
        scale: &'static StageScale,
    ) -> Result<Option<Self>> {
        let path = committed_path(ctx, backend);
        let Some(file) = CommittedFile::read(&path)? else {
            return Ok(None);
        };
        let tests = file
            .tests(scale)
            .with_context(|| format!("in {}", path.display()))?;
        Ok(Some(Self {
            tools: file.tools(),
            tests,
        }))
    }

    /// A run that observes only some tests keeps the committed entries of the
    /// others, which are only valid under the committed tool versions.
    pub fn check_partial_update(
        committed: Option<&Self>,
        ctx: &FlsContext,
        tools: &ToolVersions,
        partial: bool,
    ) -> Result<()> {
        if partial
            && ctx.options.update_results()
            && let Some(committed) = committed
            && &committed.tools != tools
        {
            bail!(
                "the tool versions differ from the committed results file; run without `--matching` so that every test is observed with the current tools"
            )
        }
        Ok(())
    }
}

impl ResultsFile {
    /// The results file after this run: observed entries for the tests that
    /// ran, committed entries for the tests `--matching` left out, no entry
    /// for tests that no longer exist or lost their tier.
    pub fn merged(
        backend: BackendName,
        committed: Option<&Committed>,
        tools: ToolVersions,
        with_tier: &[&FlsTest],
        selected: &[&FlsTest],
        stages: &Stages,
    ) -> Result<Self> {
        let observed = selected
            .iter()
            .map(|test| {
                let stage = *stages
                    .get(&test.path)
                    .with_context(|| format!("no stage observed for `{}`", test.path))?;
                Ok((
                    test.path.clone(),
                    TestResult {
                        stage,
                        tier: test.tiers[&backend],
                    },
                ))
            })
            .collect::<Result<BTreeMap<_, _>>>()?;

        let committed_tests = committed.map(|c| c.tests.clone()).unwrap_or_default();
        let with_tier: HashSet<&str> = with_tier.iter().map(|t| t.path.as_str()).collect();
        Ok(Self {
            backend,
            tools,
            tests: merge_tests(committed_tests, &with_tier, observed),
        })
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        out += &format!("backend = {}\n", toml_string(&self.backend.to_string()));
        out += "\n[tools]\n";
        for (name, version) in &self.tools {
            out += &format!("{name} = {}\n", toml_string(version));
        }
        for (path, result) in &self.tests {
            out += &format!("\n[tests.{}]\n", toml_string(path));
            out += &format!("stage = {}\n", toml_string(result.stage.as_str()));
            out += &format!("tier = {}\n", toml_string(result.tier.as_str()));
        }
        out
    }
}

fn committed_path(ctx: &FlsContext, backend: BackendName) -> PathBuf {
    ctx.options
        .tests_crate_dir()
        .join("fls-results")
        .join(format!("{backend}.toml"))
}

/// A TOML string.
fn toml_string(s: &str) -> String {
    toml::Value::String(s.to_string()).to_string()
}

/// Writes the merged results to the cache directory and compares them with
/// the committed file; with `--update-results` the committed file is
/// rewritten instead.
pub async fn check(ctx: &FlsContext, file: &ResultsFile, job: JobKind) -> Result<()> {
    for (path, result) in &file.tests {
        job.report_message(format!(
            "{path}: {} ({})",
            result.stage.as_str(),
            classify(result.tier, result.stage).as_str()
        ));
    }
    let rendered = file.render();
    let observed_path = ctx
        .options
        .cache_dir()
        .join("fls-results")
        .join(format!("{}.toml", file.backend));
    fs::create_dir_all(observed_path.parent().unwrap())?;
    fs::write(&observed_path, &rendered)?;

    let committed_path = committed_path(ctx, file.backend);
    let committed = fs::read_to_string(&committed_path).unwrap_or_default();
    if committed == rendered {
        job.report_message(format!("{} tests, results unchanged", file.tests.len()));
        return Ok(());
    }
    if ctx.options.update_results() {
        fs::create_dir_all(committed_path.parent().unwrap())?;
        fs::write(&committed_path, &rendered)?;
        job.report_message(format!("wrote {}", committed_path.display()));
        return Ok(());
    }
    let diff = similar::TextDiff::from_lines(&committed, &rendered)
        .unified_diff()
        .header(&committed_path.display().to_string(), "observed")
        .to_string();
    job.report_message(diff);
    bail!(
        "observed results differ from {}; rerun with `--update-results`, or copy {}",
        committed_path.display(),
        observed_path.display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fls::stage::LEAN;

    fn result(stage: &str) -> TestResult {
        TestResult {
            stage: LEAN.stage(stage),
            tier: Tier::Supported,
        }
    }

    #[test]
    fn merge_keeps_unselected_and_drops_removed_tests() {
        let committed = BTreeMap::from([
            ("a".to_string(), result("checked")),
            ("b".to_string(), result("extracted")),
            ("gone".to_string(), result("checked")),
        ]);
        let with_tier = HashSet::from(["a", "b", "new"]);
        let observed = BTreeMap::from([
            ("b".to_string(), result("checked")),
            ("new".to_string(), result("elaborates")),
        ]);
        let merged = merge_tests(committed, &with_tier, observed);
        assert_eq!(merged.keys().collect::<Vec<_>>(), ["a", "b", "new"]);
        assert_eq!(merged["a"].stage, LEAN.stage("checked"));
        assert_eq!(merged["b"].stage, LEAN.stage("checked"));
        assert_eq!(merged["new"].stage, LEAN.stage("elaborates"));
    }

    #[test]
    fn rendered_file_reads_back() {
        let file = ResultsFile {
            backend: BackendName::Lean,
            tools: vec![
                ("hax".into(), "1".into()),
                ("hax-lean-lib".into(), "v0".into()),
            ],
            tests: BTreeMap::from([("c/s/t".to_string(), result("aeneas-rejected"))]),
        };
        let back: CommittedFile = toml::from_str(&file.render()).unwrap();
        assert_eq!(back.tools(), file.tools);
        assert_eq!(back.tests(&LEAN).unwrap(), file.tests);
    }
}
