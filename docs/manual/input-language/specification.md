---
weight: 2
---

# Specification surface

Beyond the Rust it accepts, the input language includes a vocabulary for *stating properties* and *supplying models*, provided by the `hax-lib` crate and carried to the backend via `-specs hax`. These constructs compile away to nothing in an ordinary Rust build. This vocabulary is also the mechanism behind opaque items (see [signature and implementation support](index.md#signature-and-implementation)): an opaque item plus a specification.

## Contracts {#contracts}

Contracts attach pre- and post-conditions to a function, and measures to recursive functions and loops.

- `#[requires(cond)]` states a precondition: a boolean expression over the function's arguments that callers must establish.
- `#[ensures(|result| cond)]` states a postcondition as a closure binding the return value, so the condition can relate inputs to `result`.
- `#[ensures_ref(|result| cond)]` is the same, but the closure binds the return value by reference (`&T`).
- `future(x)`, used inside a postcondition, names the post-state of a mutable argument `x`: the value its backward function writes back ([the ownership model](language.md#ownership-model)). This is how a postcondition talks about what a `&mut` mutation produced rather than the value on entry; the worked idiom is in [talking about a mutation](idioms.md#mutation-results).
- `#[decreases(measure)]` gives a termination measure for a recursive function, discharging the well-foundedness obligation the `Result` monad's `div` outcome would otherwise leave open ([the ownership model](language.md#ownership-model)).
- `#[attributes]` on an `impl` block enables `#[requires]`, `#[ensures]`, and `#[decreases]` on the methods inside it.
- `loop_invariant!(|i| cond)` and `loop_decreases!(measure)` play the same roles inside a loop body. The invariant is a closure over the loop's index (or, for a `loop`/`while`, a chosen variable) and holds on every iteration; the measure proves the loop terminates.

```rust
#[requires(n < 1000)]              // keeps acc within u32, so `+=` cannot overflow
#[ensures(|result| result >= n)]
fn triangle(n: u32) -> u32 {
    let mut acc = 0;
    for i in 0..n + 1 {
        loop_invariant!(|i: u32| 2 * acc == i * i - i);  // acc holds 0 + 1 + ... + (i-1)
        acc += i;
    }
    acc
}
```

## Logic and propositions {#logic}

Contract conditions are ordinary Rust booleans where that suffices, but proof obligations often need quantifiers and implication, which Rust booleans cannot express. The `Prop` type is the language of logical propositions for exactly this. It carries the usual connectives (`and`, `or`, `not`, `implies`, equality) and the quantifiers `forall` and `exists`. The connectives `and`, `or`, and `not` are methods on `Prop` (also available as the operators `&`, `|`, `!`), while `implies`, equality, and the quantifiers are free functions, the quantifiers taking a closure over the bound variable. `implies!(a, b)` is the lazy form of `implies`: it evaluates `b` only when `a` holds, for a `b` that is only well-defined under `a`, such as an index guarded by a bounds check:

```rust
// For all i in bounds, the buffer is zeroed.
forall(|i: usize| implies!(i < buf.len(), buf[i] == 0))
```

The `ToProp` trait lifts a plain `bool` into a `Prop`, so boolean and propositional conditions compose. Two macros drive props at specific program points: `assert_prop!(p)` records a proposition to prove there, and `assume!(p)` introduces one as a hypothesis without proof (a deliberate, auditable addition to what you trust). `hax_lib::assert!(cond)` behaves like `assert!` in Rust and carries the check into the model as an assertion; `hax_lib::debug_assert!` behaves like `debug_assert!` in Rust and disappears from the model. A plain `assert!` is an ordinary panic ([the ownership model](language.md#ownership-model)).

## Refinement and abstraction {#refinement}

Two mechanisms let you state properties in terms of cleaner mathematical objects than the machine types a program actually runs on.

**Refinement types.** `#[refinement_type(|x| invariant)]` defines a base type carrying a logical invariant, so the invariant travels with every value of that type instead of being restated at each use. The `Refinement` trait gives the carved-out type its constructor and projection, and `RefineAs` connects a base value to its refined form. A `NonZeroU32`-style bound, or "an index known to be in range," is a natural fit.

**The mathematical-integer model.** Machine integers overflow; mathematical integers do not, and proofs are far easier over the latter. The `Int` type is an arbitrary-precision integer, written with the `int!` literal macro, with `ToInt` lifting a machine integer into it and `Int` methods such as `pow2` and `rem_euclid`. You reason in `Int` and connect back to machine types at the boundary.

**Abstraction and concretization.** More generally, `Abstraction` and `Concretization` relate a concrete representation to an abstract model of it (for example, a byte array abstracted as the number it encodes). This is the idiom for proving a fast concrete implementation matches a simple abstract specification, and it is the same shape of reasoning the proven models of [assumed vs. proven models](trust.md#assumed-vs-proven) use.

## Controlling extraction and supplying models {#supplying-models}

This is the toolkit for items without a translatable body (see [signature and implementation support](index.md#signature-and-implementation)): deciding what crosses into the model and, where a body cannot be translated, supplying its meaning by hand.

- `#[opaque]` hides a function body or a type's representation, keeping the signature while dropping the implementation. `#[transparent]` is the opposite, forcing a definition to be visible.
- The model of an opaque item is supplied on the target side. The Lean extraction declares every opaque item as an axiom in the `Assumptions` files of the generated package; the proofs see the definition written there by hand, or shipped by `core-models` for a library item. `#[opaque]` plus such a model is *the* way a signature-supported item acquires a hand-written implementation, and it is how every item of the input language without a translatable body gets its meaning. `hax-lib` offers no attribute that names the model from the Rust side; the `#[modeled_by(path)]` attribute once drafted for this purpose is disabled until a backend consumes it.
- `#[include(...)]` / `#[exclude]` select which items are extracted at all.
- `#[lemma]` marks an item as a proof obligation or reusable fact rather than executable code.

**This is the only sanctioned escape hatch.** Raw target-language code injection is deliberately not supported. The engine-era `<backend>_expr!` and `#[<backend>_replace]` injection macros belong to the hax engine, which the Aeneas path bypasses, and are intentionally not offered here. The reason is the trust accounting of [model trust](trust.md): a model has a defined trust status, assumed or proven, whereas injected code would be unconstrained trusted input and would punch an untracked hole in the trusted kernel.
