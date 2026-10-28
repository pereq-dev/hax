#![allow(unused)]
// `coverage(off)` is unstable; `cfg(coverage_nightly)` is set only by
// `cargo llvm-cov`, so normal builds and extraction never see this.
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
// `Vec::drain` resolves its range with `core::slice::range`, which is unstable.
#![feature(slice_range)]

#[cfg(test)]
mod testing {
    pub trait Inject {
        type Model;
        fn inject(&self) -> Self::Model;
    }

    /// Asserts the model and real `alloc` both panic on the same input.
    // Test harness, not model code: its failure branches only run when a test fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    #[track_caller]
    pub fn panics_like_core<A, B>(model: impl FnOnce() -> A, core: impl FnOnce() -> B) {
        use std::panic::{AssertUnwindSafe, catch_unwind};
        let m = catch_unwind(AssertUnwindSafe(model));
        let c = catch_unwind(AssertUnwindSafe(core));
        assert!(m.is_err(), "the model did not panic");
        assert!(
            c.is_err(),
            "real `alloc` did not panic, so the model must not either"
        );
    }
}

mod alloc {
    pub trait Allocator {}

    #[cfg_attr(test, derive(PartialEq, Debug))]
    #[derive(Clone)]
    pub struct Global;

    impl Allocator for Global {}
}

mod borrow {
    struct Cow<T>(T);

    pub trait ToOwned {
        fn to_owned(self) -> Self;
    }
    impl<T> ToOwned for T {
        fn to_owned(self) -> Self {
            self
        }
    }

    #[cfg(test)]
    mod tests {
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn test_to_owned(v in prop::collection::vec(any::<u8>(), 0..20)) {
                prop_assert_eq!(super::ToOwned::to_owned(v.clone()), v);
            }
        }
    }
}

mod boxed {
    pub struct Box<T>(pub T);
    impl<T> Box<T> {
        // Hax removes boxes, so this should be the identity
        fn new(v: T) -> T {
            v
        }
    }

    /// See [`std::ops::Deref`] for `Box<T>`
    impl<T> core::ops::Deref for Box<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    /// See [`std::clone::Clone`] for `Box<T>`
    impl<T: Clone> Clone for Box<T> {
        fn clone(&self) -> Box<T> {
            Box(self.0.clone())
        }
    }

    /// See [`std::cmp::PartialEq`] for `Box<T>`
    //
    // Lean-only: hax renders `self.0 == other.0` as F*'s primitive `=.`, which
    // demands `T: eqtype` — a `Type0` parameter is not one.
    #[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
    impl<T: PartialEq<U>, U> PartialEq<Box<U>> for Box<T> {
        #[cfg(not(hax_backend_fstar))]
        fn ne(&self, other: &Box<U>) -> bool {
            self.eq(other) == false
        }
        fn eq(&self, other: &Box<U>) -> bool {
            self.0 == other.0
        }
    }

    #[cfg(test)]
    mod tests {
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn test_new_is_identity(x in any::<u8>()) {
                prop_assert_eq!(super::Box::<u8>::new(x), x);
            }

            // The trait impls, each against the same operation on std's `Box`.
            #[test]
            fn test_deref(x in any::<u8>()) {
                prop_assert_eq!(
                    *core::ops::Deref::deref(&super::Box(x)),
                    *core::ops::Deref::deref(&std::boxed::Box::new(x))
                );
            }

            #[test]
            fn test_clone(x in any::<u8>()) {
                prop_assert_eq!(
                    super::Box(x).clone().0,
                    *std::boxed::Box::new(x).clone()
                );
            }

            #[test]
            fn test_eq(x in 0u8..4, y in 0u8..4) {
                prop_assert_eq!(
                    super::Box(x) == super::Box(y),
                    std::boxed::Box::new(x) == std::boxed::Box::new(y)
                );
            }

            #[cfg(not(hax_backend_fstar))]
            #[test]
            fn test_ne(x in 0u8..4, y in 0u8..4) {
                prop_assert_eq!(
                    super::Box(x) != super::Box(y),
                    std::boxed::Box::new(x) != std::boxed::Box::new(y)
                );
            }
        }
    }
}

mod collections {
    // All implementations are dummy (for interfaces only)

    mod binary_heap {
        #[hax_lib::fstar::before("open Rust_primitives.Notations")]
        use crate::vec::*;
        struct BinaryHeap<T, A>(Vec<T>, std::marker::PhantomData<A>);

        impl BinaryHeap<(), ()> {}
        impl BinaryHeap<(), ()> {}
        impl BinaryHeap<(), ()> {}
        impl BinaryHeap<(), ()> {}
        impl BinaryHeap<(), ()> {}
        impl BinaryHeap<(), ()> {}
        impl BinaryHeap<(), ()> {}
        impl BinaryHeap<(), ()> {}
        impl BinaryHeap<(), ()> {}
        impl BinaryHeap<(), ()> {}

