//! @off: fstar, lean, coq, ssprove
//!
//! Integer literals: each literal renders as an opaque constant `nat_<value>`,
//! so a large value carries no unary-`nat` cost. `+ 1` and `- 1` reduce over
//! the small declared constants `nat_0 .. nat_16`; other arithmetic and slice
//! indexing beyond the head stay opaque.

fn small() -> u32 {
    3
}

fn big() -> u64 {
    18446744073709551615
}

fn negative() -> i32 {
    -5
}

fn increment(n: u32) -> u32 {
    n + 1
}

fn decrement(n: u32) -> u32 {
    n - 1
}

fn opaque_sum(a: u32, b: u32) -> u32 {
    a + b
}

fn head(xs: &[u8]) -> u8 {
    xs[0]
}
