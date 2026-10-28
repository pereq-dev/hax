use rust_primitives::{sequence::*, slice::*};

/// See [`std::array::TryFromSliceError`]
pub struct TryFromSliceError;

// Dummy type to allow impls
// F*-only: `charon::exclude` would drop this dummy type while its `impl`
// blocks still reference it (see f32.rs).
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
struct Array<T, const N: usize>([T; N]);

// Array impls to get the right disambiguator (https://github.com/pereq-dev/hax/issues/828)
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}
impl<T> Array<T, 0> {}

impl<T, const N: usize> Array<T, N> {
    /// See [`std::array::map`]
    // `FnMut` (not `Fn`) matches `std::array::map`'s bound; aeneas synthesises the
    // closure's `FnMut` instance at a `[T; N]::map(closure)` call site, so a `Fn`
    // bound here fails to unify with it.
    #[cfg(not(hax_backend_fstar))]
    pub fn map<F: FnMut(T) -> U, U>(s: [T; N], f: F) -> [U; N] {
        array_map(s, f)
    }
    #[cfg(hax_backend_fstar)]
    pub fn map<F: crate::ops::function::FnOnce<T, Output = U>, U>(
        s: [T; N],
        f: fn(T) -> U,
    ) -> [U; N] {
        array_map(s, f)
    }
    /// See [`std::array::as_slice`]
    pub fn as_slice(s: &[T; N]) -> &[T] {
        array_as_slice(s)
    }
    /// See [`std::array::as_mut_slice`]
    // Lean-only, like the `IndexMut` impl below that consumes it.
    #[cfg(not(hax_backend_fstar))]
    pub fn as_mut_slice(s: &mut [T; N]) -> &mut [T] {
        array_as_mut_slice(s)
    }
    /// See [`std::array::each_ref`]
    pub fn each_ref(s: &[T; N]) -> [&T; N] {
        array_from_fn(|i| array_index(s, i))
    }
}

#[hax_lib::fstar::replace("let from_fn = Rust_primitives.Slice.array_from_fn")]
pub fn from_fn<T, const N: usize, F: FnMut(usize) -> T>(f: F) -> [T; N] {
    array_from_fn(f)
}

/// See [`std::array::repeat`]
// Lean-only: Aeneas lowers an array repeat expression `[x; N]` to this
// function, while the F* backend uses `Rust_primitives.Hax.repeat`.
#[cfg(not(hax_backend_fstar))]
pub fn repeat<T: Clone, const N: usize>(val: T) -> [T; N] {
    // Like std, `seq_create` clones into every slot but the last and moves `val`
    // into that one; the elements are then moved out in order.
    let mut s = seq_create(val, N);
    array_from_fn(move |_i| seq_remove(&mut s, 0))
}

#[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
impl<T, const N: usize> crate::iter::traits::collect::IntoIterator for [T; N] {
    type Item = T;
    type IntoIter = iter::IntoIter<T, N>;
    fn into_iter(self) -> iter::IntoIter<T, N> {
        iter::IntoIter(seq_from_array(self))
    }
}

use crate::ops::{
    index::Index,
    range::{Range, RangeFrom, RangeFull, RangeTo},
};

#[cfg(not(hax_backend_fstar))]
#[hax_lib::attributes]
#[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
impl<T, I, const N: usize> crate::ops::index::Index<I> for [T; N]
where
    [T]: Index<I>,
{
    type Output = <[T] as Index<I>>::Output;
    fn index(&self, i: I) -> &Self::Output {
        self.as_slice().index(i)
    }
}

/// Mirrors the `Index<I>` impl above; without it, writing through a range does
/// not extract (pereq-dev/hax#2174). Lean-only, like the slice impl it delegates to.
#[cfg(not(hax_backend_fstar))]
#[hax_lib::attributes]
#[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
impl<T, I, const N: usize> crate::ops::index::IndexMut<I> for [T; N]
where
    [T]: crate::ops::index::IndexMut<I>,
{
    fn index_mut(&mut self, i: I) -> &mut Self::Output {
        <[T] as crate::ops::index::IndexMut<I>>::index_mut(Array::as_mut_slice(self), i)
    }
}

