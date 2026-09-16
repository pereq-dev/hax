//! @fls: fls_uldh10k77sng, fls_gn3dnuxm2h8m
//! @title: Negation operators on structs through the core::ops traits
//! @lean(tier): supported

use core::ops::{Neg, Not};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Signed(pub i32);

impl Neg for Signed {
    type Output = Signed;
    fn neg(self) -> Signed {
        Signed(0 - self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Flag(pub bool);

impl Not for Flag {
    type Output = Flag;
    fn not(self) -> Flag {
        Flag(!self.0)
    }
}

#[hax_test::equiv]
pub fn overloaded_negations() -> bool {
    -Signed(4) == Signed(-4) && !Flag(true) == Flag(false)
}
