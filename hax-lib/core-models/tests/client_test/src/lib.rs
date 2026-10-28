//! Regression-testing crate for the `core-models` Lean library.
//!
//! Each function below exercises an item — either from `core::*` /
//! `std::*` directly, or from the local `core_models::*` source. Aeneas
//! extracts this crate (resolving `std::*` / `core::*` references
//! through its builtin name map) and we compile the result against our
//! hand-written `Aeneas` library. Anything missing in our shims surfaces
//! here as a Lean elaboration error.

#![allow(dead_code)]

pub mod hax_lib;
pub mod trait_impls;

// ----- Option ---------------------------------------------------------------

pub fn option_unwrap(x: Option<u8>) -> u8 {
    x.unwrap()
}

pub fn option_unwrap_or(x: Option<u8>, default: u8) -> u8 {
    x.unwrap_or(default)
}

pub fn option_is_some(x: Option<u32>) -> bool {
    x.is_some()
}

pub fn option_is_none(x: Option<u32>) -> bool {
    x.is_none()
}

pub fn option_take(x: &mut Option<u16>) -> Option<u16> {
    x.take()
}

pub fn option_pattern(x: Option<u8>) -> u8 {
    match x {
        Some(v) => v,
        None => 0,
    }
}

pub fn option_double(x: Option<u8>) -> u8 {
    match x {
        Some(v) => v.wrapping_add(v),
        None => 0,
    }
}

/// `?` resolves through the `Try`/`FromResidual` impls on `Option`.
pub fn option_question_mark(x: Option<u8>) -> Option<u8> {
    Some(x? + 1)
}

// ----- Result ---------------------------------------------------------------

pub fn result_ok(x: Result<u8, u8>) -> Option<u8> {
    x.ok()
}

pub fn result_err(x: Result<u8, u32>) -> Option<u32> {
    x.err()
}

pub fn result_is_ok(x: Result<u8, u8>) -> bool {
    x.is_ok()
}

pub fn result_is_err(x: Result<u8, u8>) -> bool {
    x.is_err()
}

pub fn result_pattern(x: Result<u8, u8>) -> u8 {
    match x {
        Ok(v) => v,
        Err(_) => 0,
    }
}

// ----- fmt ------------------------------------------------------------------
//
// No fmt smoke test: extracting a call into the fmt traits makes Aeneas abort
// (internal `Unreachable`); the impls themselves elaborate fine.

// ----- mem ------------------------------------------------------------------

pub fn mem_swap_u32(a: &mut u32, b: &mut u32) {
    core::mem::swap(a, b);
}

pub fn mem_replace_u8(dst: &mut u8, src: u8) -> u8 {
    core::mem::replace(dst, src)
}

// ----- Scalar arithmetic ----------------------------------------------------

pub fn add_u8(x: u8, y: u8) -> u8 {
    x + y
}

pub fn sub_u32(x: u32, y: u32) -> u32 {
    x - y
}

pub fn mul_u16(x: u16, y: u16) -> u16 {
    x * y
}

pub fn wrapping_neg_u32(x: u32) -> u32 {
    x.wrapping_neg()
}

// ----- Comparisons ----------------------------------------------------------

pub fn lt_u8(x: u8, y: u8) -> bool {
    x < y
}

pub fn ge_usize(x: usize, y: usize) -> bool {
    x >= y
}

