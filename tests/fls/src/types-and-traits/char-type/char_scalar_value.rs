//! @fls: fls_vnwbs0exbwcn
//! @title: Equality of char values
//! @lean(tier): supported

pub fn is_newline(c: char) -> bool {
    c == '\n'
}

#[hax_test::equiv]
pub fn compared_by_scalar_value() -> bool {
    is_newline('\n') && !is_newline('n') && 'a' != 'b'
}
