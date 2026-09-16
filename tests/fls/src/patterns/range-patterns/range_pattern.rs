//! @fls: fls_okupyoav13rm, fls_vrpr6ttpfpal, fls_nk48gregn3me, fls_83v1xqbebs58, fls_q86j23iiqv8w, fls_3PyquOKjA7SI, fls_jhchm7dy927k, fls_3js1645tgh31, fls_8Q6NfRx4j5V7, fls_rgr7t33s0m7m, fls_4o4ge6x9a8rs, fls_3wwpq8i6mo2a, fls_mrh9vfdek5fi, fls_6kgj2fjccoig, fls_EDL1Pi56KQ2H, fls_7nxkgls0a5os, fls_n4t3xah1pk7i, fls_eppmiloh7bgg
//! @title: Inclusive, exclusive, and half-open range patterns
//! @lean(tier): supported

pub const HIGH: u8 = 200;

pub fn grade(score: u8) -> u8 {
    match score {
        0..50 => 0,
        50..=89 => 1,
        90..=99 => 2,
        100..=HIGH => 3,
        201.. => 4,
    }
}

pub fn sign(x: i8) -> i8 {
    match x {
        i8::MIN..=-1 => -1,
        0 => 0,
        1..=i8::MAX => 1,
    }
}

#[hax_test::equiv]
pub fn ranges_match_bounds() -> bool {
    grade(0) == 0
        && grade(49) == 0
        && grade(50) == 1
        && grade(89) == 1
        && grade(90) == 2
        && grade(200) == 3
        && grade(255) == 4
}

#[hax_test::equiv]
pub fn full_coverage_without_wildcard() -> bool {
    sign(-7) == -1 && sign(0) == 0 && sign(7) == 1
}
