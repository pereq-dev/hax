//! @fls: fls_pah15qa54irs, fls_COQKJC0dvtNO, fls_co60bzvwashg, fls_fqclaznjgtb1, fls_fqkhhgushje9, fls_m01eo9sa55s, fls_294jtwbfq3p9, fls_q0z46h1gnzez
//! @title: Literal patterns of integer and boolean type
//! @lean(tier): supported

pub fn name_of(n: i32) -> u8 {
    match n {
        0 => 2,
        1 => 3,
        _ => 0,
    }
}

pub fn flip(b: bool) -> u8 {
    match b {
        true => 0,
        false => 1,
    }
}

#[hax_test::equiv]
pub fn literals_match_equal_values() -> bool {
    name_of(0) == 2 && name_of(1) == 3 && name_of(9) == 0 && flip(true) == 0 && flip(false) == 1
}
