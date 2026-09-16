//! @fls: hax_exclude
//! @title: An item removed from the extraction with exclude
//! @lean(tier): supported

#[hax_lib::exclude]
pub fn not_extracted(x: u32) -> u32 {
    x.leading_zeros()
}

pub fn extracted(x: u32) -> u32 {
    x + 1
}

#[hax_test::equiv]
pub fn excluded_item_leaves_the_rest() -> bool {
    extracted(1) == 2
}
