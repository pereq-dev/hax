//! @fls: fls_2kkpr955i4lm, fls_303r0u6ug215, fls_3136k1y6x3cu, fls_3zd59yuywz6l, fls_4f24nyx0ix0j, fls_4gxj18t6cnzq, fls_7p64lgnjxylz, fls_8tb22c6zbp3, fls_8trozue35xe4, fls_caxn774ij8lk, fls_d456ummq6vrk, fls_dimu987fw4kg, fls_f0p70y92k14f, fls_f4o8xlu67okn, fls_f6mmva3lbj1i, fls_fbazfgd5m1ot, fls_gf9tyu1idpjk, fls_gurl2ve58drz, fls_htw2tpujktwt, fls_i9iqtobheivu, fls_ieh1itrkcnf6, fls_j7ujcuthga1i, fls_kchprk9z6xun, fls_kp747xqekyrr, fls_kqntxbwnc58v, fls_m0pdk78dah6n, fls_m2hsk41hwm2j, fls_n269ufyesndz, fls_osfse0t6ua8a, fls_p9rlmjhbnbao, fls_r02OGonXp93A, fls_s6hkt5fg598y, fls_t709sl4co3al, fls_u5irwqswbsvu, fls_vprp53kv64q6, fls_yq8rtwfp3nv0
//! @title: Bit and, or, xor, and the shift operators on integers
//! @lean(tier): supported

pub fn ops(a: u8, b: u8) -> [u8; 3] {
    [a & b, a | b, a ^ b]
}

pub fn shifts(x: u16, n: u32) -> [u16; 2] {
    [x << n, x >> n]
}

pub fn signed_shift(x: i16) -> i16 {
    x >> 2
}

#[hax_test::equiv]
pub fn bit_results() -> bool {
    ops(0b1100, 0b1010) == [0b1000, 0b1110, 0b0110]
        && shifts(0b1011, 2) == [0b101100, 0b10]
        && signed_shift(-16) == -4
}
