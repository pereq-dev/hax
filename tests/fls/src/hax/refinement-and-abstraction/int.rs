//! @fls: hax_int
//! @title: Mathematical integers built with the int literal macro
//! @lean(tier): supported

// The hax-lib API under test exists only in hax mode.
#![cfg(hax)]

use hax_lib::{Abstraction, Int, int};

pub fn big_sum() -> Int {
    int!(340282366920938463463374607431768211455) + int!(1)
}

#[hax_lib::requires(x.lift() + int!(1) <= int!(255))]
pub fn increment(x: u8) -> u8 {
    x + 1
}

#[hax_test::equiv]
pub fn mathematical_integers() -> bool {
    increment(41) == 42 && big_sum() > int!(0)
}
