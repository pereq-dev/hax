//! @fls: fls_13hmhzqz82v6, fls_1i7hm645h7ox, fls_23uluvhhoct6, fls_5x0du3u1jwd3, fls_P8iyTN6KZCVA, fls_ajby242tnu7c, fls_j08k3brdpgno, fls_k7g4cac93617, fls_s6hRa5spz64w
//! @title: While loops driven by an iteration expression
//! @lean(tier): supported

pub fn count_bits(mut x: u32) -> u32 {
    let mut n = 0;
    while x != 0 {
        n += x & 1;
        x >>= 1;
    }
    n
}

#[hax_test::equiv]
pub fn iteration_until_false() -> bool {
    count_bits(0b1011) == 3 && count_bits(0) == 0
}
