//! @fls: fls_s76un78zyd0j, fls_7fuctvtvdi7x, fls_ycy5ee6orjx, fls_6nt5kls21xes, fls_1pdzwkt5txfj
//! @title: Simple string literals with plain characters and escapes
//! @lean(tier): supported
//! @lean(reason): The non-ASCII characters of a string literal extract as decimal byte escapes, which Lean rejects (`invalid escape sequence`), so the extraction fails to elaborate; the types section of the input-language reference.

pub fn escaped() -> &'static str {
    "\x41\u{e9}\t'\"\\"
}

#[hax_test::equiv]
pub fn escapes_denote_characters() -> bool {
    escaped().as_bytes() == [0x41, 0xC3, 0xA9, 9, 39, 34, 92]
}
