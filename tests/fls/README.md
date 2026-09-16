# FLS tests

One test file per language construct, each run through every backend as a whole crate by `test-driver --suite fls`. The files under `src/` are also compiled as modules of the `fls-tests` crate in this directory, where `cargo test` runs the Rust half of every `equiv` function: once in hax mode, with the `--cfg hax` of the repository's `.cargo/config.toml`, and once with `RUSTFLAGS=""`, where the `hax_lib` assertions are checked.

## Checklist for a new test

1. **Location.** `src/<chapter>/<section>/<name>.rs`, where `<chapter>` is the slug of the FLS source file (`expressions`) and `<section>` the slug of the innermost section heading above the paragraph (`call-expressions`). Both are columns of `tests/fls-ids.txt` (`tests/hax-ids.txt` for the `hax` chapter): the `chapter` column is the chapter directory and the `section` column the section directory. A paragraph directly under the chapter heading has the chapter slug as its section, so its test lives at `src/<chapter>/<chapter>/<name>.rs`. Every identifier a test claims must belong to its chapter directory, and at least one to its section directory.
2. **Header.** The first lines of the file are `//!` directives; only `//!` lines are read, not `#![doc = "..."]` or `/*! */` comments. The header ends at the first line that is neither empty nor a `//!` line.
   - `@fls: <id>, <id>, ...` lists the paragraph identifiers the test covers. Required.
   - `@title: <text>` is the row label of the report. Required.
   - `@<backend>(tier): supported | unsupported`, one line per backend the test is run on, at least one. A backend without a tier line does not run the test.
   - `@<backend>(reason): <text>` explains a stage below the tier, or a tier that is not `supported`. Required for every gap; see "Triaging a chapter". The results check warns when a reason accompanies a test whose classification is `supported`, since the report never shows it.
   - `@no-equiv: <reason>` declares that the test has no `equiv` function on purpose; see step 5.
   - `@cli`, `@off`, and `@fail` are not accepted.
3. **One construct per test.** A test exercises one construct with the smallest program that gives it an observable value. Use nothing beyond `core`, `alloc`, and `hax-lib`, and limit the supporting constructs to those the FLS chapters before it introduce, so that a stage change can be attributed to the construct under test. One section holds as many tests as it has constructs, typically one to four.
   - **Formatting.** CI checks every test file with rustfmt. Syntax that rustfmt rewrites, such as an empty statement, an implicit `extern` ABI, `pub(in super)`, or nested parentheses, keeps its form under `#[rustfmt::skip]` on the item.
   - **Naming.** Snake case, naming the construct and the aspect exercised: `labeled_break_outer`, `tuple_struct`, `option_map`.
4. **Semantic oracle.** Mark every guard function with `#[hax_test::equiv]`. A guard is a `pub fn <name>() -> bool` free function, at the crate root or in an inline module whose name aeneas keeps in Lean (not `core` or `alloc`, which it renames), without parameters or generics, and with no `#[cfg]` attribute on it or on an enclosing inline module, since the driver checks every guard on the backend side. The macro adds a `#[test]` that asserts the function returns `true`; the driver asserts the same on the backend side, for Lean with a `#guard (<name>).reducesTo true` line. The macro's arguments `skip_lean` and `panics` are not accepted in FLS tests.
5. **Constructs without an observable value** declare `@no-equiv: <reason>` instead of a guard function: lifetimes, `#[repr]`, `PhantomData`, marker traits, and the other rows the input-language reference lists as erased by the value model. A test whose tier is `unsupported` for every backend needs neither.
6. **No crate-level attributes** such as `#![no_std]`, since the file is also compiled as a module of the `fls-tests` crate.
7. **No auxiliary files.** A test is a single file; a `mod name;` declaration is rejected.
8. **Coverage.** Every identifier of `tests/fls-ids.txt` and `tests/hax-ids.txt` is either claimed by a test or listed in `tests/fls-excluded.toml`, unless its chapter is in `pending-chapters` or it carries the `informational` flag; an informational paragraph is never excluded. The exclusion file has one table per chapter and category (`[expressions.legality]`) with a shared `reason`, an `ids` array, and an optional `overrides` table for identifiers whose reason differs; the categories are `lexical`, `legality`, `out-of-scope`, `unsafe`, `ffi`, `inline-assembly`, `concurrency`, and `macro-expanded`. An identifier that is both claimed and excluded is an error, since the exclusion is stale. A pending chapter may not have exclusion tables; the closing pull request of a chapter removes it from `pending-chapters` and adds its tables.

## Triaging a chapter

A chapter is triaged paragraph by paragraph, in specification order, into test claims and exclusion entries. Chapters above 300 paragraphs (`expressions`, `types-and-traits`, `entities-and-resolution`, `inline-assembly`) may land their tests in several pull requests grouped by sections while the chapter stays pending; the closing pull request adds the exclusion tables and removes the chapter from `pending-chapters`. Smaller chapters are one pull request.

