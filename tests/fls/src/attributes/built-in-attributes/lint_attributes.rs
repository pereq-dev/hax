//! @fls: fls_dtb3t5ht5ngf, fls_c5n4gzgs79vv, fls_29y8icoou1gx, fls_xheohvupr8kb, fls_5ko0q9jnxv5a, fls_NrTL2FruARAv, fls_rgjf5ibhurda, fls_s5z2q5pl14p4
//! @title: The diagnostics attributes allow, warn, deny, forbid, expect, must_use, and deprecated
//! @lean(tier): supported

#[allow(unused_variables)]
pub fn allowed() -> u32 {
    let unused = 1;
    2
}

#[warn(unused_mut)]
#[deny(unused_must_use)]
#[forbid(unsafe_code)]
pub fn checked() -> u32 {
    3
}

#[expect(unused_variables)]
pub fn expected() -> u32 {
    let unused = 1;
    4
}

#[must_use]
pub fn important() -> u32 {
    5
}

#[deprecated(note = "use important")]
pub fn old() -> u32 {
    6
}

#[hax_test::equiv]
#[allow(deprecated)]
pub fn diagnostics_do_not_change_values() -> bool {
    allowed() + checked() + expected() + important() + old() == 20
}
