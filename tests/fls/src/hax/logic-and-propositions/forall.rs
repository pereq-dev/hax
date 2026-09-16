//! @fls: hax_forall
//! @title: Universal quantification in a postcondition
//! @lean(tier): supported

#[hax_lib::ensures(|result| hax_lib::forall(|i: usize| hax_lib::implies(i < 4, result[i] == 0)))]
pub fn zeroed() -> [u8; 4] {
    [0; 4]
}

#[hax_test::equiv]
pub fn all_zero() -> bool {
    zeroed() == [0, 0, 0, 0]
}