/// Derived impls whose provided methods the crate never calls: charon drops
/// those unless they are named as translation roots, and Lean then rejects the
/// impl (pereq-dev/hax#2172).
#[derive(PartialEq)]
pub enum DerivedPartialEq {
    A,
    B,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct DerivedOrd(pub u8);

pub fn derived_eq(x: &DerivedPartialEq, y: &DerivedPartialEq) -> bool {
    x == y
}

// ----- Bitwise --------------------------------------------------------------

pub fn xor_u64(x: u64, y: u64) -> u64 {
    x ^ y
}

pub fn or_u32(x: u32, y: u32) -> u32 {
    x | y
}

pub fn and_u8(x: u8, y: u8) -> u8 {
    x & y
}

// ----- Clone / Copy ---------------------------------------------------------

pub fn clone_u64(x: u64) -> u64 {
    x.clone()
}

pub fn copy_u8(x: u8) -> (u8, u8) {
    (x, x)
}

// ----- Arrays ---------------------------------------------------------------

pub fn arr_index(a: [u32; 4], i: usize) -> u32 {
    a[i]
}

pub fn arr_set(mut a: [u32; 4], i: usize, x: u32) -> [u32; 4] {
    a[i] = x;
    a
}

pub fn arr_repeat() -> [u8; 16] {
    [0u8; 16]
}

pub fn arr_to_slice(a: &[u32; 4]) -> &[u32] {
    a
}

/// Triggers `Array.make` with a 24-element initializer (regression test for
/// the default proof tactic — `by rfl` cannot reduce `(Usize.ofNat 24).val`
/// to `24`, only `by simp` with the right lemmas can).
pub const ROUND_CONSTANTS: [u64; 24] = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808a,
    0x8000000080008000,
    0x000000000000808b,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008a,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000a,
    0x000000008000808b,
    0x800000000000008b,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800a,
    0x800000008000000a,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
];

/// Direct usage of `std::array::from_fn` — exercises the `Fn`/`FnMut`
/// trait dispatch. The std signature is `F: FnMut`; Aeneas's name map
/// reaches into the `Fn` instance via its `.FnMutInst` super-trait
/// projection, so our `core.ops.function.Fn` structure must expose that
/// field.
pub fn array_from_fn<T, F: Fn(usize) -> T>(f: F) -> [T; 10] {
    std::array::from_fn(f)
}

/// Closure as `Fn` — defines a closure inline and feeds it to
/// `array_from_fn`. Hits the auto-derived `core.ops.function.Fn` impl
/// for closures.
pub fn array_from_fn_closure() -> [usize; 10] {
    std::array::from_fn(|x: usize| x)
}

/// Array indexing returning a reference — covers `<[T; N] as Index<usize>>::index`.
pub fn array_index_ref(a: &[u8; 10], i: usize) -> &u8 {
    &a[i]
}

/// Array indexing returning by value (Copy types).
pub fn array_index_val(a: [u32; 4], i: usize) -> u32 {
    a[i]
}

/// Covers `<[T; N] as IndexMut<Range<usize>>>::index_mut` (pereq-dev/hax#2174).
pub fn array_write_range(a: &mut [u8; 8], src: &[u8]) {
    a[0..src.len()].copy_from_slice(src);
}

// ----- Slices ---------------------------------------------------------------

pub fn slice_len(x: &[u8]) -> usize {
    x.len()
}

/// `IterMut::next` yields an `&mut T`, so hax gives it a write-back return and
/// no `Iterator` instance can hold it -- `patch_lean.py` drops the ill-typed
/// record aeneas emits, keeping the function. This is what resolves to that
/// function, and what would catch its name drifting.
pub fn slice_iter_mut_bump_first(s: &mut [u32]) {
    if let Some(x) = s.iter_mut().next() {
        *x += 1;
    }
}

// NOTE: slice indexing through `SliceIndex` is currently excluded from
// our extraction.
// Re-enable the tests below once the full SliceIndex modeling lands
// (`Sealed` super-trait, `get_mut`/`index_mut` back-edges, raw-pointer
// `get_unchecked*`).
//
//   pub fn slice_index_usize(x: &[u8]) -> u8         { x[0] }
//   pub fn slice_index_range(x: &[u8]) -> &[u8]      { &x[1..3] }
//   pub fn slice_index_range_from(x: &[u8]) -> &[u8] { &x[2..] }
//   pub fn slice_index_range_to(x: &[u8]) -> &[u8]   { &x[..3] }
//   pub fn slice_index_range_full(x: &[u8]) -> &[u8] { &x[..] }

// ----- Range iteration ------------------------------------------------------

/// Exercises `core::iter::range::IteratorRange::next` (the iterator that
/// drives `for i in 0..n` loops over `Range<usize>`).
pub fn range_sum(n: usize) -> usize {
    let mut acc: usize = 0;
    for i in 0..n {
        acc = acc.wrapping_add(i);
    }
    acc
}

// ----- Closures and higher-order --------------------------------------------

pub fn call_fn<F: Fn(u8) -> u8>(f: F, x: u8) -> u8 {
    f(x)
}

