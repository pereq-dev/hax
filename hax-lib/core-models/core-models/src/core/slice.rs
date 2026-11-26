use crate::result::Result;

// Dummy type to allow impls
// F*-only: `charon::exclude` would drop this dummy type while its `impl`
// blocks still reference it (see f32.rs).
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
struct Slice<T>([T]);

pub mod iter {
    use crate::option::Option;
    use rust_primitives::{sequence::*, slice::*};

    /// See [`std::slice::Chunks`]
    pub struct Chunks<'a, T> {
        cs: usize,
        elements: &'a [T],
    }
    impl<'a, T> Chunks<'a, T> {
        pub fn new(cs: usize, elements: &'a [T]) -> Chunks<'a, T> {
            Chunks { cs, elements }
        }
    }
    /// See [`std::slice::ChunksExact`]
    pub struct ChunksExact<'a, T> {
        cs: usize,
        elements: &'a [T],
    }
    impl<'a, T> ChunksExact<'a, T> {
        pub fn new(cs: usize, elements: &'a [T]) -> ChunksExact<'a, T> {
            ChunksExact { cs, elements }
        }
    }
    /// See [`std::slice::Iter`]
    pub struct Iter<'a, T>(pub Seq<&'a T>);

    /// See [`std::slice::IterMut`]
    ///
    /// Holds the mutable references themselves, which is the only shape that
    /// compiles without `unsafe`: real `core`'s `IterMut` hands out an
    /// `&'a mut T` derived from `&mut self`, which safe Rust cannot express.
    // `&mut` returns are unsupported in the F* backend.
    #[cfg(not(hax_backend_fstar))]
    pub struct IterMut<'a, T>(pub Seq<&'a mut T>);

    impl<'a, T> crate::iter::traits::iterator::Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if seq_len(&self.0) == 0 {
                Option::None
            } else {
                let res = seq_remove(&mut self.0, 0);
                Option::Some(res)
            }
        }
    }

    // `next_back` pops the last element; `len` is the remaining count. Lean/charon
    // backend only, matching `DoubleEndedIterator`/`ExactSizeIterator`. These make
    // `slice.iter().rev()` and `slice.iter().enumerate().rev()` iterate.
    #[cfg(not(hax_backend_fstar))]
    impl<'a, T> crate::iter::traits::double_ended::DoubleEndedIterator for Iter<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            let n = seq_len(&self.0);
            if n == 0 {
                Option::None
            } else {
                let res = seq_remove(&mut self.0, n - 1);
                Option::Some(res)
            }
        }
    }

    #[cfg(not(hax_backend_fstar))]
    impl<'a, T> crate::iter::traits::iterator::Iterator for IterMut<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<Self::Item> {
            if seq_len(&self.0) == 0 {
                Option::None
            } else {
                let res = seq_remove_mut(&mut self.0, 0);
                Option::Some(res)
            }
        }
    }

    #[cfg(not(hax_backend_fstar))]
    impl<'a, T> crate::iter::traits::exact_size::ExactSizeIterator for Iter<'a, T> {
        fn len(&self) -> usize {
            seq_len(&self.0)
        }
    }

    impl<'a, T> crate::iter::traits::iterator::Iterator for Chunks<'a, T> {
        type Item = &'a [T];
        fn next(&mut self) -> Option<Self::Item> {
            if slice_length(self.elements) == 0 {
                Option::None
            } else if slice_length(self.elements) < self.cs {
                let res = self.elements;
                self.elements = slice_slice(self.elements, 0, 0);
                Option::Some(res)
            } else {
                let (res, new_elements) = slice_split_at(self.elements, self.cs);
                self.elements = new_elements;
                Option::Some(res)
            }
        }
    }

    impl<'a, T> crate::iter::traits::iterator::Iterator for ChunksExact<'a, T> {
        type Item = &'a [T];
        fn next(&mut self) -> Option<Self::Item> {
            if slice_length(self.elements) < self.cs {
                Option::None
            } else {
                let (res, new_elements) = slice_split_at(self.elements, self.cs);
                self.elements = new_elements;
                Option::Some(res)
            }
        }
    }

    /// See [`std::slice::Windows`]
    pub struct Windows<'a, T> {
        size: usize,
        elements: &'a [T],
    }
    impl<'a, T> Windows<'a, T> {
        pub fn new(size: usize, elements: &'a [T]) -> Windows<'a, T> {
            Windows { size, elements }
        }
    }
    // opaque: F* cannot prove slice bounds (1 <= length) in the else branch
    // This needs the invariant that size > 0
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    impl<'a, T> crate::iter::traits::iterator::Iterator for Windows<'a, T> {
        type Item = &'a [T];
        fn next(&mut self) -> Option<Self::Item> {
            if slice_length(self.elements) < self.size {
                Option::None
            } else {
                let res = slice_slice(self.elements, 0, self.size);
                self.elements = slice_slice(self.elements, 1, slice_length(self.elements));
                Option::Some(res)
            }
        }
    }
}

// Non-generic, so that coverage sees both branches in one function.
#[hax_lib::requires(n > 0)]
fn chunk_size_nonzero(n: usize) {
    if n == 0 {
        crate::panicking::internal::panic()
    }
}

