//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! An inlined impl method is not emitted: its calls are β-reduced, and applying
//! its function parameter has no ProVerif form.

struct Counter;

impl Counter {
    #[hax_lib::pv_inline]
    fn apply(&self, x: u8, f: fn(u8) -> u8) -> u8 {
        f(x)
    }
}

fn increment(x: u8) -> u8 {
    x
}

fn run(c: Counter, x: u8) -> u8 {
    c.apply(x, |y| increment(y))
}

trait Apply {
    fn apply_trait(&self, x: u8, f: fn(u8) -> u8) -> u8;
}

impl Apply for Counter {
    #[hax_lib::pv_inline]
    fn apply_trait(&self, x: u8, f: fn(u8) -> u8) -> u8 {
        f(x)
    }
}

// A trait method is resolved to its impl before being inlined.
fn run_trait(c: Counter, x: u8) -> u8 {
    c.apply_trait(x, |y| increment(y))
}
