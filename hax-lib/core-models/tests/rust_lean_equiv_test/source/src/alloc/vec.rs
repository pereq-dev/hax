//! Equivalence tests for `alloc::vec::Vec::*`.
//!
//! Mirrors the proptest cases in `alloc/src/lib.rs` (module `vec::tests`),
//! pinning each observation on a concrete input.
//!
//! Notes on what's tested:
//! - Indexing `v[i]` is in `ALLOC_CHARON_EXCLUDES` (see top-level `Makefile`),
//!   so we verify per-element contents by sequential `pop()`s instead of
//!   `v[i] == x`.

use crate::helpers::{Bumped, Keyed, keyed};

// ----- new -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_new_len_zero() -> bool {
    let v: Vec<u8> = Vec::new();
    v.len() == 0
}

#[hax_test::equiv]
pub fn test_vec_new_is_empty() -> bool {
    let v: Vec<u8> = Vec::new();
    v.is_empty()
}

// ----- with_capacity ---------------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_with_capacity_zero_len() -> bool {
    let v: Vec<u8> = Vec::with_capacity(0);
    v.len() == 0
}

#[hax_test::equiv]
pub fn test_vec_with_capacity_ten_len() -> bool {
    let v: Vec<u8> = Vec::with_capacity(10);
    v.len() == 0
}

#[hax_test::equiv]
pub fn test_vec_with_capacity_is_empty() -> bool {
    let v: Vec<u8> = Vec::with_capacity(100);
    v.is_empty()
}

// ----- len -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_len_empty() -> bool {
    let v: Vec<u8> = Vec::new();
    v.len() == 0
}

#[hax_test::equiv]
pub fn test_vec_len_one() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(42);
    v.len() == 1
}

#[hax_test::equiv]
pub fn test_vec_len_from_elem() -> bool {
    let v: Vec<u8> = vec![3u8; 5];
    v.len() == 5
}

#[hax_test::equiv]
pub fn test_vec_len_from_elem_zero() -> bool {
    let v: Vec<u8> = vec![0u8; 0];
    v.len() == 0
}

// ----- is_empty --------------------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_is_empty_new() -> bool {
    let v: Vec<u8> = Vec::new();
    v.is_empty() == true
}

#[hax_test::equiv]
pub fn test_vec_is_empty_after_push() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(7);
    v.is_empty() == false
}

// ----- as_slice --------------------------------------------------------------
//
// `as_slice` returns `&[T]`. We can observe it through length-via-deref
// (Vec implements Deref<Target=[T]>) but `len` on a slice is the same
// as `Vec::len`, so we mostly exercise that `as_slice` is well-typed.

#[hax_test::equiv]
pub fn test_vec_as_slice_empty_len() -> bool {
    let v: Vec<u8> = Vec::new();
    v.as_slice().len() == 0
}

#[hax_test::equiv]
pub fn test_vec_as_slice_one_len() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(9);
    v.as_slice().len() == 1
}

#[hax_test::equiv]
pub fn test_vec_as_slice_three_len() -> bool {
    let v: Vec<u8> = vec![4u8; 3];
    v.as_slice().len() == 3
}

// ----- push ------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_push_one_len() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(7);
    v.len() == 1
}

#[hax_test::equiv]
pub fn test_vec_push_many_len() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.len() == 3
}

// ----- append ----------------------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_append_both_empty() -> bool {
    let mut a: Vec<u8> = Vec::new();
    let mut b: Vec<u8> = Vec::new();
    a.append(&mut b);
    a.len() == 0 && b.len() == 0
}

#[hax_test::equiv]
pub fn test_vec_append_empty_to_nonempty() -> bool {
    let mut a: Vec<u8> = Vec::new();
    a.push(1);
    a.push(2);
    let mut b: Vec<u8> = Vec::new();
    a.append(&mut b);
    a.len() == 2 && b.len() == 0
}

// ----- extend_from_slice -----------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_extend_from_slice_empty_to_empty() -> bool {
    let mut v: Vec<u8> = Vec::new();
    let s: [u8; 0] = [];
    v.extend_from_slice(&s);
    v.len() == 0
}

// Clones each element, so the appended value is `Bumped(1).clone()`.
#[hax_test::equiv]
pub fn test_vec_extend_from_slice_applies_clone() -> bool {
    let mut v: Vec<Bumped> = Vec::new();
    v.extend_from_slice(&[Bumped(1)]);
    match v.pop() {
        Some(b) => b.0 == 2,
        None => false,
    }
}

// ----- from_elem (`vec![x; n]`) ---------------------------------------------

#[hax_test::equiv]
pub fn test_vec_from_elem_zero_len() -> bool {
    let v: Vec<u8> = vec![9u8; 0];
    v.len() == 0 && v.is_empty()
}

