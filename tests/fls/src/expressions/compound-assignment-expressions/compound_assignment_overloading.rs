//! @fls: fls_3bu3g8o5nopc, fls_ngimpabunzis, fls_ijfmnnrdlu8n, fls_8j408kckzzud, fls_4sbpfi12frwe, fls_n5ds6ydgckvo, fls_xjdu0y1slsg9
//! @title: Compound assignment on a struct through AddAssign and SubAssign
//! @lean(tier): supported

use core::ops::{AddAssign, SubAssign};

pub struct Acc(pub i32);

impl AddAssign<i32> for Acc {
    fn add_assign(&mut self, k: i32) {
        self.0 += k;
    }
}

impl SubAssign<i32> for Acc {
    fn sub_assign(&mut self, k: i32) {
        self.0 -= k;
    }
}

#[hax_test::equiv]
pub fn overloaded_compound_assignment() -> bool {
    let mut a = Acc(1);
    a += 5;
    a -= 2;
    a.0 == 4
}