        #[hax_lib::attributes]
        impl<T: Ord, A: crate::alloc::Allocator> BinaryHeap<T, A> {
            fn new() -> BinaryHeap<T, A> {
                BinaryHeap(
                    crate::vec::from_seq(rust_primitives::sequence::seq_empty()),
                    std::marker::PhantomData::<A>,
                )
            }
            #[hax_lib::requires(self.len() < core::primitive::usize::MAX)]
            fn push(&mut self, v: T) {
                self.0.push(v)
            }
            #[hax_lib::ensures(|res| (self.len() > 0) == res.is_some())]
            fn pop(&mut self) -> Option<T> {
                let mut max: Option<&T> = None;
                let mut index = 0;
                for i in 0..self.len() {
                    hax_lib::loop_invariant!(
                        |i: usize| (i > 0) == max.is_some() && (i == 0 || index < i)
                    );
                    if max.is_none_or(|max| self.0[i] > *max) {
                        max = Some(&self.0[i]);
                        index = i;
                    }
                }
                if max.is_some() {
                    Some(self.0.remove(index))
                } else {
                    None
                }
            }
        }

        #[hax_lib::attributes]
        impl<T: Ord, A: crate::alloc::Allocator> BinaryHeap<T, A> {
            fn len(&self) -> usize {
                self.0.len()
            }

            #[hax_lib::ensures(|res| (self.len() > 0) == res.is_some())]
            fn peek(&self) -> Option<&T> {
                let mut max: Option<&T> = None;
                for i in 0..self.len() {
                    hax_lib::loop_invariant!(|i: usize| (i > 0) == max.is_some());
                    if max.is_none_or(|max| self.0[i] > *max) {
                        max = Some(&self.0[i]);
                    }
                }
                max
            }
        }

        #[hax_lib::fstar::after(
            "
assume val lemma_peek_pop: #t:Type -> (#a: Type) -> (#i: Core_models.Cmp.t_Ord t) 
  -> (#i1: Alloc.Alloc.t_Allocator a) -> h: t_BinaryHeap t a
  -> Lemma (impl_11__peek h == snd (impl_10__pop h))
          [SMTPat (impl_11__peek #t #a h)]
        "
        )]
        use core::*;

        #[cfg(test)]
        mod tests {
            use proptest::prelude::*;

            proptest! {
                #[test]
                fn test_push_pop(elements in prop::collection::vec(any::<u8>(), 1..20)) {
                    let mut model = super::BinaryHeap::<u8, crate::alloc::Global>::new();
                    let mut std_heap = std::collections::BinaryHeap::new();
                    for &e in &elements {
                        model.push(e);
                        std_heap.push(e);
                    }
                    prop_assert_eq!(model.len(), std_heap.len());

                    loop {
                        let model_val = model.pop();
                        let std_val = std_heap.pop();
                        prop_assert_eq!(model_val, std_val);
                        if model_val.is_none() {
                            break;
                        }
                    }
                }

                #[test]
                fn test_peek(elements in prop::collection::vec(any::<u8>(), 1..20)) {
                    let mut model = super::BinaryHeap::<u8, crate::alloc::Global>::new();
                    let mut std_heap = std::collections::BinaryHeap::new();
                    for &e in &elements {
                        model.push(e);
                        std_heap.push(e);
                    }
                    prop_assert_eq!(model.peek().copied(), std_heap.peek().copied());
                }
            }

            #[test]
            fn test_new() {
                let mut model = super::BinaryHeap::<u8, crate::alloc::Global>::new();
                let mut std_heap = std::collections::BinaryHeap::<u8>::new();
                assert_eq!(model.len(), std_heap.len());
                assert_eq!(model.pop(), std_heap.pop());
            }
        }
    }
    mod btree {
        mod set {
            #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
            struct BTreeSet<T, U>(Option<T>, Option<U>);

            impl BTreeSet<(), ()> {}
            impl BTreeSet<(), ()> {}
            impl BTreeSet<(), ()> {}
            impl BTreeSet<(), ()> {}
            impl BTreeSet<(), ()> {}
            impl BTreeSet<(), ()> {}
            impl BTreeSet<(), ()> {}
            impl BTreeSet<(), ()> {}
            impl BTreeSet<(), ()> {}
            impl BTreeSet<(), ()> {}
            impl BTreeSet<(), ()> {}

            impl<T, U> BTreeSet<T, U> {
                // Excluded from coverage: the set is a dummy with nowhere to
                // hold elements, so the only thing a test could pin is the
                // dummy shape itself.
                #[cfg_attr(coverage_nightly, coverage(off))]
                #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
                fn new() -> BTreeSet<T, U> {
                    BTreeSet(None, None)
                }
            }
        }
    }
    mod vec_deque {
        use rust_primitives::sequence::*;
        pub struct VecDeque<T, A>(pub Seq<T>, std::marker::PhantomData<A>);

        impl VecDeque<(), ()> {}
        impl VecDeque<(), ()> {}
        impl VecDeque<(), ()> {}
        impl VecDeque<(), ()> {}

        impl<T> VecDeque<T, crate::alloc::Global> {
            fn new() -> VecDeque<T, crate::alloc::Global> {
                VecDeque(seq_empty(), std::marker::PhantomData)
            }
            fn with_capacity(_capacity: usize) -> VecDeque<T, crate::alloc::Global> {
                VecDeque::new()
            }
        }

        #[hax_lib::attributes]
        impl<T, A> VecDeque<T, A> {
            #[hax_lib::requires(seq_len(&self.0) < core::primitive::usize::MAX)]
            fn push_back(&mut self, x: T) {
                seq_push(&mut self.0, x)
            }
            fn len(&self) -> usize {
                seq_len(&self.0)
            }
            fn pop_front(&mut self) -> Option<T> {
                if self.len() == 0 {
                    None
                } else {
                    Some(seq_remove(&mut self.0, 0))
                }
            }
        }

