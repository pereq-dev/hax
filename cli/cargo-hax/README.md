# cargo-hax

hax is a tool for high assurance translations of a large subset of Rust into formal languages such as [Lean](https://lean-lang.org/), [F\*](https://www.fstar-lang.org/) or [Rocq](https://rocq-prover.org/).
This crate provides `cargo hax`, the command line interface of hax.

## Usage

`cargo hax into BACKEND` translates a Rust crate to the backend `BACKEND`, such as `lean` or `fstar`; further subcommands run proof scenarios (`extract`), export the typed AST of a crate (`json`), and manage the external tools hax depends on (`tools`). See [Usage](https://hax.cryspen.com/manual/#usage) in the manual for all subcommands and the [backend overview](https://github.com/pereq-dev/hax#backends) for the supported backends and their maturity.

Use `--help` on any subcommand for options (e.g. `cargo hax into fstar --z3rlimit 100`).

## Installation

hax runs on Linux and macOS; see [Installation](https://hax.cryspen.com/manual/#installation) in the manual for the supported platforms.

The Lean backend runs the [Charon](https://github.com/AeneasVerif/charon) + [Aeneas](https://github.com/AeneasVerif/aeneas) pipeline instead of the hax engine, so it needs no other hax component than the `cargo-hax` binary this crate builds:

```bash
cargo install --locked cargo-hax
```

To skip that build, use [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) to download the binary the release published: `cargo binstall cargo-hax`.

All other backends need the hax frontend driver and engine as well; see [Installation](https://hax.cryspen.com/manual/#installation) in the manual for the methods installing everything (from source, Nix, Docker). The target provers (Lean, F\*, ...) must be installed separately in any case; the quick start of the respective backend covers that.

## Learn more

 - [Manual](https://hax.cryspen.com/manual/)
    + Quick start: [Lean](https://hax.cryspen.com/manual/lean/quick_start/), [F\*](https://hax.cryspen.com/manual/fstar/quick_start/)
    + Tutorial: [Lean](https://hax.cryspen.com/manual/lean/tutorial/), [F\*](https://hax.cryspen.com/manual/fstar/tutorial/)
 - [Repository](https://github.com/pereq-dev/hax), including [examples](https://github.com/pereq-dev/hax/tree/main/examples) of what hax can do.

Questions? Join us on [Zulip](https://hacspec.zulipchat.com/) or open a [GitHub Discussion](https://github.com/pereq-dev/hax/discussions). For bugs, file an [Issue](https://github.com/pereq-dev/hax/issues).
