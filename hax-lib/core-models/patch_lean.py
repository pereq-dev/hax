#!/usr/bin/env python3
"""
Post-process Aeneas-generated Lean files into the layout expected by our
Aeneas core replacement library.

The `patch` target of the Makefile installs these four Aeneas-generated files
into ../proof-libs/lean/CoreModels/Core (for `core`) and
../proof-libs/lean/CoreModels/Alloc (for `alloc`) before running this script:
    Funs.lean
    Types.lean
    FunsExternal_Template.lean
    TypesExternal_Template.lean

This script:
  * Rewrites imports & opens to match our package layout.
  * Comments out the type-level items that we forward-declare
    in `TypesPrologue.lean`.
  * Comments out a small number of generated function definitions that have
    known elaboration issues.
  * Fixes some other elaboration issues via search and replace.

Note that some items are being removed before this script even runs:
* some items are excluded via charon's `--exclude`, passed by our `Makefile`
* some items are excluded via `hax_lib::exclude` annotations in the Rust sources.
"""

from __future__ import annotations
import os
import re
import shutil
import sys
from pathlib import Path

LEAN_DIR = Path(__file__).parent.parent / "proof-libs" / "lean"
CORE_DIR = LEAN_DIR / "CoreModels" / "Core"
ALLOC_DIR = LEAN_DIR / "CoreModels" / "Alloc"

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

# Every rewrite below is a *textual* workaround for a specific shape of Aeneas
# output. When Aeneas changes, a pattern can silently stop matching and the
# workaround becomes a no-op. We therefore count what each pass matches and
# report the ones that never fired: either the output moved, or the workaround
# is obsolete and can be deleted. Warnings only — a stale pattern is usually
# caught by the Lean build, and failing here would block re-extraction.
# The counts are only meaningful on *fresh* Aeneas output: re-running the
# patcher over already-patched files reports most passes as unmatched.
_MATCHES: dict[str, int] = {}


def _record(label: str, n: int) -> None:
    _MATCHES[label] = _MATCHES.get(label, 0) + n


def sub(label: str, pattern: str, repl: str, text: str, flags: int = 0) -> str:
    """`re.sub` that records how many times it fired."""
    new, n = re.subn(pattern, repl, text, flags=flags)
    _record(label, n)
    return new


def replace(label: str, text: str, old: str, new: str) -> str:
    """`str.replace` that records how many times it fired."""
    _record(label, text.count(old))
    return text.replace(old, new)


def report_unmatched() -> None:
    for label in sorted(l for l, n in _MATCHES.items() if n == 0):
        print(
            f"patch_lean.py: warning: `{label}` matched nothing; the Aeneas "
            f"output may have changed, or this workaround is obsolete.",
            file=sys.stderr,
        )


def read(path: Path) -> str:
    return path.read_text()


def write(path: Path, contents: str) -> None:
    path.write_text(contents)


def rewrite_imports_and_opens(text: str) -> str:
    """Rewrite `imports` and `open` to match our package layout."""
    # import
    text = sub(
        "core/import CoreModels",
        r"^public import CoreModels$",
        "public import CoreModels.Command\npublic import CoreModels.Core.TypesPrologue",
        text, flags=re.MULTILINE,
    )
    text = sub(
        "core/import FunsExternal",
        r"^public import CoreModels.Core.FunsExternal$",
        "public import CoreModels.RustPrimitives.Funs",
        text, flags=re.MULTILINE,
    )
    text = sub(
        "core/import TypesExternal",
        r"^public import CoreModels.Core.TypesExternal$",
        "public import CoreModels.RustPrimitives.Types",
        text, flags=re.MULTILINE,
    )
    return text


def escape_keyword_binders(text: str) -> str:
    """Escape binder names that are Lean keywords, e.g. `(end : Std.U8)`.

    Aeneas registers a bare name that collides with a Lean keyword under its
    French-quoted spelling (`«end»`), and prints every *use* that way — but the
    binder that introduces it is printed raw, which does not parse. Rather than
    carry a copy of Aeneas's keyword list, we read the answer off the file: a
    name Aeneas spells `«x»` anywhere is a name it considers keyword-escaped,
    so any binder introducing that same `x` unescaped is the unescaped half of
    the same name and gets quoted to match.

    A no-op once Aeneas escapes binders too, so it can stay until then.
    """
    escaped = set(re.findall(r"«(\w+)»", text))
    if not escaped:
        return text
    alt = "|".join(re.escape(name) for name in sorted(escaped))
    # Binder positions only: `(x :`, `{x :`, `⦃x :`, and record fields `{ x :`.
    # `:=` is excluded so that record *literals* (`{ start := ... }`) and
    # definitions are left alone.
    return re.sub(
        rf"([({{⦃]\s*)({alt})(\s*:(?!=))",
        lambda m: f"{m.group(1)}«{m.group(2)}»{m.group(3)}",
        text,
    )

