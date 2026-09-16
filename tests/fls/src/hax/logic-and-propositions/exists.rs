//! @fls: hax_exists
//! @title: Existential quantification in a postcondition
//! @lean(tier): supported

#[hax_lib::ensures(|result| hax_lib::exists(|i: usize| i < 4 && result[i] == 9))]
pub fn with_marker() -> [u8; 4] {
    [0, 9, 0, 0]
}

#[hax_test::equiv]
pub fn marker_present() -> bool {
    with_marker()[1] == 9
}
