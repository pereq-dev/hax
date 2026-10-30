//! Equivalence tests for `alloc::slice::*` — `<[T]>::to_vec` and friends.
//!
//! The model in `alloc/src/lib.rs` exposes `to_vec` / `into_vec` on a
//! sacrificial `Dummy<T>` type because Rust forbids `impl` blocks on
//! foreign slice types. The Lean post-extraction layer
//! (`lean/CoreModels/FunsEpilogue.lean`) re-exports those bodies at the
//! std-map names `alloc::slice::{[@T]}::to_vec` and
//! `alloc::slice::{alloc::boxed::Box<[@T], @A>}::into_vec`, so calls to
//! `slice.to_vec()` and `Box<[T]>::into_vec` in this file resolve.
//!
//! We pin observations by sequential `pop()`s from the resulting `Vec`,
//! because direct `v[i]` is in `ALLOC_CHARON_EXCLUDES`.

use crate::helpers::Bumped;

// ----- [T]::to_vec -----------------------------------------------------------

#[hax_test::equiv]
pub fn test_slice_to_vec_empty_len() -> bool {
    let s: [u8; 0] = [];
    let v = s.to_vec();
    v.len() == 0 && v.is_empty()
}

#[hax_test::equiv]
pub fn test_slice_to_vec_one_len() -> bool {
    let s: [u8; 1] = [7];
    let v = s.to_vec();
    v.len() == 1
}

#[hax_test::equiv]
pub fn test_slice_to_vec_one_value() -> bool {
    let s: [u8; 1] = [7];
    let mut v = s.to_vec();
    v.pop().unwrap_or(0) == 7
}

#[hax_test::equiv]
pub fn test_slice_to_vec_three_len() -> bool {
    let s: [u8; 3] = [1, 2, 3];
    let v = s.to_vec();
    v.len() == 3
}

#[hax_test::equiv]
pub fn test_slice_to_vec_three_order() -> bool {
    let s: [u8; 3] = [1, 2, 3];
    let mut v = s.to_vec();
    // Last element popped first.
    v.pop().unwrap_or(0) == 3 && v.pop().unwrap_or(0) == 2 && v.pop().unwrap_or(0) == 1
}

#[hax_test::equiv]
pub fn test_slice_to_vec_max_value() -> bool {
    let s: [u8; 2] = [u8::MAX, 0];
    let mut v = s.to_vec();
    v.len() == 2 && v.pop().unwrap_or(1) == 0 && v.pop().unwrap_or(0) == u8::MAX
}

#[hax_test::equiv]
pub fn test_slice_to_vec_then_push() -> bool {
    // to_vec produces a fresh Vec we can mutate.
    let s: [u8; 2] = [1, 2];
    let mut v = s.to_vec();
    v.push(3);
    v.len() == 3
        && v.pop().unwrap_or(0) == 3
        && v.pop().unwrap_or(0) == 2
        && v.pop().unwrap_or(0) == 1
}

// ----- Box<[T]>::into_vec ---------------------------------------------------

// ----- sort_by (excluded) ----------------------------------------------------

// TODO(vec-index-excluded): alloc_models::slice::_::sort_by is in
// ALLOC_CHARON_EXCLUDES (closure extraction is unsupported).

// ----- closure-using methods (excluded) --------------------------------------

// TODO(slice-method-missing): `[T]::sort_by_key`, `[T]::sort_by`, and other
// closure-taking slice methods are skipped — Aeneas handles closures
// (see core::array::from_fn) but these methods are not in the model:
// `alloc_models::slice::_::sort_by` is in `ALLOC_CHARON_EXCLUDES` and the
// generic slice impl is not provided either.

// Rust-only: `Vec::into_boxed_slice` has no model.
#[cfg(test)]
mod boxed_slice {
    #[test]
    fn test_box_slice_into_vec_empty() {
        let s: [u8; 0] = [];
        let b: Box<[u8]> = s.to_vec().into_boxed_slice();
        let v: Vec<u8> = b.into_vec();
        assert!(v.is_empty());
    }

    #[test]
    fn test_box_slice_into_vec_three() {
        let s: [u8; 3] = [1, 2, 3];
        let b: Box<[u8]> = s.to_vec().into_boxed_slice();
        let mut v: Vec<u8> = b.into_vec();
        assert_eq!(v.len(), 3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), Some(1));
    }
}

// ----- dictionary-applying tests ---------------------------------------------

// `to_vec` clones every element; there is nothing to move in.
#[hax_test::equiv]
pub fn test_to_vec_applies_element_clone() -> bool {
    let a = [Bumped(1), Bumped(2)];
    let v = a.as_slice().to_vec();
    v[0].0 == 2 && v[1].0 == 3
}

#[hax_test::equiv]
pub fn test_to_vec_empty_of_a_cloneable_element() -> bool {
    let a: [Bumped; 0] = [];
    a.as_slice().to_vec().len() == 0
}
