//! @fls: fls_rbwwczom3agt, fls_w30su9x4q13r, fls_wdpbg5xzgmwu, fls_g061yzws1m45
//! @title: Integer, boolean, character, and byte literal expressions
//! @lean(tier): supported

pub fn integer() -> u32 {
    0xFF + 0o10 + 0b11 + 1_000
}

pub fn negative() -> i64 {
    -7
}

pub fn boolean() -> bool {
    true
}

pub fn character() -> char {
    'z'
}

pub fn byte() -> u8 {
    b'a'
}

#[hax_test::equiv]
pub fn literal_values() -> bool {
    integer() == 1266 && negative() == -7 && boolean() && character() == 'z' && byte() == 97
}
