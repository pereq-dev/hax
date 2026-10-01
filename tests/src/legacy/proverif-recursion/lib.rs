//! @fail(extraction): proverif(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001)
//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! A function in a call-graph cycle has no ProVerif encoding, so it is
//! rejected with a diagnostic and opacified to an uninterpreted `fun`. A
//! non-recursive caller stays a `letfun`.

fn count_down(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        count_down(n - 1)
    }
}

fn ping(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        pong(n - 1)
    }
}

fn pong(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        ping(n - 1)
    }
}

fn caller(n: u32) -> u32 {
    count_down(n)
}

fn nullary() -> u32 {
    through_nullary(1)
}

fn through_nullary(n: u32) -> u32 {
    if n == 0 { 0 } else { nullary() }
}

// `d` and `c` only depend on the `a`/`b` cycle, so they are emitted after it.
struct Chain;

impl Chain {
    fn d(&self, n: u32) -> u32 {
        self.c(n)
    }
    fn c(&self, n: u32) -> u32 {
        self.a(n)
    }
    fn a(&self, n: u32) -> u32 {
        if n == 0 { 0 } else { self.b(n - 1) }
    }
    fn b(&self, n: u32) -> u32 {
        if n == 0 { 1 } else { self.a(n - 1) }
    }
}
