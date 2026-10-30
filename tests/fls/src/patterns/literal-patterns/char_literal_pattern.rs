//! @fls: fls_pah15qa54irs, fls_co60bzvwashg, fls_m01eo9sa55s, fls_294jtwbfq3p9
//! @title: Character literal patterns
//! @lean(tier): supported
//! @lean(reason): aeneas rejects a `match` on a `char` scrutinee (`Unsupported switch scrutinee`); the control flow section of the input-language reference.

pub fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn char_literals_match() -> bool {
    is_vowel('e') && !is_vowel('x')
}
