//! @fls: fls_s76un78zyd0j, fls_t63zfv5JdUhj, fls_moe3zfx39ox2, fls_vffxb6arj9jf, fls_3dcqhuosqb84
//! @title: Simple byte string literals with plain characters and escapes
//! @lean(tier): supported
//! @lean(reason): aeneas fails on the byte string literals (`There should be no bottoms in the value`); the types section of the input-language reference.

pub fn bytes() -> &'static [u8; 7] {
    b"a'\"\\\n\x7F\0"
}

#[hax_test::equiv]
pub fn byte_string_values() -> bool {
    *bytes() == [97, 39, 34, 92, 10, 127, 0]
}