        #[hax_lib::attributes]
        impl<T, A> std::ops::Index<usize> for VecDeque<T, A> {
            type Output = T;
            #[hax_lib::requires(i < self.len())]
            fn index(&self, i: usize) -> &T {
                seq_index(&self.0, i)
            }
        }

        pub mod into_iter {
            use rust_primitives::sequence::*;
            pub struct IntoIter<T, A>(pub Seq<T>, pub std::marker::PhantomData<A>);
            impl<T, A> Iterator for IntoIter<T, A> {
                type Item = T;
                fn next(&mut self) -> Option<Self::Item> {
                    if seq_len(&self.0) == 0 {
                        None
                    } else {
                        Some(seq_remove(&mut self.0, 0))
                    }
                }
            }
        }

        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        impl<T, A> IntoIterator for VecDeque<T, A> {
            type Item = T;
            type IntoIter = into_iter::IntoIter<T, A>;
            fn into_iter(self) -> Self::IntoIter {
                into_iter::IntoIter(self.0, std::marker::PhantomData)
            }
        }

        // Like `Vec`, `FromIterator` is only implemented for the `Global`
        // allocator (std has no way to thread an allocator through
        // `from_iter`), so `Self` is `VecDeque<T, Global>` — matching the
        // `VecDequeTGlobal` impl name downstream expects.
        #[hax_lib::attributes]
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        impl<T> std::iter::FromIterator<T> for VecDeque<T, crate::alloc::Global> {
            fn from_iter<I>(iter: I) -> Self
            where
                I: IntoIterator<Item = T>,
            {
                let mut res = VecDeque(seq_empty(), std::marker::PhantomData);
                for el in iter {
                    res.push_back(el)
                }
                res
            }
        }

        #[cfg(hax_backend_fstar)]
        #[hax_lib::fstar::after(
            "
[@@ FStar.Tactics.Typeclasses.tcinstance]
let update_at_usize (#v_T #v_A: Type0)
    : Rust_primitives.Hax.update_at_tc (t_VecDeque v_T v_A) usize =
  {
    super_index = impl_6 #v_T #v_A;
    // `i` is deliberately left unannotated: the class gives it the refinement
    // `f_index_pre self i` (here `i < len self`), and annotating it `usize`
    // would drop exactly the bound `Seq.upd` needs.
    update_at = (fun self i x -> VecDeque (FStar.Seq.upd self._0 (v i) x) self._1)
  }
        "
        )]
        use core::*;

        #[cfg(test)]
        mod tests {
            use proptest::prelude::*;

            type Model<T> = super::VecDeque<T, crate::alloc::Global>;

            proptest! {
                #[test]
                fn test_push_back_len_index(elements in prop::collection::vec(any::<u8>(), 0..20)) {
                    let mut model = Model::new();
                    let mut std_deque = std::collections::VecDeque::new();
                    for &e in &elements {
                        model.push_back(e);
                        std_deque.push_back(e);
                    }
                    prop_assert_eq!(model.len(), std_deque.len());
                    for i in 0..std_deque.len() {
                        prop_assert_eq!(model[i], std_deque[i]);
                    }
                }

                #[test]
                fn test_into_iter(elements in prop::collection::vec(any::<u8>(), 0..20)) {
                    let mut model = Model::new();
                    let mut std_deque = std::collections::VecDeque::new();
                    for &e in &elements {
                        model.push_back(e);
                        std_deque.push_back(e);
                    }
                    let mut it = IntoIterator::into_iter(model);
                    let mut collected = std::vec::Vec::new();
                    while let Some(x) = it.next() {
                        collected.push(x);
                    }
                    prop_assert_eq!(collected, std_deque.into_iter().collect::<std::vec::Vec<u8>>());
                }

                #[test]
                fn test_from_iter(elements in prop::collection::vec(any::<u8>(), 0..20)) {
                    let model: Model<u8> = elements.clone().into_iter().collect();
                    let std_deque: std::collections::VecDeque<u8> =
                        elements.into_iter().collect();
                    prop_assert_eq!(model.len(), std_deque.len());
                    for i in 0..std_deque.len() {
                        prop_assert_eq!(model[i], std_deque[i]);
                    }
                }

                #[test]
                fn test_pop_front(elements in prop::collection::vec(any::<u8>(), 0..20)) {
                    let mut model = Model::with_capacity(elements.len());
                    let mut std_deque = std::collections::VecDeque::with_capacity(elements.len());
                    for &e in &elements {
                        model.push_back(e);
                        std_deque.push_back(e);
                    }
                    for _ in 0..=elements.len() {
                        prop_assert_eq!(model.pop_front(), std_deque.pop_front());
                    }
                }
            }

            #[test]
            fn test_new() {
                let mut model = Model::<u8>::new();
                let mut std_deque = std::collections::VecDeque::<u8>::new();
                assert_eq!(model.len(), std_deque.len());
                assert_eq!(model.pop_front(), std_deque.pop_front());
            }
        }
    }
}

mod fmt {
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    fn format(args: core::fmt::Arguments) -> String {
        String::new()
    }

    #[cfg(test)]
    mod tests {
        use proptest::prelude::*;

