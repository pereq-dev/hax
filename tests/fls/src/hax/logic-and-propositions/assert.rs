//! @fls: hax_assert
//! @title: A boolean assertion carried into the model with hax_lib::assert
//! @lean(tier): supported
//! @lean(reason): The Lean backend has no definition for `hax_lib::assert` (`Unknown identifier hax_lib.assert`), so the extraction fails to elaborate; the logic section of the input-language reference.

pub fn clamp(x: u32) -> u32 {
    let y = if x > 10 { 10 } else { x };
    hax_lib::assert!(y <= 10);
    y
}

#[hax_test::equiv]
pub fn asserted_program_point() -> bool {
    clamp(3) == 3 && clamp(30) == 10
}