#[hax_lib::attributes]
impl<T> Slice<T> {
    /// See [`std::slice::len`]
    fn len(s: &[T]) -> usize {
        rust_primitives::slice::slice_length(s)
    }
    /// See [`std::slice::chunks`]
    #[hax_lib::requires(cs > 0)]
    fn chunks<'a>(s: &'a [T], cs: usize) -> iter::Chunks<'a, T> {
        if cs == 0 {
            crate::panicking::internal::panic()
        }
        iter::Chunks::new(cs, s)
    }
    /// See [`std::slice::iter`]
    fn iter(s: &[T]) -> iter::Iter<'_, T> {
        iter::Iter(rust_primitives::sequence::seq_from_slice(s))
    }
    /// See [`std::slice::chunks_exact`]
    #[hax_lib::requires(cs > 0)]
    fn chunks_exact<'a>(s: &'a [T], cs: usize) -> iter::ChunksExact<'a, T> {
        if cs == 0 {
            crate::panicking::internal::panic()
        }
        iter::ChunksExact::new(cs, s)
    }
    /// See [`std::slice::copy_from_slice`]
    #[hax_lib::requires(Slice::len(s) == Slice::len(src))]
    fn copy_from_slice(s: &mut [T], src: &[T])
    where
        T: Copy,
    {
        slice_clone_from_slice(s, src);
    }
    /// See [`std::slice::clone_from_slice`]
    #[hax_lib::requires(Slice::len(s) == Slice::len(src))]
    fn clone_from_slice(s: &mut [T], src: &[T])
    where
        T: Clone,
    {
        slice_clone_from_slice(s, src);
    }
    /// See [`std::slice::split_at`]
    #[hax_lib::requires(mid <= Slice::len(s))]
    fn split_at(s: &[T], mid: usize) -> (&[T], &[T]) {
        rust_primitives::slice::slice_split_at(s, mid)
    }
    /// See [`std::slice::split_at_mut`]
    // `&mut` returns are unsupported in the F* backend.
    #[cfg(not(hax_backend_fstar))]
    #[hax_lib::requires(mid <= Slice::len(s))]
    fn split_at_mut(s: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
        rust_primitives::slice::slice_split_at_mut(s, mid)
    }
    /// See [`std::slice::iter_mut`]
    // `&mut` returns are unsupported in the F* backend.
    #[cfg(not(hax_backend_fstar))]
    fn iter_mut(s: &mut [T]) -> iter::IterMut<'_, T> {
        iter::IterMut(rust_primitives::sequence::seq_from_slice_mut(s))
    }
    /// See [`std::slice::as_chunks`]
    #[hax_lib::requires(N > 0)]
    fn as_chunks<const N: usize>(s: &[T]) -> (&[[T; N]], &[T]) {
        chunk_size_nonzero(N);
        rust_primitives::slice::slice_as_chunks(s)
    }
    /// See [`std::slice::as_chunks_mut`]
    // `&mut` returns are unsupported in the F* backend.
    #[cfg(not(hax_backend_fstar))]
    #[hax_lib::requires(N > 0)]
    fn as_chunks_mut<const N: usize>(s: &mut [T]) -> (&mut [[T; N]], &mut [T]) {
        chunk_size_nonzero(N);
        rust_primitives::slice::slice_as_chunks_mut(s)
    }
    /// See [`std::slice::split_first_chunk`]
    fn split_first_chunk<const N: usize>(s: &[T]) -> Option<(&[T; N], &[T])> {
        if Self::len(s) < N {
            Option::None
        } else {
            let (first, tail) = Self::split_at(s, N);
            Option::Some((rust_primitives::slice::slice_as_array(first), tail))
        }
    }
    /// See [`std::slice::as_array`]
    fn as_array<const N: usize>(s: &[T]) -> Option<&[T; N]> {
        if Self::len(s) == N {
            Option::Some(rust_primitives::slice::slice_as_array(s))
        } else {
            Option::None
        }
    }
    /// See [`std::slice::split_at_checked`]
    fn split_at_checked(s: &[T], mid: usize) -> Option<(&[T], &[T])> {
        if mid <= Slice::len(s) {
            Option::Some(Self::split_at(s, mid))
        } else {
            Option::None
        }
    }
    /// See [`std::slice::is_empty`]
    fn is_empty(s: &[T]) -> bool {
        Self::len(s) == 0
    }
    /// See [`std::slice::contains`]
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    fn contains(s: &[T], v: &T) -> bool
    where
        T: PartialEq,
    {
        rust_primitives::slice::slice_contains(s, v)
    }
    /// See [`std::slice::copy_within`]
    #[hax_lib::requires(
        match index::try_range(src, crate::ops::range::RangeTo { end: Slice::len(s) }) {
            Option::Some(r) => dest <= Slice::len(s) - (r.end - r.start),
            Option::None => false,
        }
    )]
    fn copy_within<R: crate::ops::range::RangeBounds<usize>>(s: &mut [T], src: R, dest: usize)
    where
        T: Copy,
    {
        let r = index::range(src, crate::ops::range::RangeTo { end: Slice::len(s) });
        rust_primitives::slice::slice_copy_within(s, r.start, r.end, dest)
    }
    /// See [`std::slice::binary_search`]
    // F*-only: the equivalence tests call this, so Lean needs the body; it is
    // written over primitives the Lean library provides.
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    // The Rust documentation leaves the behavior unspecified when there are multiple
    // matches or when the array is unsorted. We follow the Rust core implementation.
    fn binary_search(s: &[T], x: &T) -> Result<usize, usize>
    where
        T: crate::cmp::Ord,
    {
        let mut size = Self::len(s);
        if size == 0 {
            return Result::Err(0);
        }
        let mut base = 0;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            // Only `Greater` keeps `base`, so an equal run resolves to its last.
            base = match crate::cmp::Ord::cmp(rust_primitives::slice::slice_index(s, mid), x) {
                crate::cmp::Ordering::Greater => base,
                _ => mid,
            };
            size -= half;
        }
        match crate::cmp::Ord::cmp(rust_primitives::slice::slice_index(s, base), x) {
            crate::cmp::Ordering::Equal => Result::Ok(base),
            crate::cmp::Ordering::Less => Result::Err(base + 1),
            crate::cmp::Ordering::Greater => Result::Err(base),
        }
    }
    /// See [`std::slice::get`]
    fn get<I: SliceIndex<[T]>>(s: &[T], index: I) -> Option<&<I as SliceIndex<[T]>>::Output> {
        index.get(s)
    }
    /// See [`std::slice::get_unchecked`]
    // opaque for F*: the generic precondition isn't provable here (the concrete
    // `SliceIndex` impls verify).
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    #[cfg_attr(not(hax_backend_lean), hax_lib::requires(index.get(s).is_some()))]
    fn get_unchecked<I: SliceIndex<[T]>>(s: &[T], index: I) -> &<I as SliceIndex<[T]>>::Output {
        index.get_unchecked(s)
    }
    // `&mut` returns are unsupported in the F* backend.
    /// See [`std::slice::get_mut`]
    #[cfg(not(hax_backend_fstar))]
    fn get_mut<I: SliceIndex<[T]>>(
        s: &mut [T],
        index: I,
    ) -> Option<&mut <I as SliceIndex<[T]>>::Output> {
        index.get_mut(s)
    }
    /// See [`std::slice::get_unchecked_mut`]
    #[cfg(not(hax_backend_fstar))]
    #[cfg_attr(not(hax_backend_lean), hax_lib::requires(index.get(s).is_some()))]
    fn get_unchecked_mut<I: SliceIndex<[T]>>(
        s: &mut [T],
        index: I,
    ) -> &mut <I as SliceIndex<[T]>>::Output {
        index.get_unchecked_mut(s)
    }
    /// See [`std::slice::first`]
    fn first(s: &[T]) -> Option<&T> {
        if Self::is_empty(s) {
            Option::None
        } else {
            Option::Some(slice_index(s, 0))
        }
    }
    /// See [`std::slice::last`]
    fn last(s: &[T]) -> Option<&T> {
        if Self::is_empty(s) {
            Option::None
        } else {
            Option::Some(slice_index(s, Self::len(s) - 1))
        }
    }
    /// See [`std::slice::swap`]
    // opaque for F*: `&mut` mutation is unsupported there.
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    #[hax_lib::requires(a < Slice::len(s) && b < Slice::len(s))]
    fn swap(s: &mut [T], a: usize, b: usize) {
        rust_primitives::slice::slice_swap(s, a, b);
    }
    /// See [`std::slice::reverse`]
    // opaque for F*: see `swap`.
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    fn reverse(s: &mut [T]) {
        rust_primitives::slice::slice_reverse(s);
    }
    /// See [`std::slice::windows`]
    #[hax_lib::requires(size > 0)]
    fn windows<'a>(s: &'a [T], size: usize) -> iter::Windows<'a, T> {
        if size == 0 {
            crate::panicking::internal::panic()
        }
        iter::Windows::new(size, s)
    }
    /// See [`std::slice::fill`]
    // opaque: for-loop + indexed mutation causes F* dependency cycle through Rust_primitives.Hax
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    // std clones into every slot but the last, and moves `value` into that one.
    fn fill(s: &mut [T], value: T)
    where
        T: Clone,
    {
        let len = Self::len(s);
        if len > 0 {
            // Bound once: evaluated twice, Aeneas emits two checked subtractions.
            let last = len - 1;
            for i in 0..last {
                s[i] = value.clone();
            }
            s[last] = value;
        }
    }

    // F* names inherent methods by impl-block order, so `starts_with`/`ends_with`
    // live in this first block (as opaque vals) for F* to keep their `impl__`
    // name. The aeneas/lean copies are in the `cfg(not(hax_backend_fstar))` block
    // after the `PartialEq for [T]` impl, where source order avoids a forward
    // reference to `eq`.
    /// See [`std::slice::starts_with`]
    #[cfg(hax_backend_fstar)]
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    fn starts_with(s: &[T], needle: &[T]) -> bool
    where
        T: PartialEq,
    {
        let n = Self::len(needle);
        Self::len(s) >= n && slice_slice(s, 0, n) == needle
    }
    /// See [`std::slice::ends_with`]
    #[cfg(hax_backend_fstar)]
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    fn ends_with(s: &[T], needle: &[T]) -> bool
    where
        T: PartialEq,
    {
        let n = Self::len(needle);
        let l = Self::len(s);
        l >= n && slice_slice(s, l - n, l) == needle
    }
}

#[hax_lib::attributes]
#[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
impl<U, T: crate::cmp::PartialEq<U>> crate::cmp::PartialEq<[U]> for [T] {
    #[cfg(not(hax_backend_fstar))]
    fn ne(&self, other: &[U]) -> bool {
        self.eq(other) == false
    }
    fn eq(&self, other: &[U]) -> bool {
        if self.len() != other.len() {
            false
        } else {
            let mut res = true;
            for i in 0..self.len() {
                if res && !self[i].eq(&other[i]) {
                    // This should be an early return, but aeneas doesn't support that
                    res = false;
                }
            }
            res
        }
    }
}

#[hax_lib::attributes]
impl<T: crate::cmp::Eq> crate::cmp::Eq for [T] {}

#[hax_lib::attributes]
#[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
impl<T: crate::cmp::PartialOrd<T>> crate::cmp::PartialOrd<[T]> for [T] {
    fn partial_cmp(&self, other: &[T]) -> crate::option::Option<crate::cmp::Ordering> {
        // Lexicographic order: compare elements pairwise up to the shorter
        // length; the first non-`Equal` result (including `None`) decides.
        let l = if self.len() < other.len() {
            self.len()
        } else {
            other.len()
        };
        for i in 0..l {
            match self[i].partial_cmp(&other[i]) {
                crate::option::Option::Some(crate::cmp::Ordering::Equal) => (),
                non_eq => return non_eq,
            }
        }
        // All common elements are equal: the shorter slice is smaller.
        if self.len() < other.len() {
            crate::option::Option::Some(crate::cmp::Ordering::Less)
        } else if self.len() > other.len() {
            crate::option::Option::Some(crate::cmp::Ordering::Greater)
        } else {
            crate::option::Option::Some(crate::cmp::Ordering::Equal)
        }
    }
}

#[hax_lib::attributes]
#[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
impl<T: crate::cmp::Ord> crate::cmp::Ord for [T] {
    fn cmp(&self, other: &[T]) -> crate::cmp::Ordering {
        // Lexicographic order: compare elements pairwise up to the shorter
        // length; the first non-`Equal` result decides.
        let l = if self.len() < other.len() {
            self.len()
        } else {
            other.len()
        };
        for i in 0..l {
            match self[i].cmp(&other[i]) {
                crate::cmp::Ordering::Equal => (),
                non_eq => return non_eq,
            }
        }
        // All common elements are equal: the shorter slice is smaller.
        if self.len() < other.len() {
            crate::cmp::Ordering::Less
        } else if self.len() > other.len() {
            crate::cmp::Ordering::Greater
        } else {
            crate::cmp::Ordering::Equal
        }
    }
}

// Dummy type for real core's `impl<T, const N: usize> [[T; N]]`. Lean-only:
// a second inherent impl would take an `impl_N` name in F* that must match
// real core's.
#[cfg(not(hax_backend_fstar))]
struct SliceArray<T, const N: usize>([[T; N]]);

