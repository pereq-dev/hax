"""Renders the FLS report pages of the manual from the committed files of the
FLS test suite: the results files, the headers of the test files, the
exclusion file, and the identifier lists."""

from __future__ import annotations

import html
import re
import tomllib
from collections.abc import Callable
from dataclasses import dataclass, field
from pathlib import Path

FLS_RENDERING = "https://rust-lang.github.io/fls"
# The coverage page, relative to the backend pages (`manual/<backend>/support.md`).
COVERAGE = "../fls-coverage.md"
# The input-language section of the manual, relative to the backend pages
# (`manual/<backend>/support.md`) and to the coverage page (`manual/fls-coverage.md`).
INPUT_LANGUAGE = "../input-language/index.md"
INPUT_LANGUAGE_TIERS = f"{INPUT_LANGUAGE}#status-vocabulary"
INPUT_LANGUAGE_FROM_COVERAGE = "input-language/index.md"
FLS_SOURCES = "https://github.com/rust-lang/fls"
# The revision the test links point at. The deployed site renders the report
# from a release, and its build replaces this with that release's commit.
TESTS_REF = "main"
TESTS_URL = f"https://github.com/pereq-dev/hax/blob/{TESTS_REF}/tests/fls/src"

# The classification names the driver writes, in report order. Names outside
# this list are appended in alphabetical order.
CLASSIFICATIONS = [
    "supported",
    "planned",
    "unsupported",
    "unexpectedly accepted, to be triaged",
]
PLANNED = "planned"

# A supported test departs from nothing, so the report drops any reason it
# carries.
WITHOUT_REASON = "supported"

# The colour class of each status, styled by the report stylesheet.
STATUS_CLASSES = {
    "supported": "fls-supported",
    "planned": "fls-planned",
    "unsupported": "fls-out-of-scope",
}
UNEXPECTED = "unexpectedly accepted, to be triaged"

# The manual exclusion categories, in report order.
CATEGORIES = [
    "lexical",
    "legality",
    "out-of-scope",
    "unsafe",
    "ffi",
    "inline-assembly",
    "concurrency",
    "macro-expanded",
]

# The key of an `@<backend>(reason)` directive, with the whitespace the
# directive grammar of the test driver allows.
REASON_KEY = re.compile(r"([A-Za-z0-9]+(?:-[A-Za-z0-9]+)*)\s*\(\s*reason\s*\)")

CHAPTER_PREFIX = "# chapter:"
REVISION_PREFIX = "# fls-revision:"
VERSION_PREFIX = "# fls-version:"


@dataclass
class Paragraph:
    id: str
    chapter: str
    section: str
    section_title: str
    informational: bool


@dataclass
class IdList:
    revision: str
    version: str | None
    chapters: list[str]
    chapter_titles: dict[str, str]
    paragraphs: list[Paragraph]

    def title(self, chapter: str) -> str:
        return self.chapter_titles.get(chapter, chapter)


@dataclass
class Header:
    """The directives of a test file that the report shows."""

    title: str
    fls: list[str]
    # By backend name.
    reasons: dict[str, str]


@dataclass
class TestEntry:
    path: str
    stage: str
    tier: str
    classification: str
    title: str
    fls: list[str]
    reason: str | None

    @property
    def chapter(self) -> str:
        return self.path.split("/")[0]


@dataclass
class Backend:
    name: str
    tools: dict[str, str]
    tests: dict[str, TestEntry]


@dataclass
class ExclusionTable:
    chapter: str
    category: str
    reason: str
    ids: list[str]
    overrides: dict[str, str]


@dataclass
class Report:
    ids: IdList
    backends: dict[str, Backend]
    pending_chapters: list[str]
    exclusions: list[ExclusionTable]
    # Whether the manual has the input-language section. The site may build
    # the manual of a release that predates it, so the pages link to it only
    # when it is there.
    input_language: bool = True
    # The header of every test file by its path. A test claims its paragraphs
    # whether or not a results file lists it, as in the coverage check.
    headers: dict[str, Header] = field(default_factory=dict)
    claimed: dict[str, set[str]] = field(default_factory=dict)
    by_id: dict[str, Paragraph] = field(default_factory=dict)

    def __post_init__(self):
        self.by_id = {p.id: p for p in self.ids.paragraphs}
        for path, h in self.headers.items():
            for id in h.fls:
                self.claimed.setdefault(id, set()).add(path)


