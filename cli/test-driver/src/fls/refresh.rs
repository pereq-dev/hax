//! `fls refresh`: fetches the FLS sources at a commit and regenerates
//! `tests/fls-ids.txt` from the `:dp:` paragraph identifiers of its chapters.

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context as _, Result, bail};
use regex::Regex;

use super::ids::{IdList, Paragraph};
use crate::cli::Cli;

const FLS_REPOSITORY: &str = "https://github.com/rust-lang/fls";

/// The heading levels in the order of their underline characters.
const LEVELS: &str = "=-~^\"";

/// Fetches the sources of `rev` into `<cache_dir>/fls-source/<rev>`. A
/// previous fetch is reused only for a full commit hash, since a branch or
/// tag can move.
pub fn fetch(cache_dir: &Path, rev: &str) -> Result<PathBuf> {
    // `rev` becomes a directory that is removed below, so it must stay inside
    // the cache. No git ref is empty, starts with `/` or `-`, or contains `..`.
    if rev.is_empty()
        || rev.starts_with(['/', '-'])
        || rev.contains("..")
        || rev.contains(['\\', '\0'])
    {
        bail!("`{rev}` is not a revision of rust-lang/fls");
    }
    let dir = cache_dir.join("fls-source").join(rev);
    if is_commit_hash(rev) && dir.join("src").join("index.rst").exists() {
        return Ok(dir);
    }
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }
    fs::create_dir_all(&dir)?;
    let steps: [&[&str]; 3] = [
        &["init", "--quiet"],
        &["fetch", "--quiet", "--depth", "1", FLS_REPOSITORY, rev],
        &["checkout", "--quiet", "FETCH_HEAD"],
    ];
    for args in steps {
        if let Err(e) = git(Some(&dir), args) {
            let _ = fs::remove_dir_all(&dir);
            return Err(e.context(format!("cannot fetch revision {rev}")));
        }
    }
    Ok(dir)
}

