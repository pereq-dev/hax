//! @fls: fls_ltioqbhl14g0, fls_eb00s8fxlvjb, fls_qk5trk8wkvxb, fls_pcromhosmnf0
//! @title: Casts from char to an integer and from u8 to char
//! @lean(tier): supported
//! @lean(reason): aeneas rejects the casts between `char` and integers (`Unreachable`); the arithmetic section of the input-language reference.

pub fn code_point(c: char) -> u32 {
    c as u32
}

pub fn from_byte(b: u8) -> char {
    b as char
}

#[hax_test::equiv]
pub fn char_conversions() -> bool {
    code_point('A') == 65 && from_byte(66) == 'B' && ('a' as u8) == 97
}