#[cfg(hax_backend_fstar)]
#[hax_lib::attributes]
impl<T, const N: usize> Index<usize> for [T; N] {
    type Output = T;
    #[hax_lib::requires(i < N)]
    fn index(&self, i: usize) -> &T {
        rust_primitives::slice::array_index(self, i)
    }
}

#[cfg(hax_backend_fstar)]
#[hax_lib::attributes]
#[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
impl<T, const N: usize> Index<Range<usize>> for [T; N] {
    type Output = [T];
    #[hax_lib::requires(i.start <= i.end && i.end <= N)]
    fn index(&self, i: Range<usize>) -> &[T] {
        array_slice(self, i.start, i.end)
    }
}
#[cfg(hax_backend_fstar)]
#[hax_lib::attributes]
#[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
impl<T, const N: usize> Index<RangeTo<usize>> for [T; N] {
    type Output = [T];
    #[hax_lib::requires(i.end <= N)]
    fn index(&self, i: RangeTo<usize>) -> &[T] {
        array_slice(self, 0, i.end)
    }
}
#[cfg(hax_backend_fstar)]
#[hax_lib::attributes]
#[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
impl<T, const N: usize> Index<RangeFrom<usize>> for [T; N] {
    type Output = [T];
    #[hax_lib::requires(i.start <= N)]
    fn index(&self, i: RangeFrom<usize>) -> &[T] {
        array_slice(self, i.start, N)
    }
}
#[cfg(hax_backend_fstar)]
#[hax_lib::attributes]
#[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
impl<T, const N: usize> Index<RangeFull> for [T; N] {
    type Output = [T];
    fn index(&self, i: RangeFull) -> &[T] {
        array_slice(self, 0, N)
    }
}

// Not for `hax_backend_fstar`: that model's blanket `impl<T> Clone for T`
// already covers arrays, and both in scope fails coherence.
#[cfg(not(hax_backend_fstar))]
impl<T: crate::clone::Clone, const N: usize> crate::clone::Clone for [T; N] {
    fn clone(self) -> Self {
        Array::map(self, |x| x.clone())
    }
    // Overridden, not left to the default: charon only emits the per-impl
    // `clone_from` symbol when the impl overrides it, and clients resolve to it.
    fn clone_from(self, source: Self) -> Self {
        source.clone()
    }
}

pub mod equality {
    use rust_primitives::slice::array_index;

    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    impl<T: crate::cmp::PartialEq<U>, U, const N: usize> crate::cmp::PartialEq<[U; N]> for [T; N] {
        #[cfg(not(hax_backend_fstar))]
        fn ne(&self, other: &[U; N]) -> bool {
            self.eq(other) == false
        }
        fn eq(&self, other: &[U; N]) -> bool {
            let mut i = 0;
            while i < N {
                if !array_index(self, i).eq(array_index(other, i)) {
                    return false;
                }
                i += 1;
            }
            true
        }
    }
}

// The items below are appended at the end of the module. hax's F* disambiguator
// numbers a module's *annotated* impls top-to-bottom, and every impl above is
// annotated, so appending here leaves the published `impl_NN` names untouched.

/// See [`std::default::Default`] for `[T; N]`
impl<T: crate::default::Default, const N: usize> crate::default::Default for [T; N] {
    fn default() -> [T; N] {
        array_from_fn(|_i| <T as crate::default::Default>::default())
    }
}

/// See [`std::fmt::Debug`] for `[T; N]`
#[cfg(not(hax_backend_fstar))]
impl<T: crate::fmt::Debug, const N: usize> crate::fmt::Debug for [T; N] {
    fn fmt(&self, f: &mut crate::fmt::Formatter) -> crate::fmt::Result {
        crate::fmt::Result::Ok(())
    }
}

/// See [`std::fmt::Debug`] for [`TryFromSliceError`]
#[cfg(not(hax_backend_fstar))]
impl crate::fmt::Debug for TryFromSliceError {
    fn fmt(&self, f: &mut crate::fmt::Formatter) -> crate::fmt::Result {
        crate::fmt::Result::Ok(())
    }
}

