//! @off: lean, coq, ssprove, proverif, legacy-lean
//! @fail(tc): fstar(189)
//! F*'s `FStar.Char.char_code` excludes U+D7FF, so the F* backend rejects it.

/// @fail(extraction): fstar(HAX0001)
const BEFORE_SURROGATES: char = '\u{d7ff}';
