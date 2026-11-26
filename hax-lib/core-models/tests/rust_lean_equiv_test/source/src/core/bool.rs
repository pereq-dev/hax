//! Equivalence tests for `bool::then` and `bool::then_some`.

use rust_lean_test_macro::rust_lean_test;

#[rust_lean_test]
pub fn test_then_some() -> bool {
    true.then_some(3u8) == Some(3) && false.then_some(3u8).is_none()
}

#[rust_lean_test]
pub fn test_then() -> bool {
    true.then(|| 3u8) == Some(3) && false.then(|| 3u8).is_none()
}
