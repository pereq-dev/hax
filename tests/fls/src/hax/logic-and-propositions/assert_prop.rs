//! @fls: hax_assert_prop
//! @title: A proposition to prove at a program point stated with assert_prop
//! @lean(tier): supported
//! @lean(reason): The Lean backend has no definition for `hax_lib::assert_prop` (`Unknown identifier hax_lib.assert_prop`), so the extraction fails to elaborate; the logic section of the input-language reference.

pub fn checked_sum(xs: &[u32; 3]) -> u32 {
    let total = xs[0] + xs[1] + xs[2];
    hax_lib::assert_prop!(hax_lib::forall(|i: usize| hax_lib::implies(
        i < 3,
        xs[i] <= total
    )));
    total
}

#[hax_test::equiv]
pub fn proposition_recorded() -> bool {
    checked_sum(&[1, 2, 3]) == 6
}
