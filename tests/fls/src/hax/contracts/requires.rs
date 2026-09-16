//! @fls: hax_requires
//! @title: A precondition stated with requires
//! @lean(tier): supported

#[hax_lib::requires(divisor != 0)]
pub fn divide(dividend: u32, divisor: u32) -> u32 {
    dividend / divisor
}

#[hax_test::equiv]
pub fn precondition_holds_at_the_call() -> bool {
    divide(9, 3) == 3
}