# ----------------------------------------------------------------- loading


def parse_id_list(text: str) -> IdList:
    lines = text.splitlines()
    if not lines or not lines[0].startswith(REVISION_PREFIX):
        raise ValueError(f"identifier list does not start with `{REVISION_PREFIX}`")
    revision = lines[0][len(REVISION_PREFIX) :].strip()
    version = None
    chapters: list[str] = []
    titles: dict[str, str] = {}
    paragraphs: list[Paragraph] = []
    for line in lines[1:]:
        if line.startswith(VERSION_PREFIX):
            version = line[len(VERSION_PREFIX) :].strip()
            continue
        if line.startswith(CHAPTER_PREFIX):
            slug, title = line[len(CHAPTER_PREFIX) :].lstrip().split("\t", 1)
            titles[slug] = title.strip()
            continue
        if line.startswith("#") or not line.strip():
            continue
        id, chapter, section, section_title, flags = line.split("\t")
        if chapter not in chapters:
            chapters.append(chapter)
        paragraphs.append(
            Paragraph(
                id,
                chapter,
                section,
                section_title,
                "informational" in [f.strip() for f in flags.split(",")],
            )
        )
    return IdList(revision, version, chapters, titles, paragraphs)


def merge_id_lists(fls: IdList, hax: IdList) -> IdList:
    return IdList(
        fls.revision,
        fls.version,
        fls.chapters + hax.chapters,
        {**fls.chapter_titles, **hax.chapter_titles},
        fls.paragraphs + hax.paragraphs,
    )


def parse_header(text: str) -> Header:
    """The `@title`, `@fls`, and `@<backend>(reason)` directives of the `//!`
    header, the leading run of empty and `//!` lines of a test file."""
    title = ""
    fls: list[str] = []
    reasons: dict[str, str] = {}
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        if not line.startswith("//!"):
            break
        directive = line[3:].strip()
        if not directive.startswith("@"):
            continue
        key, _, value = directive[1:].partition(":")
        key, value = key.strip(), value.strip()
        if key == "title":
            title = value
        elif key == "fls":
            fls += [id.strip() for id in value.split(",") if id.strip()]
        elif m := REASON_KEY.fullmatch(key):
            # The driver reads the backend name case-insensitively.
            reasons[m[1].lower()] = value
    return Header(title, fls, reasons)


def parse_results(text: str, header: Callable[[str], Header]) -> Backend:
    """The results file of a backend, with the header of each test by its path."""
    data = tomllib.loads(text)
    name = data["backend"]
    tests = {}
    for path, t in data.get("tests", {}).items():
        h = header(path)
        tests[path] = TestEntry(
            path,
            t["stage"],
            t["tier"],
            t.get("classification", ""),
            h.title,
            h.fls,
            h.reasons.get(name),
        )
    return Backend(
        name,
        {k: str(v) for k, v in data.get("tools", {}).items()},
        tests,
    )


def parse_exclusions(text: str) -> tuple[list[str], list[ExclusionTable]]:
    data = tomllib.loads(text)
    pending = list(data.get("pending-chapters", []))
    tables = []
    for chapter, categories in data.items():
        if chapter == "pending-chapters":
            continue
        for category, entry in categories.items():
            tables.append(
                ExclusionTable(
                    chapter,
                    category,
                    entry["reason"],
                    list(entry["ids"]),
                    dict(entry.get("overrides", {})),
                )
            )
    return pending, tables


def inputs_present(repo_root: Path) -> bool:
    """Whether a checkout carries the suite; the hook adds no page otherwise."""
    tests = repo_root / "tests"
    return all(
        (tests / name).exists()
        for name in ("fls-ids.txt", "hax-ids.txt", "fls-excluded.toml", "fls-results", "fls/src")
    )


def load(repo_root: Path) -> Report:
    tests = repo_root / "tests"
    ids = merge_id_lists(
        parse_id_list((tests / "fls-ids.txt").read_text()),
        parse_id_list((tests / "hax-ids.txt").read_text()),
    )
    pending, exclusions = parse_exclusions((tests / "fls-excluded.toml").read_text())
    src = tests / "fls" / "src"
    # Hidden entries, such as editor lock files, are no tests, as for the
    # test driver.
    headers = {
        path.relative_to(src).with_suffix("").as_posix(): parse_header(path.read_text())
        for path in sorted(src.rglob("*.rs"))
        if not any(part.startswith(".") for part in path.relative_to(src).parts)
    }
    backends = {}
    for path in sorted((tests / "fls-results").glob("*.toml")):
        backend = parse_results(path.read_text(), headers.__getitem__)
        backends[backend.name] = backend
    input_language = (repo_root / "docs" / "manual" / "input-language" / "index.md").exists()
    return Report(ids, backends, pending, exclusions, input_language, headers)


