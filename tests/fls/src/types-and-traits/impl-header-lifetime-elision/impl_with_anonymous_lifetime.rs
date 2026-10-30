//! @fls: fls_FUdsmzN0T8XP, fls_PfS5AlkN6ANl, fls_MBa35hfS8J9l, fls_w0vwdmO8qV9j
//! @title: A trait implementation for a type with the explicitly elided lifetime '_
//! @lean(tier): supported
//! @lean(reason): aeneas fails on the method body, whose `self` is a borrow of `Bag`, which holds a borrow itself (`Internal error, please file an issue`); nested borrows, the mutation section of the input-language reference.

pub trait Total {
    fn total(&self) -> u32;
}

pub struct Bag<'a>(pub &'a [u32]);

impl Total for Bag<'_> {
    fn total(&self) -> u32 {
        let mut s = 0;
        let mut i = 0;
        while i < self.0.len() {
            s += self.0[i];
            i += 1;
        }
        s
    }
}

#[hax_test::equiv]
pub fn total_of_bag() -> bool {
    let data = [1u32, 2, 3];
    Bag(&data).total() == 6
}
