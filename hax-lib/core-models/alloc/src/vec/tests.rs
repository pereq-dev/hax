//! Test suite shared by the two `vec` models.
//!
//! `vec` exists in two cfg-exclusive variants: the default one, whose `Vec`
//! has a single type parameter, and the `hax_backend_fstar` one, which keeps
//! std's explicit allocator parameter (`Vec<T, A = Global>`). Both are
//! declared as `mod tests;` from inside `mod vec`, resolving to `src/vec/tests.rs`, so exactly one is
//! compiled per cfg and these tests run against whichever is selected.
//!
//! The tests only ever spell the type as `super::Vec<T>`, which resolves to
//! `Vec<T>` in the default variant and to `Vec<T, Global>` in the F\* one.
//! Their surface is identical except for `drain`, which the default variant
//! makes generic in the allocator: tests call it through `model_drain`.

use crate::testing::Inject;
use proptest::prelude::*;

impl<T: Clone> Inject for Vec<T> {
    type Model = super::Vec<T>;
    fn inject(&self) -> super::Vec<T> {
        super::Vec::<T>(
            rust_primitives::sequence::seq_from_boxed_slice(self.clone().into_boxed_slice()),
            #[cfg(hax_backend_fstar)]
            std::marker::PhantomData,
        )
    }
}

