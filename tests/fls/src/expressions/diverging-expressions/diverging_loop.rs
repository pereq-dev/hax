//! @fls: fls_fU91m6DCB0ip, fls_cmBVodJMjZi7, fls_rpedapxnv8w3, fls_2ulbzmuuny3g
//! @title: A loop without a break as a diverging expression
//! @lean(tier): supported
//! @lean(reason): aeneas rejects a loop without a break (`(Infinite) loops which do not contain breaks are not supported yet`); the loops section of the input-language reference.

pub fn value_or_hang(x: Option<u32>) -> u32 {
    match x {
        Some(v) => v,
        None => loop {},
    }
}

#[hax_test::equiv]
pub fn present_value() -> bool {
    value_or_hang(Some(3)) == 3
}
