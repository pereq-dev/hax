# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

Changes to the hax-lib crate:
 - Add `implies!`, an implication whose right-hand side is evaluated only when
   its left-hand side holds

Lean backend and library:
 - Add models of `RangeInclusive` and `RangeToInclusive`
 - Add a model of `unsigned_abs` for signed integers
 - Add models of `contains` for `Range`, `RangeFrom`, `RangeTo` and
   `RangeToInclusive`, and of `Range::is_empty`

F* backend and library:
 - Add models of `RangeInclusive` and `RangeToInclusive`
 - Add a model of `unsigned_abs` for signed integers
 - Add models of `contains` for `Range`, `RangeFrom`, `RangeTo` and
   `RangeToInclusive`, and of `Range::is_empty`

### Fixed

Changes to the hax-lib crate:
 - The Rust semantics of `implies` is no longer reversed

Changes to the frontend:
 - Impls now list the bounds on the generic parameters of their trait as parent bounds (#1554)

Lean backend and library:
 - `v.iter().all(..)` and `v.iter().any(..)` on slices no longer fail with an
   unknown constant

F* backend and library:
 - `Iterator::{all, any, find, find_map, position}` now take `&mut self` as in
   std and return the advanced iterator with their result, matching extracted
   call sites
 - `Formatter::write_str` is now `Core_models.Fmt.impl_11__write_str`, the name
   extracted client code calls, instead of `impl_Formatter__write_str`

ProVerif backend and library:
 - Breaking: full re-implementation of the backend under the Rust engine. The library
   scope and its interface with extracted code is also refactored.
 - Reject the code that each Result model would get wrong: going on after an
   `Err` by default, inspecting the error of a hand-written model with
   `--result-errors`
 - `#[pv_inverse_of(f)]` on `g` declares its `reduc` as `f__inverse__g`, and
   accepts methods

### Changed

Changes to `cargo-hax`:
 - Breaking: `anodized` specifications need `anodized` 0.8.0 or later; hax no
   longer sets `anodized_discard_specs`

Lean backend and library:
 - Breaking: the Lean library and the extracted Lean files are Lean modules.
   A module can only import modules, so the hand-written models in
   `Assumptions/`, which the extraction imports, have to become modules too:
   start each file with `module`, turn its `import`s into `public import`s,
   and put `@[expose] public section` after them.
 - Breaking: the fields `start` and `«end»` of `ops.range.RangeInclusive` are
   now `start_` and `end_`, and `RangeInclusive.start` and `RangeInclusive.end`
   are functions
 - Breaking: bump Lean to v4.35.0 and update aeneas. Proofs may need updating:
   `if_pos`/`if_neg` no longer rewrite `if`s (use `ite_eq_left`/`ite_eq_right`),
   and hand-written models that extracted code instantiates must take `Type _`
   rather than `Type` parameters

F* backend and library:
 - Breaking: the fields `f_start` and `f_end` of `t_RangeInclusive` are now
   `f_start_` and `f_end_`

## [0.4.2] - 2026-10-01

### Fixed

Changes to the hax-lib crate:
 - Support `refine` and `order` behind `cfg_attr` in `#[hax_lib::attributes]` (#1496)
 - Reject overlapping `refine`s or `order`s on a field

### Added

Lean backend and library:
 - Add models of `RangeBounds`, `copy_within` on slices and `drain` on `Vec`

F* backend and library:
 - Add models of `RangeBounds`, `copy_within` on slices and `drain` on `Vec`,
   with preconditions for the ranges on which Rust panics

### Changed

Lean backend and library:
 - Add an `exhausted` field to `RangeInclusive`

F* backend and library:
 - Add an `f_exhausted` field to `RangeInclusive`
 - Breaking: `Core_models.Ops.Range.impl` to `impl_4` are now the `RangeBounds`
   instances, with the `Iterator` instances of `Range` moving to `impl_5` to
   `impl_16`


## [0.4.1] - 2026-09-23

### Fixed

Lean backend and library:
 - Fix the calls to generic functions when the generic argument is not present in the signature (#2274)
 - Fix the models of hax_lib.int.Int division, remainder and pow2

F* backend and library:
 - Fix the model of `Rust_primitives.Hax.Int.div`
 - Make signed division and remainder truncate towards zero as Rust does, a
   breaking change since `div`, `mod` and the signed `rem_euclid` now require
   the quotient to fit in the type (#1459)
 - Fix `fold_enumerated_slice` and `fold_range_step_by` by defining instead of assuming
   them, and drop the inconsistent `Rust_primitives.Iterators.foldi_range_step_by`
 - Correct `logand_mask_lemma`, `get_bit_cast_extend` and `lemma_get_bit_bounded'`,
   and exclude the sign bit from `num_bits`, all breaking changes
 - Require `false` on the models of `mem::{uninitialized, zeroed, transmute,
   transmute_copy}` and on the hand-written functions returning `t_Never`, which
   could otherwise inhabit any type
 - Correct precondition on index for `remove` on `Vec`
 - Remove unsound model of `drain` on `Vec`

F* backend and library:
 - Apply `rewrite_self` to trait annotations

## [0.4.0] - 2026-09-04

Changes to the Rust engine:
 - Fix diagnostic reporting

Changes to the OCaml engine:
 - Don't fail when an item carries a hax attribute pointing to an item that
   another backend `cfg`-ed out. Backend-specific item quotes such as
   `#[hax_lib::legacy_lean::before(..)]` used to abort extraction to any other
   backend with `Could not find item with UID ...` (#2026)
 - Drop rustc's internal `<cfg_trace>`/`<cfg_attr_trace>` marker attributes, which
   made the engine print Rust code that could not be parsed back (#2026)

Changes to the frontend:
 - Fix all observable issues in the new rust version of the THIR importer
 - Add access to representation information of other types than just enums

Changes to cargo-hax:
 - Manage aeneas and charon versions with `cargo hax tools`, pinned via a committed `hax.toml`,
   installed from pre-built binaries verified against a shipped manifest
 - `cargo hax into lean` generates a complete, buildable Lean package by default (#2142)
 - Publish a pre-built `cargo-hax` binary per supported platform with every release, so that
   `cargo binstall cargo-hax` installs it without building
 - Make `cargo install cargo-hax` build on any recent toolchain, so that the `lean` backend can be
   installed on its own and pinned per project with `cargo-run-bin`; the JSON schema exporter the
   OCaml engine's build consumes moved behind the new `legacy-engine` feature
 - Add proof scenarios: named extraction configurations committed as `[scenario.<name>]` tables in
   `hax.toml`, run with the new `cargo hax extract [NAME]...` subcommand into per-scenario
   `proofs/<scenario>/<backend>/` directories
 - Add support for specifications written with [`anodized`](https://github.com/mkovaxx/anodized):
   `#[spec(requires: ..)]`, `#[spec(ensures: ..)]` and `#[spec(maintains: ..)]` now extract as
   pre- and postconditions, on every backend
 - Validate driver reports on cargo's stderr: only haxmeta files of workspace crates under the
   run's target directory are accepted, and a lost or partial report set fails the run instead
   of silently producing an incomplete extraction
 - Exit with a failing status whenever an error was reported
 - Make traits from `core` with default methods visible to charon so that impls of these traits have the right defaulted implementations (#2172)

Changes to the hax-lib crate:
 - Add `hax_lib::ensures_ref`, an `ensures` whose closure takes the result by reference,
   for postconditions over a type that is not `Copy`
 - Remove dependency to proc_macro_error2 (unmaintained) (#2039)
 - Keep the generics of a struct in scope in the `refinement` functions generated by
   `#[hax_lib::attributes]`, fixing refinements on fields that mention a const generic (#899)
 - Allow specifications on the methods of an `impl` block whose signature mentions an associated
   type of `Self`. The cases that remain unsupported (trait declarations, and associated types the
   block does not define) now fail with an explicit error instead of invalid code (#2089)
 - Support quote annotations (`before`, `after`, `options`) on inherent `impl` blocks and on their
   items (#1698)
 - Support `requires`, and `ensures` written behind a `cfg_attr` in
   an `impl` block or a trait annotated with `#[hax_lib::attributes]`, keeping
   the `cfg_attr` predicate (#1496)
 - Restore the documentation of the proc-macros, which was missing on docs.rs and in builds without
   `--cfg hax` (#1759)
 - Fix the docs.rs build of `hax-lib`: its `docs.rs` metadata table was misspelled, and `--cfg hax`
   has to reach rustc too so that the `cfg(hax)` dependencies resolve (#2087)

Changes to core models:
 - Improve core models library (#2049, #2157, #2077, #2160, #2187, #2199)
 - Add core model testing infrastructure

Changes to the F* backend and library:
 - Fix some regressions in the F* proof lib (particularly the `vec_deque` models)
 - Specify the F* `overflowing_add`/`overflowing_sub` primitives, which were left uninterpreted.
   The `checked_*` integer models route through them, so they could not be used in a proof (#2127)
 - Specify `abs_i8/16/32/64/128/isize` (previously wholly uninterpreted) with the documented
   wrap-to-`MIN` result refinement, unblocking proofs about `i*::abs` and the AVX2
   `_mm256_abs_epi32` models (#2107)
 - Fix the overflow condition of the F* `mul_overflow` primitive, which compared against `maxint`
   twice. `overflowing_mul`/`checked_mul` reported overflow for every product except exactly `maxint`

Changes to the Lean backend and library:
 - Add Lean/Aeneas as a new backend (#2075, #2071, #2050, #2051). The previous Lean backend is
   now called `legacy-lean` in the CLI (#2064).
 - Lean library for the new Lean/Aeneas backend (#2080, #2146, #2130)
 - Legacy Lean: Hoist methods to allow (mutual) recursion between methods and associated items of
   the same impl (163)
 - Model `Iterator`'s provided methods, so `map`/`filter`/`fold`/`collect`/`rev` and the other
   adapters resolve when a downstream crate is extracted to Lean. Adds `DoubleEndedIterator`,
   `ExactSizeIterator` and the `Rev`, `FilterMap`, `TakeWhile`, `SkipWhile`, `MapWhile`, `Inspect`
   and `Fuse` adapters, makes `collect` into `Vec` and `Result` computable, and gives
   `zip`/`chain` std's `IntoIterator` bound so a collection can be passed directly (#2200)

Miscellaneous:
 - Update the required OCaml version to 5.4.1 (#2137)
 - New testing framework for the engine(s)
 - Four examples for the new Aeneas/Lean backend (#2058, #2059, #2070, #2061)
 - Fix missing tools in the Nix dev shells (#2131)
 - Check examples through `just check-examples` instead of a Nix flake check (#2131)
 - Document the supported platforms: Linux (`x86_64` and `aarch64`) and macOS (`aarch64`). Windows
   and macOS on Intel are not supported and are no longer built in CI

## 0.3.7

Changes to the Rust Engine:
 - Rename `GenericConstraint::Type` to `TypeClass` and `::Projection` to `Equality` (#1996)
 - Remove `BinOp` resugaring (#1950)
 - Apply resugarings to linked items (pre/post conditions) (#1961)
 - Add new import_thir implemented in Rust and using `FullDef`, activated with `--experimental-full-def` (#1967)

Changes to the engine:
 - Omit type aliases whose body has unresolvable trait bounds instead of crashing (#2014)
 - Report let-chains (`if let .. && let ..`) as a soft error instead of panicking (#2014)

Changes to the frontend:
 - Fix support for ellipsis: add wildcard for every field (based on type info
   rather than number of subpatterns) (#2001)
 - Fix panic on constants of type `&[&T]` (e.g. `&[&str]`) caused by a wrong type for the synthesized array length (#2014)

Changes to cargo-hax:

Changes to hax-lib:
 - Lean lib: use Rust core models (#1865)
 - Lean lib: specs for negation (#1891)
 - Lean lib: Add casting for all integer type pairs (#1837)
 - Lean lib: bump lean to v4.28.0-rc1 (#1900)
 - Lean lib: Extract more core models (#1919)
 - Lean lib: Separate symbolic and bit-blasting specs (#1933)
 - Lean lib: Communicate user-generated specs to mvcgen (#1937)
 - Lean lib: Rust primitives for prop (#1942)
 - Lean lib: For-loops for all unsigned integers (#1951)
 - Lean lib: Upgrade to Lean v4.29.0-rc1 (#1962)
 - Lean lib: Add support for Int128 and UInt128 while waiting for upstream in Lean (#1968)
 - Lean lib: Refactor `RustM` as `ExceptT Error Option` (#1994)
 - Lean lib: Add Repr instance for tuples (#2000)
 - Lean lib: Make the proof of `RustM.toBVRustM_bind` compatible with Lean 4.29.0 (#2005)
 - F* lib: Add bit-extensionality (`lemma_int_t_eq_via_bits`), concrete `Core_models.Num.impl_u64__rotate_left` delegating to a new `Rust_primitives.Integers.rotate_left_u`, `logand_commutative`, fixed `bit_or` SMTPat semantics, `bit_xor`/`get_bit_xor`/`get_bit_lognot` SMTPats, and `Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range` for libcrux SHA-3 equivalence proofs

Changes to the Lean backend:
 - Add `hax_zify` and `hax_construct_pure` tactics (#1888)
 - Add support for opaque `impl`s (#1887)
 - Fix support for associated constants in trait impls (#1906)
 - Gather definitions in namespaces, shortening names (#1901)
 - Add support for associated types with constraints and inheritance (#1909)
 - Fix bug with monadic wrapping of trait constants (#1929)
 - Add type annotation for cast_op (#1925)
 - Add attributes for pureEnsures/pureRequires (#1931)
 - Extract correct `PhantomData` structure (#1932)
 - Standardize generated Lean naming to lowercase namespaces (#1914)
 - Fix associated constants with default values (#1941)
 - New default proof for the Lean backend & proof method attribute (#1938)
 - Prettier proof_mode annotations (#1943)
 - Detect recursive functions and mark them partial_fixpoint (#1946)
 - Add more binops (#1963)
 - Add a resugaring for ellipsis patterns (#2002)

Miscellaneous:
 - Fix Nix development shell: add an `fstar` devShell providing F* and the
   required environment variables (#1972)

## 0.3.6

Changes to the Rust Engine:
 - Add a rejection phase for interleaving of expressions and statements not
   supported by the Lean do-notation syntax (#1739).
 - Add a phase to handle the monadic encoding: it explicitly introduces two new
   Hax primitives `pure` (to wrap values as monadic computations) and `lift` (to
   lift monadic computations into values) (#1746)
 - Add a mechanism to lookup pre- and post-conditions (#1805)
 - Add a proper Rust backend (#1898)

Changes to the frontend:
 - Update the pin of rustc (#1765)
 - Miscellaneous changes related to Charon (#1765)

Change to cargo-hax:

Changes to hax-lib:
 - Add Lean core models for options, results, default (#1747)
 - F* lib: improved while loops support, additions of some specific arithmetic operations and fixed `TryInto` for integer types (#1742)
 - Lean lib: use macros for int operations (#1795)
 - Lean lib: add new setup for `bv_decide` (#1828)
 - Lean lib: base specs on mathematical integers (#1829)
 - Lean lib: represent `usize` via a copy of `UInt64` (#1829)
 - Lean lib: Add support for while loops (#1857, #1863)
 - Core models: integers, arrays, iterators, full replacement of the F* proof-lib (#1898)

Changes to the Lean backend:
 - Support for constants with arbitrary computation (#1738)
 - Add support for base-expressions of structs (#1736)
 - Use the explicit monadic phase to insert `pure` and `←` only on demand, and
   not introduce extra `do` block (#1746)
 - Rename `Result` monad to `RustM` to avoid confusion with Rust `Result` type (#1768)
 - Add support for shift-left (#1785)
 - Add support for default methods of traits (#1777)
 - Add support for floats (#1784)
 - Add support for pattern matching on constant literals (#1789)
 - Add support for binding subpatterns in match constructs (#1790)
 - Add error when using patterns in function parameters (#1792)
 - Add grind annotations for various lemmas in the Lean library (#1802)
 - Add support for constant parameters to functions and traits (#1797)
 - Add support for associated types with equality constraints (#1806)
 - Make trait-level arguments explicit for all trait functions, adding them as
   extra parameters (#1803)
 - Add generation of specs from requires/ensures-annotations (#1815)
 - Add support for nonliteral array sizes (#1826)
 - Add `hax_lib::lean::proof` attribute (#1831)
 - Add support for `#[hax_lib::opaque]` (#1846)
 - Turn rejection phase into a transformation phase (#1840)
 - Fix string escaping (#1834)

Miscellaneous:
- Reserve extraction folder for auto-generated files in Lean examples (#1754)
- Add `lean_adc` example to the Lean examples section, demonstrating tactics introduced in PR(#1933)

## 0.3.5

Changes to the Rust Engine:
 - The module `names` now produces `ExplicitDefId`s instead of `DefId`s (#1648)
 - Add a resugaring `FunctionsToConstants` (#1559)
 - Drop the tuple nodes of the AST, add resugaring node for tuples (#1662)
 - Add support for enums and structs to the Lean backend (type definitions,
   expressions, pattern-matching) (#1623)
 - Update name rendering infrastructure in the Lean backend (#1623, #1624)
 - Printers now emit proper diagnostics (PR #1669)
 - Global identifiers are now interned (#1689)
 - Global identifiers are encapsulated properly, and provide easy destructuring as tuple identifiers (#1693)
 - Add support for `trait` and `impl` in the Lean backend (#1679): trait definitions, trait bounds
   on functions, impl definitions. The typeclass resolution in the generated code is left implicit
   (relies on Lean). Limited support for associated types. No support for default implementations.
 - Refactor of the printing infrastructure: lowers the boilerplate, get rid of most lifetimes annotation, add proper contextual span support (#1735)

Changes to the frontend:
- Add an explicit `Self: Trait` clause to trait methods and consts (#1559)
- Fix `ImplExpr::Builtin` that had some type errors (#1559)
- Improve the translation of `Drop` information (#1559)
- Add variance information to type parameters (#1559)
- Cleanup the `State` infrastructure a little bit (#1559)
- Add information about the metadata to use in unsize coercions (#1559)
- Resolve `dyn Trait` predicates (#1559)
- Many improvements to `FullDef` (#1559)
- Add infrastructure to get a monomorphized `FullDef`; this is used in charon to monomorphize a crate graph (#1559)
- Fix a regression affecting projection predicates (#1678)

Change to cargo-hax:
- Improve the caching of rustc when using `cargo hax` commands (#1719)
- Add hidden commands and flags to explicitly manipulate `haxmeta` files (#1722)

Changes to hax-lib:
- New behavior for `hax_lib::include`: it now forces inclusion when in contradiction with `-i` flag.
- hax-lib requires edition 2021 instead of 2024 (#1726)
- Improved `VecDeque` model in F* proof lib (#1728)
- Split the Lean library into several files, update to lean 4.23.0 (#1696)

Changes to the Lean backend:
- Improve support for functionalized loops (#1695)
- Improve error messages, having each error (coming from the Lean backend) point to a specific github issue (#1717).

Miscellaneous:
 - A lean tutorial has been added to the hax website (#1626)
 - Add end-to-end tests for the website (#1690)
 - Diagnostics reporting were improved (#1692)

## 0.3.4

The release of `0.3.3` got troubles because of the new Rust Engine crates.
This release is mostly empty.

## 0.3.3

Changes to the frontend:
 - A field `visibility` was added to HIR items (#1643)

Rust Engine:
 - A Lean backend was introduced (#1593, #1591, #1590, #1607)
 - The Rust engine was improved (#1624, #1603, #1600, #1585)
 - The F* backend has been improved (#1587, #1585)

## 0.3.2

Changes to the frontend:
 - Provide the `FnOnce` shim for closures (#1477)
 - Update pin of rustc (#1482)
 - Add `Ty::FnDef` (splitting `FnPtr` and `FnDef`) (#1487)
 - Regroup generic and trait arguments in a struct `ItemRef` (#1514)
 - Support trait aliases in `FullDef` (#1494)
 - Separate `{Add,Sub,Mul}Unchecked` and `{Add,Sub,Mul}` (#1513)
 - Our pin to rustc was updated (#1534)

Changes to the engine:
 - introduce an experimental Rust engine (#1501, #1502, #1504, #1505, #1518)

Changes the `hax-lib`:
 - Support hax octal and binary literals in the `int!` macro
 - F*: additions of integer function implementations (#1520)
 - F*: change the definition of the `Clone` tyepclass (#1552)


## 0.3.1 (2025-05-26)

Changes to `hax-lib`:
- Bug fix with PartialOrd in f* lib: [#1473](https://github.com/cryspen/hax/pull/1473)
- Move `proof-libs` into `hax-lib` to allow dependencies using crates.io

## 0.3.0 (2025-05-16)

Changes to `hax-lib`:
- Support for SMT patterns in lemmas: [#1428](https://github.com/cryspen/hax/pull/1428)
- While loop invariants and termination (`loop_decreases`): [#1375](https://github.com/cryspen/hax/pull/1375)
- Removal of deprecated dependencies: [#1385](https://github.com/cryspen/hax/pull/1385) and [#1394](https://github.com/cryspen/hax/pull/1394)
- Support for mathematical integers and logical propositions has been strengthened: [#1372](https://github.com/cryspen/hax/pull/1372), [#1352](https://github.com/cryspen/hax/pull/1352), [#1351](https://github.com/cryspen/hax/pull/1351)
- `hax_lib::BACKEND::replace_body`: [#1321](https://github.com/cryspen/hax/pull/1321)
- `hax_lib::decreases`: [#1342](https://github.com/cryspen/hax/pull/1342)

## 0.2.0 (2024-01-20)
 - Initial release