// std clones all but the last element, so this is `[Bumped(2), Bumped(1)]`.
#[hax_test::equiv]
pub fn test_vec_from_elem_applies_clone() -> bool {
    let mut v: Vec<Bumped> = vec![Bumped(1); 2];
    match v.pop() {
        Some(last) => match v.pop() {
            Some(first) => last.0 == 1 && first.0 == 2,
            None => false,
        },
        None => false,
    }
}

// ----- index (excluded) ------------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_index_first() -> bool {
    let v: Vec<u8> = vec![7u8; 1];
    v[0] == 7
}

#[hax_test::equiv]
pub fn test_vec_index_range() -> bool {
    let v: Vec<u8> = vec![7u8; 3];
    v[0..2].len() == 2
}

// ----- sort_by (excluded) ----------------------------------------------------

// TODO(vec-index-excluded): Vec::sort_by is in ALLOC_CHARON_EXCLUDES (closure
// extraction is unsupported).

// ----- from_iter (excluded) --------------------------------------------------

// TODO(vec-index-excluded): Vec::from_iter is in ALLOC_CHARON_EXCLUDES.

// ----- remove ----------------------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_remove_only_element() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(7);
    let x = v.remove(0);
    x == 7 && v.is_empty()
}

// ----- swap_remove -----------------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_swap_remove_only_element() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(9);
    let x = v.swap_remove(0);
    x == 9 && v.is_empty()
}

#[hax_test::equiv]
pub fn test_vec_swap_remove_back() -> bool {
    // Removing the last element: no swap happens, so this behaves like `pop`.
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let x = v.swap_remove(2);
    x == 3 && v.len() == 2 && v[0] == 1 && v[1] == 2
}

#[hax_test::equiv]
pub fn test_vec_swap_remove_front_moves_last() -> bool {
    // The case that distinguishes `swap_remove` from `remove`: the last
    // element takes the removed slot rather than everything shifting down.
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    let x = v.swap_remove(0);
    x == 1 && v.len() == 3 && v[0] == 4 && v[1] == 2 && v[2] == 3
}

#[hax_test::equiv]
pub fn test_vec_swap_remove_middle() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let x = v.swap_remove(1);
    x == 2 && v.len() == 2 && v[0] == 1 && v[1] == 3
}

// ----- truncate / resize / clear ---------------------------------------------

#[hax_test::equiv]
pub fn test_vec_truncate_shortens() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.truncate(1);
    v.len() == 1 && v[0] == 1
}

#[hax_test::equiv]
pub fn test_vec_truncate_longer_is_noop() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    v.push(2);
    v.truncate(5);
    v.len() == 2 && v[0] == 1 && v[1] == 2
}

#[hax_test::equiv]
pub fn test_vec_truncate_zero_empties() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    v.truncate(0);
    v.is_empty()
}

#[hax_test::equiv]
pub fn test_vec_clear_empties() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    v.push(2);
    v.clear();
    v.is_empty()
}

#[hax_test::equiv]
pub fn test_vec_resize_grows() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    v.resize(3, 7);
    v.len() == 3 && v[0] == 1 && v[1] == 7 && v[2] == 7
}

#[hax_test::equiv]
pub fn test_vec_resize_shrinks() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.resize(1, 0);
    v.len() == 1 && v[0] == 1
}

#[hax_test::equiv]
pub fn test_vec_resize_same_len_is_noop() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    v.resize(1, 9);
    v.len() == 1 && v[0] == 1
}

// ----- drain (iterator) ------------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_drain() -> bool {
    let mut a: Vec<u8> = Vec::new();
    a.push(1u8);
    a.push(2u8);
    a.push(3u8);
    a.push(4u8);
    let (first, second, third) = {
        let mut d = a.drain(1..3);
        (d.next(), d.next(), d.next())
    };
    let mut ea: Vec<u8> = Vec::new();
    ea.push(1u8);
    ea.push(4u8);
    first == Some(2u8) && second == Some(3u8) && third == None && a == ea
}

#[hax_test::equiv(panics)]
pub fn test_vec_drain_past_end() -> bool {
    let mut a: Vec<u8> = Vec::new();
    a.push(1u8);
    a.push(2u8);
    let _ = a.drain(1..3);
    a.len() == 1
}

// ----- closure-using methods (excluded) --------------------------------------

// TODO(closure-extraction + vec-extraction-arity-mismatch): Vec::retain takes
// a closure. Aeneas can extract the closure (see core::array::from_fn) but
// every Vec method first trips on the arity-mismatch issue above. Revisit
// once Vec tests can compile in the first place.
// TODO(vec-iter-extraction): Vec::iter / Vec::into_iter use iterator traits
// whose Lean models we don't have yet.

