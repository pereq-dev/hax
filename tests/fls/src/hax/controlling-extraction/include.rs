//! @fls: hax_include
//! @title: An item selected for extraction with include
//! @lean(tier): supported

#[hax_lib::include]
pub fn included(x: u32) -> u32 {
    x * 3
}

#[hax_test::equiv]
pub fn included_item() -> bool {
    included(2) == 6
}
