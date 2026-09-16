//! @fls: fls_vnwbs0exbwcn
//! @title: Ordering of char values by their scalar value
//! @lean(tier): supported
//! @lean(reason): aeneas rejects the ordering operators on `char` (`Invalid inputs for binop`); the types section of the input-language reference.

pub fn is_upper_ascii(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

#[hax_test::equiv]
pub fn ordered_by_code_point() -> bool {
    is_upper_ascii('Q') && !is_upper_ascii('q') && 'a' < 'b'
}