def fix_fail_panic(text: str) -> str:
    """In the definition of a Lean `item` called `panic`, the name `panic`
    does not resolve to `Error.panic` as it should."""
    return replace("fix_fail_panic", text, "  fail panic\n", "  fail Error.panic\n")

def rename_namespace(text: str) -> str:
    """
    The extracted Rust-crate is called `core-models`, extracted as `core_models` by Aeneas.
    We rename the namespace to `CoreModels.core`.
    """
    text = replace("rename/namespace core_models", text,
                   "namespace core_models", "namespace CoreModels.core")
    text = replace("rename/end core_models", text,
                   "end core_models", "end CoreModels.core")
    text = replace("rename/Core_models", text, "Core_models", "Core")
    return text

def rename_alloc_models(text: str) -> str:
    """Rewrite every occurrence of the staged crate name back to `alloc`.
    
    `core-models/alloc/` is a Rust crate named `alloc`. Aeneas's builtin path
    map already has entries for `alloc::vec::Vec`, `alloc::boxed::Box`, etc.,
    so extracting the crate as-is collides on those names and crashes the type
    analyser. The Makefile works around this by staging a copy under
    `.alloc-extract/` whose Cargo.toml is rewritten to `name = "alloc_models"`.
    Aeneas then writes its output into `lean/Aeneas/Alloc/` with everything in
    the `alloc_models` namespace; the helpers below put the namespace back to
    `alloc` (where downstream Aeneas-extracted code expects to find it).
    """
    text = replace("rename/namespace alloc_models", text,
                   "namespace alloc_models", "namespace CoreModels.alloc")
    text = replace("rename/end alloc_models", text,
                   "end alloc_models", "end CoreModels.alloc")
    text = replace("rename/alloc_models", text, "alloc_models", "alloc")
    # Aeneas also PascalCases the staged crate name when it synthesises
    # trait-impl identifiers (e.g. `Alloc_modelsAllocAllocator`), which the
    # lowercase replace above misses.
    text = replace("rename/Alloc_models", text, "Alloc_models", "Alloc")
    return text


def rename_boxed_box_insts(text: str) -> str:
    """Publish `Box`'s trait impls under the name client crates reference.

    Aeneas names a builtin self type by its last path segment, so a client
    calling `!=` on a `Box` emits `alloc.Box.Insts.…`, while this crate emits
    the impl at its module path, `alloc.boxed.Box.Insts.…`. Renaming the
    definitions covers every `Box` impl, present and future.
    """
    return replace("rename/boxed.Box.Insts", text,
                   "boxed.Box.Insts.", "Box.Insts.")


def add_vec_into_iter_allocator(text: str) -> str:
    """Give `Vec`'s `IntoIterator` instance std's allocator parameter.

    The Lean-lane `Vec` model has no allocator, but a client's
    `IntoIterator for Vec<T, A>` dictionary is applied to `A` too (aeneas only
    erases it for the impls in its own builtin table). The parameter is
    unused, as `A` is everywhere in the Lean `Vec`.
    """
    return replace("vec/into_iter allocator", text,
                   "def vec.Vec.Insts.CoreIterTraitsCollectIntoIteratorTIntoIter (T : Type _) :",
                   "def vec.Vec.Insts.CoreIterTraitsCollectIntoIteratorTIntoIter (T : Type _)\n"
                   "  (A : Type _) :")


