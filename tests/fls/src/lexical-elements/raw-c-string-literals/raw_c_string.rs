//! @fls: fls_s76un78zyd0j, fls_VKCW830CzhhN, fls_gLrei65i8Uzq, fls_9nJHsg9dCi66
//! @title: A raw C string literal that keeps backslashes and quotes
//! @lean(tier): supported
//! @lean(reason): aeneas rejects the C string literals (`Found unexpected constant`); the types section of the input-language reference.

use core::ffi::CStr;

pub fn unescaped() -> &'static CStr {
    cr#"a\n""#
}

#[hax_test::equiv]
pub fn raw_c_string_bytes() -> bool {
    *unescaped().to_bytes_with_nul() == [97, 92, 110, 34, 0]
}
