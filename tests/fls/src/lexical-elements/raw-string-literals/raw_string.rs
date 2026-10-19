//! @fls: fls_s76un78zyd0j, fls_7fuctvtvdi7x, fls_36suwhbwmq1t, fls_ms43w1towz40
//! @title: Raw string literals that keep backslashes and quotes
//! @lean(tier): supported
//! @lean(reason): The double quotes inside a raw string literal extract unescaped into the Lean string (`unexpected token '#'`), so the extraction fails to elaborate; the types section of the input-language reference.

pub fn unescaped() -> &'static str {
    r"a\n"
}

pub fn quoted() -> &'static str {
    r##"say "#hi"#"##
}

#[hax_test::equiv]
pub fn raw_string_values() -> bool {
    unescaped() == "a\\n" && quoted() == "say \"#hi\"#"
}
