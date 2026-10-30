---
weight: 4
---

# Idioms and workarounds

When the input language will not take something, this section gives what to write instead. Each idiom is a "you cannot do X, so do Y" pair.

## Panicking operations: constrain the inputs {#panicking-operations}

Indexing, slicing, division, and arithmetic can all panic, and each becomes a proof obligation in the `Result` model ([the ownership model](language.md#ownership-model)). Rather than leave the obligation open, state the condition that rules the panic out as a precondition, and let callers discharge it. This is the single most common annotation in real code:

```rust
#[requires(input.len() >= 4)]
fn parse_header(input: &[u8]) -> Header { /* reading input[0..4] is now in-bounds */ }
```

The same move handles arithmetic bounds: a `#[requires]` that bounds an input, paired with an `#[ensures]` that bounds the output, keeps every intermediate operation within range.

## Overflow: reason over mathematical integers {#overflow}

Machine arithmetic overflows, and proving it does not can drown a proof in side conditions. Lift the quantity into the `Int` model ([refinement and abstraction](specification.md#refinement)), prove the arithmetic fact there where there is no overflow, and connect back to the machine type at the boundary. Bound-carrying contracts and `Int` reasoning together are the standard way bound-heavy code discharges its arithmetic.

## Loops: state the invariant {#loop-invariants}

A loop that a proof must reason through needs an invariant. `loop_invariant!` takes a closure over the loop's index and asserts a property that holds on every iteration:

```rust
for i in 0..blocks.len() {
    loop_invariant!(|i: usize| i <= blocks.len());
    // ...
}
```

For a recursive function, the analogue is `#[decreases(measure)]`, which supplies the well-founded measure that discharges termination.

## Talking about the result of a mutation {#mutation-results}

When a function mutates through a `&mut`, its postcondition needs to refer to the *updated* value, not the value on entry. Use `future(...)` for the post-state of a mutable argument:

```rust
// `f` here stands for any pure function of the entry value.
#[ensures(|_| future(arr) == f(*arr))]
fn transform_in_place<const N: usize>(arr: &mut [u8; N]) { /* ... */ }
```

This is the specification-level face of the backward-function model ([the ownership model](language.md#ownership-model)): `future(arr)` names what the backward function will write back.

## An unmodelable body: make it opaque and model it {#opaque-and-model}

When a function cannot be translated, because it bottoms out in an intrinsic or `unsafe` ([the trusted kernel](trust.md#trusted-kernel)), do not fight the extraction. Mark the item `#[opaque]` and give it a model, so callers see a well-specified signature while the body is supplied by hand. `core-models` uses the same mechanism for a library item without a translatable body: the item is opaque, and its model carries the meaning. Start from an assumed model and discharge it to a proven one later ([the direction of travel](trust.md#direction-of-travel)); no caller changes when you do. A foreign function or static declared in an `extern` block is the same case: it has a signature but no Rust body (or no initializer), so it is opaque by nature and reached only through a model. A C-variadic signature (`extern "C" fn(fmt: *const c_char, ...)`) is the exception: with no fixed arity it has no arrow type to express, so it is signature-unsupported ([outside the value model](language.md#outside-the-value-model)) rather than merely opaque.

## Shared mutable state: thread it explicitly {#shared-state}

Interior mutability is out of the input language ([outside the value model](language.md#outside-the-value-model)), so a design built on `Cell`, `RefCell`, or a shared mutable singleton does not translate. Rewrite it to pass the state in and return the updated state out, or to hold it behind a `&mut` that the caller threads through. The value model then handles the updates through backward functions, and there is no shared mutation left to reason about.
