//! @off: fstar, lean, coq, ssprove
//!
//! A binding sub-pattern (`x @ p`) names the whole value and destructures it at
//! once, which no ProVerif pattern does. `x` is bound alone and `p` becomes a
//! nested destructure of `x`, wherever the pattern opens a scope.

#![allow(dead_code)]

fn in_match_arm(x: (u8, (u8, u8))) -> u8 {
    match x {
        (0, pair @ (a, b)) => a + b + pair.0 + pair.1,
        _ => 0,
    }
}

fn in_let(x: (u8, u8)) -> u8 {
    let pair @ (a, b) = x;
    a + b + pair.0
}

struct Point {
    x: u8,
    y: u8,
}

fn nested(p: Point) -> u8 {
    let whole @ Point { x, y } = p;
    x + y + whole.x
}