/// See [`std::convert::AsRef`] for `[T; N]`
impl<T, const N: usize> crate::convert::AsRef<[T]> for [T; N] {
    fn as_ref(&self) -> &[T] {
        array_as_slice(self)
    }
}

mod iter {
    use crate::option::Option;
    use rust_primitives::sequence::*;
    pub struct IntoIter<T, const N: usize>(pub Seq<T>);
    #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
    impl<T, const N: usize> crate::iter::traits::iterator::Iterator for IntoIter<T, N> {
        type Item = T;
        fn next(&mut self) -> Option<T> {
            if seq_len(&self.0) == 0 {
                Option::None
            } else {
                let res = seq_remove(&mut self.0, 0);
                Option::Some(res)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(hax_backend_fstar))]
    use crate::testing::CloneWitness;
    use crate::testing::Inject;

    impl<T: Inject, const N: usize> Inject for [T; N] {
        type Model = [T::Model; N];
        fn inject(&self) -> Self::Model {
            std::array::from_fn(|i| self[i].inject())
        }
    }

    use proptest::prelude::*;

    #[test]
    fn test_array_default() {
        assert_eq!(
            <[u8; 4] as crate::default::Default>::default(),
            <[u8; 4] as std::default::Default>::default()
        );
        assert_eq!(
            <[u8; 0] as crate::default::Default>::default(),
            <[u8; 0] as std::default::Default>::default()
        );
    }

    /// `Debug` for arrays and for `TryFromSliceError` render nothing, like every
    /// other `Debug` in the model.
    #[cfg(not(hax_backend_fstar))]
    #[test]
    fn test_array_debug() {
        let mut f = crate::fmt::Formatter;
        assert!(crate::fmt::Debug::fmt(&[1u8, 2, 3], &mut f).is_ok());
        assert!(crate::fmt::Debug::fmt(&super::TryFromSliceError, &mut f).is_ok());
    }

    #[cfg(not(hax_backend_fstar))]
    proptest! {
        #[test]
        fn test_array_clone_from(a in any::<[u8; 3]>(), b in any::<[u8; 3]>()) {
            let mut std_dst = a;
            std::clone::Clone::clone_from(&mut std_dst, &b);
            prop_assert_eq!(crate::clone::Clone::clone_from(a, b), std_dst);
        }

        #[test]
        fn test_array_as_ref(a in any::<[u8; 3]>()) {
            prop_assert_eq!(
                crate::convert::AsRef::<[u8]>::as_ref(&a),
                std::convert::AsRef::<[u8]>::as_ref(&a)
            );
        }

        #[test]
        fn test_array_index_mut(a in any::<[u8; 3]>(), i in 0usize..3, v in any::<u8>()) {
            let mut model = a;
            let mut std_ = a;
            *crate::ops::index::IndexMut::index_mut(&mut model, i) = v;
            *std::ops::IndexMut::index_mut(&mut std_, i) = v;
            prop_assert_eq!(model, std_);
        }
    }

    // Equal arrays are the case `ne` inverts, so reach that case explicitly.
    #[cfg(not(hax_backend_fstar))]
    proptest! {
        #[test]
        fn test_array_ne(a in any::<[u8; 3]>(), b in any::<[u8; 3]>(), use_equal in any::<bool>()) {
            let b = if use_equal { a } else { b };
            prop_assert_eq!(
                <[u8; 3] as crate::cmp::PartialEq<[u8; 3]>>::ne(&a, &b),
                a != b
            );
        }
    }

    // Under the F* cfg `map` takes a `fn` pointer plus a phantom `F: FnOnce<..>`
    // parameter (the backend types the function through it). Nothing in the model
    // implements that trait, so the test supplies a witness.
    #[cfg(hax_backend_fstar)]
    mod fstar_map {
        use crate::testing::Inject;
        use proptest::prelude::*;

        fn triple(x: u8) -> u8 {
            x.wrapping_mul(3)
        }

        struct Triple;

        impl crate::ops::function::FnOnce<u8> for Triple {
            type Output = u8;
            fn call_once(&self, args: u8) -> u8 {
                triple(args)
            }
        }

