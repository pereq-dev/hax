---
weight: -5
---

# Introduction

hax is a tool for high assurance translations of a large subset of
Rust into formal languages such as [Lean](https://lean-lang.org/), [F\*](https://www.fstar-lang.org/) or [Rocq](https://rocq-prover.org/).

## Usage

hax is a cargo subcommand. 
The command `cargo hax` accepts the following subcommands:

--8<-- "README.md:subcommands"

Note:

* `BACKEND` can be `lean`, `legacy-lean`, `fstar`, `coq`, `pro-verif`, `ssprove` or `easycrypt`. See the [backend overview](https://github.com/pereq-dev/hax#backends) for the maturity of each backend. This manual covers the [Lean](lean/index.md) and [F\*](fstar/index.md) backends.
* The subcommands `cargo hax`, `cargo hax into` and `cargo hax into
   <BACKEND>` take options. For instance, you can `cargo hax into
   fstar --z3rlimit 100`. Use `--help` on those subcommands to list
   all options.

## Installation

--8<-- "README.md:installation"

