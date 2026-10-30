//! @fls: fls_s76un78zyd0j, fls_vkk2krfn93ry, fls_f1e29aj0sqvl, fls_u83mffscqm6, fls_hpkkvuj1z1ez, fls_xrv4q56lmoo3, fls_66e3q5um6cwc, fls_5asyk66y7c9d, fls_76fifqjka0lx, fls_fsaimo419gf0, fls_hvzacbu7yiwc, fls_7yq2fep848ky, fls_bzm8lwq3qlat, fls_l4cx36brc1r5, fls_wthchinwx996, fls_7uoaet2pm3am, fls_p4rw583o2qbi
//! @title: Integer literals with every integer suffix
//! @lean(tier): supported

pub fn unsigned() -> (u8, u16, u32, u64, u128, usize) {
    (255u8, 0xFFFF_u16, 7u32, 1_u64, 0b11u128, 0o10usize)
}

pub fn signed() -> (i8, i16, i32, i64, i128, isize) {
    (-128i8, 300i16, -7i32, 1_i64, 0x10i128, -2isize)
}

#[hax_test::equiv]
pub fn suffixed_values() -> bool {
    let (a, b, c, d, e, f) = unsigned();
    let (g, h, i, j, k, l) = signed();
    a == 255
        && b == 65535
        && c == 7
        && d == 1
        && e == 3
        && f == 8
        && g == -128
        && h == 300
        && i == -7
        && j == 1
        && k == 16
        && l == -2
}