def rewrite_alloc_imports(text: str) -> str:
    """Adjust the imports / opens emitted by Aeneas for the staged alloc
    crate. The `alloc_models` rename has already happened by the time this
    runs, so the import paths still look like `import CoreModels.Alloc.<X>`.
    """
    # Replace `import CoreModels` with the explicit pieces it needs.
    text = sub(
        "alloc/import CoreModels",
        r"^public import CoreModels$",
        "public import CoreModels.Core.TypesPrologue\npublic import CoreModels.Core.Types\n"
        "public import CoreModels.RustPrimitives.Types",
        text,
        flags=re.MULTILINE,
    )
    # Additional imports just for `Funs.lean`
    text = sub(
        "alloc/import Alloc.Types",
        r"^public import CoreModels.Alloc.Types$",
        "public import CoreModels.Alloc.Types\npublic import CoreModels.RustPrimitives.Funs\n"
        "public import CoreModels.Core.Funs",
        text,
        flags=re.MULTILINE,
    )
    # Drop imports of the alloc-side external files: their dependencies now live
    # in the parent's `lean/RustPrimitives/Funs.lean`, so the modules
    # `Aeneas.Alloc.{Funs,Types}External` don't exist anymore.
    text = sub(
        "alloc/drop External imports",
        r"^public import CoreModels\.Alloc\.(?:Funs|Types)External$",
        "-- (alloc-side externals live in parent CoreModels.RustPrimitives)",
        text, flags=re.MULTILINE,
    )
    return text

def fix_result_match(text: str) -> str:
    """ A match on `result.Result` cannot be parsed properly by Lean in
    `I128.Insts.Core_modelsIterStepStep.steps_between`.
    """
    return sub("fix_result_match", r"\| result\.Result\.",
               r"| core.result.Result.", text)

def rewrite_phantom_data(text: str) -> str:
    """Redefine `PhantomData`.

    Aeneas extracts `core_models::marker::PhantomData` as a reducible alias
    `def marker.PhantomData (T : Type) := T` in `Core/Types.lean`, and
    constructs phantom values with `()` at call sites (Charon models
    PhantomData as a ZST). So there is already a mismatch between type definition
    and call site code. Even worse, none of the two options works for us.
    Defining `PhantomData` as the identity would require us to produce values
    for the given type out of thin air. The `()` approach doesn't work either because
    when `PhantomData A` appears as the second component of `vec.Vec T A := Seq T × PhantomData A`,
    Lean unfolds it and loses the `A` during unification, which then breaks
    the `{A : Type}` implicit at call sites like `alloc.vec.Vec.clear v`.

    `TypesPrologue.lean` defines a replacement: `structure PhantomData (A : Type)
    where mk ::` — a *non-reducible* carrier that keeps `A` syntactically
    present. This pass rewires the Aeneas output to use it. It runs on both
    `Core/{Types,Funs}.lean` and `Alloc/{Types,Funs}.lean`:

      1. Rewrite the `()` constructor in phantom-field position to
         `core.Phantom.mk`. One textual shape is handled:
           - `, ())` — the common form, where the phantom is the second
             slot of a 2-tuple (`vec.Vec`, `VecDeque`, `Drain`, …).
         Destructured forms like `(seq, pd)` don't textually match `, ()`
         and are left alone.

      2. Comment out Aeneas's own `core_models::marker::PhantomData`
         definition block in `Core/Types.lean`.
    """
    text = sub("phantom/tuple ()", r",\s*\(\)\)",
               ", core.marker.PhantomData.mk)", text)
    return comment_out_blocks(
        text, ["core_models::marker::PhantomData"],
        trailer="replaced by rewrite_phantom_data in favor of the def in `TypesPrologue.lean`",
    )


