//! The identifier lists `tests/fls-ids.txt` and `tests/hax-ids.txt`: one
//! paragraph per row with its chapter and section, tab-separated, after
//! comment lines carrying the revision, the columns, and the chapter titles.

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    path::Path,
};

use anyhow::{Context as _, Result, bail};

/// A specification paragraph with the section it belongs to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Paragraph {
    pub id: String,
    pub chapter: String,
    pub section: String,
    pub section_title: String,
    pub informational: bool,
}

/// An identifier list. The chapter order is the order of first appearance in
/// the rows, which the writer preserves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdList {
    pub revision: String,
    /// The release the revision is tagged with, where it is a tagged one.
    pub version: Option<String>,
    pub chapters: Vec<String>,
    /// Chapter slug to title, from the `# chapter:` lines.
    pub chapter_titles: BTreeMap<String, String>,
    pub paragraphs: Vec<Paragraph>,
}

const REVISION_PREFIX: &str = "# fls-revision:";
const VERSION_PREFIX: &str = "# fls-version:";
const COLUMNS_LINE: &str = "# columns: id\tchapter\tsection\tsection-title\tflags";
const CHAPTER_PREFIX: &str = "# chapter:";

impl IdList {
    pub fn read(path: &Path) -> Result<Self> {
        let text =
            fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;
        Self::parse(&text).with_context(|| format!("in {}", path.display()))
    }

    pub fn parse(text: &str) -> Result<Self> {
        let mut lines = text.lines().enumerate();
        let Some((_, first)) = lines.next() else {
            bail!("empty identifier list");
        };
        let Some(revision) = first.strip_prefix(REVISION_PREFIX) else {
            bail!("line 1: expected `{REVISION_PREFIX} <revision>`");
        };
        let revision = revision.trim().to_string();
        if revision.is_empty() {
            bail!("line 1: empty revision");
        }
        let mut version = None;
        let mut paragraphs = vec![];
        let mut chapter_titles = BTreeMap::new();
        let mut seen: HashMap<&str, usize> = HashMap::new();
        for (i, line) in lines {
            let line_no = i + 1;
            if let Some(rest) = line.strip_prefix(VERSION_PREFIX) {
                if rest.trim().is_empty() {
                    bail!("line {line_no}: empty version");
                }
                version = Some(rest.trim().to_string());
                continue;
            }
            if let Some(rest) = line.strip_prefix(CHAPTER_PREFIX) {
                let Some((slug, title)) = rest.trim_start().split_once('\t') else {
                    bail!("line {line_no}: expected `{CHAPTER_PREFIX} <slug>\t<title>`");
                };
                if slug.is_empty() || title.trim().is_empty() {
                    bail!("line {line_no}: empty chapter slug or title");
                }
                if chapter_titles
                    .insert(slug.to_string(), title.trim().to_string())
                    .is_some()
                {
                    bail!("line {line_no}: chapter `{slug}` has two titles");
                }
                continue;
            }
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            let fields: Vec<&str> = line.split('\t').collect();
            let [id, chapter, section, section_title, flags] = fields[..] else {
                bail!(
                    "line {line_no}: expected 5 tab-separated columns, found {}",
                    fields.len()
                );
            };
            if id.is_empty() || chapter.is_empty() || section.is_empty() || section_title.is_empty()
            {
                bail!("line {line_no}: empty column");
            }
            let mut informational = false;
            for flag in flags.split(',').map(str::trim).filter(|f| !f.is_empty()) {
                match flag {
                    "informational" => informational = true,
                    other => bail!("line {line_no}: unknown flag `{other}`"),
                }
            }
            if let Some(previous) = seen.insert(id, line_no) {
                bail!("line {line_no}: `{id}` already listed on line {previous}");
            }
            paragraphs.push(Paragraph {
                id: id.to_string(),
                chapter: chapter.to_string(),
                section: section.to_string(),
                section_title: section_title.to_string(),
                informational,
            });
        }
        let mut list = Self::from_paragraphs(revision, version, paragraphs);
        if let Some(slug) = chapter_titles.keys().find(|c| !list.chapters.contains(c)) {
            bail!("chapter `{slug}` has a title but no row");
        }
        list.chapter_titles = chapter_titles;
        Ok(list)
    }

