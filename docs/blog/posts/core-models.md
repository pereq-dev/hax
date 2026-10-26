---
authors:
  - maxime
title: "Core-models: handling dependencies of Rust code with Rust"
date: 2026-09-30
---

Verification does not happens in a vacuum, but as part of a stack: verifying a
system amounts to explicitly stating what properties it assumes of its
*dependencies*, what guarantees it provides to its *users*, and in between those
two interfaces, checking that the system behaves accordingly.  For code,
dependencies can be external libraries, system calls, or the primitives of the
language itself. The set of assumptions about the dependencies is referred to as
a *model*.

Specifically for Rust, almost all crates depend on the functions and type
definitions coming from `core` and `alloc` libraries. Any Rust verification
pipeline, like hax, needs to model them in some way. While they are written in
Rust, those libraries are highly optimized and intertwined with the compiler,
making them a hassle to handle. A typical workaround is to exclude those libraries
from the translation, and provide directly in the backend (Lean, F\*, etc) a
model of what they are doing. Such model is crucially part of the *Trusted
Computing Base* (TCB).

Yet, writing backend specific models is time-consuming, error-prone, hard to
review, test and extend. This post present hax' solution to this problem: we
write the models of `core` and `alloc` directly in rust, providing an equivalent
but simpler implementation. Being in Rust, this model can be heavily tested
against the real `core` library, using a variety of rust testing tools (see
below), can be extracted to various backends, factoring out the modeling effort,
and can be reviewed at the rust level, improving the overall confidence.

This post gives more details and motivation for the setup.

## The issues with backend-specific models of core

For example, let us consider the following Rust code:

```rust
fn length(x: &[u8]) -> usize {
    x.len()
}
```

[Open this code snippet in the playground](https://hax-playground.cryspen.com/#fstar/f8fe69339b/gist=ff368134f461edc228c2212b49320742)

Extracts to the following F* code:

```ocaml
let length (x: t_Slice u8) : usize =
  Core_models.Slice.impl__len #u8 x
```

Here, the translation didn't do anything clever: the semantics of the rust
program only depends on the definitions of `t_Slice` and
`Core_models.Slice.impl__len` that are provided by the hax F* proof library.

Until about a year ago, we relied on standard library models written
specifically for each backend. This approach had many issues:
- The work has to be redone for each backend (Lean, F*, ProVerif, etc), increasing the TCB.
- Writing a semantically correct model for every possible corner case is hard,
  even just for bounded arithmetic (see [this hax
  bug](https://github.com/pereq-dev/hax/issues/1459) for instance).
- The models have to be updated every time there is a change in the
  translation. Keeping up with upstream updates of core is challenging.
- We extended the models *on-demand*, when we stumbled upon missing entries in
  real proof campaigns, or when hax users reported missing definitions. The
  models were always somewhat incomplete.

One natural idea to address all these would be to directly extract the Rust
`core` and `alloc` crates using the same toolchain. However, `core` is an
extremely challenging rust crate: besides it sheer size, it uses nightly and
hard-to-support Rust features, and is highly optimized.

Overall, even if hax could extract `core`, the resulting backend library would
be hard to use in proofs and reason about. There is value in abstracting the
content of `core` and only reason about simpler model of the library.

## Presenting hax's *core-models*

The key idea of hax's approach to handling core is surprisingly simple: we provide
a model of `core` written in Rust. This model depends on a small, well
identified set of primitives (integers, sequences, strings, etc), called
*rust-primitives*. This has several benefits:

- **Factoring backend-models**. Extracting the models from rust effectively
  reduces the bakend-specific modeling problem from the whole of core to only
  the rust-primitives, factoring out the backend-agnostic part. This makes any
  improvement of coverage automatically multi-backend.
- **Rust level reviews**. While reviewing models of core requires expertise in
  both Rust and the given backend, writing them directly in Rust makes the
  review process much easier.
- **Heavy testing**. Detailed below.

We started using this about a year ago and this has helped us improve our coverage of Rust `core` with less maintenance burden. The models reproduce the behavior as simply as possible so they produce a proof library that is as good for proofs as the hand-written models were. We first replaced the hand-written F* library by it, and then started using it for the legacy Lean backend. More recently, we changed our Lean translation to use `aeneas` instead of the hax engine, and we also use an extraction of `core-models` as the basis for our proof library. This required adding a specific option to `aeneas` so that the calls to `core` it produces are compatible, but overall using `core-models` to get a proof library for a new backend is faster than writing it by hand.

## Testing that core-models behaves as `core`

The current architecture has three main trusted components:
- The Rust models
- The hand-written model of rust-primitives for each backend
- The translation process

We are trying to increase trust in those three parts, and having models in Rust gives us an advantage. Indeed, as the models are executable, it becomes much easier to test that the models behave like the functions from `core` that they specify. For this we use **property-based testing**: each model type implements a trait `Inject` that allows to take a value from the `core` type and convert it to its value in the corresponding `core-models` type. Then most tests are quite simple to write and look like: for any value `x`, `core_f(x).inject() == core_models_f(x.inject())`. We also added CI checks for test coverage with a requirement to cover all models with a property test. We use **`llvm-cov`** for this and require 100% coverage of lines, functions and regions. We have a handful of justified exclusions but this has helped strengthening testing where needed. We also run **`cargo-mutants`** once a week to get more subtle indications of where testing is insufficient. And this has helped us to improve the testing of functions like `min`, `max` and more `Ord` operations where it is easy to forget to test that the model returns the right element in case of equality.

But this doesn't catch bugs in the primitives (or in the translation), so we added a second kind of testing: Rust equivalence tests. The idea is to check that a certain call to a core function still returns the same value after translation. For this we made sure that all the models are executable in Lean after translation. Then we wrote a series of test of the form `core_f(x0) == y0`, `core_f(x1) == y1`, etc. These tests are then translated to Lean and re-checked on the Lean side, executing through the translated core_models and hand-written Lean for rust primitives. These equivalence tests are in the Lean backend only for now, and we don't check coverage so we have room to improve.

This process is quite new so we still need to evaluate how effective it is to prevent soundness bugs. We perform regular AI audits to track inconsistencies, and we would like to encourage contributors to report bugs, or give feedback about how we can increase trust.

## Increasing the coverage of `core`

We like the extensibility and trust that this approach brings, however we still cover a limited (even if increased) subset of the Rust core library. We added some [generated measurements](https://github.com/cryspen/hax/blob/main/hax-lib/core-models/COVERAGE.md) of the coverage we provide (for the modules we model) to track the progress. As I write this blogpost, we cover 35 out of 55 modules of `core`, and in these modules we cover about 20% of the items (8% of 11 modules for `alloc`, with 4 more modules to add). We hope to make progress on these metrics in the coming months. With the testing in place, and the clear specification for the models, we believe that AI agents can be effective for this, which has the potential to speed up progress. The `core-models` are also a great place to welcome external contributions as they are quite self-contained, and `hax` users can help improve the support for the `core` functions they need for their projects. If you want to contribute, learn more details and check the guidelines on [github](https://github.com/cryspen/hax/blob/main/hax-lib/core-models/README.md).
