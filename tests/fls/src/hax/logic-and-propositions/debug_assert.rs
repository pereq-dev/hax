//! @fls: hax_debug_assert
//! @title: A debug assertion with hax_lib::debug_assert, which disappears from the model
//! @lean(tier): supported

pub fn double(x: u32) -> u32 {
    let y = x * 2;
    hax_lib::debug_assert!(y % 2 == 0);
    y
}

#[hax_test::equiv]
pub fn debug_asserted_program_point() -> bool {
    double(21) == 42
}
