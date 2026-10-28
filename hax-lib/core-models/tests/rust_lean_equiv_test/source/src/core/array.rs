//! Equivalence tests for `[T; N]` (`core::array`) operations.

use crate::helpers::Bumped;
use rust_lean_test_macro::rust_lean_test;

// ----- Index<RangeTo<usize>> -------------------------------------------------

#[rust_lean_test]
pub fn test_index_range_to_zero() -> bool {
    let a: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    a[..0] == []
}

#[rust_lean_test]
pub fn test_index_range_to_three() -> bool {
    let a: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    a[..3] == [1, 2, 3]
}

#[rust_lean_test]
pub fn test_index_range_to_full() -> bool {
    let a: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    a[..8] == [1, 2, 3, 4, 5, 6, 7, 8]
}

// ----- IndexMut --------------------------------------------------------------
//
// The write-back offsets of `<[T; N] as IndexMut<I>>::index_mut` (pereq-dev/hax#2174).

#[rust_lean_test]
pub fn test_index_mut_range_middle() -> bool {
    let mut a: [u8; 8] = [0; 8];
    a[2..5].copy_from_slice(&[7, 8, 9]);
    a == [0, 0, 7, 8, 9, 0, 0, 0]
}

#[rust_lean_test]
pub fn test_index_mut_range_from() -> bool {
    let mut a: [u8; 4] = [1, 2, 3, 4];
    a[2..].copy_from_slice(&[9, 9]);
    a == [1, 2, 9, 9]
}

#[rust_lean_test]
pub fn test_index_mut_range_to() -> bool {
    let mut a: [u8; 4] = [1, 2, 3, 4];
    a[..2].copy_from_slice(&[9, 9]);
    a == [9, 9, 3, 4]
}

#[rust_lean_test]
pub fn test_index_mut_range_full_reverse() -> bool {
    let mut a: [u8; 4] = [1, 2, 3, 4];
    a[..].reverse();
    a == [4, 3, 2, 1]
}

#[rust_lean_test]
pub fn test_index_mut_range_empty_is_noop() -> bool {
    let mut a: [u8; 4] = [1, 2, 3, 4];
    a[2..2].copy_from_slice(&[]);
    a == [1, 2, 3, 4]
}

// ----- PartialEq -------------------------------------------------------------

#[rust_lean_test]
pub fn test_eq_same() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let b: [u8; 4] = [1, 2, 3, 4];
    a == b
}

#[rust_lean_test]
pub fn test_eq_different_last() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let b: [u8; 4] = [1, 2, 3, 5];
    (a == b) == false
}

#[rust_lean_test]
pub fn test_eq_different_first() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let b: [u8; 4] = [0, 2, 3, 4];
    (a == b) == false
}

#[rust_lean_test]
pub fn test_eq_all_zeros() -> bool {
    let a: [u8; 4] = [0, 0, 0, 0];
    let b: [u8; 4] = [0, 0, 0, 0];
    a == b
}

#[rust_lean_test]
pub fn test_from_fn_identity() -> bool {
    let a: [u8; 4] = core::array::from_fn(|i| i as u8);
    a == [0, 1, 2, 3]
}

// ----- map / each_ref --------------------------------------------------------

#[rust_lean_test]
pub fn test_map_add_one() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    a.map(|x| x + 1) == [2, 3, 4, 5]
}

#[rust_lean_test]
pub fn test_each_ref_first() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let refs: [&u8; 4] = a.each_ref();
    *refs[0] == 1
}

// ----- Clone for [T; N] ------------------------------------------------------

#[rust_lean_test]
pub fn test_array_clone_applies_element_clone() -> bool {
    let a = [Bumped(1), Bumped(2)];
    let b = a.clone();
    b[0].0 == 2 && b[1].0 == 3
}

// `clone_from` clones element-wise, so the receiver's values do not survive.
#[rust_lean_test]
pub fn test_array_clone_from_applies_element_clone() -> bool {
    let mut dst = [Bumped(10), Bumped(20)];
    let src = [Bumped(1), Bumped(2)];
    dst.clone_from(&src);
    dst[0].0 == 2 && dst[1].0 == 3
}
