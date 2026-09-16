//! @fls: hax_ensures_ref
//! @title: A postcondition stated with ensures_ref, binding the result by reference
//! @lean(tier): supported

#[hax_lib::ensures_ref(|result| result[0] == x)]
pub fn pair(x: u32) -> [u32; 2] {
    [x, x + 1]
}

#[hax_test::equiv]
pub fn postcondition_over_a_reference() -> bool {
    pair(4) == [4, 5]
}
