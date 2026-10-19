//! @fls: fls_s76un78zyd0j, fls_vag2oy4q7d4n, fls_n8z6p6g564r2, fls_j9q9ton57rvl, fls_5v9gx22g5wPm
//! @title: Non-ASCII character literals and Unicode escapes
//! @lean(tier): supported
//! @lean(reason): aeneas rejects the non-ASCII character literals (`Float, string, non-ASCII chars and byte string literals are unsupported`); the types section of the input-language reference.

pub fn plain() -> char {
    'é'
}

pub fn unicode_escapes() -> (char, char) {
    ('\u{e9}', '\u{1F600}')
}

#[hax_test::equiv]
pub fn unicode_values() -> bool {
    let (e, s) = unicode_escapes();
    plain() as u32 == 0xE9 && e == plain() && s as u32 == 0x1F600
}
