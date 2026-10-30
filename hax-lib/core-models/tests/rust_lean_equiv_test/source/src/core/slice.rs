//! Equivalence tests for `[T]` (`core::slice`) operations.
//!
//! These mirror the proptest block in `core-models/src/core/slice.rs`.

use crate::helpers::{Bumped, keyed};

// ----- len -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_len_empty() -> bool {
    let a: [u8; 0] = [];
    a.as_slice().len() == 0
}

#[hax_test::equiv]
pub fn test_len_one() -> bool {
    let a: [u8; 1] = [42];
    a.as_slice().len() == 1
}

#[hax_test::equiv]
pub fn test_len_eight() -> bool {
    let a: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    a.as_slice().len() == 8
}

// ----- is_empty --------------------------------------------------------------

#[hax_test::equiv]
pub fn test_is_empty_empty() -> bool {
    let a: [u8; 0] = [];
    a.as_slice().is_empty()
}

#[hax_test::equiv]
pub fn test_is_empty_one() -> bool {
    let a: [u8; 1] = [0];
    a.as_slice().is_empty() == false
}

#[hax_test::equiv]
pub fn test_is_empty_many() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    a.as_slice().is_empty() == false
}

// ----- contains --------------------------------------------------------------

#[hax_test::equiv]
pub fn test_contains_present() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    a.as_slice().contains(&3)
}

#[hax_test::equiv]
pub fn test_contains_first() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    a.as_slice().contains(&1)
}

#[hax_test::equiv]
pub fn test_contains_absent() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    a.as_slice().contains(&99) == false
}

#[hax_test::equiv]
pub fn test_contains_empty() -> bool {
    let a: [u8; 0] = [];
    a.as_slice().contains(&0) == false
}

// `contains` stops at the first match, so `Bumped::eq` never sees `u8::MAX`
// — unless the model keeps comparing past it.
#[hax_test::equiv]
pub fn test_contains_stops_at_first_match() -> bool {
    let a = [Bumped(1), Bumped(u8::MAX)];
    a.as_slice().contains(&Bumped(1))
}

// ----- split_at --------------------------------------------------------------

#[hax_test::equiv]
pub fn test_split_at_zero() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let (l, r) = a.as_slice().split_at(0);
    l.is_empty() && r.len() == 4
}

#[hax_test::equiv]
pub fn test_split_at_full() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let (l, r) = a.as_slice().split_at(4);
    l.len() == 4 && r.is_empty()
}

#[hax_test::equiv]
pub fn test_split_at_middle_lens() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let (l, r) = a.as_slice().split_at(2);
    l.len() == 2 && r.len() == 2
}

#[hax_test::equiv]
pub fn test_split_at_middle_left_first() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let (l, _r) = a.as_slice().split_at(2);
    l[0] == 1
}

#[hax_test::equiv]
pub fn test_split_at_middle_right_first() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let (_l, r) = a.as_slice().split_at(2);
    r[0] == 3
}

// ----- split_at_checked ------------------------------------------------------

#[hax_test::equiv]
pub fn test_split_at_checked_in_range_some() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    a.as_slice().split_at_checked(2).is_some()
}

#[hax_test::equiv]
pub fn test_split_at_checked_at_end_some() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    a.as_slice().split_at_checked(4).is_some()
}

#[hax_test::equiv]
pub fn test_split_at_checked_out_of_range_none() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    a.as_slice().split_at_checked(5).is_none()
}

// ----- first -----------------------------------------------------------------

#[hax_test::equiv]
pub fn test_first_present() -> bool {
    let a: [u8; 4] = [10, 20, 30, 40];
    match a.as_slice().first() {
        Some(v) => *v == 10,
        None => false,
    }
}

#[hax_test::equiv]
pub fn test_first_some_one() -> bool {
    let a: [u8; 1] = [7];
    match a.as_slice().first() {
        Some(v) => *v == 7,
        None => false,
    }
}

