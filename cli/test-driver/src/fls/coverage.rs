//! `fls coverage`: every paragraph of the identifier lists is claimed by a
//! test, excluded, informational, or in a pending chapter, every claim names
//! a paragraph of the chapter the test lives in, and the results files have
//! an entry for exactly the tests with a tier for their backend, with a
//! reason in the test for every entry not classified as supported.

use std::collections::{BTreeMap, HashMap, HashSet};

use anyhow::{Result, bail};

use super::{
    exclusions::{self, Category, Exclusion, Exclusions},
    ids::{IdList, Paragraph},
    results,
    test::{self, FlsTest},
};
use crate::{cli::Cli, log::JobKind};

/// The `@fls` claims of one test, keyed by its `<chapter>/<section>/<name>` path.
pub struct Claims<'a> {
    pub path: &'a str,
    pub ids: &'a [String],
}

pub struct Report {
    pub violations: Vec<String>,
    /// One line per chapter, in chapter order.
    pub summary: Vec<String>,
}

pub fn analyse(list: &IdList, exclusions: &Exclusions, tests: &[Claims<'_>]) -> Report {
    let by_id: HashMap<&str, &Paragraph> =
        list.paragraphs.iter().map(|p| (p.id.as_str(), p)).collect();
    let pending: HashSet<&str> = exclusions
        .pending_chapters
        .iter()
        .map(String::as_str)
        .collect();
    let mut violations = vec![];

    // Claims, with the location rule: every claim in the test's chapter, at
    // least one in its section.
    let mut claims: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut tests_per_chapter: HashMap<&str, usize> = HashMap::new();
    for test in tests {
        let components: Vec<&str> = test.path.split('/').collect();
        let [chapter, section, _name] = components[..] else {
            violations.push(format!(
                "{}: a test lies at <chapter>/<section>/<name>.rs",
                test.path
            ));
            continue;
        };
        let dir = format!("{chapter}/{section}");
        *tests_per_chapter.entry(chapter).or_default() += 1;
        let mut in_section = false;
        let mut known = false;
        for id in test.ids {
            let Some(p) = by_id.get(id.as_str()) else {
                violations.push(format!("{}: @fls names unknown identifier {id}", test.path));
                continue;
            };
            known = true;
            claims.entry(p.id.as_str()).or_default().push(test.path);
            if p.chapter != chapter {
                violations.push(format!(
                    "{}: claims {id} of {}/{}; the test lies in {dir}",
                    test.path, p.chapter, p.section
                ));
            } else if p.section == section {
                in_section = true;
            }
        }
        if known && !in_section {
            violations.push(format!(
                "{}: claims no paragraph of {dir}; a test claims at least one paragraph of its section",
                test.path
            ));
        }
    }

    // Exclusions: listed, in the right chapter, not informational, not
    // claimed, not in a pending chapter.
    let mut excluded: HashMap<&str, &Exclusion> = HashMap::new();
    let mut chapters_with_tables: Vec<&str> = vec![];
    for entry in &exclusions.entries {
        let table = format!("{}.{}", entry.chapter, entry.category.as_str());
        if !chapters_with_tables.contains(&entry.chapter.as_str()) {
            chapters_with_tables.push(&entry.chapter);
        }
        match by_id.get(entry.id.as_str()) {
            None => violations.push(format!(
                "{table}: {} is not in the identifier lists",
                entry.id
            )),
            Some(p) if p.chapter != entry.chapter => violations.push(format!(
                "{table}: {} belongs to {}/{}",
                entry.id, p.chapter, p.section
            )),
            Some(p) if p.informational => violations.push(format!(
                "{table}: {} is informational and needs no exclusion",
                entry.id
            )),
            Some(_) => {}
        }
        if let Some(tests) = claims.get(entry.id.as_str()) {
            violations.push(format!(
                "{}: excluded as {} but claimed by {}",
                entry.id,
                entry.category.as_str(),
                tests.join(", ")
            ));
        }
        excluded.insert(&entry.id, entry);
    }
    for chapter in chapters_with_tables {
        if pending.contains(chapter) {
            violations.push(format!(
                "{chapter}: pending chapters have no exclusion tables; a chapter is triaged in one pull request that also removes it from pending-chapters"
            ));
        }
    }

    // Uncovered paragraphs, in list order.
    for p in &list.paragraphs {
        if pending.contains(p.chapter.as_str())
            || claims.contains_key(p.id.as_str())
            || excluded.contains_key(p.id.as_str())
            || p.informational
        {
            continue;
        }
        violations.push(format!(
            "{}/{}: {} is neither covered by a test nor excluded",
            p.chapter, p.section, p.id
        ));
    }

    let summary = list
        .chapters
        .iter()
        .map(|chapter| {
            let paragraphs: Vec<&Paragraph> = list
                .paragraphs
                .iter()
                .filter(|p| &p.chapter == chapter)
                .collect();
            let claimed = paragraphs
                .iter()
                .filter(|p| claims.contains_key(p.id.as_str()))
                .count();
            let tests = tests_per_chapter.get(chapter.as_str()).copied().unwrap_or(0);
            let n = paragraphs.len();
            if pending.contains(chapter.as_str()) {
                return format!("{chapter}: pending, {n} paragraphs, {claimed} claimed by {tests} tests");
            }
            let mut per_category: BTreeMap<Category, usize> = BTreeMap::new();
            for p in &paragraphs {
                if let Some(e) = excluded.get(p.id.as_str()) {
                    *per_category.entry(e.category).or_default() += 1;
                }
            }
            let excluded_total: usize = per_category.values().sum();
            let per_category: Vec<String> = per_category
                .iter()
                .map(|(c, n)| format!("{} {n}", c.as_str()))
                .collect();
            let informational = paragraphs
                .iter()
                .filter(|p| p.informational && !claims.contains_key(p.id.as_str()))
                .count();
            format!(
                "{chapter}: {n} paragraphs, {claimed} claimed by {tests} tests, {excluded_total} excluded ({}), {informational} informational",
                if per_category.is_empty() { "none".to_string() } else { per_category.join(", ") }
            )
        })
        .collect();

    Report {
        violations,
        summary,
    }
}

fn check(options: &Cli, tests: &[FlsTest], job: &JobKind) -> Result<()> {
    let tests_dir = options.tests_crate_dir();
    let fls_ids = tests_dir.join("fls-ids.txt");
    if !fls_ids.exists() {
        bail!(
            "{} is missing; run `test-driver tests fls refresh --rev <rust-lang/fls commit>`",
            fls_ids.display()
        );
    }
    let hax_ids = tests_dir.join("hax-ids.txt");
    if !hax_ids.exists() {
        bail!(
            "{} is missing; it lists the identifiers of the `hax` chapter and is written by hand, not by `fls refresh`",
            hax_ids.display()
        );
    }
    let list = IdList::merged(IdList::read(&fls_ids)?, IdList::read(&hax_ids)?)?;
    let exclusions = exclusions::read(&tests_dir.join("fls-excluded.toml"), &list.chapters)?;
    let claims: Vec<Claims<'_>> = tests
        .iter()
        .map(|t| Claims {
            path: &t.path,
            ids: &t.fls,
        })
        .collect();
    let mut report = analyse(&list, &exclusions, &claims);
    report.violations.extend(results::results_violations(
        &tests_dir.join("fls-results"),
        tests,
    )?);
    for line in &report.summary {
        job.report_message(line);
    }
    for line in &report.violations {
        job.report_message(line);
    }
    if !report.violations.is_empty() {
        bail!(
            "coverage check failed with {} violations",
            report.violations.len()
        );
    }
    job.report_message(format!(
        "{} paragraphs, {} tests, no violation",
        list.paragraphs.len(),
        tests.len()
    ));
    Ok(())
}

pub async fn run(options: &Cli) -> Result<()> {
    let fls_dir = options.fls_dir();
    let tests = JobKind::ResolveTests
        .run(async |_| test::discover(&fls_dir))
        .await?;
    JobKind::Command {
        name: "coverage".into(),
    }
    .run(async |job| check(options, &tests, &job))
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(id: &str, chapter: &str, section: &str, informational: bool) -> Paragraph {
        Paragraph {
            id: id.into(),
            chapter: chapter.into(),
            section: section.into(),
            section_title: section.into(),
            informational,
        }
    }

    fn list() -> IdList {
        IdList::from_paragraphs(
            "r".into(),
            None,
            vec![
                p("fls_g1", "general", "general", true),
                p("fls_e1", "expressions", "call-expressions", false),
                p("fls_e2", "expressions", "call-expressions", false),
                p("fls_e3", "expressions", "break-expressions", false),
                p("fls_e4", "expressions", "break-expressions", true),
                p("fls_v1", "values", "statics", false),
                p("hax_requires", "hax", "contracts", false),
            ],
        )
    }

    fn exclusions(pending: &[&str], entries: &[(&str, &str, Category)]) -> Exclusions {
        Exclusions {
            pending_chapters: pending.iter().map(|s| s.to_string()).collect(),
            entries: entries
                .iter()
                .map(|(id, chapter, category)| Exclusion {
                    id: id.to_string(),
                    chapter: chapter.to_string(),
                    category: *category,
                    reason: "r".into(),
                })
                .collect(),
        }
    }

    fn ids(ids: &[&str]) -> Vec<String> {
        ids.iter().map(|s| s.to_string()).collect()
    }

    fn violations(exclusions: &Exclusions, tests: &[(&str, Vec<String>)]) -> Vec<String> {
        let claims: Vec<Claims<'_>> = tests
            .iter()
            .map(|(path, ids)| Claims { path, ids })
            .collect();
        analyse(&list(), exclusions, &claims).violations
    }

    #[test]
    fn passing_case() {
        let excl = exclusions(
            &["values", "hax"],
            &[("fls_e2", "expressions", Category::Legality)],
        );
        let tests = [
            (
                "expressions/call-expressions/call",
                ids(&["fls_e1", "fls_e3"]),
            ),
            ("general/general/intro", ids(&["fls_g1"])),
        ];
        let claims: Vec<Claims<'_>> = tests
            .iter()
            .map(|(path, ids)| Claims { path, ids })
            .collect();
        let report = analyse(&list(), &excl, &claims);
        assert_eq!(report.violations, Vec::<String>::new());
        assert_eq!(
            report.summary,
            [
                "general: 1 paragraphs, 1 claimed by 1 tests, 0 excluded (none), 0 informational",
                "expressions: 4 paragraphs, 2 claimed by 1 tests, 1 excluded (legality 1), 1 informational",
                "values: pending, 1 paragraphs, 0 claimed by 0 tests",
                "hax: pending, 1 paragraphs, 0 claimed by 0 tests",
            ]
        );
    }

    #[test]
    fn unknown_identifier() {
        let excl = exclusions(
            &["general", "values", "hax"],
            &[("fls_e2", "expressions", Category::Legality)],
        );
        let v = violations(
            &excl,
            &[(
                "expressions/call-expressions/call",
                ids(&["fls_e1", "fls_e3", "fls_typo"]),
            )],
        );
        assert_eq!(
            v,
            ["expressions/call-expressions/call: @fls names unknown identifier fls_typo"]
        );
    }

    #[test]
    fn location_rules() {
        let excl = exclusions(&["general", "values", "expressions", "hax"], &[]);
        let v = violations(
            &excl,
            &[
                ("values/statics/wrong_chapter", ids(&["fls_e1", "fls_v1"])),
                (
                    "expressions/call-expressions/wrong_section",
                    ids(&["fls_e3"]),
                ),
                ("flat", ids(&["fls_e1"])),
            ],
        );
        assert_eq!(
            v,
            [
                "values/statics/wrong_chapter: claims fls_e1 of expressions/call-expressions; the test lies in values/statics",
                "expressions/call-expressions/wrong_section: claims no paragraph of expressions/call-expressions; a test claims at least one paragraph of its section",
                "flat: a test lies at <chapter>/<section>/<name>.rs",
            ]
        );
    }

    #[test]
    fn uncovered_and_informational() {
        let excl = exclusions(&["values", "hax"], &[]);
        let v = violations(
            &excl,
            &[("expressions/call-expressions/call", ids(&["fls_e1"]))],
        );
        assert_eq!(
            v,
            [
                "expressions/call-expressions: fls_e2 is neither covered by a test nor excluded",
                "expressions/break-expressions: fls_e3 is neither covered by a test nor excluded",
            ]
        );
    }

    #[test]
    fn excluded_informational() {
        let excl = exclusions(
            &["values", "hax"],
            &[
                ("fls_e2", "expressions", Category::Legality),
                ("fls_e4", "expressions", Category::Legality),
            ],
        );
        let v = violations(
            &excl,
            &[
                (
                    "expressions/call-expressions/call",
                    ids(&["fls_e1", "fls_e3"]),
                ),
                ("general/general/intro", ids(&["fls_g1"])),
            ],
        );
        assert_eq!(
            v,
            ["expressions.legality: fls_e4 is informational and needs no exclusion"]
        );
    }

    #[test]
    fn claimed_and_excluded() {
        let excl = exclusions(
            &["general", "values", "hax"],
            &[
                ("fls_e1", "expressions", Category::Legality),
                ("fls_e2", "expressions", Category::Legality),
            ],
        );
        let v = violations(
            &excl,
            &[(
                "expressions/break-expressions/brk",
                ids(&["fls_e3", "fls_e1"]),
            )],
        );
        assert_eq!(
            v,
            ["fls_e1: excluded as legality but claimed by expressions/break-expressions/brk"]
        );
    }

    #[test]
    fn exclusion_of_unknown_or_foreign_identifier() {
        let excl = exclusions(
            &["general", "values", "hax"],
            &[
                ("fls_gone", "expressions", Category::Unsafe),
                ("fls_v1", "expressions", Category::Unsafe),
                ("fls_e1", "expressions", Category::Legality),
                ("fls_e2", "expressions", Category::Legality),
                ("fls_e3", "expressions", Category::Legality),
            ],
        );
        let v = violations(&excl, &[]);
        assert_eq!(
            v,
            [
                "expressions.unsafe: fls_gone is not in the identifier lists",
                "expressions.unsafe: fls_v1 belongs to values/statics",
            ]
        );
    }

    #[test]
    fn pending_chapter_with_table() {
        let excl = exclusions(
            &["general", "values", "expressions", "hax"],
            &[("fls_v1", "values", Category::OutOfScope)],
        );
        let v = violations(&excl, &[]);
        assert_eq!(
            v,
            [
                "values: pending chapters have no exclusion tables; a chapter is triaged in one pull request that also removes it from pending-chapters"
            ]
        );
    }
}
