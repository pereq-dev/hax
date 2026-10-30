//! @fls: fls_s76un78zyd0j, fls_fqpqnku27v99, fls_vkk2krfn93ry, fls_rn8xfd66yvst, fls_2268lchxkzjp, fls_4v7awnutbpoe, fls_nxqncu5yq4eu
//! @title: Decimal, hexadecimal, octal, and binary integer literals with underscores
//! @lean(tier): supported

pub fn decimal() -> u32 {
    1_000
}

pub fn hexadecimal() -> u32 {
    0xDead_Beef
}

pub fn octal() -> u32 {
    0o7_55
}

pub fn binary() -> u32 {
    0b1010_0101
}

#[hax_test::equiv]
pub fn radix_values() -> bool {
    decimal() == 1000 && hexadecimal() == 3735928559 && octal() == 493 && binary() == 165
}
