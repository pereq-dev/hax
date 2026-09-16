//! @fls: hax_assume
//! @title: A hypothesis introduced without proof with assume
//! @lean(tier): supported
//! @lean(reason): The Lean backend has no definition for `hax_lib::assume` (`Unknown identifier hax_lib.assume`), so the extraction fails to elaborate; the logic section of the input-language reference.

pub fn half(x: u32) -> u32 {
    hax_lib::assume!(hax_lib::Prop::from_bool(x % 2 == 0));
    x / 2
}

#[hax_test::equiv]
pub fn assumed_hypothesis() -> bool {
    half(8) == 4
}
