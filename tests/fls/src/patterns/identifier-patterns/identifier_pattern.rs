//! @fls: fls_uljdw9rf7ies, fls_hqwt3fvr063y, fls_vy9uw586wy0d, fls_joIQdDn44oIT, fls_7oioaitb075g, fls_2ahkrddxwj1n, fls_40qin0ss5sqd, fls_pivz0v7ey6sw, fls_eucnafj3uedy, fls_exo8asevh5x1, fls_f8zo4scodhcr, fls_d3fs2h7oqjl0, fls_hw26hy33guk5, fls_svfxwz4yy5i, fls_r2mb8v2lh3x0, fls_x6f6q22b5jpc, fls_24c95c56tugl, fls_sfyfdxhvhk44, fls_as0pqqmo1des, fls_wauqwmdbcpna, fls_3jyog8n6x2aa, fls_w637uvlbzsyo, fls_arz8ik3gf6u4, fls_u6o5ndnezwbe, fls_h1er04t0yta7, fls_62626ws222op
//! @title: Identifier patterns with mut, ref, ref mut, and a bound pattern
//! @lean(tier): supported

pub fn bucket(n: u32) -> u32 {
    match n {
        small @ 0..=9 => small,
        medium @ (10 | 20 | 30) => medium / 10,
        large => large / 100,
    }
}

pub fn by_reference(pair: (u32, u32)) -> u32 {
    let (ref a, ref b) = pair;
    *a + *b
}

pub fn by_mutable_reference(mut pair: (u32, u32)) -> (u32, u32) {
    let (ref mut a, _) = pair;
    *a += 10;
    pair
}

#[hax_test::equiv]
pub fn bindings() -> bool {
    let x = 5;
    let mut y = x;
    y += 1;
    bucket(3) == 3 && bucket(20) == 2 && bucket(700) == 7 && y == 6
}

#[hax_test::equiv]
pub fn reference_bindings() -> bool {
    by_reference((2, 3)) == 5 && by_mutable_reference((1, 2)) == (11, 2)
}
