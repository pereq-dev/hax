//! @fls: hax_prop
//! @title: Propositions combined with the Prop connectives
//! @lean(tier): supported

use hax_lib::Prop;

pub fn in_range(x: u32) -> Prop {
    let lower = Prop::from_bool(x >= 10);
    let upper = Prop::from_bool(x <= 20);
    lower
        .and(upper)
        .or(Prop::from_bool(x == 0))
        .and(!Prop::from_bool(x == 15))
}

#[hax_lib::requires(in_range(x))]
pub fn accept(x: u32) -> u32 {
    x
}

#[hax_test::equiv]
pub fn propositions_compose() -> bool {
    accept(12) == 12 && accept(0) == 0
}
