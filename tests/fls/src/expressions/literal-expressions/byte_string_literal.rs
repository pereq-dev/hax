//! @fls: fls_rbwwczom3agt, fls_w30su9x4q13r, fls_wdpbg5xzgmwu
//! @title: Byte string literal expressions
//! @lean(tier): supported

pub const MAGIC: &[u8; 2] = b"hi";

pub fn first_byte() -> u8 {
    MAGIC[0]
}

#[hax_test::equiv]
pub fn byte_string_bytes() -> bool {
    first_byte() == 104 && MAGIC[1] == 105
}
