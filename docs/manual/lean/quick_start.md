---
weight: 1
---

# Quick start

## Setup the tools

 - [Install hax](../index.md#installation).  
   Check: Running `cargo hax --version` should print some version info.
 - [Install Lean](https://lean-lang.org/install/)
 - *(Optional)* Aeneas and Charon are downloaded automatically on first use; to pre-install them, run `cargo hax tools install` inside your crate.
 - Add `hax-lib` as a dependency to your crate.  
   `cargo add --git https://github.com/pereq-dev/hax hax-lib`  
   *(`hax-lib` is not mandatory, but this guide assumes it is present)*

## Partial extraction

*Note: the instructions below assume you are in the folder of the
specific crate (**not workspace!**) you want to extract.*

Run the command `cargo hax into lean` to extract every item of your
crate as Lean code in the subfolder `proofs/lean`.

**What is critical? What is worth verifying?**  
Probably, your Rust crate contains mixed kinds of code: some parts are
critical (e.g. the library functions at the core of your crate) while
some others are not (e.g. the binary driver that wraps the
library). In this case, you likely want to extract only partially your
crate, so that you can focus on the important parts.

**Using the `--start-from` flag.**  
If you want to extract a function
`your_crate::some_module::my_function`, you need to tell `hax` to
extract nothing but `my_function`:

```bash
cargo hax into lean --charon-args="--start-from your_crate::some_module::my_function"
```

This command will extract `my_function`, along with all its dependencies (other functions, type definitions, etc.) from your crate.

**Unsupported Rust code.**  
hax doesn't support all Rust constructs, e.g,
`unsafe` code or interior mutability. That is another reason
for extracting only a part of your crate.

## Proof scenarios

Once an extraction configuration stabilizes, store it as a proof scenario in a `hax.toml` next to your `Cargo.toml` instead of retyping the flags:

```toml
[scenario.my-function]
backend = "lean"
include = ["your_crate::some_module::my_function"]
```

`cargo hax extract my-function` (or bare `cargo hax extract`, for all scenarios) runs the pipeline with the flags compiled from the scenario, into `proofs/my-function/lean/`, with the Lean package named after the scenario (`MyFunction`). Each scenario gets its own directory and package, so several verification targets of one crate coexist. See [the scenario reference](../tools.md#proof-scenarios) for the keys, the item-selection patterns, and the workspace rules.

For CI, `cargo hax extract` followed by `git diff --exit-code` catches extraction output that was not re-committed, and `lake build` per scenario runs the verification.

## Start Lean verification
After extracting your Rust code to Lean, the result is a complete Lean package (in `proofs/lean`, or `proofs/<scenario>/lean` for a scenario run). You can type-check the extraction with `lake build` in that folder, or directly in the IDE using the LSP. Running `lake exe cache get` beforehand downloads prebuilt binaries for mathlib, saving you from compiling it from source. Contrarily to F\*, successfully building the code doesn't prove panic freedom by default.

The package contains:

- `<PkgName>/Extraction/`, and the `<PkgName>/Extraction.lean` module importing it: the extracted modules. Both are owned by hax and rewritten on every extraction, so edits there are lost.
- `<PkgName>/Verification/`: your handwritten proofs; hax never touches this folder.
- `<PkgName>/Assumptions/`: models of the external definitions the crate uses, if any. hax seeds each file there once, from the template Aeneas generates, and never modifies it afterwards.
- a `lakefile.toml` and `lean-toolchain` pinned to the versions matching the extraction, and a root module importing the extraction and the proofs. These are created only when missing, so it is safe to re-run after editing them.

The root module is yours after its creation; since the extraction is reached through the single import of `<PkgName>.Extraction`, it never needs an update when the extracted files change, and hax only warns when one of its imports is missing or stale, see [the root module check](../tools.md#the-lean-root-module-check). A commented-out import (`-- import ...`) silences those warnings for a module, and for `Verification/ProofObligations.lean` it also stops hax from recreating the stub; extraction files are regenerated and `Assumptions/` files re-seeded regardless.
