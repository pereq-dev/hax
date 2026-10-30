//! @fls: fls_vujl3fblz6x2
//! @title: A higher-ranked trait bound over a closure taking a reference
//! @lean(tier): supported

pub fn apply_to_both<F>(f: F, a: u32, b: u32) -> u32
where
    F: for<'x> Fn(&'x u32) -> u32,
{
    f(&a) + f(&b)
}

#[hax_test::equiv]
pub fn called_at_two_lifetimes() -> bool {
    apply_to_both(|x| *x * 2, 1, 2) == 6
}
