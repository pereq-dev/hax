//! @fls: fls_2po52gv0m021, fls_5d66h7naoup6, fls_jhazc75w5vj, fls_xp9whcj2obk8
//! @title: Parenthesized expressions grouping operands
//! @lean(tier): supported

pub fn grouped(a: u32, b: u32, c: u32) -> (u32, u32) {
    ((a + b) * c, a + (b * c))
}

// rustfmt would remove the nested parentheses.
#[rustfmt::skip]
#[hax_test::equiv]
pub fn grouping_changes_the_value() -> bool {
    grouped(1, 2, 3) == (9, 7) && (((4))) == 4
}
