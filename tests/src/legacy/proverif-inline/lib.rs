//! @fail(extraction): proverif(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001)
//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! `#[hax_lib::pv_inline]` β-inlines every call to `apply_twice`, with the
//! closure body β-reduced into the call positions.

#[hax_lib::pv_inline]
fn apply_twice(x: u8, f: fn(u8) -> u8) -> u8 {
    f(f(x))
}

// One concrete callee for the closure to reduce against.
fn increment(x: u8) -> u8 {
    x
}

// Contains the inlined body of `apply_twice`, not a call to it.
fn run(start: u8) -> u8 {
    apply_twice(start, |y| increment(y))
}

// The parameter is used under a loop, and shadowed in one branch only: every
// use still refers to the argument.
#[hax_lib::pv_inline]
fn sum_or_keep(xs: Vec<u8>, keep: bool, f: fn(u8, u8) -> u8) -> u8 {
    let mut acc = 0;
    for x in xs {
        acc = f(acc, x);
    }
    if keep {
        let xs = acc;
        xs
    } else {
        acc
    }
}

fn run_sum(v: Vec<u8>) -> u8 {
    sum_or_keep(v, true, |a, b| increment(a))
}

// An argument used twice is evaluated once.
#[hax_lib::pv_inline]
fn twice(x: u8, f: fn(u8, u8) -> u8) -> u8 {
    f(x, x)
}

fn run_twice(start: u8) -> u8 {
    twice(increment(start), |a, b| a)
}

// A hand-written function is not inlined.
#[hax_lib::pv_inline]
#[hax_lib::pv_handwritten]
fn handwritten(x: u8) -> u8 {
    increment(x)
}

fn run_handwritten(x: u8) -> u8 {
    handwritten(x)
}

// Mutually recursive functions are not inlined.
#[hax_lib::pv_inline]
fn ping(x: u8) -> u8 {
    if x == 0 { 0 } else { pong(x - 1) }
}

#[hax_lib::pv_inline]
fn pong(x: u8) -> u8 {
    ping(x)
}

// A function passed as a value to a non-inlined function has no ProVerif form:
// rejected rather than becoming a one-way term.
fn store(f: fn(u8) -> u8) -> Option<fn(u8) -> u8> {
    Some(f)
}

fn pass_std() -> Option<fn(u8) -> u8> {
    store(core::convert::identity)
}

// A closure through a generic `F: Fn` parameter is not inlined: rejected.
fn apply_generic<F: Fn(u8) -> u8>(f: F, x: u8) -> u8 {
    f(x)
}

fn call_generic(x: u8) -> u8 {
    apply_generic(|y| increment(y), x)
}