    /// A list without chapter titles whose chapter order is the order of
    /// first appearance.
    pub fn from_paragraphs(
        revision: String,
        version: Option<String>,
        paragraphs: Vec<Paragraph>,
    ) -> Self {
        let mut chapters: Vec<String> = vec![];
        for p in &paragraphs {
            if !chapters.contains(&p.chapter) {
                chapters.push(p.chapter.clone());
            }
        }
        Self {
            revision,
            version,
            chapters,
            chapter_titles: BTreeMap::new(),
            paragraphs,
        }
    }

    /// The rows in file order: chapter order, then section order of first
    /// appearance, then identifier.
    pub fn rows(&self) -> Vec<String> {
        let chapter_index: HashMap<&str, usize> = self
            .chapters
            .iter()
            .enumerate()
            .map(|(i, c)| (c.as_str(), i))
            .collect();
        let mut section_index: HashMap<(&str, &str), usize> = HashMap::new();
        for p in &self.paragraphs {
            let next = section_index.len();
            section_index
                .entry((p.chapter.as_str(), p.section.as_str()))
                .or_insert(next);
        }
        let mut sorted: Vec<&Paragraph> = self.paragraphs.iter().collect();
        sorted.sort_by_key(|p| {
            (
                chapter_index[p.chapter.as_str()],
                section_index[&(p.chapter.as_str(), p.section.as_str())],
                p.id.as_str(),
            )
        });
        sorted.iter().map(|p| row(p)).collect()
    }

    pub fn render(&self) -> String {
        let mut out = format!("{REVISION_PREFIX} {}\n", self.revision);
        if let Some(version) = &self.version {
            out += &format!("{VERSION_PREFIX} {version}\n");
        }
        out += &format!("{COLUMNS_LINE}\n");
        // A chapter without paragraphs has no row, and `parse` rejects a
        // title without rows.
        let with_rows: HashSet<&str> = self.paragraphs.iter().map(|p| p.chapter.as_str()).collect();
        for chapter in &self.chapters {
            if let Some(title) = self.chapter_titles.get(chapter)
                && with_rows.contains(chapter.as_str())
            {
                out += &format!("{CHAPTER_PREFIX} {chapter}\t{title}\n");
            }
        }
        for row in self.rows() {
            out += &row;
            out.push('\n');
        }
        out
    }

    /// One list for the coverage check, keeping the revision of the FLS list.
    pub fn merged(fls: Self, hax: Self) -> Result<Self> {
        if let Some(chapter) = hax.chapters.iter().find(|c| fls.chapters.contains(c)) {
            bail!("chapter `{chapter}` appears in both identifier lists");
        }
        let mut paragraphs = fls.paragraphs;
        paragraphs.extend(hax.paragraphs);
        let mut chapters = fls.chapters;
        chapters.extend(hax.chapters);
        let mut chapter_titles = fls.chapter_titles;
        chapter_titles.extend(hax.chapter_titles);
        Ok(Self {
            revision: fls.revision,
            version: fls.version,
            chapters,
            chapter_titles,
            paragraphs,
        })
    }
}

