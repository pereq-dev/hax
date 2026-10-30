//! @fls: hax_to_prop
//! @title: A boolean lifted to a proposition with ToProp
//! @lean(tier): supported

use hax_lib::{Prop, ToProp};

pub fn small(x: u32) -> Prop {
    (x < 100).to_prop()
}

#[hax_lib::requires(small(x))]
pub fn accept(x: u32) -> u32 {
    x + 1
}

#[hax_test::equiv]
pub fn lifted_boolean() -> bool {
    accept(41) == 42
}