#[cfg(not(hax_backend_fstar))]
impl<T, const N: usize> SliceArray<T, N> {
    /// See [`std::slice::as_flattened`]
    fn as_flattened(s: &[[T; N]]) -> &[T] {
        rust_primitives::slice::slice_as_flattened(s)
    }
}

// aeneas/lean copies of `starts_with`/`ends_with`: they compare with `==`, so
// they sit after the `PartialEq for [T]` impl (source order avoids a forward
// reference to `eq`). Also defined in the F* `impl__` block above.
#[cfg(not(hax_backend_fstar))]
#[hax_lib::attributes]
impl<T> Slice<T> {
    /// See [`std::slice::starts_with`]
    // opaque: slice equality requires eqtype in F*, but T is extracted as Type0
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    fn starts_with(s: &[T], needle: &[T]) -> bool
    where
        T: PartialEq,
    {
        let n = Self::len(needle);
        Self::len(s) >= n && slice_slice(s, 0, n) == needle
    }
    /// See [`std::slice::ends_with`]
    // opaque: slice equality requires eqtype in F*, but T is extracted as Type0
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    fn ends_with(s: &[T], needle: &[T]) -> bool
    where
        T: PartialEq,
    {
        let n = Self::len(needle);
        let l = Self::len(s);
        l >= n && slice_slice(s, l - n, l) == needle
    }
}

#[hax_lib::attributes]
#[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
impl<'a, T> crate::iter::traits::collect::IntoIterator for &'a [T] {
    type Item = &'a T;
    type IntoIter = iter::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Slice::iter(self)
    }
}
use crate::option::Option;
use rust_primitives::slice::*;

/// `slice::index` follows std's layout: the `SliceIndex` trait and its
/// `usize`/`Range*` impls live in this submodule, and the parent
/// `slice` module re-exports the trait below for backward compat.
/// See [`std::slice::index`].
pub mod index {
    use super::Option;
    use rust_primitives::slice::*;

    /// See [`std::slice::SliceIndex`]. `get_unchecked` is the same in-bounds
    /// projection as `index` (no raw pointers); the `*_mut` variants take
    /// `&mut T` and return `&mut Output`.
    #[hax_lib::attributes]
    pub trait SliceIndex<T: ?Sized> {
        type Output: ?Sized;

        #[hax_lib::requires(true)]
        fn get(self, slice: &T) -> Option<&Self::Output>;

        fn index(self, slice: &T) -> &Self::Output;

        /// See [`std::slice::SliceIndex::get_unchecked`]. In-bounds precondition per impl.
        fn get_unchecked(self, slice: &T) -> &Self::Output;

        // `&mut` returns are unsupported in the F* backend.
        /// See [`std::slice::SliceIndex::get_mut`]. Total, like `get`.
        #[cfg(not(hax_backend_fstar))]
        #[hax_lib::requires(true)]
        fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;

        /// See [`std::slice::SliceIndex::get_unchecked_mut`]. In-bounds precondition per impl.
        #[cfg(not(hax_backend_fstar))]
        fn get_unchecked_mut(self, slice: &mut T) -> &mut Self::Output;

        /// See [`std::slice::SliceIndex::index_mut`]: the `&mut` counterpart of
        /// `index`, same in-bounds projection, precondition per impl.
        #[cfg(not(hax_backend_fstar))]
        fn index_mut(self, slice: &mut T) -> &mut Self::Output;
    }

    #[hax_lib::attributes]
    #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
    impl<T> SliceIndex<[T]> for usize {
        type Output = T;
        fn get(self, slice: &[T]) -> Option<&T> {
            if self < slice_length(slice) {
                Option::Some(slice_index(slice, self))
            } else {
                Option::None
            }
        }
        #[hax_lib::requires(self < slice_length(slice))]
        fn index(self, slice: &[T]) -> &T {
            slice_index(slice, self)
        }
        #[hax_lib::requires(self < slice_length(slice))]
        fn get_unchecked(self, slice: &[T]) -> &T {
            slice_index(slice, self)
        }
        #[cfg(not(hax_backend_fstar))]
        fn get_mut(self, slice: &mut [T]) -> Option<&mut T> {
            if self < slice_length(slice) {
                Option::Some(slice_index_mut(slice, self))
            } else {
                Option::None
            }
        }
        #[cfg(not(hax_backend_fstar))]
        #[hax_lib::requires(self < slice_length(slice))]
        fn get_unchecked_mut(self, slice: &mut [T]) -> &mut T {
            slice_index_mut(slice, self)
        }
        #[cfg(not(hax_backend_fstar))]
        #[hax_lib::requires(self < slice_length(slice))]
        fn index_mut(self, slice: &mut [T]) -> &mut T {
            slice_index_mut(slice, self)
        }
    }

    #[hax_lib::attributes]
    #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
    impl<T> SliceIndex<[T]> for crate::ops::range::RangeFull {
        type Output = [T];
        fn get(self, slice: &[T]) -> Option<&[T]> {
            Option::Some(slice)
        }
        fn index(self, slice: &[T]) -> &[T] {
            slice
        }
        fn get_unchecked(self, slice: &[T]) -> &[T] {
            slice
        }
        #[cfg(not(hax_backend_fstar))]
        fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
            Option::Some(slice)
        }
        #[cfg(not(hax_backend_fstar))]
        fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] {
            slice
        }
        #[cfg(not(hax_backend_fstar))]
        fn index_mut(self, slice: &mut [T]) -> &mut [T] {
            slice
        }
    }

    #[hax_lib::attributes]
    #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
    impl<T> SliceIndex<[T]> for crate::ops::range::RangeFrom<usize> {
        type Output = [T];
        fn get(self, slice: &[T]) -> Option<&[T]> {
            if self.start <= slice_length(slice) {
                Option::Some(slice_slice(slice, self.start, slice_length(slice)))
            } else {
                Option::None
            }
        }
        #[hax_lib::requires(self.start <= slice_length(slice))]
        fn index(self, slice: &[T]) -> &[T] {
            slice_slice(slice, self.start, slice_length(slice))
        }
        #[hax_lib::requires(self.start <= slice_length(slice))]
        fn get_unchecked(self, slice: &[T]) -> &[T] {
            slice_slice(slice, self.start, slice_length(slice))
        }
        #[cfg(not(hax_backend_fstar))]
        fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
            let len = slice_length(slice);
            if self.start <= len {
                Option::Some(slice_slice_mut(slice, self.start, len))
            } else {
                Option::None
            }
        }
        #[cfg(not(hax_backend_fstar))]
        #[hax_lib::requires(self.start <= slice_length(slice))]
        fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] {
            let len = slice_length(slice);
            slice_slice_mut(slice, self.start, len)
        }
        #[cfg(not(hax_backend_fstar))]
        #[hax_lib::requires(self.start <= slice_length(slice))]
        fn index_mut(self, slice: &mut [T]) -> &mut [T] {
            let len = slice_length(slice);
            slice_slice_mut(slice, self.start, len)
        }
    }
    #[hax_lib::attributes]
    #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
    impl<T> SliceIndex<[T]> for crate::ops::range::RangeTo<usize> {
        type Output = [T];
        fn get(self, slice: &[T]) -> Option<&[T]> {
            if self.end <= slice_length(slice) {
                Option::Some(slice_slice(slice, 0, self.end))
            } else {
                Option::None
            }
        }
        #[hax_lib::requires(self.end <= slice_length(slice))]
        fn index(self, slice: &[T]) -> &[T] {
            slice_slice(slice, 0, self.end)
        }
        #[hax_lib::requires(self.end <= slice_length(slice))]
        fn get_unchecked(self, slice: &[T]) -> &[T] {
            slice_slice(slice, 0, self.end)
        }
        #[cfg(not(hax_backend_fstar))]
        fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
            if self.end <= slice_length(slice) {
                Option::Some(slice_slice_mut(slice, 0, self.end))
            } else {
                Option::None
            }
        }
        #[cfg(not(hax_backend_fstar))]
        #[hax_lib::requires(self.end <= slice_length(slice))]
        fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] {
            slice_slice_mut(slice, 0, self.end)
        }
        #[cfg(not(hax_backend_fstar))]
        #[hax_lib::requires(self.end <= slice_length(slice))]
        fn index_mut(self, slice: &mut [T]) -> &mut [T] {
            slice_slice_mut(slice, 0, self.end)
        }
    }
    #[hax_lib::attributes]
    #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
    impl<T> SliceIndex<[T]> for crate::ops::range::Range<usize> {
        type Output = [T];
        fn get(self, slice: &[T]) -> Option<&[T]> {
            if self.start <= self.end && self.end <= slice_length(slice) {
                Option::Some(slice_slice(slice, self.start, self.end))
            } else {
                Option::None
            }
        }
        #[hax_lib::requires(self.start <= self.end && self.end <= slice_length(slice))]
        fn index(self, slice: &[T]) -> &[T] {
            slice_slice(slice, self.start, self.end)
        }
        #[hax_lib::requires(self.start <= self.end && self.end <= slice_length(slice))]
        fn get_unchecked(self, slice: &[T]) -> &[T] {
            slice_slice(slice, self.start, self.end)
        }
        #[cfg(not(hax_backend_fstar))]
        fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
            if self.start <= self.end && self.end <= slice_length(slice) {
                Option::Some(slice_slice_mut(slice, self.start, self.end))
            } else {
                Option::None
            }
        }
        #[cfg(not(hax_backend_fstar))]
        #[hax_lib::requires(self.start <= self.end && self.end <= slice_length(slice))]
        fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] {
            slice_slice_mut(slice, self.start, self.end)
        }
        #[cfg(not(hax_backend_fstar))]
        #[hax_lib::requires(self.start <= self.end && self.end <= slice_length(slice))]
        fn index_mut(self, slice: &mut [T]) -> &mut [T] {
            slice_slice_mut(slice, self.start, self.end)
        }
    }

    /// Generic `Index<I>` for `[T]`, matching std's
    /// `impl<T, I: SliceIndex<[T]>> Index<I> for [T]`
    /// in `core/src/slice/index.rs`. Body delegates to
    /// `SliceIndex::get` (we removed the `index`/`index_mut` methods
    /// from the trait to avoid modeling raw pointers; std would call
    /// `index.index(self)` instead).
    #[hax_lib::attributes]
    #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
    impl<T, I> crate::ops::index::Index<I> for [T]
    where
        I: SliceIndex<[T]>,
    {
        type Output = I::Output;
        #[cfg_attr(not(hax_backend_lean), hax_lib::requires(i.get(self).is_some()))]
        fn index(&self, i: I) -> &I::Output {
            match i.get(self) {
                Option::Some(r) => r,
                Option::None => crate::panicking::internal::panic(),
            }
        }
    }

    /// Generic `IndexMut<I>` for `[T]`, mirroring the `Index<I>` impl above and
    /// std's `impl<T, I: SliceIndex<[T]>> IndexMut<I> for [T]`. Delegates to
    /// `SliceIndex::get_mut` (Lean-only, as the mutable accessors are).
    #[cfg(not(hax_backend_fstar))]
    #[hax_lib::attributes]
    #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
    impl<T, I> crate::ops::index::IndexMut<I> for [T]
    where
        I: SliceIndex<[T]>,
    {
        // `get_unchecked_mut` (not a `get_mut` + panicking `match`): a panic in
        // the `None` arm would have to produce the `&mut` return, which aeneas
        // lowers to a `(value, write-back)` pair and cannot synthesise from a
        // divergent `panic`. The precondition mirrors `Index::index`.
        // Kept out of the Lean lane: routed through hax's spec channel, this
        // precondition makes aeneas fail with an internal `Invalid_argument`.
        #[cfg_attr(not(hax_backend_lean), hax_lib::requires(i.get(self).is_some()))]
        fn index_mut(&mut self, i: I) -> &mut I::Output {
            i.get_unchecked_mut(self)
        }
    }

    // In F*, `range` below shadows the integer refinement `range` in every item
    // hax emits after it in `Core_models.Slice.Index`, whatever the order here.
    /// See [`std::slice::try_range`]
    pub fn try_range<R: crate::ops::range::RangeBounds<usize>>(
        range: R,
        bounds: crate::ops::range::RangeTo<usize>,
    ) -> Option<crate::ops::range::Range<usize>> {
        let len = bounds.end;
        match start_index(&range) {
            Option::Some(start) => match end_index(&range, len) {
                Option::Some(end) => {
                    if start > end || end > len {
                        Option::None
                    } else {
                        Option::Some(crate::ops::range::Range { start, end })
                    }
                }
                Option::None => Option::None,
            },
            Option::None => Option::None,
        }
    }

    // Inline into `try_range`, as std does, once AeneasVerif/aeneas#1369 is fixed.
    fn start_index<R: crate::ops::range::RangeBounds<usize>>(range: &R) -> Option<usize> {
        use crate::ops::range::Bound;
        match range.start_bound() {
            Bound::Included(start) => Option::Some(*start),
            Bound::Excluded(start) => crate::num::usize::checked_add(*start, 1),
            Bound::Unbounded => Option::Some(0),
        }
    }

    // Inline into `try_range`, as std does, once AeneasVerif/aeneas#1369 is fixed.
    fn end_index<R: crate::ops::range::RangeBounds<usize>>(range: &R, len: usize) -> Option<usize> {
        use crate::ops::range::Bound;
        match range.end_bound() {
            Bound::Included(end) => crate::num::usize::checked_add(*end, 1),
            Bound::Excluded(end) => Option::Some(*end),
            Bound::Unbounded => Option::Some(len),
        }
    }

    /// See [`std::slice::range`]
    #[hax_lib::requires(try_range(range, bounds).is_some())]
    pub fn range<R: crate::ops::range::RangeBounds<usize>>(
        range: R,
        bounds: crate::ops::range::RangeTo<usize>,
    ) -> crate::ops::range::Range<usize> {
        match try_range(range, bounds) {
            Option::Some(r) => r,
            Option::None => crate::panicking::internal::panic(),
        }
    }
}