def rename_iter_param(text: str) -> str:
    """Aeneas sometimes names a function parameter `iter`, which shadows the
    `iter` *namespace* (`core_models::iter`). Inside the body, a qualified
    reference like `iter.adapters.step_by...next_loop.body` then resolves to
    the local parameter instead of the namespace and fails to elaborate.
    See https://github.com/AeneasVerif/aeneas/issues/1098.

    For every top-level block whose signature binds a parameter named `iter`,
    rename that parameter (and its value-level uses) to `iter_`. We leave
    `iter.<...>` namespace paths, `::iter::` Rust paths (in doc headers), and
    `iter :=` struct-field names untouched.

    One subtlety: an adapter constructor such as `Skip::new` builds its value
    with Rust field-shorthand, which Aeneas extracts as `ok { iter, n }`. Here
    `iter` is simultaneously the struct field *name* and the (binder) value.
    Blindly renaming it to `{ iter_, n }` would reference a nonexistent field
    `iter_`, so we first expand the shorthand to `{ iter := iter_, n }` —
    keeping the field name `iter` while pointing it at the renamed binder.
    """
    # A standalone `iter` identifier (binder or value), i.e. NOT:
    #   - part of a `::iter::` Rust path or `.iter` projection (lookbehind),
    #   - the prefix of `iter1` or a namespace path `iter.foo` (lookahead),
    #   - the struct-field name in `iter := ...` (negative lookahead for `:=`).
    token = re.compile(r"(?<![\w.:])iter(?![\w.])(?!\s*:=)")
    # `iter` as the leading field of a record literal `{ iter, ... }` / `{ iter }`
    # (but not the already-explicit `{ iter := ... }`).
    shorthand = re.compile(r"(\{\s*)iter\b(?!\s*:=)")

    def fn(ident: str, block_lines: list[str]) -> str | None:
        block = "\n".join(block_lines)
        # Only act on blocks that actually bind an `iter` parameter.
        if not re.search(r"\(\s*iter\s*:(?!=)", block):
            return None
        # Expand field-shorthand before the generic rename so the field name
        # survives (the inserted `iter := iter_` is then left alone: `iter` is
        # guarded by the `:=` lookahead, `iter_` by the trailing-`_` lookahead).
        block = shorthand.sub(r"\1iter := iter_", block)
        _record("rename_iter_param", 1)
        return token.sub("iter_", block)

    _MATCHES.setdefault("rename_iter_param", 0)
    return transform_blocks(text, fn)


# Standalone `ok` tokens, i.e. NOT already part of a dotted path such as
# `result.Result.ok` or `Aeneas.Std.RustM.ok`.
_BARE_OK_RE = re.compile(r"(?<![\w.])ok\b")


def qualify_result_monad_impls(text: str) -> str:
    """Fully qualify the Aeneas error monad's `ok` inside any trait impl whose
    `Self` is `Result<_, _>` (e.g. the `Try` impl's `from_output` / `«branch»`,
    and the `FromIterator<Result<_, _>>` impl's `from_iter`).

    Those defs are emitted into the `result.Result.*` namespace. Inside that
    namespace the bare name `ok` resolves to *our* `result.Result.ok`
    projection rather than to Aeneas's `Aeneas.Std.RustM.ok`, so the generated
    bodies fail to elaborate.

    In every block of an impl `for ... result::Result`, rewrite each
    *standalone* `ok` -> `Aeneas.Std.RustM.ok` (dotted paths like
    `result.Result.Ok` are left untouched). The doc-comment header is
    preserved verbatim. The match keys on the un-renamed Rust path in the doc
    header (`core_models` survives `rename_namespace`, which only rewrites
    `namespace`/`end` lines and `Core_models`).
    """
    def fn(ident: str, block_lines: list[str]) -> str | None:
        # Match both trait impls whose `Self` is `Result` (`... for
        # core_models::result::Result<...>`) and *inherent* methods on
        # `Result` (`{core_models::result::Result<...>}::method`, e.g.
        # `unwrap_or` / `map_err`) — both land in the `result.Result.*`
        # namespace and hit the same bare-`ok` monad clash.
        if "for core_models::result::Result" not in ident \
                and "{core_models::result::Result<" not in ident:
            return None
        # Preserve the `/-- ... -/` doc comment; only rewrite the code below it.
        doc_end = 0
        while doc_end < len(block_lines) and \
                not block_lines[doc_end].rstrip().endswith("-/"):
            doc_end += 1
        doc_end += 1
        head = block_lines[:doc_end]
        body = "\n".join(block_lines[doc_end:])
        body = _BARE_OK_RE.sub("Aeneas.Std.RustM.ok", body)
        _record("qualify_result_monad_impls", 1)
        return "\n".join(head) + ("\n" + body if body else "")

    _MATCHES.setdefault("qualify_result_monad_impls", 0)
    return transform_blocks(text, fn)


