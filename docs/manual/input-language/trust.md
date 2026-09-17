---
weight: 3
---

# Model trust

Opaque items and the modeled library get their meaning from a hand-written model, and not all models are equally trustworthy. This section accounts for the trusted computing base (TCB): which kind of trust each model requires, and the bounded kernel that can never be discharged.

## Assumed vs. proven models {#assumed-vs-proven}

An opaque item has a known signature, and its meaning comes from a hand-written model. The two kinds differ in *where the trust bottoms out*; the signature is identical either way, only the justification differs.

**Assumed model.** You give the opaque item a specification (a postcondition, or a Lean definition asserted as an axiom) and nothing checks it against the real Rust. You are trusting, by hand, that "this item behaves as this spec says." The entire specification is part of the TCB; if it is wrong, every proof built on it is vacuous.

**Proven model.** The item's meaning is reduced to primitives that are already trusted, with a machine-checked proof of the reduction, so the TCB shrinks to just those primitives. There are two routes:

1. **Reduce to a reference in the supported subset.** Write a plain-Rust reference implementation that itself extracts, and prove the model equal to it. The trust becomes a discharged proof obligation. Applies only when a pure reference exists.
2. **Reduce to a lower trusted semantics.** Prove the model against a small, well-understood theory: bitvectors (Lean's `bv_decide`) or mathematical integers. Something is still trusted at the very bottom, but it is a tiny, universal core rather than a bespoke specification of one complex function.

The practical reading: an assumed model asks you to trust a handwritten description of a specific function; a proven model asks you to trust only bitvectors and integers, plus a proof you can inspect.

**Coverage of the modeled library.** [Signature and implementation support](index.md#signature-and-implementation) and [the library surface](language.md#library-surface) present the modeled library as covering the in-scope part of `core` plus the modeled `alloc` and `std` containers. That is the target; today the models reach a fraction of it, and which items have a model is tracked per module in the generated [`COVERAGE.md`](https://github.com/pereq-dev/hax/blob/main/hax-lib/core-models/COVERAGE.md) of `core-models`, which is authoritative over anything written here. The shortfall is of three kinds. Some surface has no faithful functional reading, so a model can only assume it or leave it out of scope: memory layout and representation (`mem`, `transmute`), raw pointers and interior mutability (`ptr`, `cell`), and the impure or platform modules (formatting, I/O, clocks, randomness, threading, networking, SIMD, intrinsics, and async). These are the boundaries [outside the value model](language.md#outside-the-value-model) and [the trusted kernel](#trusted-kernel) record, and they will not close. The floating-point operations are a second kind: they have a functional reading, but only an assumed model of it is provided ([arithmetic](language.md#arithmetic)). The rest is transient: a faithful model exists, or could, but the extraction pipeline cannot yet lower it. These are extraction gaps, not modeling ones; `COVERAGE.md` and the [planned constructs](../lean/support.md#planned) of the Lean backend record them.

## The trusted kernel {#trusted-kernel}

The library is modeled all the way down, but modeling has a floor. A bounded, enumerable set of `core` items can only ever be *trusted* models, never faithful extractions: their behavior has no functional reading in the value-based world of [the ownership model](language.md#ownership-model), or, for the floating-point operations, no faithful model of their reading is provided. That set is the trusted kernel, the part of the TCB a user inherits from the library that can never be discharged; everything else in the library is modeled *on top of* it, and its assumed models can shrink to the kernel over time.

The kernel is small and does not grow with the size of the program being verified. The table enumerates the items an extraction actually relies on, and for each, whether the model can be reduced to a proven form or remains irreducibly assumed.

| Kernel item | Why it cannot be extracted | Path to a proven model |
| --- | --- | --- |
| Raw-pointer / unsafe memory internals (`NonNull`, `MaybeUninit`, the unsafe cores of `[T]` and `str`) | Manipulate addresses and possibly-uninitialized memory; no value semantics | The *safe* API built on them is proven against the value model; the unsafe core stays assumed |
| Compiler and platform intrinsics (`copy_nonoverlapping`, SIMD, `transmute`) | Defined by the compiler or hardware, no Rust-level body to translate | Some (byte copies) reduce to array-update lemmas; the rest remain assumed |
| Floating-point operations | IEEE-754 rounding, NaN, and signed zero are not modeled (see [arithmetic](language.md#arithmetic)) | Assumed only; faithful reasoning is not provided |

Interior mutability and concurrency ([outside the value model](language.md#outside-the-value-model)) are unsupported, so an extraction cannot use them and inherits no trust from them; they enter the TCB only through a restricted, value-compatible safe facade, where one exists at all. Both sit on the separation-logic horizon of [outside the value model](language.md#outside-the-value-model), outside the kernel above.

## The direction of travel {#direction-of-travel}

The "path to a proven model" column above describes a *trajectory*, not a per-item guarantee at any given moment: an assumed model is discharged into a proven one, against a reference implementation or a bitvector model, which shrinks the TCB without touching any signature ([signature and implementation support](index.md#signature-and-implementation)). This is why this section attaches no fixed "assumed" or "proven" label to each library item: that status moves over time, so consult an item's model in the Lean libraries rather than a snapshot here. This section does promise the boundary: the kernel above is the whole of the library that must stay trusted, and it does not grow with your program.
