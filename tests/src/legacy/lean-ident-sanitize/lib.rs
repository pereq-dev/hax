//! Rust identifiers that collide with a Lean keyword (`structure`, `theorem`,
//! `deriving`, `def`) are `_`-prefixed for the legacy-lean backend; they are
//! legal elsewhere and extract unchanged.

pub fn structure() -> u8 {
    0
}

pub fn theorem() -> u8 {
    1
}

pub fn deriving() -> u8 {
    2
}

pub fn def() -> u8 {
    3
}