def desugar_pure_num_const_binds(text: str) -> str:
    """The generated `Funs.lean` uses monadic bind syntax to fetch the integer
    constants:

        let i ← num.Isize.MIN
        let i ← num.U64.MAX

    because in the original Aeneas extraction they are `RustM <T>` (computed
    via `rust_primitives.arithmetic.*`). `comment_out_num_consts` below
    replaces them with PURE values, so the call sites must use `:=` instead of
    `←`. Rewrite all such bind occurrences.
    """
    int_alt = "(?:U8|U16|U32|U64|U128|Usize|I8|I16|I32|I64|I128|Isize)"
    pat = re.compile(
        rf"(let\s+\w+)\s+←\s+(num\.{int_alt}\.(?:MIN|MAX|BITS))\b"
    )
    new, n = pat.subn(r"\1 := \2", text)
    _record("desugar_pure_num_const_binds", n)
    return new


def comment_out_num_consts(text: str) -> str:
    """Aeneas extracts `core.num.<X>.{MIN,MAX,BITS}` as a mix of pure literals
    and monadic definitions, depending on whether the model spells the value as
    a literal or as a reference to a `rust_primitives` const (as the
    pointer-sized types must). A consumer's `<int>::{MIN,MAX,BITS}`, however, is
    always the PURE `Aeneas.Std.core.num.<X>.*`, so a monadic one here fails to
    unify with every extracted crate that mentions it.

    `FunsPrologue` therefore declares all of them as pure -- which also puts
    them ahead of the earlier code in `Funs.lean` that references them via
    `IScalar.cast`. The duplicates that follow in `Funs.lean` are commented out
    here.
    """
    types = ("u8", "u16", "u32", "u64", "u128", "usize",
             "i8", "i16", "i32", "i64", "i128", "isize")
    subs = [f"{{core_models::num::{t}}}::{c}"
            for t in types for c in ("MIN", "MAX", "BITS")]
    return comment_out_blocks(text, subs, trailer="provided by CoreModels.Core.FunsPrologue")

def drop_itermut_iterator_instance(text: str) -> str:
    """Drop the generated `Iterator` *instance* for `slice::iter::IterMut`.

    `IterMut::next` yields an `&'a mut T`, so hax gives it a write-back and the
    extracted function has type
    `Self -> Result ((Option T) x Self x (Self -> Option T -> Self))` -- exactly
    the shape Aeneas gives
    `core::slice::iter::{Iterator<IterMut<'a, @T>, &'a mut @T>}::next`. That does
    not fit `Iterator`'s `next` field, so the instance record aeneas emits next
    to it is ill-typed. Aeneas's own library has the same asymmetry: the `next`
    function exists, the `Iterator` instance does not.

    Only the record is dropped -- hence the exact ident match rather than
    `comment_out_blocks`, whose containment mode would take the `::next`
    function with it. That function is what a client referencing
    `core::slice::iter::{Iterator<IterMut<...>, &mut _>}::next` resolves to.
    """
    target = (
        "core_models::slice::iter::{impl "
        "core_models::iter::traits::iterator::Iterator<&'a mut T> for "
        "core_models::slice::iter::IterMut<'a, T>}"
    )
    trailer = (
        "IterMut::next has a write-back return, which no Iterator instance "
        "can hold (Aeneas has no such instance either)"
    )

    def fn(ident: str, block_lines: list[str]) -> str | None:
        if ident != target:
            return None
        _record("drop IterMut Iterator instance", 1)
        return "/-\n" + "\n".join(block_lines) + "\n-/  -- " + trailer

    _MATCHES.setdefault("drop IterMut Iterator instance", 0)
    return transform_blocks(text, fn)


def comment_out_types(text: str) -> str:
    """
    Some type declarations in Types.lean are commented out: most are provided
    by TypesPrologue.lean, while `array::Array` / `slice::Slice` are redundant
    aliases for the Aeneas builtins `Array T N` / `Slice T` (see below).
    """
    return comment_out_blocks(text, [
        "core_models::ops::function::FnOnce",
        "core_models::ops::function::FnMut",
        "core_models::ops::function::Fn",
        "core_models::cmp::Ordering",
        "core_models::option::Option",
        "core_models::result::Result",
        # `array.Array` is just an alias for Aeneas's builtin `Array T N`, so
        # the generated `def` is redundant and only shadows the builtin.
        #
        # `slice.Slice` is the dummy `struct Slice<T>([T])` we declare in Rust
        # only to hang the `Slice::*` impls off of; Aeneas translates it to a
        # `def slice.Slice`, but every actual slice *type* reference in the
        # generated Lean uses the bare `Slice T` (the opened Aeneas builtin) —
        # i.e. exactly what `[T]` translates to — and the `slice.Slice.*`
        # methods only need `slice.Slice` as a name prefix, not as a type. So
        # we drop the dummy def and let `Slice T` resolve to the builtin, just
        # like `array.Array`.
        "core_models::array::Array",
        "core_models::slice::Slice",
    ])

