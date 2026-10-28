# FLS tests

One test file per language construct, each run through every backend as a whole crate by `test-driver --suite fls`. The files under `src/` are also compiled as modules of the `fls-tests` crate in this directory, where `cargo test` runs the Rust half of every `equiv` function: once in hax mode, with the `--cfg hax` of the repository's `.cargo/config.toml`, and once with `RUSTFLAGS=""`, where the `hax_lib` assertions are checked.

## Checklist for a new test

1. **Location.** `src/<chapter>/<section>/<name>.rs`, where `<chapter>` is the slug of the FLS source file (`expressions`) and `<section>` the slug of the innermost section heading above the paragraph (`call-expressions`). Both are columns of `tests/fls-ids.txt` (`tests/hax-ids.txt` for the `hax` chapter): the `chapter` column is the chapter directory and the `section` column the section directory. A paragraph directly under the chapter heading has the chapter slug as its section, so its test lives at `src/<chapter>/<chapter>/<name>.rs`. Every identifier a test claims must belong to its chapter directory, and at least one to its section directory.
2. **Header.** The first lines of the file are `//!` directives; only `//!` lines are read, not `#![doc = "..."]` or `/*! */` comments. The header ends at the first line that is neither empty nor a `//!` line.
   - `@fls: <id>, <id>, ...` lists the paragraph identifiers the test covers. Required.
   - `@title: <text>` is the row label of the report. Required.
   - `@<backend>(tier): supported | unsupported`, one line per backend the test is run on, at least one. A backend without a tier line does not run the test.
   - `@<backend>(reason): <text>` explains a stage below the tier, or a tier that is not `supported`. Required for every gap.
   - `@no-equiv: <reason>` declares that the test has no `equiv` function on purpose; see step 5.
   - `@cli`, `@off`, and `@fail` are not accepted.
3. **One construct per test.** A test exercises one construct with the smallest program that gives it an observable value. Use nothing beyond `core`, `alloc`, and `hax-lib`, and limit the supporting constructs to those the FLS chapters before it introduce, so that a stage change can be attributed to the construct under test. One section holds as many tests as it has constructs, typically one to four.
   - **Naming.** Snake case, naming the construct and the aspect exercised: `labeled_break_outer`, `tuple_struct`, `option_map`.
4. **Semantic oracle.** Mark every guard function with `#[hax_test::equiv]`. A guard is a `pub fn <name>() -> bool` free function, at the crate root or in an inline module whose name aeneas keeps in Lean (not `core` or `alloc`, which it renames), without parameters or generics, and with no `#[cfg]` attribute on it or on an enclosing inline module, since the driver checks every guard on the backend side. The macro adds a `#[test]` that asserts the function returns `true`; the driver asserts the same on the backend side, for Lean with a `#guard (<name>).reducesTo true` line. The macro's arguments `skip_lean` and `panics` are not accepted in FLS tests.
5. **Constructs without an observable value** declare `@no-equiv: <reason>` instead of a guard function: lifetimes, `#[repr]`, `PhantomData`, marker traits, and the other rows the input-language reference lists as erased by the value model. A test whose tier is `unsupported` for every backend needs neither.
6. **No crate-level attributes** such as `#![no_std]`, since the file is also compiled as a module of the `fls-tests` crate.
7. **No auxiliary files.** A test is a single file; a `mod name;` declaration is rejected.
8. **Coverage.** Every identifier of `tests/fls-ids.txt` and `tests/hax-ids.txt` is either claimed by a test or listed in `tests/fls-excluded.toml`, unless its chapter is in `pending-chapters` or it carries the `informational` flag; an informational paragraph is never excluded. The exclusion file has one table per chapter and category (`[expressions.legality]`) with a shared `reason`, an `ids` array, and an optional `overrides` table for identifiers whose reason differs; the categories are `lexical`, `legality`, `out-of-scope`, `unsafe`, `ffi`, `inline-assembly`, `concurrency`, and `macro-expanded`. An identifier that is both claimed and excluded is an error, since the exclusion is stale. A chapter is triaged in one pull request: it removes the chapter from `pending-chapters`, adds its tests, and adds its exclusion tables, which a pending chapter may not have.

## Running the suite

From the repository root, with `cargo-hax` and the backend toolchain on `PATH`:

```
cargo run -p test-driver -- tests --suite fls -b lean
```

The observed stage of every test is compared with `tests/fls-results/<backend>.toml`; any difference fails the run and prints the diff. `--matching <path>` restricts the run to the test at that path, such as `<chapter>/<section>/<name>`, or to the tests under it, such as `<chapter>/`; it may be given several times, and a test runs when it matches any of the values. A value that matches no test fails the run. `--no-verify` stops after extraction and skips the comparison.

The `cargo-hax` on `PATH` must be built from this checkout. The driver rejects a release of another version, but it accepts any build from a git checkout, since the commit such a build reports can be older than the build, and records the package version for it.

The Lean backend also needs `rustup` on `PATH`, since charon installs its Rust toolchain through it. Before the tests, the driver extracts a trivial crate and fails the run if that crate does not reach `extracted`, so a tool chain that cannot run, such as charon without `rustup`, is not recorded as a rejection of every test.

## Coverage and identifier lists

`fls coverage` reads the test headers, the identifier lists, the exclusion file, and the results files, and fails on any violation, including a results entry whose test is gone, has no tier for the backend, or has another tier, and a test without a reason for an entry not classified as supported; it runs no backend and needs neither `cargo-hax` nor a toolchain:

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

## Updating the results

- `--update-results` rewrites the results file of the backend from the observed stages. Run it without `--matching` when the tool versions changed, so that every test is observed with the current tools. The `hax` row is the version of the checkout, which the release bump rewrites.
- The results file records the stage and the tier of every test; the report reads the title, the identifiers, and the reason from the test file. Editing those needs no update of the results file, while a tier change needs a run of the test.
