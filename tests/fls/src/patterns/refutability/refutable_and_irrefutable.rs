//! @fls: fls_9ntc4qmjmo90, fls_9fjspnefoyvz, fls_uq7ftuuq1sig, fls_sgu9bnp7xajv, fls_cl1g4fxfa020, fls_l76ycteulo8e, fls_lh0d85tl4qvy
//! @title: Irrefutable patterns in let statements and refutable patterns in match arms
//! @lean(tier): supported

pub const ONLY: () = ();
pub const LIMIT: u32 = 10;

pub fn unwrap_or_zero(value: Option<i32>) -> i32 {
    match value {
        Some(y) => y,
        None => 0,
    }
}

pub fn at_limit(n: u32) -> bool {
    match n {
        LIMIT => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn irrefutable_bindings() -> bool {
    let x: i32 = 42;
    let (a, b) = (1u32, 2u32);
    let ONLY = ();
    x == 42 && a + b == 3
}

#[hax_test::equiv]
pub fn refutable_arms() -> bool {
    unwrap_or_zero(Some(5)) == 5 && unwrap_or_zero(None) == 0 && at_limit(10) && !at_limit(9)
}
