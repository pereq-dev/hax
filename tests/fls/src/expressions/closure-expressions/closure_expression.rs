//! @fls: fls_2d141c9a0yui, fls_7w15ccc1zzxl, fls_81KOEXwps2HS, fls_UgJgur0z6d4a, fls_XWJ9SFggdVeH, fls_af1WL2mBKMfW, fls_c3rzwUxjmBMY, fls_f59fw17gsasn, fls_fg8lx0yyt6oq, fls_mPWkIxTJErqx, fls_oey0ivaiu1l, fls_sje6cdvifgv5, fls_srbl7ptknjyk, fls_wLVeE6cNG8oa, fls_yn30xuejcfxo, fls_zsaTK9snhXs0
//! @title: Closure expressions with inferred and annotated parameters and return types
//! @lean(tier): supported

pub fn apply<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
    f(x)
}

pub fn build(offset: u32) -> u32 {
    let inferred = |x| x + offset;
    let annotated = |x: u32| -> u32 {
        let y = x * 2;
        y + 1
    };
    let (a, b) = (1u32, 2u32);
    let destructuring = |(p, q): (u32, u32)| p + q;
    apply(inferred, 1) + apply(annotated, 2) + destructuring((a, b))
}

#[hax_test::equiv]
pub fn closure_values() -> bool {
    build(10) == 19
}
