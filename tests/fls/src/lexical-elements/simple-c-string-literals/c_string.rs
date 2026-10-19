//! @fls: fls_s76un78zyd0j, fls_VKCW830CzhhN, fls_nPI7j0siGP8G, fls_Ae7LM4Wg0NA7, fls_fnwQHo7twAom
//! @title: A simple C string literal with escapes and its implicit nul byte
//! @lean(tier): supported
//! @lean(reason): aeneas rejects the C string literals (`Found unexpected constant`); the types section of the input-language reference.

use core::ffi::CStr;

pub fn greeting() -> &'static CStr {
    c"hé\n\x41"
}

#[hax_test::equiv]
pub fn c_string_bytes() -> bool {
    *greeting().to_bytes_with_nul() == [104, 0xC3, 0xA9, 10, 65, 0]
}
