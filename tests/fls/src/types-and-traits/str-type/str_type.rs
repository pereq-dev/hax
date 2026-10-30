//! @fls: fls_wlnoq1qoq2kr, fls_1xa6fas6laha, fls_emcNJzl2tHSA
//! @title: A str slice, its byte length, and its equality
//! @lean(tier): supported
//! @lean(reason): The Lean core model has neither `str::len` nor `PartialEq for str` (`Unknown identifier core.str.Str.len`, `core.Str.Insts.CoreCmpPartialEqStr.eq`); the model-trust section of the input-language reference.

pub fn byte_len(s: &str) -> usize {
    s.len()
}

#[hax_test::equiv]
pub fn ascii_length() -> bool {
    byte_len("hello") == 5 && byte_len("") == 0
}

#[hax_test::equiv]
pub fn str_equality() -> bool {
    let s: &str = "ab";
    s == "ab" && s != "ba"
}
