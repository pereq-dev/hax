//! @fls: hax_decreases
//! @title: A termination measure for a recursive function stated with decreases
//! @lean(tier): supported

#[hax_lib::decreases(n)]
pub fn sum_to(n: u32) -> u32 {
    if n == 0 { 0 } else { n + sum_to(n - 1) }
}

#[hax_test::equiv]
pub fn recursion_terminates() -> bool {
    sum_to(4) == 10
}
