---
weight: 20
---

# ProVerif backend

The ProVerif backend extracts the protocol logic of a Rust crate into a symbolic (Dolev-Yao) model for [ProVerif](https://proverif.inria.fr/). Every Rust type becomes a ProVerif `bitstring`: the model keeps the term structure (constructors, function calls, control flow), and cryptography is modelled by ideal functionalities. The extracted functions become ProVerif `letfun`s, the processes and security queries that use them are written by hand.

## Extracting a crate

Run from the crate root:

```bash
cargo hax into proverif
```

The backend accepts three options (see `cargo hax into proverif --help`):

- `--keep-trait-calls <PATHS>` keeps calls of the given trait items as calls of the trait item rather than of the impl they dispatch to, so that a hand-written model covers every impl.
- `--result-errors` keeps `Ok` and `Err` as constructors, so error paths stay reachable. By default, a `Result` is its `Ok` payload and producing an `Err` ends the trace, which keeps the state space small but only fits code that never goes on after an `Err`: a match arm or an `else` branch that handles an `Err` without returning one, `is_err()`, `unwrap_or(..)` or a discarded `Result` is rejected, and needs `--result-errors`.
- `--assume-items <CLAUSES>` extracts only the signature of the selected items, as abstract functions.

For a project, store the configuration as a [proof scenario](../tools.md#proof-scenarios) in `hax.toml`, and run it with `cargo hax extract`:

```toml
[scenario.nsl]
backend = "proverif"
keep-trait-calls = ["proverif_nsl::PublicKeyEncryption::**"]
```

## Extracted files

The extraction writes to `proofs/proverif/extraction/` (`proofs/<scenario>/proverif/extraction/` for a scenario):

- `lib.pvl`: the model of the crate, one `letfun` (or `fun`, `reduc`, ...) per Rust item. It contains no `process`. Its header lists the libraries it must be loaded with.
- `missingdecl.pvl`: an abstract declaration of every symbol `lib.pvl` uses that no library defines, such as items of dependencies or large integer literals. Each one is an ideal-functionality boundary: keep it, or replace its declaration by a definition.

An item the backend cannot express is reported in an `INCOMPLETE MODEL` banner at the top of `lib.pvl`, together with a query that fails on every ProVerif run, so that an incomplete model never verifies.

## Libraries

hax ships ProVerif libraries in [`hax-lib/proof-libs/proverif`](https://github.com/pereq-dev/hax/tree/main/hax-lib/proof-libs/proverif):

- `primitives.pvl`: the public channel `c`, booleans, `Option`, small integer literals, and the models of the Rust primitives and `core` items the extraction uses.
- `result.pvl` or `result_errors.pvl`: the model of `Result`, matching extraction without or with `--result-errors`. Loading the wrong one is a type error.
- `cryptolib.pvl`: a Dolev-Yao model of the standard primitives (hash, AEAD, KDF, Diffie-Hellman, KEM, signatures, MAC, injective serialization), as `crypto__*` functions.

A model is checked by loading the libraries, then `missingdecl.pvl` and `lib.pvl`, then the analysis file:

```bash
proverif -lib <hax>/hax-lib/proof-libs/proverif/primitives \
         -lib <hax>/hax-lib/proof-libs/proverif/result \
         -lib <hax>/hax-lib/proof-libs/proverif/cryptolib \
         -lib proofs/proverif/extraction/missingdecl \
         -lib proofs/proverif/extraction/lib \
         proofs/proverif/analysis.pv
```

## Writing the model

Two things are written by hand: abstract models for the functions whose Rust code should not be extracted, and the analysis file.

### Abstract models

Cryptographic code, parsing and serialization are not meaningful as symbolic terms, so they get an abstract model through `hax_lib` attributes:

- `#[hax_lib::proverif::replace_body("...")]` replaces the body of a function by a ProVerif expression, typically a call to `cryptolib.pvl`, such as `crypto__aead_enc(key, message, empty)`. For a function returning a `Result`, the expression returns `hax_result_ok(..)` or `hax_result_err(..)`, as the `*_result` functions of `cryptolib.pvl` do, so that it fits both Result models. Its error is a term of the model, not the Rust error value, so with `--result-errors` a match on a specific error variant of it is rejected.
- `#[hax_lib::pv_constructor]` makes a function a ProVerif constructor, for an injective encoding, and `#[hax_lib::pv_inverse_of(f)]` makes a function its inverse, for the matching decoding. An inverse returning a `Result` returns `Ok`, and an `Err` off the image of `f`. With `--result-errors`, a `pv_constructor` or opaque function returning a `Result` is rejected, as its value would be neither `Ok` nor `Err`.
- `#[hax_lib::pv_extern]` makes a function an abstract one-way function of its arguments.
- `#[hax_lib::opaque]` drops the code of an item. With `--keep-trait-calls`, the trait items are then defined once in a hand-written `.pvl` file, loaded before `lib.pvl`.
- `#[hax_lib::proverif::before("...")]`, `after` and `replace` insert verbatim ProVerif declarations around, or instead of, an item.

### Processes and queries

The analysis file (`analysis.pv` in the examples) declares the events, the secrets and the queries, defines a process per role that calls the extracted functions and communicates on `c`, and composes them in the main `process`. ProVerif exits with success whatever the verdicts, so the examples compare the `RESULT` lines to the expected verdicts in their `Makefile`.

## Examples

Two examples show a full project setup: `hax.toml` scenario, abstract models in the Rust code, committed extraction, analysis file, and a `Makefile` running the analysis with `make check`.

- [`examples/proverif-psk`](https://github.com/pereq-dev/hax/tree/main/examples/proverif-psk): a pre-shared-key protocol whose cryptography is modelled with `replace_body` and `cryptolib.pvl`.
- [`examples/proverif-nsl`](https://github.com/pereq-dev/hax/tree/main/examples/proverif-nsl): the Needham–Schroeder–Lowe protocol, generic over a public-key encryption trait modelled once in a hand-written library with `keep-trait-calls`, with authentication and secrecy queries.
