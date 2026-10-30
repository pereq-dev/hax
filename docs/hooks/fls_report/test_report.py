import unittest
from pathlib import Path

import report

REPO_ROOT = Path(__file__).resolve().parents[3]

IDS = """# fls-revision: 0123456789abcdef
# columns: id\tchapter\tsection\tsection-title\tflags
# chapter: values\tValues
# chapter: general\tGeneral
fls_v1\tvalues\tstatics\tStatics\t
fls_v2\tvalues\tstatics\tStatics\t
fls_v3\tvalues\tconstants\tConstants\tinformational
fls_v4\tvalues\tconstants\tConstants\t
fls_v5\tvalues\tconstants\tConstants\t
fls_g1\tgeneral\tgeneral\tGeneral\tinformational
fls_g2\tgeneral\tgeneral\tGeneral\t
"""

HAX_IDS = """# fls-revision: hand-written
# columns: id\tchapter\tsection\tsection-title\tflags
# chapter: hax\thax
hax_requires\thax\tcontracts\tContracts\t
"""

EXCLUDED = """pending-chapters = ["general", "hax"]

[values.legality]
reason = "rustc rejects these before any backend runs"
ids = ["fls_v4", "fls_v5"]

[values.legality.overrides]
fls_v5 = "a special case"
"""

RESULTS = """backend = "lean"

[tools]
hax = "1.0"
lean = "leanprover/lean4:v4.31.0"

[tests."values/statics/supported"]
stage = "checked"
tier = "supported"
classification = "supported"

[tests."values/statics/gap"]
stage = "aeneas-rejected"
tier = "supported"
classification = "planned"

[tests."values/statics/unsupported"]
stage = "charon-rejected"
tier = "unsupported"
classification = "unsupported"

[tests."values/statics/accepted"]
stage = "extracted"
tier = "unsupported"
classification = "unexpectedly accepted, to be triaged"
"""

HEADERS = {
    "values/statics/supported": """\
//! @fls: fls_v1, fls_v3
//! @title: Plain static
""",
    "values/statics/gap": """\
//! @fls: fls_v2
//! @title: Static with | a pipe
//! @lean(reason): the loops section of the input-language reference
""",
    "values/statics/unsupported": """\
//! @fls: fls_v1
//! @title: Outside the value model
//! @lean(reason): the value-model boundary of the input-language reference
""",
    "values/statics/accepted": """\
//! @fls: fls_v2
//! @title: Should have been rejected
""",
}


def header(test: str) -> report.Header:
    return report.parse_header(HEADERS[test])


def synthetic() -> report.Report:
    ids = report.merge_id_lists(report.parse_id_list(IDS), report.parse_id_list(HAX_IDS))
    pending, exclusions = report.parse_exclusions(EXCLUDED)
    backend = report.parse_results(RESULTS, header)
    headers = {test: header(test) for test in HEADERS}
    return report.Report(ids, {backend.name: backend}, pending, exclusions, headers=headers)


