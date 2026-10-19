//! @fls: fls_s76un78zyd0j, fls_1lll64ftupjd, fls_pgngble3ilyx
//! @title: The boolean literals true and false
//! @lean(tier): supported

pub fn both() -> (bool, bool) {
    (true, false)
}

#[hax_test::equiv]
pub fn boolean_values() -> bool {
    let (t, f) = both();
    t && !f
}
