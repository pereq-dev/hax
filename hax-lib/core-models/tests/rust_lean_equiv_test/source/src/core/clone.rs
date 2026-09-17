//! Equivalence tests for `core::clone::Clone`.
//!
//! Mirrors the proptest block in `core-models/src/core/clone.rs`, which
//! exercises `Clone for u8` — the identity clone. We extend the same
//! observation to a handful of other integer widths so each `clone_impl`
//! macro instantiation gets hit on at least one corner value.

// ----- u8 --------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_clone_u8_zero() -> bool {
    let x: u8 = 0;
    x.clone() == x
}

#[hax_test::equiv]
pub fn test_clone_u8_max() -> bool {
    let x: u8 = u8::MAX;
    x.clone() == x
}

#[hax_test::equiv]
pub fn test_clone_u8_mid() -> bool {
    let x: u8 = 42;
    x.clone() == x
}

// ----- u16 -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_clone_u16_zero() -> bool {
    let x: u16 = 0;
    x.clone() == x
}

#[hax_test::equiv]
pub fn test_clone_u16_max() -> bool {
    let x: u16 = u16::MAX;
    x.clone() == x
}

// ----- u32 -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_clone_u32_zero() -> bool {
    let x: u32 = 0;
    x.clone() == x
}

#[hax_test::equiv]
pub fn test_clone_u32_max() -> bool {
    let x: u32 = u32::MAX;
    x.clone() == x
}

// ----- u64 -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_clone_u64_zero() -> bool {
    let x: u64 = 0;
    x.clone() == x
}

#[hax_test::equiv]
pub fn test_clone_u64_max() -> bool {
    let x: u64 = u64::MAX;
    x.clone() == x
}

// ----- usize -----------------------------------------------------------------

#[hax_test::equiv]
pub fn test_clone_usize_zero() -> bool {
    let x: usize = 0;
    x.clone() == x
}

// ----- i8 --------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_clone_i8_zero() -> bool {
    let x: i8 = 0;
    x.clone() == x
}

#[hax_test::equiv]
pub fn test_clone_i8_max() -> bool {
    let x: i8 = i8::MAX;
    x.clone() == x
}

#[hax_test::equiv]
pub fn test_clone_i8_min() -> bool {
    let x: i8 = i8::MIN;
    x.clone() == x
}

#[hax_test::equiv]
pub fn test_clone_i8_neg_one() -> bool {
    let x: i8 = -1;
    x.clone() == x
}

// ----- i16 -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_clone_i16_zero() -> bool {
    let x: i16 = 0;
    x.clone() == x
}

#[hax_test::equiv]
pub fn test_clone_i16_min() -> bool {
    let x: i16 = i16::MIN;
    x.clone() == x
}

// ----- i32 -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_clone_i32_zero() -> bool {
    let x: i32 = 0;
    x.clone() == x
}

#[hax_test::equiv]
pub fn test_clone_i32_min() -> bool {
    let x: i32 = i32::MIN;
    x.clone() == x
}

// ----- i64 -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_clone_i64_zero() -> bool {
    let x: i64 = 0;
    x.clone() == x
}

#[hax_test::equiv]
pub fn test_clone_i64_min() -> bool {
    let x: i64 = i64::MIN;
    x.clone() == x
}

// ----- bool ------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_clone_bool_true() -> bool {
    let x: bool = true;
    x.clone() == x
}

#[hax_test::equiv]
pub fn test_clone_bool_false() -> bool {
    let x: bool = false;
    x.clone() == x
}
