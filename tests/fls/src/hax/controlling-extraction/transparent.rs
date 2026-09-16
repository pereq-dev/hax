//! @fls: hax_transparent
//! @title: A function forced into the extraction with transparent
//! @lean(tier): supported

#[hax_lib::transparent]
pub fn visible(x: u32) -> u32 {
    x + 1
}

#[hax_test::equiv]
pub fn transparent_body() -> bool {
    visible(1) == 2
}