proptest! {
    #[test]
    fn test_len(v in prop::collection::vec(any::<u8>(), 0..100)) {
        prop_assert_eq!(v.inject().len(), v.len());
    }

    #[test]
    fn test_is_empty(v in prop::collection::vec(any::<u8>(), 0..100)) {
        prop_assert_eq!(v.inject().is_empty(), v.is_empty());
    }

    #[test]
    fn test_as_slice(v in prop::collection::vec(any::<u8>(), 0..100)) {
        let model = v.inject();
        prop_assert_eq!(model.as_slice(), v.as_slice());
    }

    // `as_mut_slice` is `cfg(not(hax_backend_fstar))`, like `DerefMut for Vec`
    // (its only user): F* cannot model a `&mut [T]` return.
    // `DerefMut for Vec` has the same F* gating as `as_mut_slice`, its only user.
    #[cfg(not(hax_backend_fstar))]
    #[test]
    fn test_deref_mut(v in prop::collection::vec(any::<u8>(), 0..100), x in any::<u8>()) {
        let mut model = v.clone().inject();
        let mut std_v = v.clone();
        if !v.is_empty() {
            // `&mut *model` is what goes through `DerefMut`.
            let s: &mut [u8] = &mut *model;
            s[0] = x;
            std_v[0] = x;
        }
        prop_assert_eq!(model.as_slice(), std_v.as_slice());
    }

    #[cfg(not(hax_backend_fstar))]
    #[test]
    fn test_as_mut_slice(v in prop::collection::vec(any::<u8>(), 0..100), x in any::<u8>()) {
        let mut model = v.clone().inject();
        let mut std_v = v.clone();
        if !v.is_empty() {
            model.as_mut_slice()[0] = x;
            std_v.as_mut_slice()[0] = x;
        }
        prop_assert_eq!(model.as_slice(), std_v.as_slice());
    }

    #[test]
    fn test_push(v in prop::collection::vec(any::<u8>(), 0..50), x in any::<u8>()) {
        let mut model = v.inject();
        model.push(x);
        let mut std_v = v.clone();
        std_v.push(x);
        prop_assert_eq!(model, std_v.inject());
    }

    #[test]
    fn test_pop(v in prop::collection::vec(any::<u8>(), 0..50)) {
        let mut model = v.inject();
        let mut std_v = v.clone();
        prop_assert_eq!(model.pop(), std_v.pop());
        prop_assert_eq!(model, std_v.inject());
    }

    #[test]
    fn test_index(v in prop::collection::vec(any::<u8>(), 1..50)) {
        let model = v.inject();
        for i in 0..v.len() {
            prop_assert_eq!(model[i], v[i]);
        }
    }

    #[test]
    fn test_index_range(v in prop::collection::vec(any::<u8>(), 0..50), start in 0usize..50, len in 0usize..50) {
        let start = start.min(v.len());
        let end = (start + len).min(v.len());
        let model = v.inject();
        prop_assert_eq!(&model[start..end], &v[start..end]);
    }

    #[test]
    fn test_insert(v in prop::collection::vec(any::<u8>(), 0..50), x in any::<u8>(), idx in 0usize..50) {
        if idx <= v.len() {
            let mut model = v.inject();
            model.insert(idx, x);
            let mut std_v = v.clone();
            std_v.insert(idx, x);
            prop_assert_eq!(model, std_v.inject());
        }
    }

    #[test]
    fn test_remove(v in prop::collection::vec(any::<u8>(), 1..50), idx in 0usize..50) {
        if idx < v.len() {
            let mut model = v.inject();
            let mut std_v = v.clone();
            prop_assert_eq!(model.remove(idx), std_v.remove(idx));
            prop_assert_eq!(model, std_v.inject());
        }
    }

    // `use_last` reaches the `n == len - 1` arm, which returns the popped
    // element directly; a uniform index hits it about once every `len` cases,
    // so leaving it to chance makes the arm's coverage flaky.
    #[test]
    fn test_swap_remove(
        v in prop::collection::vec(any::<u8>(), 1..50),
        idx in 0usize..50,
        use_last in any::<bool>(),
    ) {
        let idx = if use_last { v.len() - 1 } else { idx };
        if idx < v.len() {
            let mut model = v.inject();
            let mut std_v = v.clone();
            prop_assert_eq!(model.swap_remove(idx), std_v.swap_remove(idx));
            prop_assert_eq!(model, std_v.inject());
        }
    }

    #[test]
    fn test_truncate(v in prop::collection::vec(any::<u8>(), 0..50), n in 0usize..60) {
        let mut model = v.inject();
        let mut std_v = v.clone();
        model.truncate(n);
        std_v.truncate(n);
        prop_assert_eq!(model, std_v.inject());
    }

    #[test]
    fn test_clear(v in prop::collection::vec(any::<u8>(), 0..50)) {
        let mut model = v.inject();
        let mut std_v = v.clone();
        model.clear();
        std_v.clear();
        prop_assert_eq!(model, std_v.inject());
    }

    #[test]
    fn test_resize(v in prop::collection::vec(any::<u8>(), 0..50), n in 0usize..60, x in any::<u8>()) {
        let mut model = v.inject();
        let mut std_v = v.clone();
        model.resize(n, x);
        std_v.resize(n, x);
        prop_assert_eq!(model, std_v.inject());
    }

    #[test]
    fn test_split_off(v in prop::collection::vec(any::<u8>(), 0..50), at in 0usize..50) {
        if at <= v.len() {
            let mut model = v.inject();
            let mut std_v = v.clone();
            let model_tail = model.split_off(at);
            let std_tail = std_v.split_off(at);
            prop_assert_eq!(model, std_v.inject());
            prop_assert_eq!(model_tail, std_tail.inject());
        }
    }

    #[test]
    fn test_append(v1 in prop::collection::vec(any::<u8>(), 0..50), v2 in prop::collection::vec(any::<u8>(), 0..50)) {
        let mut model1 = v1.inject();
        model1.append(&mut v2.inject());
        let mut std_v = v1.clone();
        std_v.append(&mut v2.clone());
        prop_assert_eq!(model1, std_v.inject());
    }

    #[test]
    fn test_extend_from_slice(v in prop::collection::vec(any::<u8>(), 0..50), ext in prop::collection::vec(any::<u8>(), 0..50)) {
        let mut model = v.inject();
        model.extend_from_slice(&ext);
        let mut std_v = v.clone();
        std_v.extend_from_slice(&ext);
        prop_assert_eq!(model, std_v.inject());
    }

    #[test]
    fn test_from_elem(x in any::<u8>(), len in 0usize..100) {
        let model = super::from_elem(x, len);
        prop_assert_eq!(model, vec![x; len].inject());
    }

    #[test]
    fn test_from_iter(v in prop::collection::vec(any::<u8>(), 0..50)) {
        let model: super::Vec<u8> = v.iter().copied().collect();
        prop_assert_eq!(model, v.inject());
    }

    /// Every range form, built with `start <= end <= len` so that none panics.
    #[test]
    fn test_drain(
        (v, start, end) in prop::collection::vec(any::<u8>(), 0..20)
            .prop_flat_map(|v| {
                let n = v.len();
                (Just(v), 0..=n)
            })
            .prop_flat_map(|(v, start)| {
                let n = v.len();
                (Just(v), Just(start), start..=n)
            }),
    ) {
        use std::ops::Bound;
        check_drain(&v, start..end)?;
        check_drain(&v, start..)?;
        check_drain(&v, ..end)?;
        check_drain(&v, ..)?;
        check_drain(&v, (Bound::Included(start), Bound::Excluded(end)))?;
        if end > start {
            check_drain(&v, start..=end - 1)?;
        }
        if end > 0 {
            check_drain(&v, ..=end - 1)?;
        }
        if start > 0 {
            check_drain(&v, (Bound::Excluded(start - 1), Bound::Excluded(end)))?;
        }
    }
}