pub fn call_fn_mut<F: FnMut(u8) -> u8>(mut f: F, x: u8) -> u8 {
    f(x)
}

// ----- alloc::vec::Vec ------------------------------------------------------
//
// Same gating as slices: `Vec::index` is excluded via ALLOC_CHARON_EXCLUDES.
//
//   pub fn vec_index_usize(x: &Vec<usize>) -> usize { x[0] }
//   pub fn vec_index_and_len(x: Vec<usize>) -> usize { x[0] + x.len() }
//   pub fn vec_index_range(x: &Vec<u8>) -> &[u8] { &x[1..3] }

// --- pure shape (Aeneas marks `~can_fail:false ~lift:false`) ---

pub fn vec_new() -> Vec<u32> {
    Vec::new()
}

pub fn vec_with_capacity(c: usize) -> Vec<u8> {
    Vec::with_capacity(c)
}

pub fn vec_len(v: &Vec<u32>) -> usize {
    v.len()
}

// --- monadic shape (push/insert/resize/extend_from_slice) ---

pub fn vec_push(mut v: Vec<u32>, x: u32) -> Vec<u32> {
    v.push(x);
    v
}

pub fn vec_push_two(mut v: Vec<u8>, x: u8, y: u8) -> Vec<u8> {
    v.push(x);
    v.push(y);
    v
}

pub fn vec_insert(mut v: Vec<u8>, i: usize, x: u8) -> Vec<u8> {
    v.insert(i, x);
    v
}

/// Indexed assignment resolves through `IndexMut` (`arr_set` above goes
/// through `Slice.update` instead, so it does not cover this).
pub fn vec_index_set(mut v: Vec<u32>, i: usize, x: u32) -> Vec<u32> {
    v[i] = x;
    v
}

pub fn vec_resize(mut v: Vec<u8>, n: usize, x: u8) -> Vec<u8> {
    v.resize(n, x);
    v
}

pub fn vec_extend_from_slice(mut v: Vec<u8>, s: &[u8]) -> Vec<u8> {
    v.extend_from_slice(s);
    v
}

/// `vec![x; n]` lowers to a call to `alloc::vec::from_elem`.
pub fn vec_from_elem(x: u32, n: usize) -> Vec<u32> {
    vec![x; n]
}

// ----- alloc::slice methods -------------------------------------------------
//
// `[T]::to_vec` is in Aeneas's builtin map.

pub fn slice_to_vec(s: &[u8]) -> Vec<u8> {
    s.to_vec()
}

// ----- alloc::boxed::Box ----------------------------------------------------
//
// Hax/Aeneas erase `Box<T>` to `T`, but we still want to make sure the
// surface compiles end-to-end.

pub fn box_new(x: u32) -> Box<u32> {
    Box::new(x)
}

pub fn box_deref(b: &Box<u8>) -> u8 {
    **b
}

// ----- Iterator provided methods (shim canary) ------------------------------
//
// The hand-written shims in `CoreModels/Core/FunsEpilogue.lean` are matched by
// name AND signature against aeneas's emission, which only extracting a client
// crate can check. One function per shim, so a missing or mis-signed one names
// itself in the Lean error. Receivers vary on purpose: concrete ones may get a
// per-impl specialisation, generic ones (`poly_*`) the dictionary-passing
// `.default` form, and a shim covering one need not cover the other.

// --- eager consumers, concrete receiver ---

pub fn iter_fold(n: usize) -> usize {
    (0..n).fold(0usize, |acc, x| acc.wrapping_add(x))
}

pub fn iter_count(n: usize) -> usize {
    (0..n).count()
}

pub fn iter_last(n: usize) -> Option<usize> {
    (0..n).last()
}

pub fn iter_all(n: usize) -> bool {
    (0..n).all(|x| x < n)
}

pub fn iter_any(n: usize) -> bool {
    (0..n).any(|x| x == 3)
}

pub fn iter_find(n: usize) -> Option<usize> {
    (0..n).find(|x| *x > 2)
}

pub fn iter_find_map(n: usize) -> Option<usize> {
    (0..n).find_map(|x| if x > 2 { Some(x) } else { None })
}

pub fn iter_position(n: usize) -> Option<usize> {
    (0..n).position(|x| x > 2)
}

