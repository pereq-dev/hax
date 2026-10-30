//! @fls: fls_s76un78zyd0j, fls_q0qwr83frszx, fls_fggytrv5jvw0, fls_3hpzf12h60u4
//! @title: Byte literals with plain characters and escapes
//! @lean(tier): supported

pub fn plain() -> (u8, u8) {
    (b'a', b'"')
}

pub fn escapes() -> (u8, u8, u8, u8, u8) {
    (b'\'', b'\\', b'\n', b'\0', b'\x7F')
}

#[hax_test::equiv]
pub fn byte_values() -> bool {
    let (a, d) = plain();
    let (q, b, n, z, x) = escapes();
    a == 97 && d == 34 && q == 39 && b == 92 && n == 10 && z == 0 && x == 127
}