### Rules

- A paragraph is **claimed by a test** when it defines a construct, states what the construct means or evaluates to, or illustrates it: Dynamic Semantics, definitional Legality Rules, Syntax, and Examples paragraphs. A test claims every paragraph of its section that its code exercises, and the Examples paragraph of the section when there is one. Several tests may claim the same paragraph.
- A Legality Rules paragraph goes to **`legality`** when its violation is rejected by rustc before hax sees the program, since the suite holds compiling programs only: "shall" and "shall not" constraints, static errors, type inference, type unification, type coercion, subtyping and variance, lifetime elision, object safety, implementation coherence and conformance, generic conformance, name resolution, visibility, and the crate and module structure.
- Undefined Behavior paragraphs go to **`unsafe`**, since undefined behavior needs unsafe code. Paragraphs about layout, representation, `#[repr]`, alignment, ABI, and the panic and unwinding strategy go to **`out-of-scope`**: the value model erases them and they have no value semantics to observe. Implementation Permissions of `ffi` go to **`ffi`**.
- An **unsupported construct** (the value-model boundary of the input-language reference: `union`, `static mut`, raw pointers and the other `unsafe` operations, `Cell`, `async fn`, atomics) gets one test when a compiling crate can exhibit it, with tier `unsupported` and a reason citing that boundary, so that the report shows the rejection and the classification catches an unexpected acceptance. The paragraphs that detail the construct beyond what the test exhibits go to the matching category: `unsafe`, `ffi`, `inline-assembly`, `concurrency`, or `out-of-scope` for `async`, generators, and `catch_unwind`.
- **Macros.** `macro_rules!` declarations, invocations, and `#[derive]` get tests showing that the expansion extracts. Paragraphs about the macro machinery (matching, transcription, hygiene, procedural macros) go to `macro-expanded`.
- **Attributes.** One test per attribute the input-language reference lists as transparent or erased (`derive`, `cfg`, `cfg_attr`, `repr`, `inline`, `must_use`, `non_exhaustive`, `track_caller`, the lint attributes) showing that the annotated item extracts and behaves as unannotated, with `@no-equiv` where nothing is observable. Attributes about compilation, linkage, testing, documentation, and macros go to `legality`, `out-of-scope` (linkage, `#[test]`, `#[doc]`), `macro-expanded` (`#[proc_macro*]`), or `ffi` (`#[link*]`, `#[no_mangle]`), whichever names the reason.
- **Erased constructs** (lifetimes, `PhantomData`, marker traits, type aliases, visibility, `Box` indirection) get tests with `@no-equiv` and tier `supported`; a test may combine an erased construct with a guard that observes the value it decorates, in which case it needs no `@no-equiv`.
- **Tiers** follow the quick-reference table of the input-language reference. A use in scope is `supported`, including one whose observable behaviour goes through a `core` or `alloc` model (floating point, containers, `Rc`, iterator adapters) or through the model of an opaque item (`extern` functions).
- **Reasons.** A stage below the tier gets a `@<backend>(reason)` that stands on its own: which tool stops and on what (the aeneas or charon message, shortened) and the section of the input-language reference that places the construct in the input language, as in "aeneas rejects a `break` to an outer loop (`Breaks to outer loops are not supported yet`); the loops section of the input-language reference". The section is a pointer, never the whole reason, since the report lists the gap on its own. An `unexpectedly accepted` classification is resolved before the pull request lands, by fixing the tier or by a reason stating that the acceptance is under triage. No chapter lands with an unexplained gap.

### Procedure

1. `cargo run -p test-driver -- tests fls triage <chapter>` prints the paragraphs of the chapter that are neither claimed nor excluded nor informational, grouped by section, each with its rubric and its text from the FLS sources at the vendored revision (fetched on the first run). Work through the sections in order and decide per paragraph between a test claim and a category. `--all` prints the claimed and excluded paragraphs too, marked, for the review of an existing chapter.
2. Write the tests under `tests/fls/src/<chapter>/<section>/`; `cargo test --manifest-path tests/fls/Cargo.toml` and `RUSTFLAGS="" cargo test --manifest-path tests/fls/Cargo.toml` run the guards in Rust.
3. Add the tables to `tests/fls-excluded.toml`, remove the chapter from `pending-chapters` (closing pull request only), and run `fls coverage` until it reports no violation.
4. Run the backend on the chapter, `test-driver tests --suite fls -b lean --update-results -m <chapter>/`, and review the stages: every stage below the tier gets a reason, an unexpected acceptance is resolved as above.
5. Append the rustc coverage files whose constructs the tests now cover to `tests/fls/src/rustc-coverage-retired.txt`, one path per line relative to `tests/src/rustc-coverage/`, with `#` comments naming the FLS tests that replace a group; move files whose value lies in combining constructs to `tests/src/legacy`. `fls coverage` fails on a listed path that does not exist or is listed twice.
6. Revert `tests/Cargo.lock`, which every driver run rewrites. The pull request description lists the tests per section and the exclusion counts per category.