pub use index::SliceIndex;

use crate::ops::{
    index::Index,
    range::{Range, RangeFrom, RangeFull, RangeTo},
};

#[hax_lib::attributes]
#[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
impl<T> Index<Range<usize>> for &[T] {
    type Output = [T];
    #[hax_lib::requires(i.start <= i.end && i.end <= slice_length(self))]
    fn index(&self, i: Range<usize>) -> &[T] {
        slice_slice(self, i.start, i.end)
    }
}
#[hax_lib::attributes]
#[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
impl<T> Index<RangeTo<usize>> for &[T] {
    type Output = [T];
    #[hax_lib::requires(i.end <= slice_length(self))]
    fn index(&self, i: RangeTo<usize>) -> &[T] {
        slice_slice(self, 0, i.end)
    }
}
#[hax_lib::attributes]
#[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
impl<T> Index<RangeFrom<usize>> for &[T] {
    type Output = [T];
    #[hax_lib::requires(i.start <= slice_length(self))]
    fn index(&self, i: RangeFrom<usize>) -> &[T] {
        slice_slice(self, i.start, slice_length(self))
    }
}
#[hax_lib::attributes]
#[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
impl<T> Index<RangeFull> for &[T] {
    type Output = [T];
    fn index(&self, i: RangeFull) -> &[T] {
        slice_slice(self, 0, slice_length(self))
    }
}

#[hax_lib::attributes]
#[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
impl<T> crate::ops::index::Index<usize> for &[T] {
    type Output = T;
    #[hax_lib::requires(i < slice_length(self))]
    fn index(&self, i: usize) -> &T {
        rust_primitives::slice::slice_index(self, i)
    }
}

/// `PartialEq<[U; N]> for [T]` — comparing a slice to an array (`s == [..]`),
/// mirroring std's `impl PartialEq<[U; N]> for [T]`.
pub mod equality {
    use rust_primitives::slice::{array_index, slice_index, slice_length};

    #[hax_lib::attributes]
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    impl<T: crate::cmp::PartialEq<U>, U, const N: usize> crate::cmp::PartialEq<[U; N]> for [T] {
        #[cfg(not(hax_backend_fstar))]
        fn ne(&self, other: &[U; N]) -> bool {
            self.eq(other) == false
        }
        fn eq(&self, other: &[U; N]) -> bool {
            if slice_length(self) != N {
                false
            } else {
                let mut res = true;
                for i in 0..N {
                    if res && !slice_index(self, i).eq(array_index(other, i)) {
                        // This should be an early return, but aeneas doesn't support that
                        res = false;
                    }
                }
                res
            }
        }
    }

