//! @fls: fls_4wDpAHhnWZAB, fls_cmBVodJMjZi7
//! @title: A call of a function returning the never type as a diverging expression
//! @lean(tier): supported
//! @lean(reason): aeneas fails to evaluate the call of a function returning `!` in a match arm (`evaluate_function_symbolic failed ... Unexpected result: Cps.Unit`); the types section of the input-language reference.

pub fn halt() -> ! {
    panic!("halt")
}

pub fn value_or_halt(x: Option<u32>) -> u32 {
    match x {
        Some(v) => v,
        None => halt(),
    }
}

#[hax_test::equiv]
pub fn present_value() -> bool {
    value_or_halt(Some(3)) == 3
}
