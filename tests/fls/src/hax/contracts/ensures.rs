//! @fls: hax_ensures
//! @title: A postcondition stated with ensures over the result
//! @lean(tier): supported

#[hax_lib::ensures(|result| result >= x && result >= y)]
pub fn max(x: u32, y: u32) -> u32 {
    if x > y { x } else { y }
}

#[hax_test::equiv]
pub fn postcondition_holds() -> bool {
    max(2, 5) == 5 && max(7, 1) == 7
}