pub fn iter_reduce(n: usize) -> Option<usize> {
    (0..n).reduce(|a, b| a.wrapping_add(b))
}

pub fn iter_min(n: usize) -> Option<usize> {
    (0..n).min()
}

pub fn iter_max(n: usize) -> Option<usize> {
    (0..n).max()
}

// PARKED — aeneas emits an inconsistent `FnMut` instance for a unit-returning
// closure: `call_mut : closure -> Usize -> Result closure`, where its own
// `FnMut` record expects `Result (Unit x closure)`. Nothing the model can fix.

//   pub fn iter_for_each(n: usize) {
//       (0..n).for_each(|_x| {})
//   }

// `nth` is not yet shimmed: its helper `iter_nth` is `hax_lib::exclude`d for a
// Lean forward reference to `core.Usize.Insts.CoreIterRangeStep`. Enable once
// the helper is emitted from `FunsPrologue.lean`.
//
//   pub fn iter_nth(n: usize) -> Option<usize> { (0..n).nth(2) }

// --- lazy adapters, concrete receiver; each terminated by a shimmed consumer ---

pub fn iter_map(n: usize) -> usize {
    (0..n)
        .map(|x| x.wrapping_mul(2))
        .fold(0usize, |a, b| a.wrapping_add(b))
}

pub fn iter_enumerate(n: usize) -> usize {
    (0..n)
        .enumerate()
        .fold(0usize, |a, (i, _x)| a.wrapping_add(i))
}

pub fn iter_step_by(n: usize) -> usize {
    (0..n).step_by(2).count()
}

pub fn iter_take(n: usize) -> usize {
    (0..n).take(3).count()
}

pub fn iter_skip(n: usize) -> usize {
    (0..n).skip(3).count()
}

pub fn iter_filter(n: usize) -> usize {
    (0..n).filter(|x| *x > 2).count()
}

pub fn iter_filter_map(n: usize) -> usize {
    (0..n)
        .filter_map(|x| if x > 2 { Some(x) } else { None })
        .count()
}

pub fn iter_take_while(n: usize) -> usize {
    (0..n).take_while(|x| *x < 5).count()
}

pub fn iter_skip_while(n: usize) -> usize {
    (0..n).skip_while(|x| *x < 5).count()
}

pub fn iter_map_while(n: usize) -> usize {
    (0..n)
        .map_while(|x| if x < 5 { Some(x) } else { None })
        .count()
}

// PARKED — same unit-returning-closure `FnMut` inconsistency as `iter_for_each`.

//   pub fn iter_inspect(n: usize) -> usize {
//       (0..n).inspect(|_x| {}).count()
//   }

pub fn iter_fuse(n: usize) -> usize {
    (0..n).fuse().count()
}

pub fn iter_zip(n: usize, m: usize) -> usize {
    (0..n).zip(0..m).count()
}

pub fn iter_chain(n: usize, m: usize) -> usize {
    (0..n).chain(0..m).count()
}

// PARKED — needs a Rust-side reshape, not a shim change like `zip`/`chain`.
// std has the closure (resp. item) yield a COLLECTION while the adapter stores
// an ITERATOR, but `FlatMap<I, U, F>` holds `Option<U>`, so its `::new` demands
// `FnMut F Item U` with `U` the stored iterator — which only unifies with the
// dictionary's `IntoIter` for the blanket `IntoIterator for I: Iterator`. Fix:
// index the structs by `U::IntoIter` and apply `into_iter` inside `next`.
//
//   pub fn iter_flat_map(n: usize) -> usize {
//       (0..n).flat_map(|i| 0..i).count()
//   }
//
//   pub fn iter_flatten(n: usize) -> usize {
//       (0..n).map(|i| 0..i).flatten().count()
//   }

pub fn iter_collect_vec(n: usize) -> Vec<usize> {
    (0..n).collect()
}

/// `collect::<Result<_, _>>()` — exercises the hand-written
/// `FromIterator<Result<A, E>> for Result<V, E>` in `FunsEpilogue.lean` (the
/// Rust impl is `hax_lib::exclude`d, so nothing else would reach the shim).
pub fn iter_collect_result(n: usize) -> Result<Vec<usize>, usize> {
    (0..n)
        .map(|x| if x == 7 { Err(x) } else { Ok(x) })
        .collect()
}