# --------------------------------------------------------------- rendering


def input_language_link(report: Report, text: str, target: str) -> str:
    return f"[{text}]({target})" if report.input_language else text


def front_matter(weight: int, headless: bool = False) -> str:
    """Metadata for the nav plugin; a headless page is built but stays out of
    the navigation, reached through the links that name it."""
    out = f"---\nweight: {weight}\n"
    if headless:
        out += "headless: true\n"
    return out + "---\n\n"


# A code span; the table splitter of Python-Markdown leaves pipes inside one
# alone and would render an escape there literally.
CODE_SPAN = re.compile(r"(`+).+?\1")


def escape(text: str) -> str:
    """Text safe inside a table cell."""
    out, pos = [], 0
    for m in CODE_SPAN.finditer(text):
        out += [text[pos : m.start()].replace("|", "\\|"), m[0]]
        pos = m.end()
    out.append(text[pos:].replace("|", "\\|"))
    return "".join(out).replace("\n", " ")


def status(test: TestEntry) -> str:
    """The status of a test: its classification, coloured, carrying its reason, if
    any, as a tooltip. A tooltip is plain text, so the markup of
    the reason is dropped rather than rendered."""
    classes = ["fls-status", STATUS_CLASSES.get(test.classification, "fls-unexpected")]
    title = ""
    if test.reason and test.classification != WITHOUT_REASON:
        classes.append("fls-reason")
        plain = test.reason.replace("`", "").replace("*", "").replace("\n", " ")
        title = f' title="{html.escape(plain, quote=True).replace("|", "&#124;")}"'
    return f'<span class="{" ".join(classes)}"{title}>{test.classification}</span>'


def paragraph_link(report: Report, id: str) -> str:
    """The paragraph in the online rendering; hax identifiers are not rendered."""
    paragraph = report.by_id.get(id)
    if paragraph is None or not id.startswith("fls_"):
        return f"`{id}`"
    return f"[`{id}`]({FLS_RENDERING}/{paragraph.chapter}.html#{id})"


def paragraph_cell(report: Report, test: TestEntry) -> str:
    """The claimed paragraphs: the first two, and the rest behind a link to their
    chapter, so a test claiming dozens of them keeps the table readable."""
    shown = [paragraph_link(report, id) for id in test.fls[:2]]
    rest = test.fls[2:]
    if not rest:
        return ", ".join(shown)
    paragraph = report.by_id.get(rest[0])
    more = f"+{len(rest)}"
    if paragraph is not None and rest[0].startswith("fls_"):
        more = f"[{more}]({FLS_RENDERING}/{paragraph.chapter}.html)"
    return ", ".join(shown + [more])


def test_link(test: TestEntry) -> str:
    return f"[{escape(test.title)}]({TESTS_URL}/{test.path}.rs)"


def table(header: list[str], rows: list[list[str]]) -> str:
    out = "| " + " | ".join(header) + " |\n"
    out += "|" + "|".join(" --- " for _ in header) + "|\n"
    for row in rows:
        out += "| " + " | ".join(row) + " |\n"
    return out + "\n"


def revision_note(report: Report) -> str:
    rev = report.ids.revision
    source = f"[`{rev[:10]}`]({FLS_SOURCES}/tree/{rev}/src)"
    if report.ids.version:
        source = f"release {report.ids.version}, commit {source}"
    else:
        source = f"commit {source}"
    return (
        f"The paragraph identifiers are those of [rust-lang/fls]({FLS_SOURCES}) at "
        f"{source}. Paragraph links go to the "
        f"[online rendering]({FLS_RENDERING}/) of the specification, which follows its "
        f"`main` branch and may be ahead of that commit.\n\n"
    )