    // F* erases the reference, so this would be a second instance at the same
    // type as the impl above.
    #[hax_lib::attributes]
    #[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
    impl<T: crate::cmp::PartialEq<U>, U, const N: usize> crate::cmp::PartialEq<[U; N]> for &[T] {
        #[cfg(not(hax_backend_fstar))]
        fn ne(&self, other: &[U; N]) -> bool {
            self.eq(other) == false
        }
        fn eq(&self, other: &[U; N]) -> bool {
            <[T] as crate::cmp::PartialEq<[U; N]>>::eq(self, other)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Slice;
    use crate::iter::traits::iterator::Iterator as ModelIterator;
    use crate::option::Option as ModelOption;
    #[cfg(not(hax_backend_fstar))]
    use crate::testing::CloneWitness;
    use crate::testing::Inject;
    use proptest::prelude::*;

    /// The slice iterators are lazy; draining them is what observes them.
    fn drain<I: ModelIterator>(mut it: I) -> Vec<I::Item> {
        let mut out = Vec::new();
        while let ModelOption::Some(x) = it.next() {
            out.push(x);
        }
        out
    }

    /// `Iter`'s `DoubleEndedIterator`/`ExactSizeIterator` impls, which `rev` and
    /// `Enumerate::next_back` need.
    #[cfg(not(hax_backend_fstar))]
    proptest! {
        #[test]
        fn test_iter_next_back(slice in prop::collection::vec(any::<u8>(), 0..=20)) {
            use crate::iter::traits::double_ended::DoubleEndedIterator;
            let mut it = Slice::iter(&slice[..]);
            let mut got = Vec::new();
            while let ModelOption::Some(x) = it.next_back() {
                got.push(x);
            }
            prop_assert_eq!(got, slice.iter().rev().collect::<Vec<_>>());
        }

        #[test]
        fn test_iter_len(slice in prop::collection::vec(any::<u8>(), 0..=20)) {
            use crate::iter::traits::exact_size::ExactSizeIterator;
            let it = Slice::iter(&slice[..]);
            prop_assert_eq!(ExactSizeIterator::len(&it), slice.len());
        }
    }

    proptest! {
        #[test]
        fn test_iter(slice in prop::collection::vec(any::<u8>(), 0..=20)) {
            prop_assert_eq!(
                drain(Slice::iter(&slice[..])),
                slice.iter().collect::<Vec<_>>()
            );
        }

        // Sizes run one past the slice length: `chunks` keeps a short final
        // chunk, `chunks_exact` drops it.
        #[test]
        fn test_chunks(slice in prop::collection::vec(any::<u8>(), 0..=20), cs in 1usize..=21) {
            prop_assert_eq!(
                drain(Slice::chunks(&slice[..], cs)),
                slice.chunks(cs).collect::<Vec<_>>()
            );
        }

        #[test]
        fn test_chunks_exact(slice in prop::collection::vec(any::<u8>(), 0..=20), cs in 1usize..=21) {
            prop_assert_eq!(
                drain(Slice::chunks_exact(&slice[..], cs)),
                slice.chunks_exact(cs).collect::<Vec<_>>()
            );
        }

        #[test]
        fn test_windows(slice in prop::collection::vec(any::<u8>(), 0..=20), size in 1usize..=21) {
            prop_assert_eq!(
                drain(Slice::windows(&slice[..], size)),
                slice.windows(size).collect::<Vec<_>>()
            );
        }

        #[test]
        fn test_len(slice in prop::collection::vec(any::<u8>(), 0..=20)) {
            prop_assert_eq!(Slice::len(&slice[..]), slice.len());
        }

        #[test]
        fn test_is_empty(slice in prop::collection::vec(any::<u8>(), 0..=10)) {
            prop_assert_eq!(Slice::is_empty(&slice[..]), slice.is_empty());
        }

        // ----- SliceIndex::index_mut, one per self type ------------------
        //
        // Against std's `IndexMut` on the slice, which is what `index_mut`
        // implements; std's `SliceIndex::index_mut` is not callable directly.

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_slice_index_index_mut_usize(slice in prop::collection::vec(any::<u8>(), 4..=4), idx in 0usize..4, v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            *crate::slice::SliceIndex::index_mut(idx, &mut model[..]) = v;
            std_slice[idx] = v;
            prop_assert_eq!(model, std_slice);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_slice_index_index_mut_range_from(slice in prop::collection::vec(any::<u8>(), 8..=8), start in 0usize..=8, v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            crate::slice::SliceIndex::index_mut(crate::ops::range::RangeFrom { start }, &mut model[..]).fill(v);
            std_slice[start..].fill(v);
            prop_assert_eq!(model, std_slice);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_slice_index_index_mut_range_to(slice in prop::collection::vec(any::<u8>(), 8..=8), end in 0usize..=8, v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            crate::slice::SliceIndex::index_mut(crate::ops::range::RangeTo { end }, &mut model[..]).fill(v);
            std_slice[..end].fill(v);
            prop_assert_eq!(model, std_slice);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_slice_index_index_mut_range_full(slice in prop::collection::vec(any::<u8>(), 0..=8), v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            crate::slice::SliceIndex::index_mut(crate::ops::range::RangeFull, &mut model[..]).fill(v);
            std_slice[..].fill(v);
            prop_assert_eq!(model, std_slice);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_slice_index_index_mut_range(slice in prop::collection::vec(any::<u8>(), 8..=8), start in 0usize..8, len in 0usize..8, v in any::<u8>()) {
            let end = (start + len).min(8);
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            crate::slice::SliceIndex::index_mut(crate::ops::range::Range { start, end }, &mut model[..]).fill(v);
            std_slice[start..end].fill(v);
            prop_assert_eq!(model, std_slice);
        }

        #[test]
        fn test_contains(slice in prop::collection::vec(any::<u8>(), 0..=10), v in any::<u8>()) {
            prop_assert_eq!(Slice::contains(&slice[..], &v), slice.contains(&v));
        }

        #[test]
        fn test_split_at(slice in prop::collection::vec(any::<u8>(), 1..=10)) {
            let mid = slice.len() / 2;
            prop_assert_eq!(Slice::split_at(&slice[..], mid), slice.split_at(mid));
        }

        // The `&mut` items below have no F* model: `&mut` returns are
        // unsupported there.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_split_at_mut(slice in prop::collection::vec(any::<u8>(), 1..=10), v in any::<u8>()) {
            let mid = slice.len() / 2;
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            {
                let (a, b) = Slice::split_at_mut(&mut model[..], mid);
                a.fill(v);
                b.fill(v.wrapping_add(1));
            }
            {
                let (a, b) = std_slice.split_at_mut(mid);
                a.fill(v);
                b.fill(v.wrapping_add(1));
            }
            prop_assert_eq!(model, std_slice);
        }

        // `iter_mut` yields the elements in order, and a write through a
        // yielded reference lands in the slice -- both against std's.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_iter_mut(slice in prop::collection::vec(any::<u8>(), 0..=10), v in any::<u8>()) {
            use crate::iter::traits::iterator::Iterator as ModelIterator;

            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            {
                let mut it = Slice::iter_mut(&mut model[..]);
                let mut seen = std::vec::Vec::new();
                while let crate::option::Option::Some(r) = it.next() {
                    seen.push(*r);
                    *r = v;
                }
                prop_assert_eq!(seen, slice.clone());
            }
            for r in std_slice.iter_mut() {
                *r = v;
            }
            prop_assert_eq!(model, std_slice);
        }

        #[test]
        fn test_as_chunks(slice in prop::collection::vec(any::<u8>(), 0..=10)) {
            prop_assert_eq!(Slice::as_chunks::<3>(&slice[..]), slice.as_chunks::<3>());
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_as_chunks_mut(slice in prop::collection::vec(any::<u8>(), 0..=10), v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            {
                let (chunks, tail) = Slice::as_chunks_mut::<3>(&mut model[..]);
                prop_assert_eq!(chunks.len(), slice.len() / 3);
                for chunk in chunks {
                    chunk[0] = v;
                }
                for x in tail {
                    *x = v;
                }
            }
            {
                let (chunks, tail) = std_slice.as_chunks_mut::<3>();
                for chunk in chunks {
                    chunk[0] = v;
                }
                for x in tail {
                    *x = v;
                }
            }
            prop_assert_eq!(model, std_slice);
        }

        #[test]
        fn test_split_first_chunk(slice in prop::collection::vec(any::<u8>(), 0..=5)) {
            let model = Slice::split_first_chunk::<3>(&slice[..]);
            let expected = slice.split_first_chunk::<3>();
            match (model, expected) {
                (ModelOption::Some((a, b)), Some((c, d))) => {
                    prop_assert_eq!(a, c);
                    prop_assert_eq!(b, d);
                }
                (ModelOption::None, None) => (),
                _ => prop_assert!(false),
            }
        }

        #[test]
        fn test_as_array(slice in prop::collection::vec(any::<u8>(), 2..=4)) {
            let model = Slice::as_array::<3>(&slice[..]);
            let expected: Option<&[u8; 3]> = slice[..].try_into().ok();
            match (model, expected) {
                (ModelOption::Some(a), Some(b)) => prop_assert_eq!(a, b),
                (ModelOption::None, None) => (),
                _ => prop_assert!(false),
            }
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_as_flattened(arrays in prop::collection::vec(any::<[u8; 3]>(), 0..=4)) {
            prop_assert_eq!(super::SliceArray::as_flattened(&arrays[..]), arrays.as_flattened());
        }

        #[test]
        fn test_split_at_checked(slice in prop::collection::vec(any::<u8>(), 1..=10), mid in 0usize..15) {
            prop_assert_eq!(
                Slice::split_at_checked(&slice[..], mid),
                slice.split_at_checked(mid).inject()
            );
        }

        #[test]
        fn test_copy_from_slice(src in prop::collection::vec(any::<u8>(), 1..=10)) {
            let mut model_dest = vec![0u8; src.len()];
            let mut std_dest = model_dest.clone();
            Slice::copy_from_slice(&mut model_dest[..], &src[..]);
            std_dest.copy_from_slice(&src[..]);
            prop_assert_eq!(model_dest, std_dest);
        }

        #[test]
        fn test_clone_from_slice(src in prop::collection::vec(any::<u8>(), 1..=10)) {
            let mut model_dest = vec![0u8; src.len()];
            let mut std_dest = model_dest.clone();
            Slice::clone_from_slice(&mut model_dest[..], &src[..]);
            std_dest.clone_from_slice(&src[..]);
            prop_assert_eq!(model_dest, std_dest);
        }

        #[test]
        fn test_get_usize(slice in prop::collection::vec(any::<u8>(), 1..=10), idx in prop_oneof![0usize..=11, any::<usize>()]) {
            prop_assert_eq!(
                Slice::get(&slice[..], idx).map(|v: &u8| *v),
                slice.get(idx).copied().inject()
            );
        }

        #[test]
        fn test_get_range(slice in prop::collection::vec(any::<u8>(), 1..=10), start in 0usize..10, end in 0usize..10) {
            prop_assert_eq!(
                Slice::get(&slice[..], crate::ops::range::Range { start, end }),
                slice.get(start..end).inject()
            );
        }

        #[test]
        fn test_get_range_from(slice in prop::collection::vec(any::<u8>(), 1..=10), start in 0usize..15) {
            prop_assert_eq!(
                Slice::get(&slice[..], crate::ops::range::RangeFrom { start }),
                slice.get(start..).inject()
            );
        }

        #[test]
        fn test_get_range_to(slice in prop::collection::vec(any::<u8>(), 1..=10), end in 0usize..15) {
            prop_assert_eq!(
                Slice::get(&slice[..], crate::ops::range::RangeTo { end }),
                slice.get(..end).inject()
            );
        }

        #[test]
        fn test_get_range_full(slice in prop::collection::vec(any::<u8>(), 0..=10)) {
            prop_assert_eq!(
                Slice::get(&slice[..], crate::ops::range::RangeFull),
                slice.get(..).inject()
            );
        }

        #[test]
        fn test_first(slice in prop::collection::vec(any::<u8>(), 0..=10)) {
            prop_assert_eq!(
                Slice::first(&slice[..]).map(|v: &u8| *v),
                slice.first().copied().inject()
            );
        }

        #[test]
        fn test_last(slice in prop::collection::vec(any::<u8>(), 0..=10)) {
            prop_assert_eq!(
                Slice::last(&slice[..]).map(|v: &u8| *v),
                slice.last().copied().inject()
            );
        }

        #[test]
        fn test_swap(slice in prop::collection::vec(any::<u8>(), 2..=10)) {
            let a = 0;
            let b = slice.len() - 1;
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            Slice::swap(&mut model[..], a, b);
            std_slice.swap(a, b);
            prop_assert_eq!(model, std_slice);
        }

        #[test]
        fn test_reverse(slice in prop::collection::vec(any::<u8>(), 0..=10)) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            Slice::reverse(&mut model[..]);
            std_slice.reverse();
            prop_assert_eq!(model, std_slice);
        }

        #[test]
        fn test_starts_with(slice in prop::collection::vec(any::<u8>(), 0..=10), n in 0usize..=10) {
            let n = n.min(slice.len());
            let needle = &slice[..n];
            prop_assert_eq!(Slice::starts_with(&slice[..], needle), slice.starts_with(needle));
        }

        #[test]
        fn test_starts_with_false(slice in prop::collection::vec(any::<u8>(), 1..=10), needle in prop::collection::vec(any::<u8>(), 1..=5)) {
            prop_assert_eq!(Slice::starts_with(&slice[..], &needle[..]), slice.starts_with(&needle[..]));
        }

        #[test]
        fn test_ends_with(slice in prop::collection::vec(any::<u8>(), 0..=10), n in 0usize..=10) {
            let n = n.min(slice.len());
            let needle = &slice[slice.len() - n..];
            prop_assert_eq!(Slice::ends_with(&slice[..], needle), slice.ends_with(needle));
        }

        #[test]
        fn test_ends_with_false(slice in prop::collection::vec(any::<u8>(), 1..=10), needle in prop::collection::vec(any::<u8>(), 1..=5)) {
            prop_assert_eq!(Slice::ends_with(&slice[..], &needle[..]), slice.ends_with(&needle[..]));
        }

        #[test]
        fn test_fill(value in any::<u8>(), len in 1usize..=10) {
            let mut model = vec![0u8; len];
            let mut std_slice = vec![0u8; len];
            Slice::fill(&mut model[..], value);
            std_slice.fill(value);
            prop_assert_eq!(model, std_slice);
        }

        // `len` reaches 0, for the guard around the `len - 1` subtraction.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_fill_moves_the_last_element(value in any::<u8>(), len in 0usize..=6) {
            let mut model: Vec<CloneWitness> = (0..len).map(|_| CloneWitness::new(0)).collect();
            let mut std_slice: Vec<CloneWitness> = (0..len).map(|_| CloneWitness::new(0)).collect();
            Slice::fill(&mut model[..], CloneWitness::new(value));
            std_slice.fill(CloneWitness::new(value));
            prop_assert_eq!(model, std_slice);
        }

        #[test]
        fn test_index_usize(slice in prop::collection::vec(any::<u8>(), 4..=4), idx in 0usize..4) {
            let s: &[u8] = &slice[..];
            prop_assert_eq!(
                crate::ops::index::Index::index(&s, idx),
                &slice[idx]
            );
        }

        #[test]
        fn test_index_range(slice in prop::collection::vec(any::<u8>(), 8..=8), start in 0usize..8, len in 0usize..8) {
            let end = (start + len).min(8);
            let s: &[u8] = &slice[..];
            prop_assert_eq!(
                crate::ops::index::Index::index(&s, crate::ops::range::Range { start, end }),
                &slice[start..end]
            );
        }

        #[test]
        fn test_index_range_to(slice in prop::collection::vec(any::<u8>(), 8..=8), end in 0usize..=8) {
            let s: &[u8] = &slice[..];
            prop_assert_eq!(
                crate::ops::index::Index::index(&s, crate::ops::range::RangeTo { end }),
                &slice[..end]
            );
        }

        #[test]
        fn test_index_range_from(slice in prop::collection::vec(any::<u8>(), 8..=8), start in 0usize..=8) {
            let s: &[u8] = &slice[..];
            prop_assert_eq!(
                crate::ops::index::Index::index(&s, crate::ops::range::RangeFrom { start }),
                &slice[start..]
            );
        }

        #[test]
        fn test_index_range_full(slice in prop::collection::vec(any::<u8>(), 8..=8)) {
            let s: &[u8] = &slice[..];
            prop_assert_eq!(
                crate::ops::index::Index::index(&s, crate::ops::range::RangeFull),
                &slice[..]
            );
        }

        // ----- PartialEq / PartialOrd / Ord (lexicographic) ------------------

        #[test]
        fn test_slice_eq(
            a in prop::collection::vec(any::<u8>(), 0..=8),
            b in prop::collection::vec(any::<u8>(), 0..=8),
        ) {
            prop_assert_eq!(
                <[u8] as crate::cmp::PartialEq<[u8]>>::eq(&a[..], &b[..]),
                a == b
            );
        }

        // Equal-length pairs make the per-element comparison the deciding factor
        // more often than two independent (usually different-length) slices.
        #[test]
        fn test_slice_eq_same_len(pairs in prop::collection::vec((any::<u8>(), any::<u8>()), 0..=8)) {
            let a: Vec<u8> = pairs.iter().map(|p| p.0).collect();
            let b: Vec<u8> = pairs.iter().map(|p| p.1).collect();
            prop_assert_eq!(
                <[u8] as crate::cmp::PartialEq<[u8]>>::eq(&a[..], &b[..]),
                a == b
            );
        }

        // Two equal, non-empty slices: the element loop's accumulator is only
        // exercised in its stays-true form here, which independent draws
        // essentially never produce.
        #[test]
        fn test_slice_eq_reflexive(a in prop::collection::vec(any::<u8>(), 1..=8)) {
            prop_assert!(<[u8] as crate::cmp::PartialEq<[u8]>>::eq(&a[..], &a[..]));
        }

        #[test]
        fn test_slice_partial_cmp(
            a in prop::collection::vec(any::<u8>(), 0..=8),
            b in prop::collection::vec(any::<u8>(), 0..=8),
        ) {
            prop_assert_eq!(
                <[u8] as crate::cmp::PartialOrd<[u8]>>::partial_cmp(&a[..], &b[..]),
                a[..].partial_cmp(&b[..]).inject()
            );
        }

        #[test]
        fn test_slice_cmp(
            a in prop::collection::vec(any::<u8>(), 0..=8),
            b in prop::collection::vec(any::<u8>(), 0..=8),
        ) {
            prop_assert_eq!(
                <[u8] as crate::cmp::Ord>::cmp(&a[..], &b[..]),
                a[..].cmp(&b[..]).inject()
            );
        }

        // A shared prefix guarantees at least one `Equal` element comparison,
        // which two independent slices only reach by chance.
        #[test]
        fn test_slice_cmp_shared_prefix(
            prefix in prop::collection::vec(any::<u8>(), 1..=4),
            a_tail in prop::collection::vec(any::<u8>(), 0..=4),
            b_tail in prop::collection::vec(any::<u8>(), 0..=4),
        ) {
            let mut a = prefix.clone();
            a.extend(a_tail);
            let mut b = prefix;
            b.extend(b_tail);
            prop_assert_eq!(
                <[u8] as crate::cmp::Ord>::cmp(&a[..], &b[..]),
                a[..].cmp(&b[..]).inject()
            );
            prop_assert_eq!(
                <[u8] as crate::cmp::PartialOrd<[u8]>>::partial_cmp(&a[..], &b[..]),
                a[..].partial_cmp(&b[..]).inject()
            );
        }

        // `use_equal` makes the equal case common, which is what `ne` turns on.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_ne_slice(
            a in prop::collection::vec(0u8..4, 0..=4),
            b in prop::collection::vec(0u8..4, 0..=4),
            use_equal in any::<bool>(),
        ) {
            let b: Vec<u8> = if use_equal { a.clone() } else { b };
            prop_assert_eq!(
                <[u8] as crate::cmp::PartialEq<[u8]>>::ne(&a[..], &b[..]),
                a[..] != b[..]
            );
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_ne_slice_array(
            arr in any::<[u8; 3]>(),
            other in any::<[u8; 3]>(),
            kind in 0u8..3,
        ) {
            // All three shapes: equal, same length but different (the
            // element-wise loop), and a length mismatch (the early `false`).
            let v: Vec<u8> = match kind {
                0 => arr.to_vec(),
                1 => other.to_vec(),
                _ => vec![arr[0]],
            };
            let s: &[u8] = &v[..];
            prop_assert_eq!(
                <[u8] as crate::cmp::PartialEq<[u8; 3]>>::ne(s, &arr),
                s != arr
            );
        }

        // `index_mut` writes through the model's `IndexMut` (`v[i] = x`).
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_index_mut(v in prop::collection::vec(any::<u8>(), 1..=8), x in any::<u8>()) {
            let mut model = v.clone();
            let mut std_v = v.clone();
            let i = x as usize % v.len();
            *crate::ops::index::IndexMut::index_mut(&mut model[..], i) = x;
            std_v[i] = x;
            prop_assert_eq!(model, std_v);
        }

        // `[T]: PartialEq<[U; N]>` — slice vs array (`s == [..]`). `use_equal`
        // biases toward the equal case, which random slices rarely hit.
        #[test]
        fn test_eq_array(
            arr in any::<[u8; 3]>(),
            other in prop::collection::vec(any::<u8>(), 0..=6),
            use_equal in any::<bool>(),
        ) {
            let v: Vec<u8> = if use_equal { arr.to_vec() } else { other };
            let s: &[u8] = &v[..];
            let model = <[u8] as crate::cmp::PartialEq<[u8; 3]>>::eq(s, &arr);
            let std_eq = s == arr;
            prop_assert_eq!(model, std_eq);
        }

        // The same through a shared reference, which std models as its own impl.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_eq_array_shared(
            arr in any::<[u8; 3]>(),
            other in prop::collection::vec(any::<u8>(), 0..=6),
            use_equal in any::<bool>(),
        ) {
            let v: Vec<u8> = if use_equal { arr.to_vec() } else { other };
            let s: &[u8] = &v[..];
            let std_eq = s == arr;
            prop_assert_eq!(<&[u8] as crate::cmp::PartialEq<[u8; 3]>>::eq(&s, &arr), std_eq);
            prop_assert_eq!(<&[u8] as crate::cmp::PartialEq<[u8; 3]>>::ne(&s, &arr), !std_eq);
        }

        // ----- get_unchecked (in-bounds) -------------------------------------

        #[test]
        fn test_get_unchecked_usize(slice in prop::collection::vec(any::<u8>(), 4..=4), idx in 0usize..4) {
            let s: &[u8] = &slice[..];
            prop_assert_eq!(Slice::get_unchecked(s, idx), unsafe { s.get_unchecked(idx) });
        }

        #[test]
        fn test_get_unchecked_range(slice in prop::collection::vec(any::<u8>(), 8..=8), start in 0usize..8, len in 0usize..8) {
            let end = (start + len).min(8);
            let s: &[u8] = &slice[..];
            prop_assert_eq!(
                Slice::get_unchecked(s, crate::ops::range::Range { start, end }),
                unsafe { s.get_unchecked(start..end) }
            );
        }

        #[test]
        fn test_get_unchecked_range_from(slice in prop::collection::vec(any::<u8>(), 8..=8), start in 0usize..=8) {
            let s: &[u8] = &slice[..];
            prop_assert_eq!(
                Slice::get_unchecked(s, crate::ops::range::RangeFrom { start }),
                unsafe { s.get_unchecked(start..) }
            );
        }

        #[test]
        fn test_get_unchecked_range_to(slice in prop::collection::vec(any::<u8>(), 8..=8), end in 0usize..=8) {
            let s: &[u8] = &slice[..];
            prop_assert_eq!(
                Slice::get_unchecked(s, crate::ops::range::RangeTo { end }),
                unsafe { s.get_unchecked(..end) }
            );
        }

        #[test]
        fn test_get_unchecked_range_full(slice in prop::collection::vec(any::<u8>(), 0..=8)) {
            let s: &[u8] = &slice[..];
            prop_assert_eq!(
                Slice::get_unchecked(s, crate::ops::range::RangeFull),
                unsafe { s.get_unchecked(..) }
            );
        }

        // ----- get_mut / get_unchecked_mut (mutate through the &mut) ---------

        // `get_mut` / `get_unchecked_mut` have no F* model.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_get_mut_usize(slice in prop::collection::vec(any::<u8>(), 1..=10), idx in prop_oneof![0usize..=11, any::<usize>()], v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            if let crate::option::Option::Some(r) = Slice::get_mut(&mut model[..], idx) {
                *r = v;
            }
            if let Some(r) = std_slice.get_mut(idx) {
                *r = v;
            }
            prop_assert_eq!(model, std_slice);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_get_mut_range(slice in prop::collection::vec(any::<u8>(), 1..=10), start in 0usize..10, end in 0usize..10, v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            if let crate::option::Option::Some(r) = Slice::get_mut(&mut model[..], crate::ops::range::Range { start, end }) {
                r.fill(v);
            }
            if let Some(r) = std_slice.get_mut(start..end) {
                r.fill(v);
            }
            prop_assert_eq!(model, std_slice);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_get_mut_range_from(slice in prop::collection::vec(any::<u8>(), 1..=10), start in 0usize..=10, v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            if let crate::option::Option::Some(r) = Slice::get_mut(&mut model[..], crate::ops::range::RangeFrom { start }) {
                r.fill(v);
            }
            if let Some(r) = std_slice.get_mut(start..) {
                r.fill(v);
            }
            prop_assert_eq!(model, std_slice);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_get_mut_range_to(slice in prop::collection::vec(any::<u8>(), 1..=10), end in 0usize..=10, v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            if let crate::option::Option::Some(r) = Slice::get_mut(&mut model[..], crate::ops::range::RangeTo { end }) {
                r.fill(v);
            }
            if let Some(r) = std_slice.get_mut(..end) {
                r.fill(v);
            }
            prop_assert_eq!(model, std_slice);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_get_mut_range_full(slice in prop::collection::vec(any::<u8>(), 0..=10), v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            if let crate::option::Option::Some(r) = Slice::get_mut(&mut model[..], crate::ops::range::RangeFull) {
                r.fill(v);
            }
            if let Some(r) = std_slice.get_mut(..) {
                r.fill(v);
            }
            prop_assert_eq!(model, std_slice);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_get_unchecked_mut_usize(slice in prop::collection::vec(any::<u8>(), 4..=4), idx in 0usize..4, v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            *Slice::get_unchecked_mut(&mut model[..], idx) = v;
            unsafe { *std_slice.get_unchecked_mut(idx) = v; }
            prop_assert_eq!(model, std_slice);
        }

        // ----- binary_search -------------------------------------------------

        // Not deduplicated, and drawn narrow so equal runs are common.
        #[test]
        fn test_binary_search(
            values in prop::collection::vec(0u8..=8, 0..=12),
            needle in 0u8..=8,
        ) {
            let mut sorted = values;
            sorted.sort();
            prop_assert_eq!(
                Slice::binary_search(&sorted[..], &needle),
                sorted.binary_search(&needle).inject()
            );
        }

        #[test]
        fn test_binary_search_all_equal(len in 0usize..=12, needle in 0u8..=1) {
            let values = vec![1u8; len];
            prop_assert_eq!(
                Slice::binary_search(&values[..], &needle),
                values.binary_search(&needle).inject()
            );
        }

        #[test]
        fn test_binary_search_unsorted(
            values in prop::collection::vec(0u8..=8, 0..=12),
            needle in 0u8..=8,
        ) {
            prop_assert_eq!(
                Slice::binary_search(&values[..], &needle),
                values.binary_search(&needle).inject()
            );
        }

        // ----- SliceIndex::index and Index for [T] ---------------------------

        #[test]
        fn test_slice_index_trait_usize(slice in prop::collection::vec(any::<u8>(), 4..=4), idx in 0usize..4) {
            use crate::slice::index::SliceIndex;
            prop_assert_eq!(SliceIndex::index(idx, &slice[..]), &slice[idx]);
        }

        #[test]
        fn test_slice_index_trait_range(slice in prop::collection::vec(any::<u8>(), 8..=8), start in 0usize..8, len in 0usize..8) {
            use crate::slice::index::SliceIndex;
            let end = (start + len).min(8);
            prop_assert_eq!(
                SliceIndex::index(crate::ops::range::Range { start, end }, &slice[..]),
                &slice[start..end]
            );
        }

        #[test]
        fn test_slice_index_trait_range_to(slice in prop::collection::vec(any::<u8>(), 8..=8), end in 0usize..=8) {
            use crate::slice::index::SliceIndex;
            prop_assert_eq!(
                SliceIndex::index(crate::ops::range::RangeTo { end }, &slice[..]),
                &slice[..end]
            );
        }

        #[test]
        fn test_slice_index_trait_range_from(slice in prop::collection::vec(any::<u8>(), 8..=8), start in 0usize..=8) {
            use crate::slice::index::SliceIndex;
            prop_assert_eq!(
                SliceIndex::index(crate::ops::range::RangeFrom { start }, &slice[..]),
                &slice[start..]
            );
        }

        #[test]
        fn test_slice_index_trait_range_full(slice in prop::collection::vec(any::<u8>(), 0..=8)) {
            use crate::slice::index::SliceIndex;
            prop_assert_eq!(
                SliceIndex::index(crate::ops::range::RangeFull, &slice[..]),
                &slice[..]
            );
        }

        // `Index<I> for [T]`, as opposed to the `Index for &[T]` impls the
        // `test_index_*` tests above reach.
        #[test]
        fn test_unsized_index_usize(slice in prop::collection::vec(any::<u8>(), 4..=4), idx in 0usize..4) {
            prop_assert_eq!(
                <[u8] as crate::ops::index::Index<usize>>::index(&slice[..], idx),
                &slice[idx]
            );
        }

        // ----- in-bounds `get` / `get_mut` on `usize` ------------------------
        //
        // The `test_get_usize` / `test_get_mut_usize` tests above draw the index
        // from the whole `usize` range, so they only ever see `None`.

        #[test]
        fn test_get_usize_in_bounds(slice in prop::collection::vec(any::<u8>(), 1..=10), idx in 0usize..10) {
            let idx = idx % slice.len();
            prop_assert_eq!(
                Slice::get(&slice[..], idx).map(|v: &u8| *v),
                slice.get(idx).copied().inject()
            );
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_get_mut_usize_in_bounds(slice in prop::collection::vec(any::<u8>(), 1..=10), idx in 0usize..10, v in any::<u8>()) {
            let idx = idx % slice.len();
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            if let crate::option::Option::Some(r) = Slice::get_mut(&mut model[..], idx) {
                *r = v;
            }
            if let Some(r) = std_slice.get_mut(idx) {
                *r = v;
            }
            prop_assert_eq!(model, std_slice);
        }

        // ----- IntoIterator for &[T] -----------------------------------------

        #[test]
        fn test_slice_into_iter(slice in prop::collection::vec(any::<u8>(), 0..=10)) {
            let it = crate::iter::traits::collect::IntoIterator::into_iter(&slice[..]);
            prop_assert_eq!(drain(it), slice.iter().collect::<Vec<_>>());
        }

        // ----- get_unchecked_mut for the remaining range kinds ---------------

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_get_unchecked_mut_range_from(slice in prop::collection::vec(any::<u8>(), 8..=8), start in 0usize..=8, v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            Slice::get_unchecked_mut(&mut model[..], crate::ops::range::RangeFrom { start }).fill(v);
            unsafe { std_slice.get_unchecked_mut(start..).fill(v); }
            prop_assert_eq!(model, std_slice);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_get_unchecked_mut_range_to(slice in prop::collection::vec(any::<u8>(), 8..=8), end in 0usize..=8, v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            Slice::get_unchecked_mut(&mut model[..], crate::ops::range::RangeTo { end }).fill(v);
            unsafe { std_slice.get_unchecked_mut(..end).fill(v); }
            prop_assert_eq!(model, std_slice);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_get_unchecked_mut_range_full(slice in prop::collection::vec(any::<u8>(), 0..=8), v in any::<u8>()) {
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            Slice::get_unchecked_mut(&mut model[..], crate::ops::range::RangeFull).fill(v);
            unsafe { std_slice.get_unchecked_mut(..).fill(v); }
            prop_assert_eq!(model, std_slice);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_get_unchecked_mut_range(slice in prop::collection::vec(any::<u8>(), 8..=8), start in 0usize..8, len in 0usize..8, v in any::<u8>()) {
            let end = (start + len).min(8);
            let mut model = slice.clone();
            let mut std_slice = slice.clone();
            Slice::get_unchecked_mut(&mut model[..], crate::ops::range::Range { start, end }).fill(v);
            unsafe { std_slice.get_unchecked_mut(start..end).fill(v); }
            prop_assert_eq!(model, std_slice);
        }
    }

    proptest! {
        #[test]
        fn test_try_range(
            start in crate::testing::range_endpoint(),
            end in crate::testing::range_endpoint(),
            len in 0usize..=8,
        ) {
            for (m, s) in crate::testing::range_forms(start, end) {
                let model = super::index::try_range(m, crate::ops::range::RangeTo { end: len });
                let model = match model {
                    ModelOption::Some(r) => Some((r.start, r.end)),
                    ModelOption::None => None,
                };
                let real = std::slice::try_range(s, ..len).map(|r| (r.start, r.end));
                prop_assert_eq!(model, real);
            }
        }

        #[test]
        fn test_range(
            start in crate::testing::range_endpoint(),
            end in crate::testing::range_endpoint(),
            len in 0usize..=8,
        ) {
            for (m, s) in crate::testing::range_forms(start, end) {
                let bounds = crate::ops::range::RangeTo { end: len };
                match std::slice::try_range(s, ..len) {
                    Some(real) => {
                        let model = super::index::range(m, bounds);
                        prop_assert_eq!((model.start, model.end), (real.start, real.end));
                    }
                    None => crate::testing::panics_like_core(
                        || super::index::range(m, bounds),
                        || std::slice::range(s, ..len),
                    ),
                }
            }
        }

        #[test]
        fn test_copy_within(
            slice in prop::collection::vec(any::<u8>(), 0..=8),
            start in crate::testing::range_endpoint(),
            end in crate::testing::range_endpoint(),
            dest in crate::testing::range_endpoint(),
        ) {
            for (m, s) in crate::testing::range_forms(start, end) {
                let mut model = slice.clone();
                let mut real = slice.clone();
                let len = slice.len();
                match std::slice::try_range(s, ..len) {
                    Some(r) if dest <= len - (r.end - r.start) => {
                        Slice::copy_within(&mut model[..], m, dest);
                        real.copy_within(s, dest);
                        prop_assert_eq!(model, real);
                    }
                    _ => crate::testing::panics_like_core(
                        || Slice::copy_within(&mut model[..], m, dest),
                        || real.copy_within(s, dest),
                    ),
                }
            }
        }
    }

    #[test]
    fn test_chunks_zero_panics() {
        crate::testing::panics_like_core(
            || Slice::chunks(&[1u8, 2, 3][..], 0),
            || [1u8, 2, 3].chunks(0),
        );
    }

    #[test]
    #[allow(unconditional_panic)] // some rustc versions reject `N == 0` statically
    fn test_as_chunks_zero_panics() {
        crate::testing::panics_like_core(
            || Slice::as_chunks::<0>(&[1u8][..]).1.len(),
            || [1u8].as_chunks::<0>().1.len(),
        );
    }

    #[cfg(not(hax_backend_fstar))]
    #[test]
    #[allow(unconditional_panic)] // some rustc versions reject `N == 0` statically
    fn test_as_chunks_mut_zero_panics() {
        crate::testing::panics_like_core(
            || Slice::as_chunks_mut::<0>(&mut [1u8][..]).1.len(),
            || [1u8].as_chunks_mut::<0>().1.len(),
        );
    }

    #[test]
    fn test_chunks_exact_zero_panics() {
        crate::testing::panics_like_core(
            || Slice::chunks_exact(&[1u8, 2, 3][..], 0),
            || [1u8, 2, 3].chunks_exact(0),
        );
    }

    #[test]
    fn test_windows_zero_panics() {
        crate::testing::panics_like_core(
            || Slice::windows(&[1u8, 2, 3][..], 0),
            || [1u8, 2, 3].windows(0),
        );
    }

    #[test]
    fn test_split_at_past_end_panics() {
        crate::testing::panics_like_core(
            || Slice::split_at(&[1u8, 2, 3][..], 4),
            || [1u8, 2, 3].split_at(4),
        );
    }

    #[test]
    fn test_swap_out_of_bounds_panics() {
        crate::testing::panics_like_core(
            || Slice::swap(&mut [1u8, 2, 3][..], 0, 3),
            || [1u8, 2, 3].swap(0, 3),
        );
    }

    #[test]
    fn test_copy_from_slice_length_mismatch_panics() {
        crate::testing::panics_like_core(
            || Slice::copy_from_slice(&mut [0u8; 3][..], &[1u8, 2][..]),
            || [0u8; 3].copy_from_slice(&[1u8, 2][..]),
        );
    }

    #[test]
    fn test_clone_from_slice_length_mismatch_panics() {
        crate::testing::panics_like_core(
            || Slice::clone_from_slice(&mut [0u8; 3][..], &[1u8, 2][..]),
            || [0u8; 3].clone_from_slice(&[1u8, 2][..]),
        );
    }

    #[cfg(not(hax_backend_fstar))]
    #[test]
    fn test_index_out_of_bounds_panics() {
        // `black_box` the index: a literal one is a compile-time error, not a panic.
        let (model, real): (&[u8], [u8; 3]) = (&[1u8, 2, 3], [1u8, 2, 3]);
        let i = std::hint::black_box(3usize);
        crate::testing::panics_like_core(|| model[i], || real[i]);
    }

    // `Index<I> for [T]` panics through its `Option::None` arm.
    #[test]
    fn test_unsized_index_out_of_bounds_panics() {
        let (model, real): (&[u8], [u8; 3]) = (&[1u8, 2, 3], [1u8, 2, 3]);
        let i = std::hint::black_box(3usize);
        crate::testing::panics_like_core(
            || <[u8] as crate::ops::index::Index<usize>>::index(model, i),
            || real[i],
        );
    }

    #[cfg(not(hax_backend_fstar))]
    #[test]
    fn test_index_range_past_end_panics() {
        let (model, real): (&[u8], [u8; 3]) = (&[1u8, 2, 3], [1u8, 2, 3]);
        let end = std::hint::black_box(4usize);
        crate::testing::panics_like_core(|| &model[1..end], || &real[1..end]);
    }
}