class ParsingTest(unittest.TestCase):
    def test_id_list(self):
        ids = report.parse_id_list(IDS)
        self.assertEqual(ids.revision, "0123456789abcdef")
        self.assertEqual(ids.chapters, ["values", "general"])
        self.assertEqual(ids.chapter_titles, {"values": "Values", "general": "General"})
        self.assertEqual(len(ids.paragraphs), 7)
        self.assertTrue(ids.paragraphs[2].informational)
        self.assertFalse(ids.paragraphs[0].informational)

    def test_header(self):
        header = report.parse_header(
            "\n//! @fls: fls_a, fls_b\n//! Doc text.\n//! @fls: fls_c\n//! @title: T\n"
            "//! @lean(tier): supported\n//! @lean(reason): why: because\n"
            "pub fn f() {}\n//! @title: not read\n"
        )
        self.assertEqual(header.title, "T")
        self.assertEqual(header.fls, ["fls_a", "fls_b", "fls_c"])
        self.assertEqual(header.reasons, {"lean": "why: because"})

    def test_header_reason_whitespace(self):
        header = report.parse_header("//! @lean ( reason ): spaced\n//! @f-star(reason): x\n")
        self.assertEqual(header.reasons, {"lean": "spaced", "f-star": "x"})

    def test_header_reason_case(self):
        header = report.parse_header("//! @Lean(reason): why\n")
        self.assertEqual(header.reasons, {"lean": "why"})

    def test_escape_keeps_code_spans(self):
        self.assertEqual(report.escape("a | `|x| x` | ``b | c``"), "a \\| `|x| x` \\| ``b | c``")

    def test_claims_of_tests_without_results(self):
        ids = report.merge_id_lists(report.parse_id_list(IDS), report.parse_id_list(HAX_IDS))
        pending, exclusions = report.parse_exclusions(EXCLUDED)
        headers = {"values/statics/untiered": report.parse_header("//! @fls: fls_v4\n")}
        r = report.Report(ids, {}, pending, exclusions, headers=headers)
        self.assertEqual(r.claimed, {"fls_v4": {"values/statics/untiered"}})

    def test_results(self):
        backend = report.parse_results(RESULTS, header)
        self.assertEqual(backend.name, "lean")
        self.assertEqual(list(backend.tools), ["hax", "lean"])
        gap = backend.tests["values/statics/gap"]
        self.assertEqual(gap.classification, "planned")
        self.assertEqual(gap.chapter, "values")
        self.assertIsNone(backend.tests["values/statics/supported"].reason)

    def test_exclusions(self):
        pending, tables = report.parse_exclusions(EXCLUDED)
        self.assertEqual(pending, ["general", "hax"])
        self.assertEqual(len(tables), 1)
        self.assertEqual(tables[0].overrides, {"fls_v5": "a special case"})


class BackendPageTest(unittest.TestCase):
    def setUp(self):
        self.page = report.render_backend(synthetic(), "lean")

    def chapters(self) -> str:
        """The per-chapter tables, without the sections that follow them."""
        return self.page.split("## Constructs by chapter\n")[1].split(
            "## Planned\n"
        )[0]

    def test_front_matter_and_title(self):
        self.assertTrue(self.page.startswith("---\nweight: 50\n---\n"))
        self.assertIn("\n# Language support\n", self.page)

    def test_links_the_coverage_page(self):
        self.assertIn("[FLS coverage](../fls-coverage.md)", self.page)

    def test_summary_counts_and_shares(self):
        for name in report.CLASSIFICATIONS:
            self.assertIn(f"| {name} | 1 | 25% |", self.page)
        self.assertIn("| **all tests** | **4** | **100%** |", self.page)
        self.assertIn("| `lean` | `leanprover/lean4:v4.31.0` |", self.page)

    def test_chapter_sections_and_links(self):
        self.assertIn("### Values\n", self.page)
        self.assertIn("No test yet in: General, hax.", self.page)
        self.assertIn(
            "[Plain static](https://github.com/pereq-dev/hax/blob/main/tests/fls/src/values/statics/supported.rs)",
            self.page,
        )
        self.assertIn(
            "[`fls_v1`](https://rust-lang.github.io/fls/values.html#fls_v1)", self.page
        )
        self.assertIn("Static with \\| a pipe", self.page)

    def test_column_order(self):
        self.assertIn("| Test | Paragraphs | Status |", self.page)
        planned = self.page.split("## Planned\n")[1]
        self.assertIn("| Test | Stage | Reason |", planned)

    def test_status_is_coloured_and_carries_the_reason(self):
        chapters = self.chapters()
        self.assertIn(
            '<span class="fls-status fls-planned fls-reason" '
            'title="the loops section of the input-language reference">'
            "planned</span>",
            chapters,
        )
        # A test that reached its tier has no reason and no tooltip.
        self.assertIn(
            '<span class="fls-status fls-supported">supported</span>', chapters
        )
        self.assertIn('<span class="fls-status fls-unexpected">', chapters)

    def test_paragraph_cell_keeps_two_and_links_the_rest(self):
        data = synthetic()
        entry = report.TestEntry(
            path="values/statics/many",
            stage="checked",
            tier="supported",
            classification="supported",
            title="Many paragraphs",
            fls=["fls_v1", "fls_v2", "fls_v3", "fls_v4"],
            reason=None,
        )
        cell = report.paragraph_cell(data, entry)
        self.assertIn("#fls_v1", cell)
        self.assertIn("#fls_v2", cell)
        self.assertNotIn("#fls_v3", cell)
        self.assertTrue(
            cell.endswith("[+2](https://rust-lang.github.io/fls/values.html)"), cell
        )

    def test_status_drops_the_reason_of_a_supported_test(self):
        entry = report.TestEntry(
            path="values/statics/explained",
            stage="checked",
            tier="supported",
            classification="supported",
            title="Explained",
            fls=["fls_v1"],
            reason="not a gap",
        )
        self.assertEqual(
            report.status(entry),
            '<span class="fls-status fls-supported">supported</span>',
        )

    def test_chapter_counts(self):
        self.assertIn(
            "### Values\n\n4 tests, 2 as intended, 1 planned, "
            "1 unexpectedly accepted.\n",
            self.page,
        )

    def test_gap_and_unexpected_sections(self):
        planned = self.page.split("## Planned\n")[1].split("## Unexpectedly accepted\n")
        self.assertIn("the loops section", planned[0])
        self.assertNotIn("Plain static", planned[0])
        self.assertIn("Should have been rejected", planned[1])

    def test_no_unexpected_section_without_such_tests(self):
        data = synthetic()
        del data.backends["lean"].tests["values/statics/accepted"]
        page = report.render_backend(data, "lean")
        self.assertNotIn("Unexpectedly accepted", page)

    def test_revision_note_names_a_release(self):
        data = synthetic()
        data.ids.version = "1.98"
        page = report.render_backend(data, "lean")
        self.assertIn(
            "at release 1.98, commit "
            "[`0123456789`](https://github.com/rust-lang/fls/tree/0123456789abcdef/src)",
            page,
        )

    def test_revision_links(self):
        self.assertIn(
            "[`0123456789`](https://github.com/rust-lang/fls/tree/0123456789abcdef/src)",
            self.page,
        )


