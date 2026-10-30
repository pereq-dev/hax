//! @fls: fls_estqu395zxgk, fls_7zh6ziglo5iy, fls_fftdnwe22xrb, fls_dl763ssb54q1, fls_kdxe1ukmgl1, fls_or125cqtxg9j
//! @title: Expression, empty, item, and macro statements in a block
//! @lean(tier): supported

macro_rules! bump {
    ($x:ident) => {
        $x += 1;
    };
}

// rustfmt would drop the empty statement.
#[rustfmt::skip]
pub fn run(start: u32) -> u32 {
    let mut n = start;
    n + 1;
    ;
    fn helper(v: u32) -> u32 {
        v * 2
    }
    bump!(n);
    helper(n)
}

#[hax_test::equiv]
pub fn statements_execute_in_order() -> bool {
    run(3) == 8
}
