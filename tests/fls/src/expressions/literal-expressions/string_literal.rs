//! @fls: fls_rbwwczom3agt, fls_w30su9x4q13r, fls_wdpbg5xzgmwu
//! @title: String literal expressions
//! @lean(tier): supported
//! @lean(reason): `==` on `&str` extracts to `core.Str.Insts.CoreCmpPartialEqStr.eq`, which the Lean library does not define, so the extraction fails to elaborate; the types section of the input-language reference.

pub fn text() -> &'static str {
    "text"
}

pub fn is_text(s: &str) -> bool {
    s == "text"
}

#[hax_test::equiv]
pub fn string_literal_value() -> bool {
    is_text(text()) && !is_text("other")
}