class CoveragePageTest(unittest.TestCase):
    def setUp(self):
        self.page = report.render_coverage(synthetic())

    def test_front_matter_and_title(self):
        self.assertTrue(self.page.startswith("---\nweight: 101\nheadless: true\n---\n"))
        self.assertIn("\n# FLS coverage\n", self.page)

    def test_chapter_rows(self):
        # values: 5 paragraphs, v1-v3 covered, v4 and v5 excluded as legality.
        self.assertIn("| [Values](#values) | 5 | 3 | 2 (legality 2) | 0 |  |", self.page)
        # general: pending, g1 informational, g2 pending.
        self.assertIn("| General | 2 | 0 | 0 | 1 | 1 |", self.page)
        self.assertIn("| hax | 1 | 0 | 0 | 0 | 1 |", self.page)
        self.assertIn("| **all chapters** | **8** | **3** | **2** | **1** | **2** |", self.page)

    def test_pending_and_exclusions(self):
        self.assertIn("- General\n- hax\n", self.page)
        self.assertIn("### Values { #values }", self.page)
        self.assertIn("**legality**, 2 paragraphs: rustc rejects these", self.page)
        self.assertIn(
            "- [`fls_v5`](https://rust-lang.github.io/fls/values.html#fls_v5): a special case\n",
            self.page,
        )
        self.assertIn("- [`fls_v4`](https://rust-lang.github.io/fls/values.html#fls_v4)\n", self.page)

    def test_no_pending_section_without_pending_chapters(self):
        data = synthetic()
        data.pending_chapters = []
        self.assertNotIn("## Pending chapters", report.render_coverage(data))

    def test_hax_ids_have_no_link(self):
        data = synthetic()
        self.assertEqual(report.paragraph_link(data, "hax_requires"), "`hax_requires`")


class CommittedFilesTest(unittest.TestCase):
    def test_load_and_render(self):
        self.assertTrue(report.inputs_present(REPO_ROOT))
        data = report.load(REPO_ROOT)
        self.assertIn("lean", data.backends)
        self.assertIn("hax", data.ids.chapters)
        self.assertEqual(data.ids.title("types-and-traits"), "Types and Traits")
        backend = report.render_backend(data, "lean")
        coverage = report.render_coverage(data)
        for test in data.backends["lean"].tests.values():
            self.assertIn(test.title.replace("|", "\\|"), backend)
            self.assertIn(test.classification, backend)
        for chapter in data.pending_chapters:
            self.assertIn(f"- {data.ids.title(chapter)}\n", coverage)

    def test_missing_inputs(self):
        self.assertFalse(report.inputs_present(REPO_ROOT / "docs"))


if __name__ == "__main__":
    unittest.main()
