//! @fls: fls_2az5huhcxzzy, fls_5ucvvja4dzoc, fls_j3mmg317q442, fls_nygurv3x3wq6, fls_pv5gcy3tbjwo
//! @title: An unsafe block calling an unsafe function
//! @lean(tier): supported

pub unsafe fn unchecked_get(xs: &[u32], i: usize) -> u32 {
    unsafe { *xs.get_unchecked(i) }
}

pub fn first(xs: &[u32]) -> u32 {
    if xs.is_empty() {
        0
    } else {
        unsafe { unchecked_get(xs, 0) }
    }
}

#[hax_test::equiv]
pub fn value_from_unsafe_block() -> bool {
    first(&[5, 6]) == 5
}
