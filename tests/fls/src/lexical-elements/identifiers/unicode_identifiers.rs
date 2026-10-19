//! @fls: fls_xsdmun5uqy4c, fls_aqj9aguczgqs, fls_lwcflgezgs5z, fls_uts0hywaw1rq, fls_ktnf6zkrdy45, fls_vde7gev5rz4q, fls_j9yh8j8jgdeu
//! @title: Non-ASCII identifiers, a leading underscore, and identifiers equal after normalization
//! @lean(tier): supported

pub fn größe() -> u32 {
    3
}

// The binding is spelled with the precomposed `é` (U+00E9) and the use with
// `e` followed by a combining acute accent (U+0301); both normalize to the same
// identifier.
pub fn normalized() -> u32 {
    let café = 4;
    let _start = 5;
    café + _start
}

#[hax_test::equiv]
pub fn identifiers_resolve() -> bool {
    größe() + normalized() == 12
}