/// `drain` on either `Vec` variant; the default one leaves the allocator open.
fn model_drain<R: std::ops::RangeBounds<usize>>(
    model: &mut super::Vec<u8>,
    range: R,
) -> std::vec::Vec<u8> {
    #[cfg(not(hax_backend_fstar))]
    let drain = model.drain::<crate::alloc::Global, R>(range);
    #[cfg(hax_backend_fstar)]
    let drain = model.drain(range);
    drain.collect()
}

/// The drained elements and what is left in the `Vec` must both match std.
fn check_drain<R: std::ops::RangeBounds<usize> + Clone>(
    v: &[u8],
    range: R,
) -> Result<(), TestCaseError> {
    let mut model = v.to_vec().inject();
    let mut std_v = v.to_vec();
    let drained: std::vec::Vec<u8> = model_drain(&mut model, range.clone());
    let std_drained: std::vec::Vec<u8> = std_v.drain(range).collect();
    prop_assert_eq!(drained, std_drained);
    prop_assert_eq!(model, std_v.inject());
    Ok(())
}

#[test]
fn test_new() {
    let model: super::Vec<u8> = super::Vec::new();
    let std_v: std::vec::Vec<u8> = std::vec::Vec::new();
    assert_eq!(model, std_v.inject());
}

#[test]
fn test_with_capacity() {
    let model: super::Vec<u8> = super::Vec::with_capacity(10);
    let std_v: std::vec::Vec<u8> = std::vec::Vec::with_capacity(10);
    assert_eq!(model, std_v.inject());
}

// Only the default variant models `Default`: adding the impl to the F* variant
// would shift F*'s positional impl-block numbering and rename every `Vec` method.
#[cfg(not(hax_backend_fstar))]
#[test]
fn test_default() {
    let model: super::Vec<u8> = Default::default();
    let std_v: std::vec::Vec<u8> = Default::default();
    assert_eq!(model, std_v.inject());
}

// ----- Clone / PartialEq / IntoIterator -------
//
// The F* variant of `vec` models none of these three, so the tests below are
// specific to the default variant. (Its `PartialEq` is what `prop_assert_eq!`
// uses above; under `hax_backend_fstar` that comes from a `cfg(test)` derive.)

#[cfg(not(hax_backend_fstar))]
proptest! {
    #[test]
    fn test_vec_clone(v in prop::collection::vec(any::<u8>(), 0..30)) {
        // Compare the clone's contents to std directly (independent of
        // the model's own `PartialEq`, which is tested separately).
        let cloned = v.inject().clone();
        prop_assert_eq!(cloned.as_slice(), v.as_slice());
    }

    #[test]
    fn test_vec_eq(
        a in prop::collection::vec(any::<u8>(), 0..15),
        b in prop::collection::vec(any::<u8>(), 0..15),
    ) {
        prop_assert_eq!(a.inject() == b.inject(), a == b);
    }

    // `v[i] = x` goes through the model's `IndexMut`.
    #[test]
    fn test_vec_index_mut(v in prop::collection::vec(any::<u8>(), 1..20), x in any::<u8>()) {
        let i = x as usize % v.len();
        let mut model = v.inject();
        let mut std_v = v.clone();
        model[i] = x;
        std_v[i] = x;
        prop_assert_eq!(model.as_slice(), std_v.as_slice());
    }

    // Small domain and an explicit equal case: `ne` inverts `eq`, so the equal
    // pair is the one worth reaching.
    #[test]
    fn test_vec_ne(
        a in prop::collection::vec(0u8..4, 0..6),
        b in prop::collection::vec(0u8..4, 0..6),
        use_equal in any::<bool>(),
    ) {
        let b = if use_equal { a.clone() } else { b };
        prop_assert_eq!(a.inject() != b.inject(), a != b);
    }

    #[test]
    fn test_vec_into_iter(v in prop::collection::vec(any::<u8>(), 0..30)) {
        let mut it = v.inject().into_iter();
        let mut collected: std::vec::Vec<u8> = std::vec::Vec::new();
        while let Some(x) = it.next() {
            collected.push(x);
        }
        prop_assert_eq!(collected.as_slice(), v.as_slice());
    }
}

