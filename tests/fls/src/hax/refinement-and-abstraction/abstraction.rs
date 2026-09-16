//! @fls: hax_abstraction
//! @title: The Abstraction trait lifting a concrete value to its abstract model
//! @lean(tier): supported

// The hax-lib API under test exists only in hax mode.
#![cfg(hax)]

use hax_lib::{Abstraction, int};

#[hax_lib::ensures(|result| result.lift() == x.lift() + y.lift())]
pub fn add(x: u8, y: u8) -> u16 {
    x as u16 + y as u16
}

#[hax_test::equiv]
pub fn abstracted_sum() -> bool {
    add(200, 100) == 300 && add(1, 2).lift() == int!(3)
}