fn row(p: &Paragraph) -> String {
    let flags = if p.informational { "informational" } else { "" };
    format!(
        "{}\t{}\t{}\t{}\t{}",
        p.id, p.chapter, p.section, p.section_title, flags
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paragraph(id: &str, chapter: &str, section: &str, informational: bool) -> Paragraph {
        Paragraph {
            id: id.into(),
            chapter: chapter.into(),
            section: section.into(),
            section_title: section.to_uppercase(),
            informational,
        }
    }

    #[test]
    fn a_release_tag_round_trips_beside_the_commit() {
        let text = "# fls-revision: abc\n# fls-version: 1.98\n\
                    # columns: id\tchapter\tsection\tsection-title\tflags\n\
                    fls_a\tvalues\tstatics\tSTATICS\t\n";
        let list = IdList::parse(text).unwrap();
        assert_eq!(list.revision, "abc");
        assert_eq!(list.version.as_deref(), Some("1.98"));
        assert_eq!(list.render(), text);
    }

    #[test]
    fn render_round_trips_and_sorts() {
        let mut list = IdList::from_paragraphs(
            "abc".into(),
            None,
            vec![
                paragraph("fls_b", "values", "statics", false),
                paragraph("fls_a", "values", "statics", true),
                paragraph("fls_c", "values", "constants", false),
                paragraph("fls_d", "general", "general", true),
            ],
        );
        list.chapter_titles.insert("values".into(), "Values".into());
        assert_eq!(list.chapters, ["values", "general"]);
        let text = list.render();
        assert_eq!(
            text,
            "# fls-revision: abc\n# columns: id\tchapter\tsection\tsection-title\tflags\n\
             # chapter: values\tValues\n\
             fls_a\tvalues\tstatics\tSTATICS\tinformational\n\
             fls_b\tvalues\tstatics\tSTATICS\t\n\
             fls_c\tvalues\tconstants\tCONSTANTS\t\n\
             fls_d\tgeneral\tgeneral\tGENERAL\tinformational\n"
        );
        let back = IdList::parse(&text).unwrap();
        assert_eq!(back.revision, "abc");
        assert_eq!(back.chapters, list.chapters);
        assert_eq!(back.chapter_titles, list.chapter_titles);
        assert_eq!(back.render(), text);
    }

    #[test]
    fn chapter_title_errors() {
        let head = "# fls-revision: r\n# columns: x\n";
        let row = "fls_a\tc\ts\tS\t\n";
        let cases = [
            (
                format!("{head}# chapter: c\tC\n# chapter: c\tD\n{row}"),
                "two titles",
            ),
            (
                format!("{head}# chapter: c C\n{row}"),
                "expected `# chapter:",
            ),
            (format!("{head}# chapter: c\t\n{row}"), "empty chapter"),
            (format!("{head}# chapter: other\tO\n{row}"), "no row"),
        ];
        for (text, expected) in cases {
            let err = IdList::parse(&text).unwrap_err().to_string();
            assert!(err.contains(expected), "{text:?}: {err}");
        }
    }

    #[test]
    fn parse_errors() {
        let head = "# fls-revision: r\n# columns: x\n";
        let dup = format!("{head}fls_a\tc\ts\tS\t\nfls_a\tc\ts\tS\t\n");
        assert!(
            IdList::parse(&dup)
                .unwrap_err()
                .to_string()
                .contains("already listed")
        );
        let flag = format!("{head}fls_a\tc\ts\tS\tpending\n");
        assert!(
            IdList::parse(&flag)
                .unwrap_err()
                .to_string()
                .contains("unknown flag")
        );
        let cols = format!("{head}fls_a\tc\ts\n");
        assert!(
            IdList::parse(&cols)
                .unwrap_err()
                .to_string()
                .contains("5 tab-separated")
        );
        assert!(
            IdList::parse("fls_a\tc\ts\tS\t\n")
                .unwrap_err()
                .to_string()
                .contains("line 1")
        );
    }

    #[test]
    fn merged_rejects_shared_chapter() {
        let a = IdList::from_paragraphs(
            "r".into(),
            None,
            vec![paragraph("fls_a", "hax", "s", false)],
        );
        let b = IdList::from_paragraphs(
            "hand-written".into(),
            None,
            vec![paragraph("hax_a", "hax", "s", false)],
        );
        assert!(IdList::merged(a.clone(), b.clone()).is_err());
        let c = IdList::from_paragraphs(
            "r".into(),
            None,
            vec![paragraph("fls_a", "values", "s", false)],
        );
        let mut c = c;
        c.chapter_titles.insert("values".into(), "V".into());
        let mut b = b;
        b.chapter_titles.insert("hax".into(), "hax".into());
        let merged = IdList::merged(c, b).unwrap();
        assert_eq!(merged.revision, "r");
        assert_eq!(merged.chapters, ["values", "hax"]);
        assert_eq!(merged.chapter_titles.len(), 2);
        assert_eq!(merged.paragraphs.len(), 2);
    }
}