// The F* variant models no `IntoIterator` for `Vec`, so its `IntoIter` has to be
// built by hand.
#[cfg(hax_backend_fstar)]
proptest! {
    #[test]
    fn test_into_iter_direct(v in prop::collection::vec(any::<u8>(), 0..30)) {
        let mut it = super::into_iter::IntoIter(
            rust_primitives::sequence::seq_from_boxed_slice(v.clone().into_boxed_slice()),
        );
        let mut collected = std::vec::Vec::new();
        while let Some(x) = it.next() {
            collected.push(x);
        }
        prop_assert_eq!(collected.as_slice(), v.as_slice());
    }
}

// ----- panics ----------------------------------------------------------------

fn vec_of(n: usize) -> (super::Vec<u8>, Vec<u8>) {
    let real: Vec<u8> = (0..n as u8).collect();
    (real.inject(), real)
}

#[test]
fn test_insert_past_end_panics() {
    let (mut model, mut real) = vec_of(3);
    let i = std::hint::black_box(4usize);
    crate::testing::panics_like_core(|| model.insert(i, 9), || real.insert(i, 9));
}

#[test]
fn test_split_off_past_end_panics() {
    let (mut model, mut real) = vec_of(3);
    let at = std::hint::black_box(4usize);
    crate::testing::panics_like_core(|| model.split_off(at), || real.split_off(at));
}

#[test]
fn test_index_out_of_bounds_panics() {
    let (model, real) = vec_of(3);
    let i = std::hint::black_box(3usize);
    crate::testing::panics_like_core(|| model[i], || real[i]);
}

#[test]
fn test_drain_out_of_range_panics() {
    let end = std::hint::black_box(4usize);
    let (mut model, mut real) = vec_of(3);
    crate::testing::panics_like_core(|| model_drain(&mut model, 1..end), || real.drain(1..end));
    let (mut model, mut real) = vec_of(3);
    crate::testing::panics_like_core(|| model_drain(&mut model, 2..1), || real.drain(2..1));
    let (mut model, mut real) = vec_of(3);
    crate::testing::panics_like_core(
        || model_drain(&mut model, ..=usize::MAX),
        || real.drain(..=usize::MAX),
    );
}

#[test]
fn test_remove_past_end_panics() {
    let (mut model, mut real) = vec_of(3);
    let i = std::hint::black_box(3usize);
    crate::testing::panics_like_core(|| model.remove(i), || real.remove(i));
}

/// `From<[T; N]>` — only in the default `vec` variant (the F* one carries the
/// allocator parameter, which would need its own impl).
#[cfg(not(hax_backend_fstar))]
#[test]
fn test_from_array() {
    let a: [u8; 3] = [1, 2, 3];
    let model: super::Vec<u8> = From::from(a);
    let real: Vec<u8> = From::from(a);
    assert_eq!(model, real.inject());
}

#[cfg(not(hax_backend_fstar))]
proptest! {
    #[test]
    fn test_extend(a in prop::collection::vec(any::<u8>(), 0..8), b in prop::collection::vec(any::<u8>(), 0..8)) {
        let mut model: super::Vec<u8> = a.inject();
        std::iter::Extend::extend(&mut model, b.clone());
        std::iter::Extend::extend(&mut model, b.iter());
        let mut expected = a.clone();
        expected.extend(b.clone());
        expected.extend(b.iter());
        prop_assert_eq!(model, expected.inject());
    }
}

#[cfg(not(hax_backend_fstar))]
proptest! {
    #[test]
    fn test_borrow_slice(a in prop::collection::vec(any::<u8>(), 0..8)) {
        let model: super::Vec<u8> = a.inject();
        prop_assert_eq!(core::borrow::Borrow::<[u8]>::borrow(&model), &a[..]);
    }
}
