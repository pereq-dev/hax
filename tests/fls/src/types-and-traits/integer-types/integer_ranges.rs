//! @fls: fls_p2shoji3xg5a, fls_fsyt05u9y4sl, fls_p9ffvtajr832, fls_j6xan9f8udw7, fls_4t39p3ibkzu7, fls_egfoxke0lzje, fls_4c4qpel1tbqs, fls_cokwseo3nnr, fls_vk1skn6ek36u, fls_iikexw8ps6mk, fls_cavasxxlgs7g, fls_7sx92xsjx3pl, fls_q9f95uet7gq4, fls_yjb3kzijd19v, fls_t9oyfmgqka6u, fls_75lntwhg20l
//! @title: The value ranges of the signed and unsigned integer types
//! @lean(tier): supported

#[hax_test::equiv]
pub fn signed_ranges() -> bool {
    i8::MIN == -128
        && i8::MAX == 127
        && i16::MIN == -32768
        && i16::MAX == 32767
        && i32::MIN == -2147483648
        && i32::MAX == 2147483647
        && i64::MIN == -9223372036854775808
        && i64::MAX == 9223372036854775807
        && i128::MIN == -170141183460469231731687303715884105728
        && i128::MAX == 170141183460469231731687303715884105727
}

#[hax_test::equiv]
pub fn unsigned_ranges() -> bool {
    u8::MIN == 0
        && u8::MAX == 255
        && u16::MAX == 65535
        && u32::MAX == 4294967295
        && u64::MAX == 18446744073709551615
        && u128::MAX == 340282366920938463463374607431768211455
}

#[hax_test::equiv]
pub fn pointer_sized_integers() -> bool {
    let bits: u32 = usize::BITS;
    bits >= 16 && isize::BITS == bits && (usize::MAX as u128) == (1u128 << bits) - 1
}