#[hax_test::equiv]
pub fn test_first_empty_none() -> bool {
    let a: [u8; 0] = [];
    a.as_slice().first().is_none()
}

// ----- last ------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_last_present() -> bool {
    let a: [u8; 4] = [10, 20, 30, 40];
    match a.as_slice().last() {
        Some(v) => *v == 40,
        None => false,
    }
}

#[hax_test::equiv]
pub fn test_last_some_one() -> bool {
    let a: [u8; 1] = [7];
    match a.as_slice().last() {
        Some(v) => *v == 7,
        None => false,
    }
}

#[hax_test::equiv]
pub fn test_last_empty_none() -> bool {
    let a: [u8; 0] = [];
    a.as_slice().last().is_none()
}

// ----- get (usize) -----------------------------------------------------------

#[hax_test::equiv]
pub fn test_get_usize_in_range() -> bool {
    let a: [u8; 4] = [10, 20, 30, 40];
    match a.as_slice().get(2) {
        Some(v) => *v == 30,
        None => false,
    }
}

#[hax_test::equiv]
pub fn test_get_usize_first() -> bool {
    let a: [u8; 4] = [10, 20, 30, 40];
    match a.as_slice().get(0) {
        Some(v) => *v == 10,
        None => false,
    }
}

#[hax_test::equiv]
pub fn test_get_usize_out_of_range() -> bool {
    let a: [u8; 4] = [10, 20, 30, 40];
    a.as_slice().get(4).is_none()
}

#[hax_test::equiv]
pub fn test_get_usize_empty() -> bool {
    let a: [u8; 0] = [];
    a.as_slice().get(0).is_none()
}

// ----- starts_with / ends_with ----------------------------------------------

#[hax_test::equiv]
pub fn test_starts_with_true() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let needle: [u8; 2] = [1, 2];
    a.as_slice().starts_with(needle.as_slice())
}

#[hax_test::equiv]
pub fn test_starts_with_empty_needle() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let needle: [u8; 0] = [];
    a.as_slice().starts_with(needle.as_slice())
}

#[hax_test::equiv]
pub fn test_starts_with_false() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let needle: [u8; 2] = [2, 3];
    a.as_slice().starts_with(needle.as_slice()) == false
}

#[hax_test::equiv]
pub fn test_ends_with_true() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let needle: [u8; 2] = [3, 4];
    a.as_slice().ends_with(needle.as_slice())
}

#[hax_test::equiv]
pub fn test_ends_with_empty_needle() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let needle: [u8; 0] = [];
    a.as_slice().ends_with(needle.as_slice())
}

#[hax_test::equiv]
pub fn test_ends_with_false() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let needle: [u8; 2] = [2, 3];
    a.as_slice().ends_with(needle.as_slice()) == false
}

// ----- Range / RangeFrom indexing, get(range) --------------------------------

#[hax_test::equiv]
pub fn test_index_range() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    a.as_slice()[1..3] == [2, 3]
}

#[hax_test::equiv]
pub fn test_get_range_some() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    a.as_slice().get(1..3).is_some()
}

#[hax_test::equiv]
pub fn test_index_range_from() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    a.as_slice()[2..] == [3, 4]
}

// ----- swap / reverse (mutate through &mut [T]) ------------------------------

#[hax_test::equiv]
pub fn test_slice_swap() -> bool {
    let mut a = [1u8, 2, 3];
    let s: &mut [u8] = &mut a;
    s.swap(0, 2);
    a == [3u8, 2, 1]
}

#[hax_test::equiv]
pub fn test_slice_reverse() -> bool {
    let mut a = [1u8, 2, 3, 4];
    let s: &mut [u8] = &mut a;
    s.reverse();
    a == [4u8, 3, 2, 1]
}

// Overwrites `dst` with *clones* of `src`'s elements.
#[hax_test::equiv]
pub fn test_clone_from_slice_applies_clone() -> bool {
    let mut dst = [Bumped(0)];
    let src = [Bumped(1)];
    let d: &mut [Bumped] = &mut dst;
    d.clone_from_slice(&src);
    dst[0].0 == 2
}

