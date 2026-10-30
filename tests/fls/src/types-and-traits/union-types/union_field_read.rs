//! @fls: fls_nskmnzq95yqm
//! @title: Reading a union field
//! @lean(tier): unsupported
//! @lean(reason): The value-model boundary of the input-language reference: overlapping union fields have no single value representation, and every read is unsafe.

pub union IntOrBytes {
    pub n: u32,
    pub bytes: [u8; 4],
}

pub fn low_byte(n: u32) -> u8 {
    let u = IntOrBytes { n };
    unsafe { u.bytes[0] }
}

#[hax_test::equiv]
pub fn low_byte_of_one() -> bool {
    low_byte(1) == 1 || low_byte(1) == 0
}
