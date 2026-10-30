//! @fls: fls_FUdsmzN0T8XP, fls_3p5BdLn3JbKz, fls_QV6zE5Mdor5E, fls_MBa35hfS8J9l, fls_w0vwdmO8qV9j, fls_vImY3BrpNvNY
//! @title: A trait implementation for a reference type with an implicitly elided lifetime
//! @lean(tier): supported
//! @lean(reason): aeneas fails on the method body, whose `self` is a borrow of the reference type `&[u32]` (`Internal error, please file an issue`); nested borrows, the mutation section of the input-language reference.

pub trait Total {
    fn total(&self) -> u32;
}

impl Total for &[u32] {
    fn total(&self) -> u32 {
        let mut s = 0;
        let mut i = 0;
        while i < self.len() {
            s += self[i];
            i += 1;
        }
        s
    }
}

#[hax_test::equiv]
pub fn total_of_slice() -> bool {
    let data = [1u32, 2, 3];
    let s: &[u32] = &data;
    s.total() == 6
}
