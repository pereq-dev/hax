//! @off: lean, coq, ssprove, proverif, legacy-lean
//! Char literals across the UTF-8 encoding lengths.
#![allow(dead_code)]

const ASCII: [char; 6] = ['a', '\0', '\'', '"', '\n', '\x7f'];
const TWO_BYTES: [char; 4] = ['\u{80}', 'é', 'Ã', '\u{7ff}'];
const THREE_BYTES: [char; 4] = ['\u{800}', '中', '\u{e000}', '\u{ffff}'];
const FOUR_BYTES: [char; 3] = ['\u{10000}', '🦀', '\u{10ffff}'];

/// https://github.com/pereq-dev/hax/issues/2396
#[hax_lib::ensures(|r| r == false)]
fn distinct_chars() -> bool {
    'é' == 'Ã'
}

fn classify(c: char) -> u8 {
    match c {
        'a' => 0,
        'é' => 1,
        '中' => 2,
        '🦀' => 3,
        _ => 4,
    }
}
