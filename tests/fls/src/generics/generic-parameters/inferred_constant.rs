//! @fls: fls_reASCId0i117, fls_TvuMMQnR0drL
//! @title: A constant argument left to inference with an underscore
//! @lean(tier): supported

pub struct Buffer<const N: usize> {
    pub items: [u32; N],
}

pub fn filled<const N: usize>(value: u32) -> Buffer<N> {
    Buffer { items: [value; N] }
}

pub fn inferred() -> Buffer<3> {
    filled::<_>(7)
}

#[allow(unused_parens)]
pub fn parenthesized() -> Buffer<2> {
    filled::<((_))>(9)
}

#[hax_test::equiv]
pub fn inferred_constant_argument() -> bool {
    inferred().items == [7, 7, 7] && parenthesized().items == [9, 9]
}