#[hax_test::equiv]
pub fn test_fill() -> bool {
    let mut a: [u8; 4] = [0, 0, 0, 0];
    let s: &mut [u8] = &mut a;
    s.fill(7);
    a == [7, 7, 7, 7]
}

// Cloned into every slot but the last, moved into that one.
#[hax_test::equiv]
pub fn test_fill_moves_the_last_element() -> bool {
    let mut a = [Bumped(0), Bumped(0), Bumped(0)];
    let s: &mut [Bumped] = &mut a;
    s.fill(Bumped(1));
    a[0].0 == 2 && a[1].0 == 2 && a[2].0 == 1
}

// A one-element slice is entirely the moved-in case.
#[hax_test::equiv]
pub fn test_fill_single_element_is_moved() -> bool {
    let mut a = [Bumped(0)];
    let s: &mut [Bumped] = &mut a;
    s.fill(Bumped(1));
    a[0].0 == 1
}

// Rust-only: `slice::iter::Iter` does not translate (see `core::iter`).
#[cfg(test)]
#[test]
fn test_iter_count() {
    let a: [u8; 3] = [1, 2, 3];
    assert_eq!(a.as_slice().iter().count(), 3);
}

#[hax_test::equiv]
pub fn test_binary_search() -> bool {
    let a: [u8; 4] = [1, 3, 5, 7];
    a.as_slice().binary_search(&5) == Ok(2)
}

// Rust leaves the index unspecified on a tie; these pin which one comes back.
#[hax_test::equiv]
pub fn test_binary_search_all_equal_three() -> bool {
    let a: [u8; 3] = [1, 1, 1];
    a.as_slice().binary_search(&1) == Ok(2)
}

#[hax_test::equiv]
pub fn test_binary_search_all_equal_five() -> bool {
    let a: [u8; 5] = [1, 1, 1, 1, 1];
    a.as_slice().binary_search(&1) == Ok(4)
}

#[hax_test::equiv]
pub fn test_binary_search_run_in_the_middle() -> bool {
    let a: [u8; 6] = [1, 3, 3, 3, 5, 7];
    a.as_slice().binary_search(&3) == Ok(3)
}

// The `Err` side: insertion point below, inside and above the slice.
#[hax_test::equiv]
pub fn test_binary_search_absent_low() -> bool {
    let a: [u8; 4] = [2, 4, 6, 8];
    a.as_slice().binary_search(&1) == Err(0)
}

#[hax_test::equiv]
pub fn test_binary_search_absent_middle() -> bool {
    let a: [u8; 4] = [2, 4, 6, 8];
    a.as_slice().binary_search(&5) == Err(2)
}

#[hax_test::equiv]
pub fn test_binary_search_absent_high() -> bool {
    let a: [u8; 4] = [2, 4, 6, 8];
    a.as_slice().binary_search(&9) == Err(4)
}

#[hax_test::equiv]
pub fn test_binary_search_empty() -> bool {
    let a: [u8; 0] = [];
    a.as_slice().binary_search(&1) == Err(0)
}

#[hax_test::equiv]
pub fn test_binary_search_unsorted_found() -> bool {
    let a: [u8; 3] = [3, 1, 2];
    a.as_slice().binary_search(&1) == Ok(1)
}

#[hax_test::equiv]
pub fn test_binary_search_unsorted_present_but_missed() -> bool {
    let a: [u8; 5] = [5, 4, 3, 2, 1];
    a.as_slice().binary_search(&3) == Err(5)
}

#[hax_test::equiv]
pub fn test_copy_within() -> bool {
    let mut a: [u8; 4] = [1, 2, 3, 4];
    let s: &mut [u8] = &mut a;
    s.copy_within(0..2, 2);
    a == [1u8, 2, 1, 2]
}

