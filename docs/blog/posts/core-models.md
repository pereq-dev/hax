---
authors:
  - maxime
title: "Presenting Rust core models"
date: 2026-09-30
---

The obvious factor that limits Rust code that can be analyzed using hax is the set of features supported by the translation. But extracted code relies on the Rust standard library, which needs a model in our proof backends and this also limits what code can be analyzed with hax.

For example, the following Rust code: 
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
The translation didn't do anything clever here and in this case the semantics of the rust program mostly depends on the definitions of `t_Slice` and `Core_models.Slice.impl__len` that are provided by the hax F* proof library.


Until about a year ago, we relied on standard library models written by hand for each backend. This approach had many issues:
- The work has to be done for each backend (Lean, F* ProVerif)
- It is hard to check that the models have the right semantics which results in bugs and potentially invalid program proofs
- The models have to be updated every time there is a change in conventions in the translation
- Writing these models is time-consuming so we always added only what was needed for our projects and covered only a very small part of the Rust standard library

The obvious idea to fix all these issues would be to directly extract the Rust `core` and `alloc` crates, they are written in Rust so there is no reason not to try hax on them. We tried several times but never got anything useful out of it, for several reasons: 
- `core` is one of the most challenging crates one can imagine for a tool like hax: it is huge and it uses the most advanced and hard to support Rust features
- It is written for performance which makes its code complex while what we need is a clear specification of what it does (without the optimization details)

As `hax` supports more and more features of Rust, we may retry this in the future, at least for some parts of `core`. But with the current capabilities, we went for a third hybrid approach that I will present in this blog post: Rust core models

## Presenting hax's core-models

The basic idea of core-models is to have manual models for Rust `core`, but written in Rust. The first advantage is to remove the duplication between backends. We still rely on some backend-specific models for primitive operations like arithmetic and sequences (that are the basis for models of arrays, slices, vecs, etc.) which use the most adapted abstraction in each backend language. We call these rust-primitives and there is also a Rust version of them which is not extracted, but used in the models for `core` and `alloc`, which are actually extracted.

We started using this about a year ago and this has helped us improve our coverage of Rust `core` with less maintenance burden. The models reproduce the behavior as simply as possible so they produce a proof library that is as good for proofs as the hand-written models were. We first replaced the hand-written F* library by it, and then started using it for the legacy Lean backend. More recently, we changed our Lean translation to use `aeneas` instead of the hax engine, and we also use an extraction of `core-models` as the basis for our proof library. This required adding a specific option to `aeneas` so that the calls to `core` it produces are compatible, but overall using `core-models` to get a proof library for a new backend is faster than writing it by hand.

## Testing that core-models behaves as `core`

The current architecture has three main trusted components:
- The Rust models
- The hand-written primitives for each backend
- The translation process

We are trying to increase trust in those three parts, and having models in Rust gives us an advantage. Indeed, as the models are executable, it becomes much easier to test that the models behave like the functions from `core` that they specify. For this we use property-based testing: each model type implements a trait `Inject` that allows to take a value from the `core` type and convert it to its value in the corresponding `core-models` type. Then most tests are quite simple to write and look like: for any value `x`, `core_f(x).inject() == core_models_f(x.inject())`. We recently added some CI checks for test coverage with a requirement to cover all models with a property test. We use `llvm-cov` for this and require 100% coverage of lines, functions and regions. We have a handful of justified exclusions but this has helped strengthening testing where needed. We also run `cargo-mutants` once a week to get more subtle indications of where testing is insufficient. And this has helped us to improve the testing of functions like `min`, `max` and more `Ord` operations where it is easy to forget to test that the model returns the right element in case of equality.

But this doesn't catch bugs in the primitives (or in the translation), so we added a second kind of testing: Rust equivalence tests. The idea is to check that a certain call to a core function still returns the same value after translation. For this we made sure that all the models are executable in Lean after translation. Then we wrote a series of test of the form `core_f(x0) == y0`, `core_f(x1) == y1`, etc. These tests are then translated to Lean and re-checked on the Lean side, executing through the translated core_models and hand-written Lean for rust primitives. These equivalence tests are in the Lean backend only for now, and we don't check coverage so we have room to improve.

This process is quite new so we still need to evaluate how effective it is to prevent soundness bugs. We perform regular AI audits to track inconsistencies, and we would like to encourage contributors to report bugs, or give feedback about how we can increase trust.

## Increasing the coverage of `core`

We like the extensibility and trust that this approach brings, however we still cover a limited (even if increased) subset of the Rust core library. We added some [generated measurements](https://github.com/cryspen/hax/blob/main/hax-lib/core-models/COVERAGE.md) of the coverage we provide (for the modules we model) to track the progress. As I write this blogpost, we cover 35 out of 55 modules of `core`, and in these modules we cover about 20% of the items (8% of 11 modules for `alloc`, with 4 more modules to add). We hope to make progress on these metrics in the coming months. With the testing in place, and the clear specification for the models, we believe that AI agents can be effective for this, which has the potential to speed up progress. The `core-models` are also a great place to welcome external contributions as they are quite self-contained, and `hax` users can help improve the support for the `core` functions they need for their projects. If you want to contribute, learn more details and check the guidelines on [github](https://github.com/cryspen/hax/blob/main/hax-lib/core-models/README.md).
