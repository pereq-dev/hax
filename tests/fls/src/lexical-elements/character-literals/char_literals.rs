//! @fls: fls_s76un78zyd0j, fls_vag2oy4q7d4n, fls_n8z6p6g564r2, fls_j9q9ton57rvl, fls_5v9gx22g5wPm
//! @title: ASCII character literals with plain characters and escapes
//! @lean(tier): supported
//! @lean(reason): aeneas prints the escaped characters unescaped into Lean, so `'\''` becomes `'''` and the file does not parse (`unexpected token ''''`, AeneasVerif/aeneas#1467); the types section of the input-language reference.

pub fn plain() -> char {
    'a'
}

pub fn quote_escapes() -> (char, char, char) {
    ('\'', '"', '\\')
}

pub fn ascii_escapes() -> (char, char, char, char, char) {
    ('\n', '\r', '\t', '\0', '\x41')
}

// Each literal is compared with the Unicode escape of its code point.
#[hax_test::equiv]
pub fn character_values() -> bool {
    let (q, d, b) = quote_escapes();
    let (n, r, t, z, a) = ascii_escapes();
    plain() == '\u{61}'
        && q == '\u{27}'
        && d == '\u{22}'
        && b == '\u{5c}'
        && n == '\u{a}'
        && r == '\u{d}'
        && t == '\u{9}'
        && z == '\u{0}'
        && a == '\u{41}'
}
