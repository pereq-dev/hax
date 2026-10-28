---
weight: 0
---

# Panic freedom

Let's start with a simple example: a function that squares a `u8`
integer. To extract this function to F\* using hax, we simply need to
run the command `cargo hax into fstar` in the directory of the crate
in which the function `square` is defined.

*Note: throughout this tutorial, you can edit the snippets of code and
extract to F\* by clicking the play button (:material-play:), or even typecheck it with the button (:material-check:).*

```{.rust .playable .expect-failure }
fn square(x: u8) -> u8 {
    x * x
}
```

Though, if we try to verify this function, F\* is complaining about a
subtyping issue: F\* tells us that it is not able to prove that the
result of the multiplication `x * x` fits the range of `u8`. The
multiplication `x * x` might indeed be overflowing!

For instance, running `square(16)` panics: `16 * 16` is `256`, which
is just over `255`, the largest integer that fits `u8`. Rust does not
ensure that functions are *total*: a function might panic at any
point, or might never terminate.


## Rust and panicking code
Quoting the chapter [To `panic!` or Not to
`panic!`](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)
from the Rust book:

> The `panic!` macro signals that your program is in a state it can't
> handle and lets you tell the process to stop instead of trying to
> proceed with invalid or incorrect values.

A Rust program should panic only in a situation where an assumption
or an invariant is broken: a panic models an *invalid* state. Formal
verification is about proving such invalid state cannot occur, at all.

From this observation emerges the urge of proving Rust programs to be
panic-free!

## Fixing our squaring function
Let's come back to our example. There is an informal assumption to the
multiplication operator in Rust: the inputs should be small enough so
that the multiplication doesn't overflow.

Note that Rust also provides `wrapping_mul`, a non-panicking variant
of the multiplication on `u8` that wraps when the result is bigger
than `255`. Replacing the common multiplication with `wrapping_mul` in
`square` would fix the panic, but then, `square(16)` returns zero.
Semantically, this is not what one would expect from `square`.

Our problem is that our function `square` is well-defined only when
its input is within `0` and `15`.

### Solution A: reflect the partialness of the function in Rust
A first solution is to make `square` return an `Option<u8>` instead of a `u8`:
``` {.rust .playable}
fn square_option(x: u8) -> Option<u8> {
    if x >= 16 {
        None
    } else {
        Some(x * x)
    }
}
```

Here, F\* is able to prove panic-freedom: calling `square` with any
input is safe. Though, one may argue that `square`'s input being small
enough should really be an assumption. Having to deal with the
possible integer overflowing whenever squaring is a huge burden. Can
we do better?

### Solution B: add a precondition
The type system of Rust doesn't allow the programmer to formalize the
assumption that `square` expects a small `u8`. This becomes
possible using hax: one can annotate a function with a pre-condition
on its inputs.

The pre-conditions and post-conditions on a function form a
*contract*: "if you give me some inputs that satisfies a given formula
(*the precondition*), I will produce a return value that satisfy
another formula (*the postcondition*)". Outside this contract,
anything might happen: the function might panic, might run forever,
erase your disk, or anything.

The helper crate
[hax-lib](https://github.com/pereq-dev/hax/tree/main/hax-lib)
provides the `requires`
[proc-macro](https://doc.rust-lang.org/reference/procedural-macros.html)
which lets user writing pre-conditions directly in Rust.

```{.rust .playable}
#[hax_lib::requires(x < 16)]
fn square_requires(x: u8) -> u8 {
    x * x
}
```

With this precondition, F\* is able to prove panic freedom. From now
on, it is the responsibility of the clients of `square` to respect the
contract. The next step is thus be to verify, through hax extraction,
that `square` is used correctly at every call site.

## Common panicking situations
Multiplication is not the only panicking function provided by the Rust
library: most of the other integer arithmetic operation have such
informal assumptions.

Another source of panics is indexing. Indexing in an array, a slice or
a vector is a partial operation: the index might be out of range.

In the example folder of hax, you can find the [`chacha20`
example](https://github.com/pereq-dev/hax/blob/main/examples/chacha20/src/lib.rs)
that makes use of pre-conditions to prove panic freedom.

Another solution for safe indexing is to use the [newtype index
pattern](https://matklad.github.io/2018/06/04/newtype-index-pattern.html),
which is [also supported by
hax](https://github.com/pereq-dev/hax/blob/d668de4d17e5ddee3a613068dc30b71353a9db4f/tests/attributes/src/lib.rs#L98-L126). The [data invariants](data-invariants.md#newtype-and-refinements) chapter gives more details about this.

