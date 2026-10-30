//! @fls: hax_to_int
//! @title: Machine integers lifted to mathematical integers with ToInt
//! @lean(tier): supported

use hax_lib::{ToInt, int};

#[hax_lib::requires(x.to_int() * int!(2) <= int!(255))]
pub fn double(x: u8) -> u8 {
    x * 2
}

#[hax_test::equiv]
pub fn lifted_operand() -> bool {
    double(21) == 42
}