def add_funs_prologue_import(text: str) -> str:
    """Funs.lean needs CoreModels.Core.FunsPrologue."""
    return replace(
        "add_funs_prologue_import", text,
        "public import Aeneas\n",
        "public import Aeneas\npublic import CoreModels.Core.FunsPrologue\n",
    )

# Identifies the start of a top-level "block" (a doc comment, an attribute,
# or a bare def/structure/inductive/abbrev/instance line).
BLOCK_START_RE = re.compile(
    r"^(/--|@\[|def |structure |inductive |abbrev |instance |theorem |@\[reducible\])"
)
#  /-- [core_models::foo::Bar]:                (function definitions)
#  /-- [core_models::foo::Bar]                  (type / inductive definitions)
#
# The captured group must allow nested `[...]` because some name patterns
# include things like `<&'a ([T])>`. We use a non-greedy regex that ends at
# `]:` (function defs) or `]\n` / `]$` (type defs). Aeneas always closes the
# header `[...]` at the END of a line.
DOC_HEADER_RE = re.compile(r"^/--\s*\[(.*)\](?::.*)?\s*$")
# /-- Trait declaration: [core_models::clone::Clone]
# /-- Trait implementation: [core_models::...]
DOC_HEADER_TRAIT_RE = re.compile(
    r"^/--\s*Trait (?:declaration|implementation):\s*\[(.*)\](?::.*)?\s*$"
)


DEF_KEYWORDS = (
    "def ", "structure ", "inductive ", "abbrev ",
    "instance ", "theorem ", "noncomputable def ",
)


def _parse_doc_header(line: str) -> str | None:
    """Return the identifier inside a `/-- [path::to::name]` (or trait-decl /
    trait-impl) doc-comment header, or `None` if the line is not a header."""
    m = DOC_HEADER_RE.match(line)
    if m is None:
        m = DOC_HEADER_TRAIT_RE.match(line)
    return m.group(1) if m else None


def _find_block_end(lines: list[str], i: int, n: int) -> tuple[int, int]:
    """Given that `lines[i]` starts a `/--` doc-comment, locate the end of
    the logical block that follows.

    Returns `(end, j)` where:
      - `lines[i:end]` covers doc + `@[...]` attrs + one `DEF_KEYWORDS` line
        + body, with any trailing blank lines trimmed.
      - `j` is the cursor just past those trimmed blanks, so callers can
        re-emit `lines[end:j]` to preserve spacing.
    """
    # 1. Consume the doc-comment (until line ending with `-/`).
    j = i
    while j < n and not lines[j].rstrip().endswith("-/"):
        j += 1
    j += 1  # past the `-/`
    # 2. Consume any attributes (`@[...]`) and the def/structure/... keyword
    #    line — these belong to the same logical block.
    while j < n and lines[j].startswith("@["):
        j += 1
    if j < n and any(lines[j].startswith(k) for k in DEF_KEYWORDS):
        j += 1
    # 3. Consume the body: every line until the next top-level construct
    #    (`/--`, `@[`, a fresh def-keyword line, or a namespace `end ...`).
    while j < n:
        cur = lines[j]
        if cur.startswith("/--") or cur.startswith("@["):
            break
        if any(cur.startswith(k) for k in DEF_KEYWORDS):
            break
        if cur.startswith("end ") or cur.rstrip() == "end":
            break
        j += 1
    # Trim trailing blank lines from the block (they belong outside).
    end = j
    while end > i and lines[end - 1].strip() == "":
        end -= 1
    return end, j


