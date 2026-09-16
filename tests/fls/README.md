# FLS tests

One test file per language construct, each run through every backend as a whole crate by `test-driver --suite fls`. The files under `src/` are also compiled as modules of the `fls-tests` crate in this directory, where `cargo test` runs the Rust half of every `equiv` function: once in hax mode, with the `--cfg hax` of the repository's `.cargo/config.toml`, and once with `RUSTFLAGS=""`, where the `hax_lib` assertions are checked.

## Checklist for a new test

1. **Location.** `src/<chapter>/<section>/<name>.rs`, where `<chapter>` is the slug of the FLS source file (`expressions`) and `<section>` the slug of the section title (`call-expressions`). The slugs are listed in `tests/fls-ids.txt` once it exists.
2. **Header.** The first lines of the file are `//!` directives; only `//!` lines are read, not `#![doc = "..."]` or `/*! */` comments. The header ends at the first line that is neither empty nor a `//!` line.
   - `@fls: <id>, <id>, ...` lists the paragraph identifiers the test covers. Required.
   - `@title: <text>` is the row label of the report. Required.
   - `@<backend>(tier): supported | unsupported`, one line per backend the test is run on, at least one. A backend without a tier line does not run the test.
   - `@<backend>(reason): <text>` optionally explains a tier that is not `supported`, typically a link to an issue or to an Annex A entry of the input-language reference.
   - `@no-equiv: <reason>` declares that the test has no `equiv` function on purpose; see step 5.
   - `@cli`, `@off`, and `@fail` are not accepted.
3. **One construct per test.** A test exercises one construct with the smallest program that gives it an observable value. Use nothing beyond `core`, `alloc`, and `hax-lib`, and limit the supporting constructs to those the FLS chapters before it introduce, so that a stage change can be attributed to the construct under test. One section holds as many tests as it has constructs, typically one to four.
   - **Naming.** Snake case, naming the construct and the aspect exercised: `labeled_break_outer`, `tuple_struct`, `option_map`.
4. **Semantic oracle.** Mark every guard function with `#[hax_test::equiv]`. A guard is a `pub fn <name>() -> bool` free function, at the crate root or in an inline module whose name aeneas keeps in Lean (not `core` or `alloc`, which it renames), without parameters or generics, and with no `#[cfg]` attribute on it or on an enclosing inline module, since the driver checks every guard on the backend side. The macro adds a `#[test]` that asserts the function returns `true`; the driver asserts the same on the backend side, for Lean with a `#guard (<name>).reducesTo true` line. The macro's arguments `skip_lean` and `panics` are not accepted in FLS tests.
5. **Constructs without an observable value** declare `@no-equiv: <reason>` instead of a guard function: lifetimes, `#[repr]`, `PhantomData`, marker traits, and the other rows the input-language reference lists as erased by the value model. A test whose tier is `unsupported` for every backend needs neither.
6. **No crate-level attributes** such as `#![no_std]`, since the file is also compiled as a module of the `fls-tests` crate.
7. **No auxiliary files.** A test is a single file; a `mod name;` declaration is rejected.

## Running the suite

From the repository root, with `cargo-hax` and the backend toolchain on `PATH`:

```
cargo run -p test-driver -- tests --suite fls -b lean
```

The observed stage of every test is compared with `tests/fls-results/<backend>.toml`; any difference fails the run and prints the diff. `--matching <path>` restricts the run to the test at that path, such as `<chapter>/<section>/<name>`, or to the tests under it, such as `<chapter>/`; `--no-verify` stops after extraction and skips the comparison.

The `cargo-hax` on `PATH` must be built from this checkout. The driver rejects a release of another version, but it accepts any build from a git checkout, since the commit such a build reports can be older than the build, and records the package version for it.

The Lean backend also needs `rustup` on `PATH`, since charon installs its Rust toolchain through it. Before the tests, the driver extracts a trivial crate and fails the run if that crate does not reach `extracted`, so a tool chain that cannot run, such as charon without `rustup`, is not recorded as a rejection of every test.

## Updating the results

- `--update-results` rewrites the results file of the backend from the observed stages. Run it without `--matching` when the tool versions changed, so that every test is observed with the current tools. The `hax` row is the version of the checkout, which the release bump rewrites.
- The results file records the stage and the tier of every test; the report reads the title, the identifiers, and the reason from the test file. Editing those needs no update of the results file, while a tier change needs a run of the test.
