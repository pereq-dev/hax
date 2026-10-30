#!/usr/bin/env python3
"""Generate `source/proofs/lean/RustLeanTests/LeanTests.lean` from
`#[hax_test::equiv]` functions found anywhere under `source/src/`.

For each `#[hax_test::equiv] pub fn <name>() -> bool` we emit
`#guard rustLeanOk <fully-qualified-name>`, where the fully qualified
name is read out of the extracted `RustLeanTests/Extraction/Funs.lean`
rather than guessed from the source path: Aeneas's builtin name map rewrites some module
components (notably `core` → `core_models`, `alloc` → `alloc_models`),
so the Lean def's location is not always derivable from the Rust file
path. Reading the extracted def list directly keeps the gen script
agnostic to that mapping.

The Lean build then fails for any test whose Aeneas translation does
not evaluate to `RustM.ok true`. A `#[hax_test::equiv(panics)]` test
must instead evaluate to `.fail`, as its Rust half must panic.

Tests marked `#[hax_test::equiv(skip_lean = "why")]` get their guard
written to `SkippedTests.lean` instead. Nothing imports that file, so a
normal build ignores it; `--report-skipped` (driven by `make
check-skipped`) builds it on purpose and fails if a skipped test has
started passing, so a fixed blocker cannot go unnoticed.

Invoked from the Makefile after `make extract`. Idempotent.
"""

import pathlib
import re
import sys

HERE = pathlib.Path(__file__).resolve().parent
SRC_ROOT = HERE / "source" / "src"
LEAN_ROOT = HERE / "source" / "proofs" / "lean" / "RustLeanTests"
FUNS = LEAN_ROOT / "Extraction" / "Funs.lean"
OUT = LEAN_ROOT / "LeanTests.lean"
OUT_SKIPPED = LEAN_ROOT / "SkippedTests.lean"

# Crate namespace that Aeneas wraps every def in (must match `name` in
# `source/Cargo.toml`).
CRATE_NAMESPACE = "rust_lean_tests"

# Marker attribute, spelled `hax_test::equiv` (with optional
# `(skip_lean = "...")`), + `pub fn NAME() -> bool`. Body is
# ignored. The argument list steps over string literals
# rather than stopping at the first `)`, because a reason routinely contains
# parenthesised prose.
ATTR_FN_RE = re.compile(
    r"#\[\s*hax_test::equiv\s*(?:\((?P<args>(?:\"[^\"]*\"|[^()])*)\))?\s*\]"
    r"(?:\s*#\[[^\]]*\])*"
    r"\s*pub\s+fn\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*\(\s*\)"
    r"\s*->\s*bool",
    re.MULTILINE,
)

# The attribute imported by `use hax_test::equiv;`, which ATTR_FN_RE would
# miss and leave the test without a Lean guard.
SHORT_ATTR_RE = re.compile(r"#\[\s*equiv\s*[\]\(]")

SKIP_RE = re.compile(r'skip_lean\s*=\s*"(?P<reason>[^"]*)"')
PANICS_RE = re.compile(r'(?:^|,)\s*panics\s*(?:,|$)')

# Emitted into both generated files: `RustM` is not an inductive type, so the
# guards observe a result through `RustM.reducesTo` and `RustM.match`.
GUARD_DEFS = [
    "def rustLeanOk (x : Aeneas.Std.RustM Bool) : Bool := x.reducesTo true",
    "",
    "def rustLeanPanics (x : Aeneas.Std.RustM Bool) : Bool :=",
    "  match x.match with",
    "  | .vis (.fail _) _ => true",
    "  | _ => false",
    "",
]


def strip_comments(src: str) -> str:
    """Strip Rust comments so a commented-out test stops matching ATTR_FN_RE.
    Depth-tracked: Rust block comments nest, and disabled tests here have been
    wrapped more than once."""
    out: list[str] = []
    i, n, depth = 0, len(src), 0
    while i < n:
        if src.startswith("/*", i):
            depth += 1
            i += 2
        elif depth and src.startswith("*/", i):
            depth -= 1
            i += 2
        elif depth:
            if src[i] == "\n":
                out.append("\n")
            i += 1
        elif src.startswith("//", i):
            j = src.find("\n", i)
            i = n if j < 0 else j
        else:
            out.append(src[i])
            i += 1
    return "".join(out)


# Matches `def <dotted.path>.<name> ...` in the extracted Funs.lean.
# Captures the dotted path prefix (without the trailing name).
DEF_RE_TEMPLATE = r"^def\s+(?P<path>(?:[A-Za-z_][A-Za-z0-9_]*\.)*){name}\s"


def collect_source_tests() -> list[tuple[str, str | None, bool]]:
    """Return ordered `(name, skip_reason, panics)` for every
    `#[hax_test::equiv]` function in the source tree; `skip_reason` is None
    when the Lean half is expected to pass."""
    tests: list[tuple[str, str | None, bool]] = []
    seen: set[str] = set()
    for rs in sorted(SRC_ROOT.rglob("*.rs")):
        text = strip_comments(rs.read_text())
        if SHORT_ATTR_RE.search(text):
            sys.exit(
                f"gen_lean_tests.py: write `#[hax_test::equiv]` instead of `#[equiv]` (in {rs})"
            )
        for m in ATTR_FN_RE.finditer(text):
            name = m.group("name")
            if name in seen:
                sys.exit(f"gen_lean_tests.py: duplicate test name: {name} (in {rs})")
            seen.add(name)
            args = m.group("args") or ""
            skip = SKIP_RE.search(args)
            # Without the reason, whose prose may say "panics".
            panics = bool(PANICS_RE.search(SKIP_RE.sub("", args)))
            tests.append((name, skip.group("reason") if skip else None, panics))
    return tests


