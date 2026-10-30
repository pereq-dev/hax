//! @fls: fls_s76un78zyd0j, fls_vkk2krfn93ry, fls_g10nuv14q4jn, fls_50qipwqi3arw, fls_idzhusp2l908, fls_qqrqyc6uhol
//! @title: Unsuffixed integer literals typed by their context or defaulting to i32
//! @lean(tier): supported

pub fn from_return_type() -> u8 {
    200
}

pub fn from_parameter(x: u64) -> u64 {
    x + 1
}

// Without a constraint the literal is an `i32`: shifting 1 into bit 31 gives a
// negative value, which no other integer type would.
pub fn defaulted_is_i32() -> bool {
    let x = 1;
    (x << 31) < 0
}

#[hax_test::equiv]
pub fn inferred_types() -> bool {
    from_return_type() == 200
        && from_parameter(4_000_000_000) == 4_000_000_001
        && defaulted_is_i32()
}
