//! @fls: hax_implies
//! @title: Logical implication in a precondition
//! @lean(tier): supported

#[hax_lib::requires(hax_lib::implies(strict, x > 0))]
pub fn positive_or_any(x: u32, strict: bool) -> u32 {
    if strict { x } else { x + 1 }
}

#[hax_test::equiv]
pub fn implication_holds() -> bool {
    positive_or_any(3, true) == 3 && positive_or_any(0, false) == 1
}