// ----- pop -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_push_pop() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(9u8);
    match v.pop() {
        Some(x) => x == 9u8,
        None => false,
    }
}

// ----- PartialEq / Clone / IntoIterator on Vec (branch additions) ------------
// These exercise the extracted `eq_loop` / `clone_loop` / `IntoIter::next`.

#[hax_test::equiv]
pub fn test_vec_eq_short_circuits() -> bool {
    // `Bumped::eq` panics on `u8::MAX`. std stops at the first mismatch, so the
    // second pair is never compared; a non-short-circuiting model would panic.
    let mut a: Vec<Bumped> = Vec::new();
    a.push(Bumped(1));
    a.push(Bumped(255));
    let mut b: Vec<Bumped> = Vec::new();
    b.push(Bumped(2));
    b.push(Bumped(255));
    (a == b) == false
}

#[hax_test::equiv]
pub fn test_vec_eq_same() -> bool {
    let mut a: Vec<u8> = Vec::new();
    a.push(1u8);
    a.push(2u8);
    let mut b: Vec<u8> = Vec::new();
    b.push(1u8);
    b.push(2u8);
    (a == b) == true
}

#[hax_test::equiv]
pub fn test_vec_eq_diff() -> bool {
    let mut a: Vec<u8> = Vec::new();
    a.push(1u8);
    let mut b: Vec<u8> = Vec::new();
    b.push(2u8);
    (a == b) == false
}

#[hax_test::equiv]
pub fn test_vec_clone_preserves() -> bool {
    let mut a: Vec<u8> = Vec::new();
    a.push(3u8);
    a.push(4u8);
    let b = a.clone();
    (a == b) == true
}

#[hax_test::equiv]
pub fn test_vec_into_iter_first() -> bool {
    let mut v: Vec<u8> = Vec::new();
    v.push(7u8);
    v.push(8u8);
    let mut it = v.into_iter();
    match it.next() {
        Some(x) => x == 7u8,
        None => false,
    }
}

// ----- default ---------------------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_default_empty() -> bool {
    let a: Vec<u8> = Default::default();
    let b: Vec<u8> = Vec::new();
    (a == b) == true
}

// ----- split_off -------------------------------------------------------------

#[hax_test::equiv]
pub fn test_vec_split_off() -> bool {
    let mut a: Vec<u8> = Vec::new();
    a.push(1u8);
    a.push(2u8);
    a.push(3u8);
    let b = a.split_off(1);
    let mut ea: Vec<u8> = Vec::new();
    ea.push(1u8);
    let mut eb: Vec<u8> = Vec::new();
    eb.push(2u8);
    eb.push(3u8);
    (a == ea) && (b == eb)
}

// ----- dictionary-applying tests ---------------------------------------------

#[hax_test::equiv]
pub fn test_vec_clone_applies_element_clone() -> bool {
    let mut v: Vec<Bumped> = Vec::new();
    v.push(Bumped(1));
    v.push(Bumped(2));
    let w = v.clone();
    w[0].0 == 2 && w[1].0 == 3
}

// `resize` clones into all but the last new slot and moves `value` into that one.
#[hax_test::equiv]
pub fn test_vec_resize_moves_the_last_element() -> bool {
    let mut v: Vec<Bumped> = Vec::new();
    v.resize(3, Bumped(1));
    v[0].0 == 2 && v[1].0 == 2 && v[2].0 == 1
}

// Growing by one is entirely the moved-in case.
#[hax_test::equiv]
pub fn test_vec_resize_by_one_is_moved() -> bool {
    let mut v: Vec<Bumped> = Vec::new();
    v.resize(1, Bumped(1));
    v[0].0 == 1
}

// Shrinking never touches the dictionary.
#[hax_test::equiv]
pub fn test_vec_resize_shrink_keeps_the_prefix() -> bool {
    let mut v: Vec<Bumped> = Vec::new();
    v.push(Bumped(7));
    v.push(Bumped(8));
    v.resize(1, Bumped(0));
    v.len() == 1 && v[0].0 == 7
}

#[hax_test::equiv]
pub fn test_vec_eq_goes_through_the_dictionary() -> bool {
    let mut a: Vec<Keyed> = Vec::new();
    a.push(keyed(1, 1));
    let mut b: Vec<Keyed> = Vec::new();
    b.push(keyed(1, 2));
    a == b
}

#[hax_test::equiv]
pub fn test_vec_ne_goes_through_the_dictionary() -> bool {
    let mut a: Vec<Keyed> = Vec::new();
    a.push(keyed(1, 1));
    let mut b: Vec<Keyed> = Vec::new();
    b.push(keyed(1, 2));
    (a != b) == false
}