        proptest! {
            // `fmt::Arguments` is not modelled, so this is a deliberate
            // placeholder (kept opaque for charon; see the Makefile).
            #[test]
            fn test_format_is_a_placeholder(x in any::<u8>()) {
                // Bound first: `prop_assert!` stringifies its argument into a
                // format string, where a literal `{}` would be a placeholder.
                let formatted = super::format(format_args!("{}", x));
                prop_assert!(formatted.is_empty());
            }
        }
    }
}

mod slice {
    // F*-only: `charon::exclude` would drop this dummy type while its `impl`
    // blocks still reference it (see core-models' f32.rs).
    #[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
    struct Dummy<T>(T);

    use super::vec::{Vec, from_seq};
    use rust_primitives::sequence::*;

    impl<T> Dummy<T> {
        fn to_vec(s: &[T]) -> Vec<T>
        where
            T: Clone,
        {
            let mut seq = seq_empty();
            seq_extend(&mut seq, s);
            from_seq(seq)
        }
        #[cfg(not(hax_backend_fstar))]
        fn into_vec(s: Box<[T]>) -> Vec<T> {
            from_seq(seq_from_boxed_slice(s))
        }
        #[cfg(hax_backend_fstar)]
        fn into_vec<A>(s: Box<[T]>) -> Vec<T, A> {
            from_seq(seq_from_boxed_slice(s))
        }
        // Mirrors std's `impl<S: Borrow<[T]>, T: Clone> Concat<T> for [S]`.
        #[cfg(not(hax_backend_fstar))]
        fn concat<Item: Clone>(s: &[T]) -> Vec<Item>
        where
            T: core::borrow::Borrow<[Item]>,
        {
            let mut out = seq_empty();
            let mut i = 0;
            while i < rust_primitives::slice::slice_length(s) {
                seq_extend(&mut out, rust_primitives::slice::slice_index(s, i).borrow());
                i += 1;
            }
            from_seq(out)
        }
        // DEVIATION:
        // The F* variant cannot carry the `Borrow` bound: hax erases std's
        // `Concat` bound at call sites, so callers pass `T` and `Item` and no
        // dictionary. Without the bound nothing relates `T` to `[Item]`, so the
        // flattening is not statable and the body is only a placeholder.
        #[cfg(hax_backend_fstar)]
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        fn concat<Item>(s: &[T]) -> Vec<Item> {
            from_seq(seq_empty())
        }
        #[hax_lib::opaque]
        // Insertion sort: charon excludes `sort_by` (see the Makefile), so the
        // body models std for the Rust tests only, over the primitives.
        fn sort_by<F: Fn(&T, &T) -> core::cmp::Ordering>(s: &mut [T], compare: F) {
            use rust_primitives::slice::{slice_index, slice_length, slice_swap};
            let len = slice_length(s);
            let mut i = 1;
            while i < len {
                let mut j = i;
                while j > 0
                    && compare(slice_index(s, j - 1), slice_index(s, j))
                        == core::cmp::Ordering::Greater
                {
                    slice_swap(s, j - 1, j);
                    j -= 1;
                }
                i += 1;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn test_to_vec(v in prop::collection::vec(any::<u8>(), 0..100)) {
                let model = super::Dummy::<u8>::to_vec(&v);
                prop_assert_eq!(model.as_slice(), v.as_slice());
            }

            #[test]
            fn test_into_vec(v in prop::collection::vec(any::<u8>(), 0..100)) {
                let boxed: Box<[u8]> = v.clone().into_boxed_slice();
                let model: crate::vec::Vec<u8> = super::Dummy::<u8>::into_vec(boxed);
                prop_assert_eq!(model.as_slice(), v.as_slice());
            }

            #[test]
            fn test_sort_by(v in prop::collection::vec(any::<u8>(), 0..30)) {
                let mut model = v.clone();
                let mut std_slice = v;
                super::Dummy::<u8>::sort_by(&mut model[..], u8::cmp);
                std_slice.sort_by(u8::cmp);
                prop_assert_eq!(model, std_slice);
            }

            // Reverse order exercises the descending comparator as well.
            #[test]
            fn test_sort_by_reversed(v in prop::collection::vec(any::<u8>(), 0..30)) {
                let cmp = |a: &u8, b: &u8| b.cmp(a);
                let mut model = v.clone();
                let mut std_slice = v;
                super::Dummy::<u8>::sort_by(&mut model[..], cmp);
                std_slice.sort_by(cmp);
                prop_assert_eq!(model, std_slice);
            }
        }

        // The F* `concat` is a deliberate placeholder returning an empty `Vec`.
        #[cfg(hax_backend_fstar)]
        proptest! {
            #[test]
            fn test_concat_placeholder_is_empty(v in prop::collection::vec(any::<u8>(), 0..5)) {
                let slices: std::vec::Vec<&[u8]> = v.iter().map(std::slice::from_ref).collect();
                let model: crate::vec::Vec<u8> = super::Dummy::<&[u8]>::concat(&slices);
                prop_assert!(model.as_slice().is_empty());
            }
        }

        // Only the non-F* `concat` is a real model.
        #[cfg(not(hax_backend_fstar))]
        proptest! {
            #[test]
            fn test_concat(vs in prop::collection::vec(
                prop::collection::vec(any::<u8>(), 0..10), 0..10)) {
                let slices: std::vec::Vec<&[u8]> = vs.iter().map(|v| v.as_slice()).collect();
                let model = super::Dummy::<&[u8]>::concat(&slices);
                let expected = slices.concat();
                prop_assert_eq!(model.as_slice(), expected.as_slice());
            }
        }
    }
}

mod string {
    use rust_primitives::string::*;