pub fn iter_map_collect_vec(n: usize) -> Vec<usize> {
    (0..n).map(|x| x.wrapping_mul(2)).collect()
}

// --- `rev` / `DoubleEndedIterator` ---
//
// `rev` needs `DoubleEndedIterator` for the whole receiver prefix. The model
// has `next_back` for `Range`, slice `Iter` and `Enumerate<I: DE + ES>`, so
// these three chains are in scope; `Map`/`Filter`/`StepBy` have no `next_back`
// yet, so `.map(f).rev()` is deliberately absent.

pub fn iter_rev_range(n: usize) -> usize {
    (0..n).rev().count()
}

pub fn iter_rev_slice(x: &[u8]) -> usize {
    x.iter().rev().count()
}

// PARKED — `DoubleEndedIterator for Enumerate<I>` needs both `ExactSizeIterator`
// and `DoubleEndedIterator`, and BOTH bound orders fail: std's order breaks
// extraction (`Enumerate<I>` reaches `Iterator` by two parent paths), the other
// order hands the dictionaries to this call site in the wrong positions. See the
// note on the impl in `core-models/src/core/iter.rs`.

//   pub fn iter_rev_enumerate(x: &[u8]) -> usize {
//       x.iter().enumerate().rev().count()
//   }

// --- slice `Iter` as receiver (a different concrete instance) ---

pub fn slice_iter_count(x: &[u8]) -> usize {
    x.iter().count()
}

pub fn slice_iter_fold(x: &[u8]) -> u8 {
    x.iter().fold(0u8, |a, b| a.wrapping_add(*b))
}

pub fn slice_iter_map_count(x: &[u8]) -> usize {
    x.iter().map(|b| b.wrapping_add(1)).count()
}

pub fn slice_iter_all(x: &[u8]) -> bool {
    x.iter().all(|b| *b > 0)
}

pub fn slice_iter_any(x: &[u8]) -> bool {
    x.iter().any(|b| *b == 3)
}

// --- generic receivers: force the dictionary-passing `.default` form ---

// PARKED — a GENERIC receiver makes aeneas emit a record-field projection
// (`dict.count`), which needs the provided methods to be fields of the
// `Iterator` structure. They are deliberately not, so that `Iterator` stays free
// of the IntoIterator/DoubleEndedIterator cycles. Generic-over-iterator client
// code is therefore unsupported for now.

//   pub fn poly_count<I: Iterator>(it: I) -> usize {
//       it.count()
//   }

// PARKED — generic receiver; see `poly_count` above.

//   pub fn poly_fold<I: Iterator<Item = usize>>(it: I) -> usize {
//       it.fold(0usize, |a, x| a.wrapping_add(x))
//   }

// PARKED — generic receiver; see `poly_count` above.

//   pub fn poly_map_count<I: Iterator<Item = usize>>(it: I) -> usize {
//       it.map(|x| x.wrapping_mul(2)).count()
//   }

// PARKED — generic receiver; see `poly_count` above.

//   pub fn poly_filter_count<I: Iterator<Item = usize>>(it: I) -> usize {
//       it.filter(|x| *x > 2).count()
//   }

// PARKED — generic receiver; see `poly_count` above.

//   pub fn poly_enumerate_fold<I: Iterator<Item = usize>>(it: I) -> usize {
//       it.enumerate().fold(0usize, |a, (i, _x)| a.wrapping_add(i))
//   }

// PARKED — generic receiver; see `poly_count` above.

//   pub fn poly_take_count<I: Iterator>(it: I, k: usize) -> usize {
//       it.take(k).count()
//   }

// PARKED — generic receiver; see `poly_count` above.

//   pub fn poly_collect_vec<I: Iterator<Item = usize>>(it: I) -> Vec<usize> {
//       it.collect()
//   }

// A COLLECTION, not an iterator, as the argument: only resolves because the
// shims take the faithful `U: IntoIterator` bound.
pub fn iter_zip_slice(x: &[u8], y: &[u8]) -> usize {
    x.iter().zip(y).count()
}

pub fn iter_chain_slice(x: &[u8], y: &[u8]) -> usize {
    x.iter().chain(y).count()
}