def chapter_counts(tests: list[TestEntry]) -> str:
    """One line under a chapter heading, so a chapter can be skipped."""
    planned = sum(1 for t in tests if t.classification == PLANNED)
    unexpected = sum(1 for t in tests if t.classification == UNEXPECTED)
    parts = [f"{len(tests) - planned - unexpected} as intended"]
    if planned:
        parts.append(f"{planned} planned")
    if unexpected:
        parts.append(f"{unexpected} unexpectedly accepted")
    return f"{len(tests)} tests, " + ", ".join(parts) + ".\n\n"


def classification_order(names: set[str]) -> list[str]:
    known = [c for c in CLASSIFICATIONS if c in names]
    return known + sorted(names - set(CLASSIFICATIONS))


def render_backend(report: Report, backend_name: str) -> str:
    backend = report.backends[backend_name]
    tests = list(backend.tests.values())
    out = front_matter(50)
    out += "# Language support\n\n"
    out += (
        f"This page records what the `{backend.name}` backend supports of the Rust "
        "language, construct by construct: what extracts today, what is planned, "
        "and what the input language leaves out.\n\n"
    )

    out += "## Summary\n\n"
    names = {t.classification for t in tests}
    rows = []
    for c in classification_order(names):
        n = sum(1 for t in tests if t.classification == c)
        rows.append([c, str(n), f"{round(100 * n / len(tests))}%"])
    rows.append(["**all tests**", f"**{len(tests)}**", "**100%**"])
    out += table(["Status", "Tests", "Share"], rows)

    out += "## Constructs by chapter\n\n"
    out += (
        "Every construct has a test in the FLS test suite, which claims the paragraphs "
        f"it covers of the [FLS]({FLS_RENDERING}/), the Rust project's language "
        f"specification. The {input_language_link(report, '*status*', INPUT_LANGUAGE_TIERS)} "
        "compares the support the "
        f"{input_language_link(report, 'input-language reference', INPUT_LANGUAGE)} "
        "intends for the construct with "
        "how far the pipeline gets on the test; a status with a reason shows it on "
        "hover.\n\n"
    )
    out += revision_note(report)
    out += (
        "Which paragraphs of the specification the suite accounts for, and how, is on "
        f"the [FLS coverage]({COVERAGE}) page. The results below were produced with "
        "these tool versions:\n\n"
    )
    out += table(
        ["Tool", "Version"], [[f"`{k}`", f"`{v}`"] for k, v in backend.tools.items()]
    )
    chapters = list(report.ids.chapters)
    chapters += sorted({t.chapter for t in tests} - set(chapters))
    without_tests = []
    for chapter in chapters:
        in_chapter = sorted(
            (t for t in tests if t.chapter == chapter), key=lambda t: t.path
        )
        if not in_chapter:
            without_tests.append(report.ids.title(chapter))
            continue
        out += f"### {report.ids.title(chapter)}\n\n"
        out += chapter_counts(in_chapter)
        rows = []
        for t in in_chapter:
            rows.append(
                [
                    test_link(t),
                    paragraph_cell(report, t),
                    status(t),
                ]
            )
        out += table(["Test", "Paragraphs", "Status"], rows)
    if without_tests:
        out += "No test yet in: " + ", ".join(without_tests) + ".\n\n"

    planned = sorted((t for t in tests if t.classification == PLANNED), key=lambda t: t.path)
    out += "## Planned\n\n"
    if planned:
        out += (
            f"Constructs the {input_language_link(report, 'input-language reference', INPUT_LANGUAGE)} "
            "intends to support "
            "that the pipeline does not carry through yet.\n\n"
        )
        out += table(
            ["Test", "Stage", "Reason"],
            [
                [test_link(t), f"`{t.stage}`", escape(t.reason or "")]
                for t in planned
            ],
        )
    else:
        out += "None.\n\n"

    unexpected = sorted(
        (t for t in tests if t.classification == UNEXPECTED), key=lambda t: t.path
    )
    if unexpected:
        out += "## Unexpectedly accepted\n\n"
        out += (
            f"Constructs the {input_language_link(report, 'input-language reference', INPUT_LANGUAGE)} "
            "places outside the "
            "value model that the pipeline accepts. Each is either an outdated entry of the "
            "input-language reference or "
            "a soundness question, and is triaged before the next release.\n\n"
        )
        out += table(
            ["Test", "Stage", "Reason"],
            [[test_link(t), f"`{t.stage}`", escape(t.reason or "")] for t in unexpected],
        )
    return out