def _ident_matches(ident: str, sub: str) -> bool:
    """Substring match modes used by `comment_out_blocks` and
    `relocate_blocks_to_end`:

      * `"foo::"`  — prefix match (entry ends with `::`)
      * exact equality
      * word-bounded suffix match (so `is_some` doesn't match `is_some_and`)
      * containment, when the entry contains `<` or `{` (used to drop
        everything matching a prefix anywhere in the path)
    """
    if sub.endswith("::"):
        return ident.startswith(sub)
    if ident == sub:
        return True
    if ident.endswith(sub):
        prev_char = ident[-len(sub) - 1]
        if not (prev_char.isalnum() or prev_char == "_"):
            return True
    if "<" in sub or "{" in sub:
        if sub in ident:
            return True
    return False


def transform_blocks(text: str, transform_fn) -> str:
    """Walk every top-level doc-headed block in `text`. For each block,
    `transform_fn(ident, block_lines)` is called with the bracketed identifier
    and the list of lines that make up the block. It must return either:

      * `None` to leave the block unchanged, or
      * a string to splice in place of the block. The string is split on
        `\\n` and emitted as line entries; trailing blank lines that followed
        the block in the original are preserved.

    Replacement strings should NOT include a trailing newline — line spacing
    is reconstructed by the final `"\\n".join(...)` plus the re-emitted
    trailing blanks.
    """
    lines = text.split("\n")
    out: list[str] = []
    i = 0
    n = len(lines)
    while i < n:
        line = lines[i]
        if line.startswith("/--"):
            ident = _parse_doc_header(line)
            if ident is not None:
                end, j = _find_block_end(lines, i, n)
                replacement = transform_fn(ident, lines[i:end])
                if replacement is not None:
                    out.extend(replacement.split("\n"))
                    out.extend(lines[end:j])
                    i = j
                    continue
        out.append(line)
        i += 1
    return "\n".join(out)


def comment_out_blocks(
    text: str,
    name_substrings: list[str],
    *,
    trailer: str | None = None,
) -> str:
    """Find each top-level block whose doc-comment header contains one of the
    given substrings, and wrap that block in `/- -/`. If `trailer` is given,
    a `  -- <trailer>` suffix is appended after the closing `-/`.

    A block starts at a `/-- ... -/` doc comment (the start line) and runs
    until the next blank line followed by another block start, or to a clearly
    new block start (`/--`, `@[`, `def`, etc.).
    """
    suffix = "-/" if trailer is None else f"-/  -- {trailer}"

    def fn(ident: str, block_lines: list[str]) -> str | None:
        hit = next((s for s in name_substrings if _ident_matches(ident, s)), None)
        if hit is None:
            return None
        _record(f"comment_out {hit}", 1)
        return "/-\n" + "\n".join(block_lines) + "\n" + suffix

    for s in name_substrings:
        _MATCHES.setdefault(f"comment_out {s}", 0)
    return transform_blocks(text, fn)


