//! @fls: fls_twhq24s8kchh, fls_w4NbA7WhZfR2, fls_ie0avzljmxfm, fls_15zdiqsm1q3p, fls_3qI8FXMsyk0f
//! @title: Shared references are read-only and Copy
//! @lean(tier): supported

pub fn sum_both(a: &u32, b: &u32) -> u32 {
    *a + *b
}

#[hax_test::equiv]
pub fn copied_shared_reference() -> bool {
    let x = 4;
    let r: &u32 = &x;
    let s = r;
    sum_both(r, s) == 8
}
