//! @fls: fls_dti0uu7rz81w, fls_sxg1o4oxql51, fls_bsh7qsyvox21
//! @title: Capitalized spellings of keywords as ordinary identifiers
//! @lean(tier): supported

#[allow(non_snake_case)]
pub fn capitalized() -> u32 {
    let Loop = 1;
    let Match = 2;
    let While = 3;
    Loop + Match + While
}

#[hax_test::equiv]
pub fn keywords_are_case_sensitive() -> bool {
    capitalized() == 6
}
