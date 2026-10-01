//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! A `letfun` parameter is a plain binder, so a destructuring parameter is
//! bound whole, then destructured.

#![allow(dead_code)]

struct Point {
    x: u8,
    y: u8,
}

fn destructuring_params((a, b): (u8, u8), Point { x, y }: Point) -> u8 {
    a + b + x + y
}

fn binding_subpattern(pair @ (a, b): (u8, u8)) -> u8 {
    a + b + pair.0
}