        proptest! {

            #[test]
            fn test_map(arr in any::<[u8; 4]>()) {
                prop_assert_eq!(
                    super::super::Array::<u8, 4>::map::<Triple, u8>(arr.inject(), triple),
                    arr.map(triple)
                );
            }

            #[test]
            fn test_witness_call_once(x in any::<u8>()) {
                prop_assert_eq!(
                    crate::ops::function::FnOnce::call_once(&Triple, x),
                    triple(x)
                );
            }
        }
    }

    proptest! {
        // Under the F* cfg `map` takes a `fn`, which this closure can't coerce to.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_map(arr in any::<[u8; 4]>(), table in any::<[u8; 256]>()) {
            let f = |x: u8| table[x as usize];
            prop_assert_eq!(super::Array::<u8, 4>::map(arr.inject(), f), arr.map(f));
        }

        #[test]
        fn test_from_fn(table in any::<[u8; 256]>()) {
            let f = |i: usize| table[i];
            let model: [u8; 4] = super::from_fn(f);
            prop_assert_eq!(model, std::array::from_fn::<u8, 4, _>(f));
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_repeat(x in any::<u8>()) {
            prop_assert_eq!(super::repeat::<u8, 4>(x), std::array::repeat::<u8, 4>(x));
            prop_assert_eq!(super::repeat::<u8, 0>(x), std::array::repeat::<u8, 0>(x));
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_repeat_moves_last(x in any::<u8>()) {
            prop_assert_eq!(
                super::repeat::<CloneWitness, 3>(CloneWitness::new(x)),
                std::array::repeat::<CloneWitness, 3>(CloneWitness::new(x))
            );
        }

        #[test]
        fn test_clone(arr in any::<[u8; 4]>()) {
            prop_assert_eq!(
                crate::clone::Clone::clone(arr.inject()),
                arr.clone().inject()
            );
        }

        #[test]
        fn test_as_slice(arr in any::<[u8; 4]>()) {
            let model_arr = arr.inject();
            prop_assert_eq!(
                super::Array::<u8, 4>::as_slice(&model_arr),
                arr.as_slice()
            );
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_as_mut_slice(arr in any::<[u8; 4]>(), idx in 0usize..4, x in any::<u8>()) {
            let mut model_arr = arr.inject();
            super::Array::<u8, 4>::as_mut_slice(&mut model_arr)[idx] = x;
            let mut expected = arr;
            expected[idx] = x;
            prop_assert_eq!(model_arr, expected.inject());
        }

        #[test]
        fn test_index_usize(arr in any::<[u8; 4]>(), idx in 0usize..4) {
            let model_arr = arr.inject();
            prop_assert_eq!(model_arr[idx], arr[idx]);
        }

        #[test]
        fn test_index_range(arr in any::<[u8; 8]>(), start in 0usize..8, len in 0usize..8) {
            let end = (start + len).min(8);
            let model_arr = arr.inject();
            prop_assert_eq!(&model_arr[start..end], &arr[start..end]);
        }

        #[test]
        fn test_index_range_to(arr in any::<[u8; 8]>(), end in 0usize..=8) {
            let model_arr = arr.inject();
            prop_assert_eq!(&model_arr[..end], &arr[..end]);
        }

        #[test]
        fn test_index_range_from(arr in any::<[u8; 8]>(), start in 0usize..=8) {
            let model_arr = arr.inject();
            prop_assert_eq!(&model_arr[start..], &arr[start..]);
        }

        #[test]
        fn test_index_range_full(arr in any::<[u8; 8]>()) {
            let model_arr = arr.inject();
            prop_assert_eq!(&model_arr[..], &arr[..]);
        }

        #[test]
        fn test_each_ref(arr in any::<[u8; 4]>()) {
            let model_arr = arr.inject();
            let model_refs = super::Array::<u8, 4>::each_ref(&model_arr);
            let std_refs = arr.each_ref();
            for i in 0..4 {
                prop_assert_eq!(*model_refs[i], *std_refs[i]);
            }
        }

        #[test]
        fn test_eq(a in any::<[u8; 4]>(), b in any::<[u8; 4]>()) {
            let ma = a.inject();
            let mb = b.inject();
            prop_assert_eq!(crate::cmp::PartialEq::eq(&ma, &mb), a == b);
        }

        // Two independent arrays are essentially never equal, so the `true` exit
        // of the comparison loop needs a reflexive case.
        #[test]
        fn test_eq_reflexive(a in any::<[u8; 4]>()) {
            let ma = a.inject();
            prop_assert_eq!(crate::cmp::PartialEq::eq(&ma, &ma), a == a);
        }

        #[test]
        fn test_into_iter(arr in any::<[u8; 4]>()) {
            let mut it = crate::iter::traits::collect::IntoIterator::into_iter(arr.inject());
            let mut collected = std::vec::Vec::new();
            while let crate::option::Option::Some(x) =
                crate::iter::traits::iterator::Iterator::next(&mut it)
            {
                collected.push(x);
            }
            prop_assert_eq!(collected, arr.into_iter().collect::<std::vec::Vec<u8>>());
        }

        // `arr[idx]` above resolves to std's indexing; these spell out the
        // model's `Index` impls for `[T; N]`.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_model_index_usize(arr in any::<[u8; 4]>(), idx in 0usize..4) {
            let m = arr.inject();
            prop_assert_eq!(
                <[u8; 4] as crate::ops::index::Index<usize>>::index(&m, idx),
                &arr[idx]
            );
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_model_index_range(arr in any::<[u8; 8]>(), start in 0usize..8, len in 0usize..8) {
            let end = (start + len).min(8);
            let m = arr.inject();
            prop_assert_eq!(
                <[u8; 8] as crate::ops::index::Index<crate::ops::range::Range<usize>>>::index(
                    &m,
                    crate::ops::range::Range { start, end }
                ),
                &arr[start..end]
            );
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_model_index_mut_range(
            arr in any::<[u8; 8]>(),
            start in 0usize..8,
            len in 0usize..8,
            fill in any::<u8>(),
        ) {
            let end = (start + len).min(8);
            let mut m = arr.inject();
            <[u8; 8] as crate::ops::index::IndexMut<crate::ops::range::Range<usize>>>::index_mut(
                &mut m,
                crate::ops::range::Range { start, end },
            )
            .fill(fill);
            let mut expected = arr;
            expected[start..end].fill(fill);
            prop_assert_eq!(m, expected.inject());
        }

        // The F* variant has one `Index` impl per range kind, all going through
        // `rust_primitives::slice::array_slice`.
        #[cfg(hax_backend_fstar)]
        #[test]
        fn test_model_index_range(arr in any::<[u8; 8]>(), start in 0usize..8, len in 0usize..8) {
            let end = (start + len).min(8);
            let m = arr.inject();
            prop_assert_eq!(
                crate::ops::index::Index::index(&m, crate::ops::range::Range { start, end }),
                &arr[start..end]
            );
            prop_assert_eq!(
                crate::ops::index::Index::index(&m, crate::ops::range::RangeTo { end }),
                &arr[..end]
            );
            prop_assert_eq!(
                crate::ops::index::Index::index(&m, crate::ops::range::RangeFrom { start }),
                &arr[start..]
            );
            prop_assert_eq!(
                crate::ops::index::Index::index(&m, crate::ops::range::RangeFull),
                &arr[..]
            );
        }

        #[cfg(hax_backend_fstar)]
        #[test]
        fn test_model_index_usize(arr in any::<[u8; 4]>(), idx in 0usize..4) {
            let m = arr.inject();
            prop_assert_eq!(crate::ops::index::Index::index(&m, idx), &arr[idx]);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_array_clone_applies_element_clone(arr in any::<[u8; 3]>()) {
            prop_assert_eq!(
                crate::clone::Clone::clone(arr.map(CloneWitness::new)),
                arr.map(CloneWitness::new).clone()
            );
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_array_clone_from_applies_element_clone(
            dst in any::<[u8; 3]>(),
            src in any::<[u8; 3]>(),
        ) {
            let mut std_dst = dst.map(CloneWitness::new);
            std::clone::Clone::clone_from(&mut std_dst, &src.map(CloneWitness::new));
            prop_assert_eq!(
                crate::clone::Clone::clone_from(dst.map(CloneWitness::new), src.map(CloneWitness::new)),
                std_dst
            );
        }
    }
}
