//! @fls: fls_udn9lyf3m0z6, fls_n570za6a9nqd, fls_o7cqfdnr253y, fls_m690b8qg9d9r
//! @title: Promoted borrows of a computed constant expression and of an empty mutable slice
//! @lean(tier): supported
//! @lean(reason): aeneas fails on the promoted borrow of a computed constant (`There should be no bottoms in the value`) and on the length of the promoted `&mut []` (`Unreachable`); the ownership model section of the input-language reference.

pub fn answer() -> &'static u32 {
    &(40 + 2)
}

pub fn empty() -> &'static mut [u32] {
    &mut []
}

#[hax_test::equiv]
pub fn promoted_values() -> bool {
    *answer() == 42 && empty().len() == 0
}