def relocate_blocks_to_end(
    text: str,
    name_substrings: list[str],
    *,
    end_marker: str,
) -> str:
    """Move every top-level doc-headed block whose identifier matches one of
    `name_substrings` to the end of the namespace — just before the line
    `end_marker` — preserving their relative order.

    Aeneas orders definitions from its *generic* call graph, which does not
    see the *monomorphised* dependency a `StepBy<Range<usize>>` iterator has
    on the concrete `Usize` `Step` instance. That instance is a computable
    `def` emitted late in `Funs.lean` (interleaved with the `num.*` defs it
    relies on), so the adapter lands *before* it and elaboration fails with
    `unknown identifier core.Usize.Insts.CoreIterRangeStep`. Hoisting the
    adapter past the instance fixes the order; nothing else in the file
    references the adapter's `Iterator` impl, so no new forward reference is
    introduced. The later (already-correct) users — e.g. the slice compare
    loops — are untouched because they sit after the instance already.
    """
    captured: list[str] = []

    def fn(ident: str, block_lines: list[str]) -> str | None:
        hit = next((s for s in name_substrings if _ident_matches(ident, s)), None)
        if hit is None:
            return None
        _record(f"relocate {hit}", 1)
        captured.append("\n".join(block_lines))
        return ""

    for s in name_substrings:
        _MATCHES.setdefault(f"relocate {s}", 0)
    body = transform_blocks(text, fn)
    if not captured:
        return body

    needle = "\n" + end_marker
    idx = body.rfind(needle)
    block_text = "\n\n".join(captured)
    if idx == -1:
        return body.rstrip() + "\n\n" + block_text + "\n"
    return body[:idx] + "\n\n" + block_text + "\n" + body[idx:]


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main() -> int:
    if not LEAN_DIR.exists():
        print(f"error: {LEAN_DIR} does not exist", file=sys.stderr)
        return 1
    CORE_DIR.mkdir(exist_ok=True)

    # Apply transforms in dependency order: Types -> FunsExternal -> Funs
    types_path = CORE_DIR / "Types.lean"
    funs_path = CORE_DIR / "Funs.lean"
    funs_ext_path = CORE_DIR / "FunsExternal_Template.lean"
    types_ext_path = CORE_DIR / "TypesExternal_Template.lean"

    for path in [types_path, types_ext_path, funs_ext_path, funs_path]:
        if not path.exists():
            continue
        text = read(path)
        text = rewrite_imports_and_opens(text)
        text = rename_namespace(text)
        text = rewrite_phantom_data(text)
        text = escape_keyword_binders(text)
        if path == funs_path:
            text = fix_fail_panic(text)
            text = add_funs_prologue_import(text)
            text = comment_out_num_consts(text)
            text = desugar_pure_num_const_binds(text)
            text = fix_result_match(text)
            text = rename_iter_param(text)
            text = qualify_result_monad_impls(text)
            text = drop_itermut_iterator_instance(text)
            # The `StepBy` iterator monomorphises onto the concrete `Usize`
            # `Step` instance, which Aeneas emits *later* in the file. Hoist
            # the adapter past it so the reference resolves.
            text = relocate_blocks_to_end(
                text,
                ["iter::adapters::step_by::{impl core_models::iter::traits::iterator::Iterator"],
                end_marker="end CoreModels.core",
            )
        if path == types_path:
            text = comment_out_types(text)
        write(path, text)
        print(f"patched {CORE_DIR}.")

    # ----- alloc/ patches ---------------------------------------------------
    patch_alloc()

    report_unmatched()
    print("done.")
    return 0


def patch_alloc() -> None:
    """Process the Aeneas-extracted alloc files (under `lean/Aeneas/Alloc/`).

    Aeneas writes them with the staged crate name `alloc_models`. We rename
    everything back to `alloc`, fix imports/opens. The alloc subdirectory
    holds only the two files Aeneas can populate by itself (`Funs.lean`
    and `Types.lean`); the external dependencies it would otherwise emit
    (under `*External_Template.lean`) live in `CoreModels/RustPrimitives`,
    so we delete the templates after extraction.
    """
    if not ALLOC_DIR.exists():
        return

    funs        = ALLOC_DIR / "Funs.lean"
    types       = ALLOC_DIR / "Types.lean"
    funs_ext_t  = ALLOC_DIR / "FunsExternal_Template.lean"
    types_ext_t = ALLOC_DIR / "TypesExternal_Template.lean"

    # 1. Delete the external templates — their contents live in
    #    `lean/Aeneas/{Funs,Types}External.lean` (parent directory).
    for path in [funs_ext_t, types_ext_t]:
        if path.exists():
            path.unlink()
            print(f"removed Aeneas/Alloc/{path.name} (dependencies live in parent RustPrimitives/Funs.lean)")

    # 2. Apply patches to the remaining files.
    for path in [types, funs]:
        if not path.exists():
            continue
        text = read(path)
        text = rename_alloc_models(text)
        text = rename_boxed_box_insts(text)
        text = rewrite_alloc_imports(text)
        text = fix_fail_panic(text)
        text = rewrite_phantom_data(text)
        text = escape_keyword_binders(text)
        if path == funs:
            text = rename_iter_param(text)
            text = add_vec_into_iter_allocator(text)
            # `drop_intoiterator_iterator_inst` removed: `IntoIterator` now carries
            # the `iteratorIteratorInst` super-instance field (the `IntoIter: Iterator`
            # bound is back, mirroring Aeneas.Std), so the alloc impls must KEEP it.
            # `fill_iterator_default_fields` removed: no `Iterator` provided method is a
            # trait field any more (all are standalone `@[rust_fun]` shims in the
            # epilogue), so the alloc `Iterator` instances are just `{ next := … }` and
            # need no cross-crate back-fill.
        write(path, text)
    print(f"patched {ALLOC_DIR}.")


if __name__ == "__main__":
    sys.exit(main())
