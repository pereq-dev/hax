//! @fls: fls_s76un78zyd0j, fls_7fuctvtvdi7x, fls_ycy5ee6orjx, fls_wawtu6j3fiqn
//! @title: A string continuation that drops the line break and the leading whitespace
//! @lean(tier): supported
//! @lean(reason): `==` on `&str` extracts to `core.Str.Insts.CoreCmpPartialEqStr.eq`, which the Lean library does not define, so the extraction fails to elaborate; the types section of the input-language reference.

pub fn continued() -> &'static str {
    "ab\
     cd"
}

#[hax_test::equiv]
pub fn continuation_joins_lines() -> bool {
    continued() == "abcd"
}
