//! @fls: fls_eso51epfofxb, fls_rsqyza99vl3x, fls_qz9guhlg19j3
//! @title: Rest patterns in tuple patterns
//! @lean(tier): supported

pub fn ends(t: (u32, u32, u32, u32)) -> u32 {
    let (first, .., last) = t;
    first + last
}

pub fn middle(t: (u32, u32, u32)) -> u32 {
    let (.., m, _) = t;
    m
}

#[hax_test::equiv]
pub fn rest_matches_the_remainder() -> bool {
    ends((1, 2, 3, 4)) == 5 && middle((1, 2, 3)) == 2
}
