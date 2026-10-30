//! @fail(extraction): proverif(HAX0001)
#![allow(dead_code)]

pub fn f(n: u8) -> u8 {
    if n == 0 {
        0
    } else {
        n + f(n - 1)
    }
}
