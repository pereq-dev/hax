//! @fls: fls_1tq2o2huda9l, fls_4axr4V0icdBP, fls_4vxi1ji93dxb, fls_5gy92rsi2mqm, fls_KxWIzoh9WwK7, fls_NnGiNsH6Zgmp, fls_Sgfxp186GMvz, fls_Ufz9W5vyZkv3, fls_ZED5wJQVO6nf, fls_anzidgx02lly, fls_brwv1zwu37e8, fls_bt50fltfqcvn, fls_cPEMHZtPkctX, fls_dcm3yr3y9y0a, fls_ffjx1d5dseo4, fls_fzsrdrHnndRd, fls_giZ7w1G02JSg, fls_gv4M0DE3OMwk, fls_jLZlxIHr4w2v, fls_jpmhibm4omm7, fls_ka5b87tkf8t6, fls_ku38h562vfyl, fls_lj7x5dgbmg9i, fls_m0gbw9myylv2, fls_nman7mJVSQlm, fls_none1dykbn8c, fls_o0feajus3jtu, fls_o76QXHyrrJPG, fls_oV9Hd6OiFAZX, fls_qbrcg3cl9td, fls_qytgkbhqr5ln, fls_s4bhrpykzmm7, fls_u80htrnr2ebz, fls_wxGAOWEVT77u, fls_xm0gm2q27x2e, fls_ya05djl1d154
//! @title: Place expressions in assignment, borrow, dereference, index, field, and match contexts
//! @lean(tier): supported

pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub fn rewrite(p: &mut Point, arr: &mut [u32; 2]) -> u32 {
    p.x = 1;
    p.y += 2;
    arr[0] = p.x;
    (arr[1]) = p.y;
    let r = &mut arr[1];
    *r *= 3;
    let total = match p.x {
        1 => arr[0] + arr[1],
        _ => 0,
    };
    let borrowed = &p.y;
    total + *borrowed
}

#[hax_test::equiv]
pub fn places_are_updated() -> bool {
    let mut p = Point { x: 0, y: 0 };
    let mut arr = [0, 0];
    let temp_in_place_context = &(1 + 1);
    rewrite(&mut p, &mut arr) == 9 && p.y == 2 && arr == [1, 6] && *temp_in_place_context == 2
}
