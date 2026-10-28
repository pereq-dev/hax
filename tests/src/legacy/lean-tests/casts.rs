use hax_lib::*;

/// Returns true if all casting edge cases behave as expected.
#[ensures(|result| result)]
pub fn casting_edge_cases(_dummy: bool) -> bool {
    // 1. Truncation: u16 to u8 (256 -> 0)
    // 256 is 0x0100. Truncating to lower 8 bits gives 0x00.
    let case1 = (256u16 as u8) == 0;

    // 2. Truncation of negative: i16 to u8 (-1 -> 255)
    // -1 in i16 is 0xFFFF. Truncating to u8 gives 0xFF (255).
    let case2 = (-1i16 as u8) == 255;

    // 3. Sign extension: i8 to i16 (-1 -> -1)
    // -1 in i8 is 0xFF. Sign extending to i16 gives 0xFFFF (-1).
    let case3 = (-1i8 as i16) == -1;

    // 4. Reinterpretation of bits: u8 to i8 (128 -> -128)
    // 128 in u8 is 0x80. In i8 (two's complement), 0x80 is -128.
    let case4 = (128u8 as i8) == -128;

    // 5. Large u32 to i32 (0xFFFFFFFF -> -1)
    // 0xFFFFFFFF in u32. In i32 (two's complement), this is -1.
    let case5 = (0xFFFFFFFFu32 as i32) == -1;

    case1 && case2 && case3 && case4 && case5
}

/// https://github.com/pereq-dev/hax/issues/1912
pub fn shift_after_cast(x: u16, n: u8) -> u32 {
    (x as u32) << (n as u32)
}

/// https://github.com/pereq-dev/hax/issues/1911
pub fn add_after_cast(a: u8, b: u8, c: u8) -> u16 {
    (a as u16) + (b as u16) + (c as u16)
}
