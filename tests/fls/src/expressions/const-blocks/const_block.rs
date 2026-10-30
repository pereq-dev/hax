//! @fls: fls_0lcunL4bo8ka, fls_2i7TD7VoQk4B, fls_veEGzEbpT4ny, fls_wuwb0SnpP6Zu
//! @title: A const block expression evaluated statically
//! @lean(tier): supported

pub fn bits() -> u32 {
    let shifted = const { 1u32 << 4 };
    let bytes: u32 = const { u32::BITS / 8 };
    shifted + bytes
}

#[hax_test::equiv]
pub fn const_block_values() -> bool {
    bits() == 20
}
