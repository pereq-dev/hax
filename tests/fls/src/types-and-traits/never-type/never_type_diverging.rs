//! @fls: fls_4u0v5uy95pyf, fls_xmtc10qzw0ui
//! @title: A diverging function of the never type in a match arm
//! @lean(tier): supported
//! @lean(reason): aeneas fails to evaluate the call of a function returning `!` in a match arm (`evaluate_function_symbolic failed ... Unexpected result: Cps.Unit`); the types section of the input-language reference.

pub fn diverge() -> ! {
    panic!("unreachable")
}

pub fn checked(x: Option<u32>) -> u32 {
    match x {
        Some(v) => v,
        None => diverge(),
    }
}

#[hax_test::equiv]
pub fn present_value() -> bool {
    checked(Some(8)) == 8
}
