//! @fls: fls_1dvo1epstrdv
//! @title: A parenthesized type in a type ascription
//! @lean(tier): supported

#[allow(unused_parens)]
pub fn sum(pair: ((u32), u32)) -> (u32) {
    pair.0 + pair.1
}

#[hax_test::equiv]
pub fn parentheses_do_not_change_the_type() -> bool {
    sum((1, 2)) == 3
}
