//! @fls: fls_8QGbl2SBU3R0, fls_xkyj83mcb9d5
//! @title: Shift operators on a struct through the core::ops traits
//! @lean(tier): supported

use core::ops::{Shl, Shr};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bits(pub u32);

impl Shl<u32> for Bits {
    type Output = Bits;
    fn shl(self, n: u32) -> Bits {
        Bits(self.0 << n)
    }
}

impl Shr<u32> for Bits {
    type Output = Bits;
    fn shr(self, n: u32) -> Bits {
        Bits(self.0 >> n)
    }
}

#[hax_test::equiv]
pub fn overloaded_shifts() -> bool {
    (Bits(3) << 2) >> 1 == Bits(6)
}
