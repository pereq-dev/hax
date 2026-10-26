---
authors:
  - maxime
title: "Hax offers preliminary support for anodized annotations"
date: 2026-09-30
---

## Rust specifications with `hax_lib`

When using the F* backend of hax, all specifications and contracts proven by hax have to be added to the Rust source. This partly comes from the constraints of our F* encoding, but after using them for multiple projects, we have started to like these source annotations. They document the developer's intent, and can be a good interface between developer (in the language they know) and proof engineer. 

We developed the `hax_lib` library that allows to write Rust expressions as preconditions and post-conditions on functions and trait methods. This is what is used for top-level contracts, but we also provide more advanced features to express invariants that help the proof effort, like loop invariants or refinement types. 

Here is an example of a simple Rust function which needs a precondition for panic-freedom, and with a post-condition to specify its behavior:

```rust
#[hax_lib::requires(x <= u8::MAX / 2)]
#[hax_lib::ensures(|result| result == 2 * x)]
fn double(x: u8) -> u8 {
    x + x
}
```
[Open this code snippet in the playground](https://hax-playground.cryspen.com/#fstar/f8fe69339b/gist=d580afeaac20c86521d210f963d8ccba)

## A more general Rust specification library

These Rust specifications are useful for documenting the intent of the code, and enforced when analyzing a crate with hax. But the information they contain could be useful in more situations like generating property-based tests, or adding runtime checks. It would also be great to have compatibility with other Rust verification tools. The ideal would be a support in the Rust language itself, and there is an [effort](https://github.com/rust-lang/rust/issues/128044) towards this, but it is still experimental (nightly only) and too restricted for our use needs.

During [SVIL 2026](https://cryspen.com/post/software-verification-in-lean-2026/), we met the developer of [anodized](https://github.com/anodized-rs/anodized) and the project aligns with our goals for a specification library that could be used for more than just hax. 

This is why we decided to offer preliminary support in hax 0.4! The prototype relies on a specific branch of anodized which adds a hax-specific cfg under which the anodized macros redirect to the hax_lib macros. 

An [example](https://github.com/cryspen/hax/tree/main/examples/anodized) is available in the hax repo, and here is an example of how this looks on the same example:

```rust
use anodized::spec;

#[spec(requires: x <= u8::MAX / 2)]
#[spec(ensures: *output == 2 * x)]
fn double(x: u8) -> u8 {
    x + x
}
```

## Next steps

For now, the support is limited to pre and post conditions on standalone functions. But anodized already has macros for specs on traits, loop invariants and datatype invariants. We plan to add support for all of these using anodized as a frontend for hax_lib.
