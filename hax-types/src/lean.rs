/// Convert a crate name to CamelCase for Lean: split on `-` and `_`,
/// upper-case the first character of each segment, and concatenate. Not
/// injective: `my-crate`, `my_crate` and `my_Crate` all yield `MyCrate`.
pub fn to_camel_case(name: &str) -> String {
    name.split(['-', '_'])
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().to_string() + c.as_str(),
            }
        })
        .collect()
}

/// The source repository of the aeneas Lean proof library: the repository
/// the managed aeneas binaries are built from, so a resolved aeneas
/// version doubles as the library's rev.
pub const AENEAS_REPO: &str = "https://github.com/pereq-dev/aeneas";

/// The `[[require]]` block of a lakefile for the aeneas Lean library at `rev`.
pub fn aeneas_require(rev: &str) -> String {
    let rev = toml_basic_string(rev);
    format!(
        "[[require]]\nname = \"aeneas\"\ngit = \"{AENEAS_REPO}\"\nrev = {rev}\nsubDir = \"backends/lean\"\n"
    )
}

/// `s` as a TOML basic string.
pub fn toml_basic_string(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out += "\\\"",
            '\\' => out += "\\\\",
            c if c.is_control() => out += &format!("\\u{:04X}", c as u32),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn aeneas_require_quotes_the_rev() {
        let block = super::aeneas_require("a\"b\\c\n");
        assert!(block.contains("rev = \"a\\\"b\\\\c\\u000A\"\n"), "{block}");
    }
}