def qualify(name: str, funs_text: str) -> str | None:
    """Find `def <path>.<name>` in Funs.lean and return the fully qualified
    Lean name `<crate>.<path>.<name>`, or None if Aeneas emitted no def."""
    m = re.search(DEF_RE_TEMPLATE.format(name=re.escape(name)), funs_text, re.MULTILINE)
    return f"{CRATE_NAMESPACE}.{m.group('path')}{name}" if m else None


def render(guards: list[tuple[str, str | None, bool]], preamble: list[str]) -> str:
    """`guards` is `(qualified-name, reason, panics)`; the reason becomes the
    `-- skip_lean: …` line that `--report-skipped` reads back."""
    lines = [
        "-- AUTO-GENERATED by rust_lean_equiv_test/gen_lean_tests.py. DO NOT EDIT.",
        *preamble,
        "import RustLeanTests.Extraction.Funs",
        "",
        # Default cap is 100 errors per file; with hundreds of guards a
        # single broken extraction would otherwise hide all the others.
        "set_option maxErrors 10000",
        "",
        *GUARD_DEFS,
    ]
    for fq, reason, panics in guards:
        if reason is not None:
            lines.append(f"-- skip_lean: {reason}")
        lines.append(f"#guard rustLeanPanics {fq}" if panics else f"#guard rustLeanOk {fq}")
    lines.append("")
    return "\n".join(lines)


def generate() -> None:
    if not SRC_ROOT.exists():
        sys.exit(f"gen_lean_tests.py: source dir not found: {SRC_ROOT}")
    if not FUNS.exists():
        sys.exit(
            f"gen_lean_tests.py: extracted {FUNS} not found. "
            f"Run `make extract` first (the Makefile sequences this)."
        )
    funs_text = FUNS.read_text()
    active: list[tuple[str, None, bool]] = []
    skipped: list[tuple[str, str, bool]] = []
    missing: list[str] = []
    skipped_unextracted: list[str] = []
    for name, reason, panics in collect_source_tests():
        fq = qualify(name, funs_text)
        if reason is None:
            (active.append((fq, None, panics)) if fq else missing.append(name))
        elif fq:
            skipped.append((fq, reason, panics))
        else:
            skipped_unextracted.append(name)

    # All of them, not just the first: each round-trip costs minutes.
    if missing:
        listing = "\n".join(f"  - {n}" for n in missing)
        sys.exit(
            f"gen_lean_tests.py: {len(missing)} test(s) annotated "
            f"`#[hax_test::equiv]` have no matching `def` in {FUNS.name}:\n"
            f"{listing}\n"
            f"Either Aeneas didn't extract them, or their bodies rely on "
            f"something the extraction pipeline silently dropped."
        )

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(
        render(
            active,
            [
                "-- One `#guard` per `#[hax_test::equiv]` function in `source/src/**/*.rs`.",
                "-- Each guard fails the build if Aeneas's translation does not evaluate",
                "-- to `RustM.ok true` (to `.fail`, for a `panics` test).",
            ],
        )
    )
    OUT_SKIPPED.write_text(
        render(
            skipped,
            [
                "-- `skip_lean` tests: each is expected to still FAIL. Nothing imports",
                "-- this file; `make check-skipped` builds it and reports any that pass.",
            ],
        )
    )
    print(
        f"gen_lean_tests.py: wrote {OUT.relative_to(HERE)} ({len(active)} tests), "
        f"{OUT_SKIPPED.relative_to(HERE)} ({len(skipped)} skipped)"
    )
    for name in skipped_unextracted:
        print(f"gen_lean_tests.py: note: skipped test `{name}` was not extracted")


def report_skipped(log_path: pathlib.Path) -> None:
    """Fail if any guard in SkippedTests.lean is absent from Lean's error
    output — i.e. a skipped test now agrees and should be re-enabled."""
    if not OUT_SKIPPED.exists():
        sys.exit(f"gen_lean_tests.py: {OUT_SKIPPED} not found; run the generator first.")
    guards = re.findall(
        r"^-- skip_lean: (?P<reason>.*)\n#guard (?:rustLeanOk |rustLeanPanics )(?P<fq>\S+)",
        OUT_SKIPPED.read_text(),
        re.MULTILINE,
    )
    if not guards:
        print("gen_lean_tests.py: no skipped tests.")
        return
    log = log_path.read_text()
    # With the reason: that's what says whether to re-enable.
    passing = [(fq, reason) for reason, fq in guards if fq not in log]
    if passing:
        listing = "\n".join(f"  - {fq}\n      skipped for: {r}" for fq, r in passing)
        sys.exit(
            f"gen_lean_tests.py: {len(passing)} of {len(guards)} skipped test(s) now "
            f"agree with Rust:\n{listing}\n"
            f"Drop their `skip_lean` so the suite covers them again."
        )
    print(f"gen_lean_tests.py: all {len(guards)} skipped test(s) still fail, as expected.")


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--report-skipped":
        report_skipped(pathlib.Path(sys.argv[2]))
    else:
        generate()