    #[cfg_attr(test, derive(PartialEq, Debug))]
    struct String(&'static str);
    impl String {
        fn new() -> Self {
            String("")
        }
        fn push_str(&mut self, other: &'static str) {
            *self = String(str_concat(self.0, other))
        }
        fn push(&mut self, c: char) {
            *self = String(str_concat(self.0, str_of_char(c)))
        }
        fn pop(&mut self) -> Option<char> {
            // Char count, not `str::len`: the primitives below index by char.
            let l = str_len(self.0);
            if l > 0 {
                let c = str_index(self.0, l - 1);
                *self = String(str_sub(self.0, 0, l - 1));
                Some(c)
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::testing::Inject;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn test_push(c in any::<char>()) {
                let mut model = super::String::new();
                let mut std_s = std::string::String::new();
                model.push(c);
                std_s.push(c);
                prop_assert_eq!(model.0, std_s);
            }
        }
        #[test]
        fn test_push_str() {
            let mut model = super::String("hello");
            let mut std_s = "hello".to_string();
            model.push_str("world");
            std_s.push_str("world");
            assert_eq!(model.0, std_s);
        }

        #[test]
        fn test_new() {
            let model = super::String::new();
            assert_eq!(model.0, std::string::String::new());
        }

        proptest! {
            // Arbitrary chars, including multi-byte ones: indexing by char
            // where `str::len` counts bytes is exactly the bug this catches.
            #[test]
            fn test_pop(cs in prop::collection::vec(any::<char>(), 0..8)) {
                let mut model = super::String::new();
                let mut std_s = std::string::String::new();
                for c in &cs {
                    model.push(*c);
                    std_s.push(*c);
                }
                prop_assert_eq!(model.pop(), std_s.pop());
                prop_assert_eq!(model.0, std_s);
            }
        }
    }
}

#[cfg(not(hax_backend_fstar))]
pub mod vec {
    use hax_lib::ToInt;
    use rust_primitives::sequence::*;

    #[cfg_attr(test, derive(Debug))]
    #[hax_lib::fstar::before("open Rust_primitives.Notations")]
    pub struct Vec<T>(pub Seq<T>);

    /// Build a `Vec` from a raw sequence. Used by the `collections` and
    /// `slice` modules so their constructor sites stay identical across the
    /// two `vec` variants (the F* variant threads `A` through here).
    pub(crate) fn from_seq<T>(s: Seq<T>) -> Vec<T> {
        Vec(s)
    }

    impl<T: Clone> Clone for Vec<T> {
        fn clone(&self) -> Self {
            let mut new_vec = seq_empty();
            for it in self.iter() {
                seq_push(&mut new_vec, it.clone());
            }
            Vec(new_vec)
        }
    }
    impl<T, U> PartialEq<Vec<U>> for Vec<T>
    where
        T: PartialEq<U>,
    {
        #[cfg(not(hax_backend_fstar))]
        fn ne(&self, other: &Vec<U>) -> bool {
            self.eq(other) == false
        }
        fn eq(&self, other: &Vec<U>) -> bool {
            if !(self.len() == other.len()) {
                false
            } else {
                let mut res = true;
                for i in 0..self.len() {
                    // `res &&` keeps this short-circuiting like std's early
                    // return: once unequal, `T::eq` is not called again (it may
                    // panic). Aeneas has no early return, hence the flag.
                    if res && !(self[i] == other[i]) {
                        res = false
                    }
                }
                res
            }
        }
    }

    /// Opaque model of `std::vec::IntoIter<T, A>`. Downstream Aeneas
    /// extractions reference this type via its full path
    /// `alloc::vec::into_iter::IntoIter<T, A>`, so we expose it under a
    /// matching submodule.
    pub mod into_iter {
        use rust_primitives::sequence::*;
        pub struct IntoIter<T>(pub Seq<T>);
        impl<T> Iterator for IntoIter<T> {
            type Item = T;
            fn next(&mut self) -> Option<Self::Item> {
                if seq_len(&self.0) == 0 {
                    None
                } else {
                    Some(seq_remove(&mut self.0, 0))
                }
            }
        }
    }

    impl<T> IntoIterator for Vec<T> {
        type Item = T;
        type IntoIter = into_iter::IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter {
            into_iter::IntoIter(self.0)
        }
    }

    fn from_elem<T: Clone>(item: T, len: usize) -> Vec<T> {
        Vec(seq_create(item, len))
    }

    #[hax_lib::attributes]
    impl<T> Vec<T> {
        pub fn new() -> Vec<T> {
            Vec(seq_empty())
        }
        pub fn with_capacity(_c: usize) -> Vec<T> {
            Vec::new()
        }
    }

    /// See [`std::vec::Vec::default`]: an empty `Vec`.
    impl<T> Default for Vec<T> {
        fn default() -> Vec<T> {
            Vec::new()
        }
    }

    #[hax_lib::attributes]
    impl<T> Vec<T> {
        pub fn len(&self) -> usize {
            seq_len(&self.0)
        }
        #[hax_lib::requires(seq_len(&self.0) < usize::MAX)]
        pub fn push(&mut self, x: T) {
            seq_push(&mut self.0, x)
        }
        pub fn pop(&mut self) -> Option<T> {
            let l = seq_len(&self.0);
            if l > 0 {
                let last = seq_remove(&mut self.0, l - 1);
                Some(last)
            } else {
                None
            }
        }
        pub fn is_empty(&self) -> bool {
            seq_len(&self.0) == 0
        }
        #[hax_lib::requires(index <= seq_len(&self.0) && seq_len(&self.0) < usize::MAX)]
        pub fn insert(&mut self, index: usize, element: T) {
            let l = seq_len(&self.0);
            let mut right = seq_drain(&mut self.0, index, l);
            seq_push(&mut self.0, element);
            seq_concat(&mut self.0, &mut right);
        }
        pub fn as_slice(&self) -> &[T] {
            seq_to_slice(&self.0)
        }
        // Returns `&mut [T]`, which the F* backend cannot model (HAX0003/HAX0010,
        // pereq-dev/hax#420). Only the `cfg(not(hax_backend_fstar))` `Vec` block gets
        // it, and only `DerefMut for Vec` (same gating) uses it.
        pub fn as_mut_slice(&mut self) -> &mut [T] {
            seq_to_slice_mut(&mut self.0)
        }
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        pub fn truncate(&mut self, n: usize) {
            let l = seq_len(&self.0);
            if n < l {
                seq_drain(&mut self.0, n, l);
            }
        }
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        #[hax_lib::requires(n < self.len())]
        pub fn swap_remove(&mut self, n: usize) -> T {
            let l = seq_len(&self.0);
            let last = seq_remove(&mut self.0, l - 1);
            if n == l - 1 {
                last
            } else {
                let removed = seq_remove(&mut self.0, n);
                self.insert(n, last);
                removed
            }
        }
        /// `remove` drops one element, so it never grows the vector.
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        #[hax_lib::requires(index < self.len())]
        #[hax_lib::ensures(|_| future(self).len().to_int() <= self.len().to_int())]
        pub fn remove(&mut self, index: usize) -> T {
            seq_remove(&mut self.0, index)
        }
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        pub fn clear(&mut self) {
            self.0 = seq_empty()
        }
        #[hax_lib::requires(self.len().to_int() + other.len().to_int() <= usize::MAX.to_int())]
        pub fn append(&mut self, other: &mut Vec<T>) {
            seq_concat(&mut self.0, &mut other.0);
            other.0 = seq_empty()
        }
        /// See [`std::vec::Vec::split_off`]: truncate `self` to `[0, at)` and
        /// return the tail `[at, len)` as a new `Vec`.
        #[hax_lib::requires(at <= self.len())]
        pub fn split_off(&mut self, at: usize) -> Vec<T> {
            let l = seq_len(&self.0);
            Vec(seq_drain(&mut self.0, at, l))
        }
        // Generic in `A` like std's `Vec<T, A>::drain`, for Lean callers. This
        // `Vec` has no allocator to infer it from, so Rust callers must name it.
        #[hax_lib::requires(core::slice::try_range(range, ..self.len()).is_some())]
        pub fn drain<A, R: core::ops::RangeBounds<usize>>(
            &mut self,
            range: R,
        ) -> drain::Drain<T, A> {
            let r = core::slice::range(range, ..self.len());
            drain::Drain(
                seq_drain(&mut self.0, r.start, r.end),
                std::marker::PhantomData,
            )
        }
    }
    pub mod drain {
        use rust_primitives::sequence::*;
        pub struct Drain<T, A>(pub Seq<T>, pub std::marker::PhantomData<A>);
        impl<T, A> Iterator for Drain<T, A> {
            type Item = T;
            fn next(&mut self) -> Option<Self::Item> {
                if seq_len(&self.0) == 0 {
                    Option::None
                } else {
                    let res = seq_remove(&mut self.0, 0);
                    Option::Some(res)
                }
            }
        }
    }

    // `resize` and `extend_from_slice` both require `T: Clone`, so real `alloc`
    // keeps them in the same `impl` block; keep them together here too so the
    // generated `impl_N__` prefix (`impl_2__resize`) matches what hax derives
    // from real `alloc`.
    #[hax_lib::attributes]
    impl<T: Clone> Vec<T> {
        #[hax_lib::requires(seq_len(&self.0).to_int() + other.len().to_int() <= usize::MAX.to_int())]
        fn extend_from_slice(&mut self, other: &[T]) {
            seq_extend(&mut self.0, other)
        }
        // Like std's `extend_with`: `value` is cloned into all but the last new
        // slot, which takes `value` itself.
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        #[hax_lib::ensures(|_| future(self).len() == new_size)]
        pub fn resize(&mut self, new_size: usize, value: T) {
            let l = seq_len(&self.0);
            if new_size > l {
                let mut extra = seq_create(value, new_size - l);
                seq_concat(&mut self.0, &mut extra);
            } else {
                seq_drain(&mut self.0, new_size, l);
            }
        }
    }

    /// Generic `Index<I>` impl for `Vec`, matching std's
    /// `impl<T, I: SliceIndex<[T]>, A: Allocator> Index<I> for Vec<T, A>`
    /// (in `alloc/src/vec/mod.rs`). Delegates through `Deref` to the
    /// `<[T]>::index` impl, the same body std uses. We omit the
    /// `A: Allocator` bound because we do not model `Allocator` as a
    /// trait — functionally identical for our purposes. The trait bound
    /// references `std::slice::SliceIndex` (the real one) rather than
    /// `core_models::slice::index::SliceIndex` because this crate is
    /// standalone and does not depend on `core_models`; Aeneas's name
    /// map for `std::slice::SliceIndex` aligns the extracted Lean path
    /// with `core_models`'s SliceIndex extraction (both extract under
    /// `core.slice.index.SliceIndex`).
    #[hax_lib::attributes]
    impl<T, I> std::ops::Index<I> for Vec<T>
    where
        I: std::slice::SliceIndex<[T]>,
    {
        type Output = I::Output;
        #[hax_lib::requires(self.get(i).is_some())]
        fn index(&self, i: I) -> &I::Output {
            std::ops::Index::index(&**self, i)
        }
    }

    /// Generic `IndexMut<I>` for `Vec`, mirroring the `Index<I>` impl above and
    /// std's `impl<T, I: SliceIndex<[T]>, A: Allocator> IndexMut<I> for Vec<T, A>`,
    /// which routes through the mutable slice. Lean-only, like the slice
    /// `IndexMut` it delegates to. This is what `v[i] = x` extracts against.
    #[hax_lib::attributes]
    impl<T, I> std::ops::IndexMut<I> for Vec<T>
    where
        I: std::slice::SliceIndex<[T]>,
    {
        // Kept out of the Lean lane, as for the slice `IndexMut` it delegates to.
        #[cfg_attr(not(hax_backend_lean), hax_lib::requires(self.get(i).is_some()))]
        fn index_mut(&mut self, i: I) -> &mut I::Output {
            std::ops::IndexMut::index_mut(seq_to_slice_mut(&mut self.0), i)
        }
    }

    #[hax_lib::attributes]
    impl<T> core::ops::Deref for Vec<T> {
        type Target = [T];

        fn deref(&self) -> &[T] {
            self.as_slice()
        }
    }

    // Excluded from the F* backend to match `core::ops::DerefMut`, which is not
    // extracted there (HAX0003, pereq-dev/hax#420); aeneas/lean/native keep it.
    #[cfg(not(hax_backend_fstar))]
    #[hax_lib::attributes]
    impl<T> core::ops::DerefMut for Vec<T> {
        fn deref_mut(&mut self) -> &mut [T] {
            self.as_mut_slice()
        }
    }

    /// See [`std::convert::From`] `<[T; N]>` for `Vec<T>`
    impl<T, const N: usize> From<[T; N]> for Vec<T> {
        fn from(a: [T; N]) -> Vec<T> {
            Vec(seq_from_array(a))
        }
    }

    #[hax_lib::attributes]
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    impl<T> std::iter::FromIterator<T> for Vec<T> {
        fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = T>,
        {
            let mut res = Vec::new();
            for el in iter {
                res.push(el)
            }
            res
        }
    }

    #[cfg(test)]
    mod tests;
}

// The F* backend keeps the explicit allocator type parameter `A` on `Vec`
#[cfg(hax_backend_fstar)]
pub mod vec {
    use crate::alloc::Global;
    use hax_lib::ToInt;
    use rust_primitives::sequence::*;
    use std::marker::PhantomData;

    // Unlike the default variant, this `Vec` models no `PartialEq`; the
    // derive is test-only (invisible to extraction) so that the shared test
    // suite in `vec_tests.rs` can compare models with `assert_eq!`.
    #[cfg_attr(test, derive(Debug, PartialEq))]
    #[hax_lib::fstar::before("open Rust_primitives.Notations")]
    pub struct Vec<T, A = Global>(pub Seq<T>, pub PhantomData<A>);

    /// See the `from_seq` in the non-F* `vec` module: same role, but threads
    /// the allocator parameter so external constructor sites are identical.
    pub(crate) fn from_seq<T, A>(s: Seq<T>) -> Vec<T, A> {
        Vec(s, PhantomData)
    }

    fn from_elem<T: Clone>(item: T, len: usize) -> Vec<T, Global> {
        Vec(seq_create(item, len), PhantomData)
    }

    #[hax_lib::attributes]
    impl<T> Vec<T, Global> {
        pub fn new() -> Vec<T, Global> {
            Vec(seq_empty(), PhantomData)
        }
        pub fn with_capacity(_c: usize) -> Vec<T, Global> {
            Vec::new()
        }
    }

    #[hax_lib::attributes]
    impl<T, A> Vec<T, A> {
        pub fn len(&self) -> usize {
            seq_len(&self.0)
        }
        #[hax_lib::requires(seq_len(&self.0) < usize::MAX)]
        pub fn push(&mut self, x: T) {
            seq_push(&mut self.0, x)
        }
        pub fn pop(&mut self) -> Option<T> {
            let l = seq_len(&self.0);
            if l > 0 {
                let last = seq_remove(&mut self.0, l - 1);
                Some(last)
            } else {
                None
            }
        }
        pub fn is_empty(&self) -> bool {
            seq_len(&self.0) == 0
        }
        #[hax_lib::requires(index <= seq_len(&self.0) && seq_len(&self.0) < usize::MAX)]
        pub fn insert(&mut self, index: usize, element: T) {
            let l = seq_len(&self.0);
            let mut right = seq_drain(&mut self.0, index, l);
            seq_push(&mut self.0, element);
            seq_concat(&mut self.0, &mut right);
        }
        pub fn as_slice(&self) -> &[T] {
            seq_to_slice(&self.0)
        }
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        pub fn truncate(&mut self, n: usize) {
            let l = seq_len(&self.0);
            if n < l {
                seq_drain(&mut self.0, n, l);
            }
        }
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        #[hax_lib::requires(n < self.len())]
        pub fn swap_remove(&mut self, n: usize) -> T {
            let l = seq_len(&self.0);
            let last = seq_remove(&mut self.0, l - 1);
            if n == l - 1 {
                last
            } else {
                let removed = seq_remove(&mut self.0, n);
                self.insert(n, last);
                removed
            }
        }
        /// `remove` drops one element, so it never grows the vector.
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        #[hax_lib::requires(index < self.len())]
        #[hax_lib::ensures(|_| future(self).len().to_int() <= self.len().to_int())]
        pub fn remove(&mut self, index: usize) -> T {
            seq_remove(&mut self.0, index)
        }
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        pub fn clear(&mut self) {
            self.0 = seq_empty()
        }
        #[hax_lib::requires(self.len().to_int() + other.len().to_int() <= usize::MAX.to_int())]
        pub fn append(&mut self, other: &mut Vec<T, A>) {
            seq_concat(&mut self.0, &mut other.0);
            other.0 = seq_empty()
        }
        /// See [`std::vec::Vec::split_off`]: truncate `self` to `[0, at)` and
        /// return the tail `[at, len)` as a new `Vec`.
        #[hax_lib::requires(at <= self.len())]
        pub fn split_off(&mut self, at: usize) -> Vec<T, A> {
            let l = seq_len(&self.0);
            Vec(seq_drain(&mut self.0, at, l), PhantomData)
        }
        #[hax_lib::requires(core::slice::try_range(range, ..self.len()).is_some())]
        pub fn drain<R: core::ops::RangeBounds<usize>>(&mut self, range: R) -> drain::Drain<T, A> {
            let r = core::slice::range(range, ..self.len());
            drain::Drain(seq_drain(&mut self.0, r.start, r.end), PhantomData)
        }
    }

    pub mod into_iter {
        use rust_primitives::sequence::*;
        pub struct IntoIter<T>(pub Seq<T>);
        impl<T> Iterator for IntoIter<T> {
            type Item = T;
            fn next(&mut self) -> Option<Self::Item> {
                if seq_len(&self.0) == 0 {
                    None
                } else {
                    Some(seq_remove(&mut self.0, 0))
                }
            }
        }
    }
    pub mod drain {
        use rust_primitives::sequence::*;
        pub struct Drain<T, A>(pub Seq<T>, pub std::marker::PhantomData<A>);
        impl<T, A> Iterator for Drain<T, A> {
            type Item = T;
            fn next(&mut self) -> Option<Self::Item> {
                if seq_len(&self.0) == 0 {
                    Option::None
                } else {
                    let res = seq_remove(&mut self.0, 0);
                    Option::Some(res)
                }
            }
        }
    }

    // `resize` and `extend_from_slice` both require `T: Clone`, so real `alloc`
    // keeps them in the same `impl` block; keep them together here too so the
    // generated `impl_N__` prefix (`impl_2__resize`) matches what hax derives
    // from real `alloc`.
    #[hax_lib::attributes]
    impl<T: Clone, A> Vec<T, A> {
        #[hax_lib::requires(seq_len(&self.0).to_int() + other.len().to_int() <= usize::MAX.to_int())]
        fn extend_from_slice(&mut self, other: &[T]) {
            seq_extend(&mut self.0, other)
        }
        // Like std's `extend_with`: `value` is cloned into all but the last new
        // slot, which takes `value` itself.
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        #[hax_lib::ensures(|_| future(self).len() == new_size)]
        pub fn resize(&mut self, new_size: usize, value: T) {
            let l = seq_len(&self.0);
            if new_size > l {
                let mut extra = seq_create(value, new_size - l);
                seq_concat(&mut self.0, &mut extra);
            } else {
                seq_drain(&mut self.0, new_size, l);
            }
        }
    }

    /// Generic `Index<I>` impl, mirroring std's
    /// `impl<T, I: SliceIndex<[T]>, A: Allocator> Index<I> for Vec<T, A>`.
    #[hax_lib::attributes]
    impl<T, I, A> std::ops::Index<I> for Vec<T, A>
    where
        I: std::slice::SliceIndex<[T]>,
    {
        type Output = I::Output;
        #[hax_lib::requires(self.get(i).is_some())]
        fn index(&self, i: I) -> &I::Output {
            std::ops::Index::index(&**self, i)
        }
    }

    #[hax_lib::attributes]
    impl<T, A> core::ops::Deref for Vec<T, A> {
        type Target = [T];

        fn deref(&self) -> &[T] {
            self.as_slice()
        }
    }

    #[hax_lib::attributes]
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    impl<T> std::iter::FromIterator<T> for Vec<T, Global> {
        fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = T>,
        {
            let mut res = Vec::new();
            for el in iter {
                res.push(el)
            }
            res
        }
    }

    #[cfg(test)]
    mod tests;
}
