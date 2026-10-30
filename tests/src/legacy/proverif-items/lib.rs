//! @off: fstar, lean, coq, ssprove
//!
//! Structs and enums render as `[data]` constructors with per-field `reduc`
//! accessors, and a field projection calls the accessor. Impl items are
//! emitted in dependency order, so a method or associated constant is declared
//! before the sibling that uses it regardless of source order.

struct Point {
    x: u32,
    y: u32,
}

fn make(x: u32, y: u32) -> Point {
    Point { x, y }
}

fn get_x(p: Point) -> u32 {
    p.x
}

struct Counter;

impl Counter {
    fn outer(&self) -> u32 {
        self.inner()
    }

    fn inner(&self) -> u32 {
        Self::BASE
    }

    const BASE: u32 = 0;
}
