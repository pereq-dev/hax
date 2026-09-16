//! @fls: fls_1y2ttb466m9c, fls_33hfay24hx8u, fls_6j8cr7d5zs1l, fls_8HLaLAIZgYfs, fls_O6WYL8AUyPje, fls_aCxt2Ovmb5He, fls_e70ywb8191h, fls_eca6tl7j0afx, fls_iamnzlm430ef, fls_ip81lt2mm940, fls_l8e98pyhm08g, fls_lu779ufqhggl, fls_t695ps4lfh6z, fls_uqy5w9uc8gla, fls_wmxsd0i2yemf, fls_wvob7114tfat, fls_xTNFfkxHm5yy, fls_y9n1i4hbq8sf
//! @title: Closures capturing by immutable reference, by mutable reference, and by value
//! @lean(tier): supported

pub fn capture_modes() -> [u32; 3] {
    let shared = 5;
    let by_ref = || shared + 1;
    let a = by_ref();
    let mut counter = 0;
    let mut by_mut = || counter += 10;
    by_mut();
    by_mut();
    let owned = [1u32, 2, 3];
    let by_value = move || owned[1] + owned[2];
    let c = by_value();
    [a, counter, c]
}

#[hax_test::equiv]
pub fn captured_values() -> bool {
    capture_modes() == [6, 20, 5]
}
