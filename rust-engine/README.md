# Hax Rust Engine

This crate implements an alternative engine for Rust: the main one is implemented in OCaml and is located in [`engine/`](https://github.com/pereq-dev/hax/tree/main/engine).
This Rust engine is designed so that it can re-use some bits of the OCaml engine.

The plan is to slowly deprecate the OCaml engine, rewrite most of its components and drop it.

## Usage
The Rust engine handles the following backends:

- `legacy-lean` (`cargo hax into legacy-lean`): phases and printing run in the Rust engine; by default, importing the frontend's output is still delegated to the OCaml engine.
- `fstar` (only with the experimental flag, which precedes the subcommand: `cargo hax --experimental-full-def into fstar`): the phases run in the Rust engine, printing is delegated to the OCaml engine. Without the flag, the OCaml engine handles this backend entirely.
