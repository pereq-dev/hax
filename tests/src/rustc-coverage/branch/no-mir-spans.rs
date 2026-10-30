//! @fail(extraction): proverif(HAX0001, HAX0001, HAX0001, HAX0001)
//! @fail(tc): legacy-lean(1), fstar(2)
#![feature(coverage_attribute)]
//@ edition: 2021
//@ compile-flags: -Zcoverage-options=branch,no-mir-spans
//@ llvm-cov-flags: --show-branches=count

// Tests the behaviour of the `-Zcoverage-options=no-mir-spans` debugging flag.
// The actual code below is just some non-trivial code copied from another test
// (`while.rs`), and has no particular significance.

macro_rules! no_merge {
    () => {
        for _ in 0..1 {}
    };
}

/// @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
fn while_cond() {
    no_merge!();

    let mut a = 8;
    while a > 0 {
        a -= 1;
    }
}

/// @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
fn while_cond_not() {
    no_merge!();

    let mut a = 8;
    while !(a == 0) {
        a -= 1;
    }
}

/// @fail(extraction): ssprove(HAX0001), coq(HAX0001, HAX0001)
fn while_op_and() {
    no_merge!();

    let mut a = 8;
    let mut b = 4;
    while a > 0 && b > 0 {
        a -= 1;
        b -= 1;
    }
}

/// @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001)
fn while_op_or() {
    no_merge!();

    let mut a = 4;
    let mut b = 8;
    while a > 0 || b > 0 {
        a -= 1;
        b -= 1;
    }
}

#[coverage(off)]
fn main() {
    while_cond();
    while_cond_not();
    while_op_and();
    while_op_or();
}
