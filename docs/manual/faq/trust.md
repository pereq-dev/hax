---
title: TCB & Assumptions
weight: -4
---

# Trusted Computing Base (TCB) and assumptions

*What guarantees does verification with hax actually provide ?*

Formally verifying code with hax (or any other tool) gives a high assurance that
the code is correct, higher than what can be achieved with auditing and testing
alone. But there are still moving parts that can fail. The goal of this page is
to give an overview of the *trusted computing base* underlying hax and the
mitigation strategies.

## Overview

Verifying a Rust crate with hax usually goes through the following steps:

1. The user writes a specification for the Rust code, either as `hax-lib`
   annotations (pre- and postconditions, invariants, ...) or as backend code.
2. hax translates the Rust code (and its specification) into a backend language
   (e.g., Lean or F\* ), turning the specification into *proof obligations* that
   must be discharged by the backend.
3. Dependencies of the code are *modeled*. This includes Rust' pervasive `core`,
   `std` and `alloc` crates, for which hax provides `core-models`, but also
   foreign crates, intrinsics or code excluded from translation.
4. The user writes proofs in the backend language for the obligations, and the
   backend prover checks them.

A successful verification establishes the following claim: *the parts of the
Rust code that were extracted satisfy their specification, assuming that
dependencies are correctly modeled and that every tool involved in the
translation and the proof checking is correct.*

What can go wrong falls into two groups, of very different likelihood:

- **The claim is not the intended one.** The specification is wrong, does not
  cover the intended code, or the proofs are not actually checked. They are
  discussed in [Common pitfalls](#common-pitfalls).
- **The claim is the intended one, but a trusted component is wrong.** A tool in
  the pipeline has a bug that makes a wrong claim pass. This is much less
  likely, but cannot be ruled out. Components in the critical path form the
  trusted computing base (TCB), and are discussed in [Trusted computing
  base](#trusted-computing-base). Part of it is shared with the wider Rust and
  proof assistant ecosystems, and part of it is specific to hax.

## Common pitfalls

### Incorrect specification

The most common issue with verified code is that the specification does not
state what it is intended to. No improvements to our tools can eradicate this
fundamental issue entirely. The specification is the source of truth and is
therefore unverifiable. To mitigate this danger, auditing and testing of the
specification is crucial. After extraction via hax, one can also prove
properties about the specification itself to gain more confidence about its
correctness.

### Verification coverage

This is probably the second-most common issue with verified code. It is rare
that 100% of a crate can be extracted and verified. Understanding exactly the
scope of what should be extracted is paramount to using hax on real-world
projects. The hax CLI has various options to exclude parts of the code from
extraction, and parts of the code can also be excluded via annotations. If used
incorrectly, this can result in excluding parts of the code that should have
been verified.

For now, hax only targets safe Rust, and can only be used to prove functional
correctness and security properties. It cannot guard against side channel
attacks, verify unsafe Rust code or model concurrency. Wrappers around Rust
code that are written in other languages are out of scope, even though they
could introduce bugs.

### Proofs that are not checked

Running the prover (e.g., `lake build`) may not always run all the proofs that
are supposed to be run. For example, a contract that is stated in Rust and that
is extracted correctly, may not have a corresponding Lean theorem. Or the Lean
theorem might be in a file that is not actually imported and as a result remains
unchecked when running `lake build`.

Most provers allow users to state axioms that the prover is supposed to assume
as fact as a basis for its proofs.  Depending on the prover, such axioms can be
called `sorry`s, `admit`s, or `assume`s.  If there are unintended axioms in the
code, the verification is compromised.

In Lean, the command `#print axioms` lists all axioms used to prove a theorem
and can be used in combination with `#guard_msgs` to ensure no unwanted axioms
are used.  On top of that, the tool
[comparator](https://github.com/leanprover/comparator) can check for unintended
axioms, too, and it can ensure that a given list of theorems is actually part of
the verification.

### Models of external dependencies

In some projects, we also model external Rust crates that are not themselves
verified. If we do, correctness of the verification also relies on the
correctness of these models.

## Trusted computing base

### Shared components

These components are under scrutiny by a community:

- **The Rust compiler.** hax verifies the Rust code, not the machine code that
  the Rust compiler produces. To run the verified code, we still need to compile
  it, so we trust the Rust compiler. hax also uses the Rust compiler internally
  to parse and type-check the code before translating it.
- **charon and aeneas (Lean backend).** For Lean, hax uses charon to extract the
  Rust code from the Rust compiler, and aeneas to translate it into Lean. Both
  are trusted.
- **The backend provers.** We rely on Lean and F\* to check proofs correctly.
  Even widely used tools like Lean can have soundness bugs, allowing users (and
  AI) to prove false statements.

### hax-specific components

These components are developed as part of hax.

- **The translation.** For F\*, the hax frontend and the hax engine translate the
  Rust code. For Lean, hax uses aeneas with some modifications, which are
  trusted as well. In both cases, we rely on the `hax-lib` macros to expand
  correctly and on hax to faithfully translate them and the specification as a
  whole.
- **Rust primitives & core/std/alloc crates.** Each backend comes with a library
  modeling the semantics of Rust and its core library. For Lean and F\*, we model
  Rust's `core` and `alloc` libraries directly in Rust (see [our core models
  library](https://github.com/pereq-dev/hax/tree/main/hax-lib/core-models)). This
  Rust library gets extracted via hax into Lean and F\*. Only some Rust
  primitives that can not be modeled in terms of others are implemented directly
  in the backend languages. With this approach, we need to trust the Rust model,
  its translation into the backend language, and the primitives written in the
  backend language. The advantage of the approach is that a Rust model of Rust
  is easier to audit and that sharing the code between backends leads to a
  higher chance of finding bugs.
