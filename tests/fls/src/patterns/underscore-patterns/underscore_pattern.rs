//! @fls: fls_dreny9e0ei6r, fls_b87mvrcc13f2, fls_42fye1v0th8l, fls_j3u6x1ensrbe, fls_dvk7r1gf7pwp, fls_e0uprihqn1y6, fls_ljcq2vyo052q, fls_uxysntb3u03j
//! @title: Underscore patterns as wildcards in let statements and match arms
//! @lean(tier): supported

pub fn second(pair: (u32, u32)) -> u32 {
    let (_, b) = pair;
    b
}

pub fn is_some(x: Option<u32>) -> bool {
    match x {
        Some(_) => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn wildcards_match_anything() -> bool {
    let _ = second((1, 2));
    second((5, 6)) == 6 && is_some(Some(0)) && !is_some(None)
}
