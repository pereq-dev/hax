//! @fls: fls_s7bf7tf9206d, fls_930o6urn669w, fls_z7ufiqqujgdh, fls_f0n4g5uky9tp, fls_r3p4din7rjz8, fls_tvn08dtuilue, fls_92tqo8uas8kd, fls_bxucstrfcco8, fls_xk7lb2g02sy7, fls_n3737i320qum
//! @title: The inline attribute with and without a hint
//! @lean(tier): supported

#[inline]
pub fn plain(x: u32) -> u32 {
    x + 1
}

#[inline(always)]
pub fn always(x: u32) -> u32 {
    x * 2
}

#[inline(never)]
pub fn never(x: u32) -> u32 {
    x - 1
}

#[hax_test::equiv]
pub fn inlining_is_transparent() -> bool {
    never(always(plain(4))) == 9
}