def render_coverage(report: Report) -> str:
    ids = report.ids
    excluded: dict[str, ExclusionTable] = {}
    for t in report.exclusions:
        for id in t.ids:
            excluded[id] = t
    pending = set(report.pending_chapters)

    out = front_matter(101, headless=True)
    out += "# FLS coverage\n\n"
    out += (
        f"How the paragraphs of the [FLS]({FLS_RENDERING}/), the Rust project's language "
        "specification, are accounted for by the FLS test suite. Every paragraph is "
        "claimed by a test, excluded with a reason, informational, or in a chapter not "
        "yet triaged, so this page has no list of uncovered paragraphs. The `hax` "
        "chapter lists the specification surface of `hax-lib`, which is not part of "
        "the FLS; the "
        f"{input_language_link(report, 'input-language reference', INPUT_LANGUAGE_FROM_COVERAGE)} "
        "describes it.\n\n"
    )
    out += revision_note(report)

    out += "## Chapters\n\n"
    rows = []
    totals = {"n": 0, "covered": 0, "excluded": 0, "informational": 0, "pending": 0}
    for chapter in ids.chapters:
        paragraphs = [p for p in ids.paragraphs if p.chapter == chapter]
        covered = [p for p in paragraphs if p.id in report.claimed]
        rest = [p for p in paragraphs if p.id not in report.claimed]
        per_category: dict[str, int] = {}
        for p in rest:
            if p.id in excluded:
                c = excluded[p.id].category
                per_category[c] = per_category.get(c, 0) + 1
        excluded_n = sum(per_category.values())
        informational = [
            p for p in rest if p.informational and p.id not in excluded
        ]
        pending_n = (
            len(rest) - excluded_n - len(informational) if chapter in pending else 0
        )
        breakdown = ", ".join(
            f"{c} {per_category[c]}"
            for c in CATEGORIES + sorted(set(per_category) - set(CATEGORIES))
            if c in per_category
        )
        excluded_cell = f"{excluded_n} ({breakdown})" if breakdown else "0"
        rows.append(
            [
                f"[{ids.title(chapter)}](#{chapter})"
                if chapter in {t.chapter for t in report.exclusions}
                else ids.title(chapter),
                str(len(paragraphs)),
                str(len(covered)),
                excluded_cell,
                str(len(informational)),
                str(pending_n) if chapter in pending else "",
            ]
        )
        totals["n"] += len(paragraphs)
        totals["covered"] += len(covered)
        totals["excluded"] += excluded_n
        totals["informational"] += len(informational)
        totals["pending"] += pending_n
    rows.append(
        [
            "**all chapters**",
            f"**{totals['n']}**",
            f"**{totals['covered']}**",
            f"**{totals['excluded']}**",
            f"**{totals['informational']}**",
            f"**{totals['pending']}**",
        ]
    )
    out += table(
        ["Chapter", "Paragraphs", "Covered", "Excluded", "Informational", "Pending"],
        rows,
    )
    out += (
        "*Covered* paragraphs are claimed by at least one test of any backend. "
        "*Informational* paragraphs carry the flag of the identifier list and are "
        "neither claimed nor excluded. *Pending* paragraphs belong to a chapter not "
        "yet triaged and are not claimed.\n\n"
    )

    if report.pending_chapters:
        out += "## Pending chapters\n\n"
        out += (
            "Chapters whose paragraphs have not been triaged yet. The pull request that "
            "completes the triage of a chapter adds its exclusions and removes it from "
            "the pending chapters.\n\n"
        )
        for chapter in report.pending_chapters:
            out += f"- {ids.title(chapter)}\n"
        out += "\n"

    out += "## Excluded paragraphs\n\n"
    if not report.exclusions:
        out += "None yet.\n\n"
    for chapter in ids.chapters:
        tables = [t for t in report.exclusions if t.chapter == chapter]
        if not tables:
            continue
        out += f"### {ids.title(chapter)} {{ #{chapter} }}\n\n"
        tables.sort(
            key=lambda t: (
                CATEGORIES.index(t.category)
                if t.category in CATEGORIES
                else len(CATEGORIES),
                t.category,
            )
        )
        for t in tables:
            n = len(t.ids)
            out += f"**{t.category}**, {n} paragraph{'s' if n != 1 else ''}: {t.reason}\n\n"
            for id in t.ids:
                line = f"- {paragraph_link(report, id)}"
                if id in t.overrides:
                    line += f": {t.overrides[id]}"
                out += line + "\n"
            out += "\n"
    return out
