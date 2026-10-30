//! @fls: fls_8tgxtprtifrr, fls_9rmq7iaf092d, fls_CutpaCFCGHQs, fls_dcNtgLq2hRZb, fls_ghqvj8q71o97, fls_h7pIl1WZ8Y2t, fls_hbrg0d98jak5, fls_kqtr9c3jorvg, fls_ohu0kljfexd3, fls_pfa81kv2mru8, fls_rFFlt33a5RsZ, fls_sxLwuITs62sN, fls_tsou6yz4mfte, fls_uo6vv2yf8usx, fls_yfK3pGHzUo3x, fls_yzt6pcsvj3a, fls_zdfgqky85r1f
//! @title: Sign negation of integers and bitwise negation of integers and booleans
//! @lean(tier): supported

pub fn negate(x: i32) -> i32 {
    -x
}

pub fn invert(x: u8) -> u8 {
    !x
}

pub fn not(b: bool) -> bool {
    !b
}

#[hax_test::equiv]
pub fn negated_values() -> bool {
    negate(5) == -5
        && negate(-5) == 5
        && invert(0b1010_1010) == 0b0101_0101
        && not(true) == false
        && not(false)
}
