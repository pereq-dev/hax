//! @fls: fls_s76un78zyd0j, fls_t63zfv5JdUhj, fls_yyw7nv651580, fls_5ybq0euwya42
//! @title: Raw byte string literals that keep backslashes and quotes
//! @lean(tier): supported
//! @lean(reason): aeneas fails on the byte string literals (`There should be no bottoms in the value`); the types section of the input-language reference.

pub fn unescaped() -> &'static [u8; 3] {
    br"a\n"
}

pub fn quoted() -> &'static [u8; 3] {
    br#"""\"#
}

#[hax_test::equiv]
pub fn raw_byte_string_values() -> bool {
    *unescaped() == [97, 92, 110] && *quoted() == [34, 34, 92]
}
