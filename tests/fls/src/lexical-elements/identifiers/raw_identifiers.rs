//! @fls: fls_xsdmun5uqy4c, fls_ls7ymvgd5kfa, fls_bsh7qsyvox21, fls_w4b97ewwnql
//! @title: Raw identifiers spelling a strict and a reserved keyword
//! @lean(tier): supported

pub fn r#match(r#type: u32) -> u32 {
    r#type + 1
}

pub fn r#abstract() -> u32 {
    2
}

#[hax_test::equiv]
pub fn raw_identifiers_name_items() -> bool {
    r#match(1) + r#abstract() == 4
}
