//! @fls: fls_yzuceqx6nxwa, fls_asfrqemqviad, fls_d62qfloqk2ub, fls_xmtxkit3qpw7, fls_htrjqxiv3avh, fls_rfomib80bnn2
//! @title: Comparison operators on a struct through PartialEq and PartialOrd
//! @lean(tier): supported

use core::cmp::Ordering;

pub struct Version(pub u32, pub u32);

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        if self.0 != other.0 {
            self.0.partial_cmp(&other.0)
        } else {
            self.1.partial_cmp(&other.1)
        }
    }
}

#[hax_test::equiv]
pub fn trait_comparisons() -> bool {
    let a = Version(1, 9);
    let b = Version(2, 0);
    a < b && a != b && b >= a && a == Version(1, 9)
}
