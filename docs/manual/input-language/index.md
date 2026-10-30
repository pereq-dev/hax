---
weight: -1
---

# Input language

<!--
Maintenance notes. This is a vision document: it describes the input language as intended, in the present tense; the gaps between it and the current implementation are the planned constructs of the generated support pages, not prose here. The hax-lib names of the specification surface must stay in sync with hax-lib/macros/src/implementation.rs and hax-lib/src/. Style: no em-dashes, one line per paragraph, list item, and table row; headings carry explicit anchors; keep them stable.
-->

This section describes the Rust that hax accepts, and the vocabulary you use to state the properties you want to prove about it. Properties proved about the extracted output hold of the original Rust program, so the verification applies to the code you ship rather than to a separate model of it.

!!! note "A vision document"

    This section describes the input language as we intend it to be, written in the present tense. The exclusions it records are the ones inherent to the value model, not the current implementation backlog. The gaps between this description and what the pipeline does today are listed, as far as the FLS test suite covers them, with the [planned constructs](../lean/support.md#planned) of the Lean backend; the coverage of the modeled library is tracked in [model trust](trust.md#assumed-vs-proven).

## The pipeline {#pipeline}

The input language is the language of one pipeline:

```
Rust --> Charon --> LLBC --> Aeneas --> Lean
```

Charon lowers the Rust program to LLBC, a borrow-aware intermediate language; Aeneas turns LLBC into a pure functional model, from which the Lean output is produced. hax's own contribution along this path is the [specification surface](specification.md), carried through with `-specs hax`.

The consequence: this path consumes Aeneas's functional model directly, so the input language *is* the subset Aeneas can functionalize. The `fstar` backend does not take this path, but goes through the hax engine instead; its input language is not described here.

## The basis: Aeneas plus the modeled library {#basis}

The input language rests on two distinct pillars.

1. **Aeneas fixes which language constructs are expressible.** It functionalizes a subset of safe Rust by modeling programs as pure functions over values, with `&mut` handled by backward functions rather than by memory reasoning ([the ownership model](language.md#ownership-model)). What has a place in that functional model is in the input language; what does not, is not.
2. **The modeled library fixes which library items are available.** Standard library items are given hand-written models rather than extracted from their real (often `unsafe` or intrinsic) bodies. These models cover the part of `core` that has a value reading, plus the modeled `alloc` and `std` containers (`Vec`, `String`, `HashMap`, and the like). The rest of the library surface is out of scope ([the library surface](language.md#library-surface)).

The theme running through both pillars: most items are signature-expressible, and the limit is on a faithful reading of their behavior. Where a body does not translate, a model supplies that reading, bounded by a small, enumerable trusted kernel ([model trust](trust.md)); where no faithful reading exists, the item is out of scope ([the status vocabulary](#status-vocabulary)).

## Signature support and implementation support {#signature-and-implementation}

Every item is judged on two independent questions, with very different consequences, and on a final requirement that both answers rest on.

1. Can we express the item's **signature**?
2. Can we translate the item's **implementation**?

The two questions come apart constantly, so we treat them separately.

- **Signature support is the hard requirement.** To reason about an item at all, the basis must represent its *type*: for a function, its inputs, outputs, generics, trait bounds, and the backward-function structure a returned `&mut` induces ([the ownership model](language.md#ownership-model)); for a type, the shape a caller needs to use it. If the signature is not expressible, the item is genuinely outside the input language, with no workaround.
- **Implementation support is the soft requirement.** If a function body (or a type's internal representation) cannot be translated automatically, because it uses an intrinsic, `unsafe`, or anything else without a functional model, the item is not lost. It becomes **opaque**, and its meaning is supplied by hand: an assumed specification, or a model proved against a reference. The [specification surface](specification.md) is the mechanism; [model trust](trust.md) accounts for what that trust costs.
- **A faithful reading is the final requirement.** An opaque item is only as good as its model. When the behavior of an expressible item has no faithful reading in the value model, such as a destructor with an observable effect or `Rc::ptr_eq`, no model can supply a reading a proof could rely on, and the item is unsupported; such behavior is reached at most through a safe facade over the [trusted kernel](trust.md#trusted-kernel).

Floating point is the worked example. The type `f64` is signature-expressible, so a function over `f64` has a perfectly good type in the model; the floating-point *operations* have a functional reading, since IEEE-754 arithmetic is deterministic, but only an assumed model of it is provided. Floating point is still *Supported* ([the status vocabulary](#status-vocabulary)): the signature translates, and the implementation is an opaque item with a model. The [arithmetic section](language.md#arithmetic) works through what that lets you prove and what it does not.

The payoff of separating the two: the signature is stable and essential, while implementation support is a *dial*. An item's implementation can move from an assumed model to a proven one over time, shrinking what you must trust, without its signature ever changing and without any caller being affected.

## Status vocabulary {#status-vocabulary}

Every feature table in this section uses one of three statuses. They describe the design scope of a construct, not how far today's implementation has got:

| Status | Meaning |
| --- | --- |
| **Supported** | The construct translates, and the result means what Rust means, relative to the trusted base. An item without a translatable body is used through an opaque item and a model ([making it opaque](idioms.md#opaque-and-model)). |
| **Partial** | Only part of the construct is in scope by design. The row states which uses are supported; the rest is unsupported. A part that has an Unsupported row of its own, such as a raw-pointer cast under casts, does not make a row Partial. |
| **Unsupported** | Out of scope by design, for one of three reasons: the construct is not even signature-expressible ([outside the value model](language.md#outside-the-value-model)); it is expressible but has no faithful reading in the value model, such as a destructor with an observable effect or `Rc::ptr_eq`; or it is library surface beyond the modeled one ([the library surface](language.md#library-surface)). Some of this is on the separation-logic horizon, not a permanent boundary. |

Every status rests on the trusted base: the encoding of Rust into the value model and the models of the primitives (machine integers, the safe slice and array operations) and of `core`. Whether the model behind an item is *assumed* or *proven* is a separate axis, tracked in [model trust](trust.md); where an assumed model limits what you can prove, as for floating point, the row says so. A supported construct that is not fully implemented yet links to the [planned constructs](../lean/support.md#planned) of the Lean backend.