// The source range is read before the destination is written.
#[hax_test::equiv]
pub fn test_copy_within_overlapping() -> bool {
    let mut a: [u8; 5] = [1, 2, 3, 4, 5];
    let s: &mut [u8] = &mut a;
    s.copy_within(0..3, 2);
    a == [1u8, 2, 1, 2, 3]
}

// Each call below changes the array, so a no-op or off-by-one model fails.
#[hax_test::equiv]
pub fn test_copy_within_inclusive_ranges() -> bool {
    let mut a: [u8; 5] = [1, 2, 3, 4, 5];
    let s: &mut [u8] = &mut a;
    s.copy_within(3..=4, 0);
    s.copy_within(..=1, 2);
    a == [4u8, 5, 4, 5, 5]
}

#[hax_test::equiv]
pub fn test_copy_within_range_forms() -> bool {
    let mut a: [u8; 5] = [1, 2, 3, 4, 5];
    let s: &mut [u8] = &mut a;
    s.copy_within(3.., 0);
    s.copy_within(..2, 2);
    a == [4u8, 5, 4, 5, 5]
}

// `dest` is one past the last position that fits the copied range.
#[hax_test::equiv(panics)]
pub fn test_copy_within_dest_past_end() -> bool {
    let mut a: [u8; 4] = [1, 2, 3, 4];
    let s: &mut [u8] = &mut a;
    s.copy_within(0..2, 3);
    a == [1u8, 2, 3, 1]
}

#[hax_test::equiv(panics)]
pub fn test_copy_within_src_past_end() -> bool {
    let mut a: [u8; 4] = [1, 2, 3, 4];
    let s: &mut [u8] = &mut a;
    s.copy_within(3..=4, 0);
    a == [4u8, 2, 3, 4]
}

#[hax_test::equiv]
pub fn test_copy_within_bound_pair() -> bool {
    use core::ops::Bound;
    let mut a: [u8; 5] = [1, 2, 3, 4, 5];
    let s: &mut [u8] = &mut a;
    s.copy_within((Bound::Excluded(0), Bound::Included(2)), 3);
    a == [1u8, 2, 3, 2, 3]
}

// ----------------------------------------------------------------------------
// The `SliceIndex` impls (for `usize`, `Range`, `RangeFrom`, `RangeTo`,
// `RangeFull`) are not extracted. The Lean side resolves through the name
// map to manual definitions. Each `Range*` variant routes through a distinct
// `SliceIndex` impl, so we test all of them.
// ----------------------------------------------------------------------------

// ----- s[i] (manually defined in Lean, not extracted) -----------------------

#[hax_test::equiv]
pub fn test_slice_index_usize_first() -> bool {
    let s: [u8; 8] = [10, 20, 30, 40, 50, 60, 70, 80];
    s[0] == 10
}

#[hax_test::equiv]
pub fn test_slice_index_usize_last() -> bool {
    let s: [u8; 8] = [10, 20, 30, 40, 50, 60, 70, 80];
    s[7] == 80
}

#[hax_test::equiv]
pub fn test_slice_index_usize_middle() -> bool {
    let s: [u8; 8] = [10, 20, 30, 40, 50, 60, 70, 80];
    s[3] == 40
}

// ----- &s[..end] : RangeTo (manually defined in Lean, not extracted) --------

#[hax_test::equiv]
pub fn test_slice_index_range_to_len() -> bool {
    let s: [u8; 8] = [10, 20, 30, 40, 50, 60, 70, 80];
    let t: &[u8] = &s[..3];
    t.len() == 3
}

#[hax_test::equiv]
pub fn test_slice_index_range_to_first_elem() -> bool {
    let s: [u8; 8] = [10, 20, 30, 40, 50, 60, 70, 80];
    let t: &[u8] = &s[..3];
    t[0] == 10
}

#[hax_test::equiv]
pub fn test_slice_index_range_to_last_elem() -> bool {
    let s: [u8; 8] = [10, 20, 30, 40, 50, 60, 70, 80];
    let t: &[u8] = &s[..3];
    t[2] == 30
}