## Running the suite

From the repository root, with `cargo-hax` and the backend toolchain on `PATH`:

```
cargo run -p test-driver -- tests --suite fls -b lean
```

The observed stage of every test is compared with `tests/fls-results/<backend>.toml`; any difference fails the run and prints the diff. `--matching <path>` restricts the run to the test at that path, such as `<chapter>/<section>/<name>`, or to the tests under it, such as `<chapter>/`; it may be given several times, and a test runs when it matches any of the values. A value that matches no test fails the run. `--no-verify` stops after extraction and skips the comparison.

The `cargo-hax` on `PATH` must be built from this checkout. The driver rejects a release of another version, but it accepts any build from a git checkout, since the commit such a build reports can be older than the build, and records the package version for it.

The Lean backend also needs `rustup` on `PATH`, since charon installs its Rust toolchain through it. Before the tests, the driver extracts a trivial crate and fails the run if that crate does not reach `extracted`, so a tool chain that cannot run, such as charon without `rustup`, is not recorded as a rejection of every test.

## Coverage and identifier lists

`fls coverage` reads the test headers, the identifier lists, the exclusion file, the results files, and the rustc coverage retirement list, and fails on any violation, including a results entry whose test is gone, has no tier for the backend, or has another tier, and a test without a reason for an entry not classified as supported; it runs no backend and needs neither `cargo-hax` nor a toolchain:

```
cargo run -p test-driver -- tests fls coverage
```

`fls refresh` regenerates `tests/fls-ids.txt` from the FLS sources at a revision of `rust-lang/fls`; `--check` writes nothing; it prints the rows added and removed against the committed file and exits with code 3 when the regenerated file would differ apart from its revision, including a change of chapter or section order or of a chapter title:

```
cargo run -p test-driver -- tests fls refresh --rev <release>
cargo run -p test-driver -- tests fls refresh --rev <release> --check
```

The revision is a release tag of `rust-lang/fls`, one per Rust version, which the list records as `fls-version` beside the commit it names; a commit hash or a branch works too and records no version. The suite tracks the newest release rather than `main`, so that the identifiers move in reviewed steps, except where a chapter was reworked after that release: there the list is vendored from the commit that reworked it and names no version, and the next release subsumes it.

After a refresh, run `fls coverage`: paragraphs the new revision added show up as uncovered, and claims or exclusions of removed paragraphs fail.

## Continuous integration

`fls_tests.yml` runs on every pull request and in the merge queue:

- The `coverage and Rust side` job runs `fls coverage`, runs `cargo test` on the `fls-tests` crate in hax mode and without it for the Rust side of the `equiv` functions, and runs the unit tests of the report renderer under `docs/hooks/fls_report`. It needs no backend toolchain.
- The `select tests` job decides what the `lean backend` job runs, with `.utils/fls-changed-tests.sh <base>`: the tests whose file the pull request adds, modifies, or renames, or whose entry in a results file it adds or changes, passed as `--matching` values; everything when `cli/` (cargo-hax, the driver and its templates), `hax-lib` outside `core-models` and the proof libraries of the other backends, `hax-types`, the macros, the root manifest, the lockfile, the cargo configuration, the toolchain, the flake, the workflow, this selection script, or the header of a results file (its tool versions) changed; nothing when neither a test file nor a results entry changed, in which case the Lean job is skipped. A deleted test is caught by `fls coverage` without Lean.
- The `lean backend` job builds the driver with `cargo run` and installs `cargo hax` with `nix profile install .`, so it uses the `cargo hax` of the checkout. Every run pays the setup of the Lean dependencies; there is no cache.

The whole Lean suite runs weekly and on `workflow_dispatch`; the scheduled run skips the `coverage and Rust side` job, whose inputs the merge queue already checked. A failing weekly run keeps the issue labeled `fls-weekly` current and the next passing run closes it.

When the Lean job fails because the observed stages differ from `tests/fls-results/lean.toml`, the job summary shows the diff and the artifact `fls-results-lean` holds the observed file: download it, copy it over the committed file, and commit, or apply the diff by hand. A change of the tool versions needs a full run, which the selection triggers on its own once the results file header changed.

## Updating the results

- `--update-results` rewrites the results file of the backend from the observed stages. Run it without `--matching` when the tool versions changed, so that every test is observed with the current tools. The `hax` row is the version of the checkout, which the release bump rewrites.
- The results file records the stage and the tier of every test, and the classification derived from them; the report reads the title, the identifiers, and the reason from the test file. Editing those needs no update of the results file, while a tier change needs a run of the test.
