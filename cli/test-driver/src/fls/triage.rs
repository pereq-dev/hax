//! `fls triage <chapter>`: the paragraphs of a chapter still to be decided
//! between a test claim and an exclusion entry, with the rubric and the text
//! the decision needs, read from the FLS sources at the vendored revision.

use std::collections::{BTreeMap, HashMap};

use anyhow::{Context as _, Result, bail};

use super::{
    coverage,
    exclusions::{self, Exclusions},
    ids::IdList,
    refresh::{self, NO_RUBRIC, ParsedParagraph},
    test::{self, FlsTest},
};
use crate::{cli::Cli, log::JobKind};

/// What the coverage check knows about one paragraph.
enum Status<'a> {
    Open,
    Informational,
    Claimed(Vec<&'a str>),
    Excluded(&'a str),
}

/// The lines `fls triage` prints for `chapter`, in the document order of
/// `sources`. Paragraphs without a source follow in list order.
pub fn render(
    chapter: &str,
    list: &IdList,
    sources: &[ParsedParagraph],
    exclusions: &Exclusions,
    tests: &[FlsTest],
    all: bool,
) -> Vec<String> {
    let mut claims: HashMap<&str, Vec<&str>> = HashMap::new();
    for test in tests {
        for id in &test.fls {
            claims.entry(id).or_default().push(&test.path);
        }
    }
    let excluded: HashMap<&str, &str> = exclusions
        .entries
        .iter()
        .map(|e| (e.id.as_str(), e.category.as_str()))
        .collect();

    let position: HashMap<&str, usize> = sources
        .iter()
        .enumerate()
        .map(|(i, s)| (s.paragraph.id.as_str(), i))
        .collect();
    let mut paragraphs: Vec<_> = list
        .paragraphs
        .iter()
        .filter(|p| p.chapter == chapter)
        .collect();
    paragraphs.sort_by_key(|p| position.get(p.id.as_str()).copied().unwrap_or(usize::MAX));

    let mut lines = vec![];
    let mut per_rubric: BTreeMap<&str, usize> = BTreeMap::new();
    let mut open = 0;
    let mut current_section: Option<&str> = None;
    for p in paragraphs {
        let status = if let Some(tests) = claims.get(p.id.as_str()) {
            Status::Claimed(tests.clone())
        } else if let Some(category) = excluded.get(p.id.as_str()) {
            Status::Excluded(category)
        } else if p.informational {
            Status::Informational
        } else {
            Status::Open
        };
        if !all && !matches!(status, Status::Open) {
            continue;
        }
        if current_section != Some(p.section.as_str()) {
            current_section = Some(&p.section);
            lines.push(String::new());
            lines.push(format!("{}  {}", p.section, p.section_title));
        }
        let source = position.get(p.id.as_str()).map(|&i| &sources[i]);
        let rubric = source.map_or(NO_RUBRIC, |s| s.rubric.as_str());
        let text = source.map_or("", |s| s.text.as_str());
        let mark = match &status {
            Status::Open => {
                open += 1;
                *per_rubric.entry(rubric).or_default() += 1;
                String::new()
            }
            Status::Informational => " (informational)".to_string(),
            Status::Claimed(tests) => format!(" (claimed by {})", tests.join(", ")),
            Status::Excluded(category) => format!(" (excluded: {category})"),
        };
        lines.push(format!("{} [{rubric}]{mark} {text}", p.id));
    }
    let per_rubric: Vec<String> = per_rubric
        .iter()
        .map(|(rubric, n)| format!("{rubric} {n}"))
        .collect();
    lines.push(String::new());
    lines.push(format!(
        "{open} paragraphs to triage in {chapter}: {}",
        if per_rubric.is_empty() {
            "none".to_string()
        } else {
            per_rubric.join(", ")
        }
    ));
    lines
}

pub async fn run(options: &Cli, chapter: &str, all: bool) -> Result<()> {
    let fls_dir = options.fls_dir();
    let tests = JobKind::ResolveTests
        .run(async |_| test::discover(&fls_dir))
        .await?;
    JobKind::Command {
        name: format!("triage {chapter}"),
    }
    .run(async |job| {
        let tests_dir = options.tests_crate_dir();
        let list = coverage::read_lists(&tests_dir)?;
        if !list.chapters.iter().any(|c| c == chapter) {
            bail!(
                "`{chapter}` is not a chapter of the identifier lists; the chapters are {}",
                list.chapters.join(", ")
            );
        }
        let exclusions = exclusions::read(&tests_dir.join("fls-excluded.toml"), &list.chapters)?;
        // The `hax` chapter has hand-written rows and no source text.
        let sources = if chapter == "hax" {
            vec![]
        } else {
            let dir = refresh::fetch(options.cache_dir(), &list.revision)?;
            let path = dir.join("src").join(format!("{chapter}.rst"));
            let text = std::fs::read_to_string(&path)
                .with_context(|| format!("cannot read {}", path.display()))?;
            refresh::parse_chapter(chapter, &text)?.1
        };
        for line in render(chapter, &list, &sources, &exclusions, &tests, all) {
            job.report_message(line);
        }
        Ok(())
    })
    .await
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;
    use crate::fls::ids::Paragraph;

    fn write(dir: &Path, name: &str, text: &str) {
        let path = dir.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    #[test]
    fn renders_open_paragraphs_grouped_by_section() {
        let dir = std::env::temp_dir().join(format!("fls-triage-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        write(
            &dir,
            "fls/src/values/statics/s.rs",
            "//! @fls: fls_a\n//! @title: T\n//! @lean(tier): supported\n#[hax_test::equiv]\npub fn f() -> bool { true }\n",
        );
        let tests = test::discover(&dir.join("fls/src")).unwrap();
        fs::remove_dir_all(&dir).unwrap();

        let p = |id: &str, section: &str, informational: bool| Paragraph {
            id: id.into(),
            chapter: "values".into(),
            section: section.into(),
            section_title: section.to_uppercase(),
            informational,
        };
        let list = IdList::from_paragraphs(
            "r".into(),
            None,
            vec![
                p("fls_a", "statics", false),
                p("fls_b", "statics", false),
                p("fls_c", "constants", false),
                p("fls_d", "constants", true),
                p("fls_e", "constants", false),
            ],
        );
        let source = |id: &str, rubric: &str, text: &str| ParsedParagraph {
            paragraph: p(id, "s", false),
            rubric: rubric.into(),
            text: text.into(),
        };
        // The document order differs from the identifier order of the list.
        let sources = [
            source("fls_b", "legality-rules", "B."),
            source("fls_a", "legality-rules", "A."),
            source("fls_c", "dynamic-semantics", "C."),
            source("fls_d", "none", "D."),
        ];
        let exclusions = exclusions::parse(
            "[values.legality]\nreason = \"r\"\nids = [\"fls_e\"]\n",
            &["values".to_string()],
        )
        .unwrap();

        let lines = render("values", &list, &sources, &exclusions, &tests, false);
        assert_eq!(
            lines,
            [
                "",
                "statics  STATICS",
                "fls_b [legality-rules] B.",
                "",
                "constants  CONSTANTS",
                "fls_c [dynamic-semantics] C.",
                "",
                "2 paragraphs to triage in values: dynamic-semantics 1, legality-rules 1",
            ]
        );

        let lines = render("values", &list, &sources, &exclusions, &tests, true);
        assert_eq!(lines[2], "fls_b [legality-rules] B.");
        assert_eq!(
            lines[3],
            "fls_a [legality-rules] (claimed by values/statics/s) A."
        );
        assert_eq!(lines[7], "fls_d [none] (informational) D.");
        assert_eq!(lines[8], "fls_e [none] (excluded: legality) ");
        assert_eq!(lines.len(), 11);
    }
}
