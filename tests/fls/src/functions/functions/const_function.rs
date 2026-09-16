//! @fls: fls_87jnkimc15gi, fls_2049qu3ji5x7, fls_otr3hgp8lj1q
//! @title: A const function called in a constant context and at runtime
//! @lean(tier): supported

pub const fn square(x: u32) -> u32 {
    x * x
}

pub const NINE: u32 = square(3);

#[hax_test::equiv]
pub fn const_function_values() -> bool {
    NINE == 9 && square(4) == 16
}