/// Runs `git` without prompting for credentials and returns its stdout.
fn git(dir: Option<&Path>, args: &[&str]) -> Result<String> {
    let mut command = Command::new("git");
    command.args(args).env("GIT_TERMINAL_PROMPT", "0");
    if let Some(dir) = dir {
        command.current_dir(dir);
    }
    let output = command
        .output()
        .context("cannot run `git`; is it on PATH?")?;
    if !output.status.success() {
        bail!(
            "`git {}` failed:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// The commit the fetch checked out, so that a tag is recorded by the commit
/// it names rather than by a name that can move.
fn resolved_commit(dir: &Path) -> Result<String> {
    Ok(git(Some(dir), &["rev-parse", "HEAD"])?.trim().to_string())
}

fn is_commit_hash(rev: &str) -> bool {
    rev.len() == 40 && rev.chars().all(|c| c.is_ascii_hexdigit())
}

/// Whether `rev` is a tag of rust-lang/fls, which names a release.
fn is_release_tag(rev: &str) -> Result<bool> {
    let tag = format!("refs/tags/{rev}");
    Ok(!git(None, &["ls-remote", "--tags", FLS_REPOSITORY, &tag])?.is_empty())
}

/// The chapter files listed by the `.. toctree::` block of `src/index.rst`.
pub fn chapters_of_index(text: &str) -> Result<Vec<String>> {
    let mut lines = text.lines();
    lines
        .by_ref()
        .find(|l| l.trim_end() == ".. toctree::")
        .context("index.rst has no `.. toctree::` block")?;
    let mut entries = vec![];
    let mut in_entries = false;
    for line in lines {
        let trimmed = line.trim();
        if !in_entries {
            if trimmed.starts_with(':') {
                continue;
            }
            if trimmed.is_empty() {
                in_entries = true;
                continue;
            }
            bail!("index.rst: unexpected line in the toctree options: `{line}`");
        }
        if trimmed.is_empty() || trimmed.starts_with(".. ") {
            break;
        }
        if !line.starts_with(' ') {
            bail!("index.rst: toctree entry `{line}` is not indented");
        }
        entries.push(trimmed.to_string());
    }
    if entries.is_empty() {
        bail!("index.rst: the toctree block lists no chapter");
    }
    Ok(entries)
}

/// ASCII lower-case with every run of non-alphanumeric characters replaced by
/// one hyphen.
pub fn slug(title: &str) -> String {
    let mut out = String::new();
    let mut pending_hyphen = false;
    for c in title.chars() {
        if c.is_ascii_alphanumeric() {
            if pending_hyphen && !out.is_empty() {
                out.push('-');
            }
            pending_hyphen = false;
            out.push(c.to_ascii_lowercase());
        } else {
            pending_hyphen = true;
        }
    }
    out
}

/// A non-empty line of one repeated punctuation character.
fn is_underline(line: &str) -> bool {
    let mut chars = line.chars();
    match chars.next() {
        Some(c) if "=-~^\"+*#".contains(c) => chars.all(|d| d == c),
        _ => false,
    }
}

struct Heading {
    level: usize,
    title: String,
    slug: String,
}

/// A paragraph with what the triage needs from its source: the rubric it
/// appears under and its text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedParagraph {
    pub paragraph: Paragraph,
    /// Slug of the last `.. rubric::` above the paragraph in its section,
    /// `none` without one.
    pub rubric: String,
    /// The source lines of the paragraph up to the next blank line, joined
    /// with single spaces, without the `:dp:` marker.
    pub text: String,
}

/// The rubric of a paragraph without a `.. rubric::` line above it.
pub const NO_RUBRIC: &str = "none";

/// The text of the paragraph whose `:dp:` marker is on line `start`.
fn paragraph_text(lines: &[&str], start: usize, marker: &str) -> String {
    let mut words: Vec<String> = vec![];
    for (i, line) in lines.iter().enumerate().skip(start) {
        if line.trim().is_empty() {
            break;
        }
        let line = if i == start {
            line.replacen(marker, "", 1)
        } else {
            line.to_string()
        };
        // Bullet and list-table markers in front of the text.
        let line = line.trim().trim_start_matches(['*', '-', ' ']);
        if !line.is_empty() {
            words.push(line.to_string());
        }
    }
    words.join(" ")
}

/// The title and the paragraphs of one chapter file, in document order.
pub fn parse_chapter(chapter: &str, text: &str) -> Result<(String, Vec<ParsedParagraph>)> {
    let anchor = Regex::new(r"^\.\. _fls_[A-Za-z0-9]+:$").unwrap();
    let paragraph = Regex::new(r":dp:`(fls_[A-Za-z0-9]+)`").unwrap();

    let lines: Vec<&str> = text.lines().collect();
    let mut stack: Vec<Heading> = vec![];
    // `.. informational-page::` flags the whole chapter, `.. informational-section::`
    // the paragraphs directly under the current heading, not its subsections,
    // as the FLS's own `paragraph-ids.json` does.
    let mut informational_sections: HashSet<String> = HashSet::new();
    let mut page_informational = false;
    let mut rubric = NO_RUBRIC.to_string();
    let mut slugs: HashMap<String, String> = HashMap::new();
    let mut paragraphs = vec![];

    let at = |i: usize, what: &str| format!("{chapter}.rst:{}: {what}", i + 1);

    for (i, line) in lines.iter().enumerate() {
        let is_underline = i > 0 && is_underline(line) && {
            let title = lines[i - 1].trim_end();
            // docutils compares the width of the title without trailing
            // spaces, so a non-ASCII title needs no longer underline.
            !title.is_empty()
                && !title.starts_with(char::is_whitespace)
                && !title.starts_with("..")
                && line.trim_end().chars().count() >= title.chars().count()
        };
        if is_underline {
            let title = lines[i - 1].trim_end();
            if i < 3 || !lines[i - 2].trim().is_empty() || !anchor.is_match(lines[i - 3]) {
                bail!(
                    "{}",
                    at(
                        i - 1,
                        &format!("heading `{title}` has no `.. _fls_<id>:` anchor two lines above")
                    )
                );
            }
            let c = line.chars().next().unwrap();
            let level = LEVELS
                .find(c)
                .with_context(|| at(i, &format!("unknown heading underline `{c}`")))?;
            if stack.is_empty() && level != 0 {
                bail!(
                    "{}",
                    at(
                        i - 1,
                        "the first heading of a chapter must be underlined with `=`"
                    )
                );
            }
            if !stack.is_empty() && level == 0 {
                bail!("{}", at(i - 1, "second chapter-level heading"));
            }
            while stack.last().is_some_and(|h| h.level >= level) {
                stack.pop();
            }
            let heading_slug = if level == 0 {
                chapter.to_string()
            } else {
                slug(title)
            };
            if let Some(other) = slugs.insert(heading_slug.clone(), title.to_string())
                && other != title
            {
                bail!(
                    "{}",
                    at(
                        i - 1,
                        &format!(
                            "sections `{other}` and `{title}` share the slug `{heading_slug}`"
                        )
                    )
                );
            }
            stack.push(Heading {
                level,
                title: title.to_string(),
                slug: heading_slug,
            });
            rubric = NO_RUBRIC.to_string();
            continue;
        }
        let directive = line.trim();
        if let Some(name) = directive.strip_prefix(".. rubric::") {
            rubric = slug(name);
            continue;
        }
        if directive == ".. informational-page::" {
            page_informational = true;
            continue;
        }
        if directive == ".. informational-section::" {
            let Some(current) = stack.last() else {
                bail!(
                    "{}",
                    at(i, "informational section before the chapter heading")
                );
            };
            informational_sections.insert(current.slug.clone());
            continue;
        }
        for m in paragraph.captures_iter(line) {
            let Some(section) = stack.last() else {
                bail!("{}", at(i, "paragraph before the chapter heading"));
            };
            paragraphs.push(ParsedParagraph {
                paragraph: Paragraph {
                    id: m[1].to_string(),
                    chapter: chapter.to_string(),
                    section: section.slug.clone(),
                    section_title: section.title.clone(),
                    informational: page_informational
                        || informational_sections.contains(&section.slug),
                },
                rubric: rubric.clone(),
                text: paragraph_text(&lines, i, &m[0]),
            });
        }
    }
    let Some(title) = stack.first().map(|h| h.title.clone()) else {
        bail!("{chapter}.rst: no heading");
    };
    Ok((title, paragraphs))
}

/// The identifier list of the sources in `dir`.
pub fn read_sources(dir: &Path, rev: &str, version: Option<String>) -> Result<IdList> {
    let src = dir.join("src");
    let index = fs::read_to_string(src.join("index.rst"))
        .with_context(|| format!("cannot read {}", src.join("index.rst").display()))?;
    let chapters = chapters_of_index(&index)?;
    let mut paragraphs: Vec<Paragraph> = vec![];
    let mut chapter_titles = BTreeMap::new();
    let mut seen: HashMap<String, String> = HashMap::new();
    for chapter in &chapters {
        let path = src.join(format!("{chapter}.rst"));
        let text = fs::read_to_string(&path).with_context(|| {
            format!(
                "chapter `{chapter}` listed in index.rst has no file {}",
                path.display()
            )
        })?;
        let (title, chapter_paragraphs) = parse_chapter(chapter, &text)?;
        for p in chapter_paragraphs.into_iter().map(|p| p.paragraph) {
            if let Some(other) = seen.insert(p.id.clone(), chapter.clone()) {
                bail!("`{}` appears in both {other}.rst and {chapter}.rst", p.id);
            }
            paragraphs.push(p);
        }
        chapter_titles.insert(chapter.clone(), title);
    }
    Ok(IdList {
        revision: rev.to_string(),
        version,
        chapters,
        chapter_titles,
        paragraphs,
    })
}

/// A revision for the log, naming the release where there is one.
fn revision_line(commit: &str, version: &Option<String>) -> String {
    match version {
        Some(version) => format!("{commit} ({version})"),
        None => commit.to_string(),
    }
}

fn ids_path(options: &Cli) -> PathBuf {
    options.tests_crate_dir().join("fls-ids.txt")
}

/// The rows of `fresh` that `committed` lacks and vice versa, grouped by chapter.
fn diff_rows(
    committed: &IdList,
    fresh: &IdList,
) -> (BTreeMap<String, Vec<String>>, BTreeMap<String, Vec<String>>) {
    let group = |rows: &[String], other: &HashSet<String>| {
        let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for row in rows.iter().filter(|r| !other.contains(*r)) {
            let chapter = row.split('\t').nth(1).unwrap_or("").to_string();
            out.entry(chapter).or_default().push(row.clone());
        }
        out
    };
    let committed_rows = committed.rows();
    let fresh_rows = fresh.rows();
    let committed_set: HashSet<String> = committed_rows.iter().cloned().collect();
    let fresh_set: HashSet<String> = fresh_rows.iter().cloned().collect();
    (
        group(&fresh_rows, &committed_set),
        group(&committed_rows, &fresh_set),
    )
}

/// Whether writing `fresh` would change the file of `committed`, apart from
/// the revision, which differs by design.
fn changes_file(committed: &IdList, fresh: &IdList) -> bool {
    let fresh = IdList {
        revision: committed.revision.clone(),
        version: committed.version.clone(),
        ..fresh.clone()
    };
    fresh.render() != committed.render()
}

/// The error of `refresh --check` when the identifier lists differ.
#[derive(Debug)]
pub struct Drift(String);

impl std::fmt::Display for Drift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Drift {}

pub fn run(options: &Cli, rev: &str, check: bool) -> Result<()> {
    let dir = fetch(options.cache_dir(), rev)?;
    let commit = resolved_commit(&dir)?;
    // A release tag is recorded beside the commit; a commit hash or a branch
    // names no release.
    let version = (!is_commit_hash(rev) && is_release_tag(rev)?).then(|| rev.to_string());
    let fresh = read_sources(&dir, &commit, version.clone())?;
    println!("chapters: {}", fresh.chapters.join(", "));

    let path = ids_path(options);
    if check {
        let committed = IdList::read(&path)?;
        let (added, removed) = diff_rows(&committed, &fresh);
        println!(
            "committed revision: {}",
            revision_line(&committed.revision, &committed.version)
        );
        println!("fetched revision:   {}", revision_line(&commit, &version));
        for (label, groups) in [("added", &added), ("removed", &removed)] {
            for (chapter, rows) in groups {
                println!("{chapter}: {} rows {label}", rows.len());
                for row in rows {
                    println!("  {label}: {row}");
                }
            }
        }
        if !changes_file(&committed, &fresh) {
            println!("{} is up to date", path.display());
            return Ok(());
        }
        if added.is_empty() && removed.is_empty() {
            println!("the order of the chapters or sections, or the chapter titles, differ");
        }
        return Err(Drift(format!(
            "{} differs from the sources at {rev}: {} rows added, {} rows removed",
            path.display(),
            added.values().map(Vec::len).sum::<usize>(),
            removed.values().map(Vec::len).sum::<usize>()
        ))
        .into());
    }

    fs::write(&path, fresh.render())?;
    for chapter in &fresh.chapters {
        let n = fresh
            .paragraphs
            .iter()
            .filter(|p| &p.chapter == chapter)
            .count();
        println!("{chapter}: {n} paragraphs");
    }
    println!(
        "wrote {} ({} paragraphs)",
        path.display(),
        fresh.paragraphs.len()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHAPTER: &str = "\
.. SPDX-License-Identifier: MIT

.. default-domain:: spec

.. _fls_chapter:

Values And Things
=================

:dp:`fls_top`
Directly under the chapter.

.. _fls_s1:

Attribute ``no_std``
--------------------

.. informational-section::

:dp:`fls_i1`
Informational.

.. _fls_s1a:

Deeper
~~~~~~

:dp:`fls_i2`
Under a subsection.

.. _fls_s1b:

Deepest
^^^^^^^

* :dp:`fls_I3Mixed`
  A bullet item.

.. _fls_s2:

Sibling
-------

.. rubric:: Legality Rules

:dp:`fls_n1`
Not informational any more.

.. list-table::

   * - :dp:`fls_n2`
     - In a table.
";

    #[test]
    fn synthetic_chapter() {
        let (title, paragraphs) = parse_chapter("values", CHAPTER).unwrap();
        assert_eq!(title, "Values And Things");
        let rows: Vec<(&str, &str, &str, bool)> = paragraphs
            .iter()
            .map(|p| &p.paragraph)
            .map(|p| {
                (
                    p.id.as_str(),
                    p.section.as_str(),
                    p.section_title.as_str(),
                    p.informational,
                )
            })
            .collect();
        assert_eq!(
            rows,
            [
                ("fls_top", "values", "Values And Things", false),
                ("fls_i1", "attribute-no-std", "Attribute ``no_std``", true),
                ("fls_i2", "deeper", "Deeper", false),
                ("fls_I3Mixed", "deepest", "Deepest", false),
                ("fls_n1", "sibling", "Sibling", false),
                ("fls_n2", "sibling", "Sibling", false),
            ]
        );
        let sources: Vec<(&str, &str)> = paragraphs
            .iter()
            .map(|p| (p.rubric.as_str(), p.text.as_str()))
            .collect();
        assert_eq!(
            sources,
            [
                ("none", "Directly under the chapter."),
                ("none", "Informational."),
                ("none", "Under a subsection."),
                ("none", "A bullet item."),
                ("legality-rules", "Not informational any more."),
                ("legality-rules", "In a table."),
            ]
        );
    }

    #[test]
    fn rubric_resets_at_headings() {
        let text = ".. _fls_c:\n\nChapter\n=======\n\n.. rubric:: Dynamic Semantics\n\n:dp:`fls_a`\nFirst line\nsecond line.\n\n.. _fls_s:\n\nS\n---\n\n:dp:`fls_b` inline text.\n";
        let (_, paragraphs) = parse_chapter("c", text).unwrap();
        assert_eq!(paragraphs[0].rubric, "dynamic-semantics");
        assert_eq!(paragraphs[0].text, "First line second line.");
        assert_eq!(paragraphs[1].rubric, "none");
        assert_eq!(paragraphs[1].text, "inline text.");
    }

    #[test]
    fn heading_without_anchor_fails() {
        let text = "\
.. _fls_c:

Chapter
=======

Orphan
------

:dp:`fls_a`
";
        let err = parse_chapter("c", text).unwrap_err().to_string();
        assert!(err.contains("c.rst:6") && err.contains("anchor"), "{err}");
    }

    #[test]
    fn heading_errors() {
        let wrong_level = ".. _fls_c:\n\nChapter\n-------\n";
        assert!(
            parse_chapter("c", wrong_level)
                .unwrap_err()
                .to_string()
                .contains("`=`")
        );
        let unknown = ".. _fls_c:\n\nChapter\n=======\n\n.. _fls_s:\n\nS\n###\n";
        assert!(
            parse_chapter("c", unknown)
                .unwrap_err()
                .to_string()
                .contains("unknown heading underline")
        );
        let dup =
            ".. _fls_c:\n\nChapter\n=======\n\n.. _fls_s:\n\nA B\n---\n\n.. _fls_t:\n\nA-B\n---\n";
        assert!(
            parse_chapter("c", dup)
                .unwrap_err()
                .to_string()
                .contains("share the slug")
        );
        let early_section = ".. informational-section::\n\n.. _fls_c:\n\nChapter\n=======\n";
        assert!(
            parse_chapter("c", early_section)
                .unwrap_err()
                .to_string()
                .contains("before the chapter heading")
        );
    }

    #[test]
    fn fetch_rejects_revisions_outside_the_cache() {
        let cache = Path::new("/nonexistent-cache");
        for rev in ["", "/home", "../..", "a/../../b", "-c"] {
            assert!(fetch(cache, rev).is_err(), "{rev:?}");
        }
    }

    #[test]
    fn informational_page_flags_the_chapter() {
        let text = ".. informational-page::\n\n.. _fls_c:\n\nChapter\n=======\n\n:dp:`fls_a`\n\n.. _fls_s:\n\nS\n---\n\n:dp:`fls_b`\n";
        let (_, paragraphs) = parse_chapter("c", text).unwrap();
        assert!(paragraphs.iter().all(|p| p.paragraph.informational));
    }

    #[test]
    fn index_chapters() {
        let text = "\
FLS
===

.. toctree::
   :numbered:
   :maxdepth: 2
   :caption: Contents:

   general
   lexical-elements
   inline-assembly

.. appendices::
   :numbered:

   licenses
";
        assert_eq!(
            chapters_of_index(text).unwrap(),
            ["general", "lexical-elements", "inline-assembly"]
        );
        assert!(
            chapters_of_index("FLS\n===\n")
                .unwrap_err()
                .to_string()
                .contains("toctree")
        );
        assert!(
            chapters_of_index(".. toctree::\n   :numbered:\n\n.. appendices::\n")
                .unwrap_err()
                .to_string()
                .contains("no chapter")
        );
    }

    #[test]
    fn slugs() {
        assert_eq!(slug("Attribute ``no_std``"), "attribute-no-std");
        assert_eq!(slug("Break Expressions"), "break-expressions");
        assert_eq!(slug("  Struct Types  "), "struct-types");
        assert_eq!(slug("FFI"), "ffi");
        assert_eq!(slug("Type Classification"), "type-classification");
    }

    #[test]
    fn diff_groups_by_chapter() {
        let p = |id: &str, ch: &str| Paragraph {
            id: id.into(),
            chapter: ch.into(),
            section: "s".into(),
            section_title: "S".into(),
            informational: false,
        };
        let committed =
            IdList::from_paragraphs("a".into(), None, vec![p("fls_1", "x"), p("fls_2", "y")]);
        let fresh =
            IdList::from_paragraphs("b".into(), None, vec![p("fls_1", "x"), p("fls_3", "y")]);
        let (added, removed) = diff_rows(&committed, &fresh);
        assert_eq!(added.keys().collect::<Vec<_>>(), ["y"]);
        assert!(added["y"][0].starts_with("fls_3\t"));
        assert!(removed["y"][0].starts_with("fls_2\t"));
    }

    #[test]
    fn check_sees_chapter_order_and_titles() {
        let p = |id: &str, ch: &str| Paragraph {
            id: id.into(),
            chapter: ch.into(),
            section: "s".into(),
            section_title: "S".into(),
            informational: false,
        };
        let list = |revision: &str, paragraphs| {
            let mut list = IdList::from_paragraphs(revision.into(), None, paragraphs);
            list.chapter_titles = [("x", "X"), ("y", "Y")]
                .into_iter()
                .map(|(c, t)| (c.into(), t.into()))
                .collect();
            list
        };
        let committed = list("a", vec![p("fls_1", "x"), p("fls_2", "y")]);
        assert!(!changes_file(
            &committed,
            &list("b", vec![p("fls_1", "x"), p("fls_2", "y")])
        ));
        assert!(changes_file(
            &committed,
            &list("b", vec![p("fls_2", "y"), p("fls_1", "x")])
        ));
        let q = |id: &str, section: &str| Paragraph {
            section: section.into(),
            ..p(id, "x")
        };
        let sections = list("a", vec![q("fls_1", "s1"), q("fls_2", "s2")]);
        assert!(changes_file(
            &sections,
            &list("b", vec![q("fls_2", "s2"), q("fls_1", "s1")])
        ));
        let mut renamed = list("b", vec![p("fls_1", "x"), p("fls_2", "y")]);
        renamed.chapter_titles.insert("y".into(), "Z".into());
        assert!(changes_file(&committed, &renamed));
    }
}