// ----- &s[..] : RangeFull (manually defined in Lean, not extracted) ---------

#[hax_test::equiv]
pub fn test_slice_index_range_full_len() -> bool {
    let s: [u8; 8] = [10, 20, 30, 40, 50, 60, 70, 80];
    let t: &[u8] = &s[..];
    t.len() == 8
}

// ----- PartialEq / Ord on slices (branch additions) --------------------------

#[hax_test::equiv]
pub fn test_slice_eq_same() -> bool {
    let a: &[u8] = &[1u8, 2, 3];
    let b: &[u8] = &[1u8, 2, 3];
    (a == b) == true
}

#[hax_test::equiv]
pub fn test_slice_eq_diff() -> bool {
    let a: &[u8] = &[1u8, 2, 3];
    let b: &[u8] = &[1u8, 2, 4];
    (a == b) == false
}

// ----- Ord / PartialOrd on slices (lexicographic) ----------------------------
// These exercise the extracted `partial_cmp_loop` / `cmp_loop`.

#[hax_test::equiv]
pub fn test_slice_cmp_less_by_elem() -> bool {
    let a: &[u8] = &[1, 2, 3];
    let b: &[u8] = &[1, 5, 0];
    match a.cmp(b) {
        std::cmp::Ordering::Less => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn test_slice_cmp_less_by_len() -> bool {
    // `[1,2]` is a prefix of `[1,2,3]`, so it is `Less`.
    let a: &[u8] = &[1, 2];
    let b: &[u8] = &[1, 2, 3];
    match a.cmp(b) {
        std::cmp::Ordering::Less => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn test_slice_cmp_greater_by_elem() -> bool {
    let a: &[u8] = &[2];
    let b: &[u8] = &[1, 9, 9];
    match a.cmp(b) {
        std::cmp::Ordering::Greater => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn test_slice_cmp_equal() -> bool {
    let a: &[u8] = &[4, 5, 6];
    let b: &[u8] = &[4, 5, 6];
    match a.cmp(b) {
        std::cmp::Ordering::Equal => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn test_slice_partial_cmp_less() -> bool {
    let a: &[u8] = &[1, 2];
    let b: &[u8] = &[1, 3];
    match a.partial_cmp(b) {
        Some(std::cmp::Ordering::Less) => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn test_slice_eq_diff_len() -> bool {
    let a: &[u8] = &[1, 2];
    let b: &[u8] = &[1, 2, 3];
    (a == b) == false
}

// ----- get_unchecked (in-bounds) ---------------------------------------------
// In-bounds is the only defined behaviour; the model projects like `index`.

#[hax_test::equiv]
pub fn test_slice_get_unchecked_first() -> bool {
    let s: &[u8] = &[10u8, 20, 30];
    unsafe { *s.get_unchecked(0) == 10 }
}

#[hax_test::equiv]
pub fn test_slice_get_unchecked_last() -> bool {
    let s: &[u8] = &[10u8, 20, 30];
    unsafe { *s.get_unchecked(2) == 30 }
}

#[hax_test::equiv]
pub fn test_slice_get_unchecked_range() -> bool {
    let s: &[u8] = &[10u8, 20, 30, 40];
    let sub: &[u8] = unsafe { s.get_unchecked(1..3) };
    let expected: &[u8] = &[20u8, 30];
    sub == expected
}

#[hax_test::equiv]
pub fn test_slice_get_unchecked_range_from() -> bool {
    let s: &[u8] = &[10u8, 20, 30, 40];
    let sub: &[u8] = unsafe { s.get_unchecked(2..) };
    let expected: &[u8] = &[30u8, 40];
    sub == expected
}

#[hax_test::equiv]
pub fn test_slice_get_unchecked_range_to() -> bool {
    let s: &[u8] = &[10u8, 20, 30, 40];
    let sub: &[u8] = unsafe { s.get_unchecked(..2) };
    let expected: &[u8] = &[10u8, 20];
    sub == expected
}

#[hax_test::equiv]
pub fn test_slice_get_unchecked_range_full() -> bool {
    let s: &[u8] = &[10u8, 20, 30, 40];
    let sub: &[u8] = unsafe { s.get_unchecked(..) };
    sub == s
}

// ----- get_mut / get_unchecked_mut (mutate through the &mut) -----------------

#[hax_test::equiv]
pub fn test_slice_get_mut_usize() -> bool {
    let mut a = [10u8, 20, 30];
    let s: &mut [u8] = &mut a;
    if let Some(r) = s.get_mut(1) {
        *r = 99;
    }
    a == [10u8, 99, 30]
}

#[hax_test::equiv]
pub fn test_slice_get_mut_usize_oob() -> bool {
    let mut a = [10u8, 20, 30];
    let s: &mut [u8] = &mut a;
    // out of bounds: `get_mut` returns `None`, leaving the slice unchanged
    s.get_mut(5).is_none() && a == [10u8, 20, 30]
}

#[hax_test::equiv]
pub fn test_slice_get_unchecked_mut_usize() -> bool {
    let mut a = [10u8, 20, 30];
    let s: &mut [u8] = &mut a;
    unsafe {
        *s.get_unchecked_mut(2) = 99;
    }
    a == [10u8, 20, 99]
}

// ----- PartialEq<[T; N]> for [T]  (slice == array) ---------------------------

#[hax_test::equiv]
pub fn test_slice_eq_array_true() -> bool {
    let s: &[u8] = &[1u8, 2, 3];
    *s == [1u8, 2, 3]
}

#[hax_test::equiv]
pub fn test_slice_eq_array_false_value() -> bool {
    let s: &[u8] = &[1u8, 2, 3];
    (*s == [1u8, 2, 9]) == false
}

#[hax_test::equiv]
pub fn test_slice_eq_array_false_len() -> bool {
    let s: &[u8] = &[1u8, 2];
    (*s == [1u8, 2, 3]) == false
}

// ----- chunks / chunks_exact / windows / copy_from_slice ---------------------

#[hax_test::equiv]
pub fn test_chunks_first() -> bool {
    let a: [u8; 5] = [1, 2, 3, 4, 5];
    let mut it = a.as_slice().chunks(2);
    match it.next() {
        Some(c) => {
            let e: &[u8] = &[1, 2];
            c == e
        }
        None => false,
    }
}

#[hax_test::equiv]
pub fn test_chunks_last_is_partial() -> bool {
    let a: [u8; 5] = [1, 2, 3, 4, 5];
    let mut it = a.as_slice().chunks(2);
    it.next();
    it.next();
    match it.next() {
        Some(c) => {
            let e: &[u8] = &[5];
            c == e
        }
        None => false,
    }
}

#[hax_test::equiv]
pub fn test_chunks_exact_drops_remainder() -> bool {
    let a: [u8; 5] = [1, 2, 3, 4, 5];
    let mut it = a.as_slice().chunks_exact(2);
    it.next();
    it.next();
    match it.next() {
        Some(_) => false,
        None => true,
    }
}

#[hax_test::equiv]
pub fn test_windows_first() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let mut it = a.as_slice().windows(2);
    match it.next() {
        Some(w) => {
            let e: &[u8] = &[1, 2];
            w == e
        }
        None => false,
    }
}

#[hax_test::equiv]
pub fn test_windows_second_overlaps() -> bool {
    let a: [u8; 4] = [1, 2, 3, 4];
    let mut it = a.as_slice().windows(2);
    it.next();
    match it.next() {
        Some(w) => {
            let e: &[u8] = &[2, 3];
            w == e
        }
        None => false,
    }
}

#[hax_test::equiv]
pub fn test_copy_from_slice() -> bool {
    let src: [u8; 3] = [7, 8, 9];
    let mut dst: [u8; 3] = [0, 0, 0];
    let d: &mut [u8] = &mut dst;
    d.copy_from_slice(&src);
    dst == [7, 8, 9]
}

// ----- dictionary-applying tests ---------------------------------------------

// `starts_with`/`ends_with` compare through `T`'s `PartialEq`.
#[hax_test::equiv]
pub fn test_starts_with_goes_through_the_dictionary() -> bool {
    let a = [keyed(1, 9), keyed(2, 9)];
    let needle = [keyed(1, 0)];
    a.as_slice().starts_with(needle.as_slice())
}

#[hax_test::equiv]
pub fn test_ends_with_goes_through_the_dictionary() -> bool {
    let a = [keyed(1, 9), keyed(2, 9)];
    let needle = [keyed(2, 0)];
    a.as_slice().ends_with(needle.as_slice())
}

#[hax_test::equiv]
pub fn test_starts_with_differing_keys() -> bool {
    let a = [keyed(1, 0), keyed(2, 0)];
    let needle = [keyed(3, 0)];
    a.as_slice().starts_with(needle.as_slice()) == false
}

// `contains` on a type whose `eq` ignores half the value.
#[hax_test::equiv]
pub fn test_contains_goes_through_the_dictionary() -> bool {
    let a = [keyed(1, 9), keyed(2, 9)];
    a.as_slice().contains(&keyed(2, 0))
}

// Slice equality: same length, equal by the dictionary, different structurally.
#[hax_test::equiv]
pub fn test_slice_eq_goes_through_the_dictionary() -> bool {
    let a = [keyed(1, 1), keyed(2, 1)];
    let b = [keyed(1, 2), keyed(2, 2)];
    a.as_slice() == b.as_slice()
}

#[hax_test::equiv]
pub fn test_slice_ne_goes_through_the_dictionary() -> bool {
    let a = [keyed(1, 1), keyed(2, 1)];
    let b = [keyed(1, 2), keyed(2, 2)];
    (a.as_slice() != b.as_slice()) == false
}

#[hax_test::equiv]
pub fn test_slice_array_eq_goes_through_the_dictionary() -> bool {
    let a = [keyed(1, 1), keyed(2, 1)];
    let b = [keyed(1, 2), keyed(2, 2)];
    a.as_slice() == b
}

// Lexicographic through `T`'s `Ord`, so a coarser `Ord` makes two structurally
// different slices compare `Equal`.
#[hax_test::equiv]
pub fn test_slice_cmp_goes_through_the_dictionary() -> bool {
    let a = [keyed(1, 1), keyed(2, 1)];
    let b = [keyed(1, 2), keyed(2, 2)];
    match a.as_slice().cmp(b.as_slice()) {
        core::cmp::Ordering::Equal => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn test_slice_cmp_shorter_is_less() -> bool {
    let a = [keyed(1, 0)];
    let b = [keyed(1, 0), keyed(9, 0)];
    match a.as_slice().cmp(b.as_slice()) {
        core::cmp::Ordering::Less => true,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn test_slice_partial_cmp_goes_through_the_dictionary() -> bool {
    let a = [keyed(1, 1), keyed(2, 1)];
    let b = [keyed(1, 2), keyed(2, 2)];
    match a.as_slice().partial_cmp(b.as_slice()) {
        Some(core::cmp::Ordering::Equal) => true,
        _ => false,
    }
}

// The `Ord` dictionary, without pinning which member of an equal run returns.
#[hax_test::equiv]
pub fn test_binary_search_goes_through_the_dictionary() -> bool {
    let a = [keyed(1, 9), keyed(3, 9), keyed(5, 9)];
    a.as_slice().binary_search(&keyed(3, 0)) == Ok(1)
}

#[hax_test::equiv]
pub fn test_binary_search_absent_goes_through_the_dictionary() -> bool {
    let a = [keyed(1, 9), keyed(3, 9), keyed(5, 9)];
    a.as_slice().binary_search(&keyed(4, 0)) == Err(2)
}
