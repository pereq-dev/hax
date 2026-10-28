// This model of iterators doesn't respect the signatures of the original definitions in Rust core.
// We avoid default implementations for trait methods, and instead provide them as external to the trait.
// This means overriding them is not possible.
// We also avoid the coinductivity between `IntoIter` and `Iterator`.

pub mod traits {
    pub mod iterator {
        use super::super::adapters::{
            chain::Chain, enumerate::Enumerate, filter::Filter, filter_map::FilterMap,
            flat_map::FlatMap, flatten::Flatten, fuse::Fuse, inspect::Inspect, map::Map,
            map_while::MapWhile, skip::Skip, skip_while::SkipWhile, step_by::StepBy, take::Take,
            take_while::TakeWhile, zip::Zip,
        };
        use crate::option::Option;
        /// See [`std::iter::Iterator`]
        #[hax_lib::attributes]
        pub trait Iterator {
            type Item;
            #[hax_lib::requires(true)]
            fn next(&mut self) -> Option<Self::Item>;
        }

        // Holds the `Iterator` provided methods, which the F* backend cannot take as
        // trait defaults. Nothing here reaches Lean: the blanket impl below is
        // aeneas-excluded, and each method is instead a standalone
        // `@[rust_fun]`-tagged function in `FunsEpilogue.lean` (the shape aeneas
        // itself uses for `rev`/`collect`). Keeping them off `Iterator` also keeps
        // it clear of the `IntoIterator`/`DoubleEndedIterator` cycles.
        #[hax_lib::attributes]
        pub(crate) trait IteratorMethods: Iterator {
            fn fold<B, F: FnMut(B, Self::Item) -> B>(self, init: B, f: F) -> B;
            fn all<F: FnMut(Self::Item) -> bool>(self, f: F) -> bool;
            fn map<O, F: FnMut(Self::Item) -> O>(self, f: F) -> Map<Self, F>
            where
                Self: Sized;
            fn enumerate(self) -> Enumerate<Self>
            where
                Self: Sized;
            #[hax_lib::requires(step > 0)]
            fn step_by(self, step: usize) -> StepBy<Self>
            where
                Self: Sized;
            fn take(self, n: usize) -> Take<Self>
            where
                Self: Sized;
            fn skip(self, n: usize) -> Skip<Self>
            where
                Self: Sized;
            fn filter<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> Filter<Self, P>
            where
                Self: Sized;
            fn filter_map<B, F: FnMut(Self::Item) -> Option<B>>(self, f: F) -> FilterMap<Self, F>
            where
                Self: Sized;
            fn take_while<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> TakeWhile<Self, P>
            where
                Self: Sized;
            fn skip_while<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> SkipWhile<Self, P>
            where
                Self: Sized;
            fn map_while<B, F: FnMut(Self::Item) -> Option<B>>(self, f: F) -> MapWhile<Self, F>
            where
                Self: Sized;
            fn inspect<F: FnMut(&Self::Item)>(self, f: F) -> Inspect<Self, F>
            where
                Self: Sized;
            fn fuse(self) -> Fuse<Self>
            where
                Self: Sized;
            fn flat_map<U: Iterator, F: FnMut(Self::Item) -> U>(self, f: F) -> FlatMap<Self, U, F>
            where
                Self: Sized;
            fn flatten(self) -> Flatten<Self>
            where
                Self::Item: Iterator,
                Self: Sized;
            fn zip<I2: Iterator>(self, it2: I2) -> Zip<Self, I2>
            where
                Self: Sized;
            fn chain<U: Iterator<Item = Self::Item>>(self, other: U) -> Chain<Self, U>
            where
                Self: Sized;
            fn any<F: FnMut(Self::Item) -> bool>(self, f: F) -> bool;
            fn find<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> Option<Self::Item>;
            fn find_map<B, F: FnMut(Self::Item) -> Option<B>>(self, f: F) -> Option<B>;
            fn position<P: FnMut(Self::Item) -> bool>(self, predicate: P) -> Option<usize>;
            fn count(self) -> usize;
            fn nth(self, n: usize) -> Option<Self::Item>;
            fn last(self) -> Option<Self::Item>;
            fn for_each<F: FnMut(Self::Item)>(self, f: F);
            fn reduce<F: FnMut(Self::Item, Self::Item) -> Self::Item>(
                self,
                f: F,
            ) -> Option<Self::Item>;
            fn min(self) -> Option<Self::Item>
            where
                Self::Item: crate::cmp::Ord;
            fn max(self) -> Option<Self::Item>
            where
                Self::Item: crate::cmp::Ord;
            fn collect<B: super::super::traits::collect::FromIterator<Self::Item>>(self) -> B
            where
                Self: Sized;
        }

        // Opaque helper functions for loop-based iterator methods.
        // #[hax_lib::opaque] only works at the function/impl-block level, not on individual
        // methods within an impl block, so we use standalone functions and delegate.

        // opaque: while-let loop is not supported by hax FunctionalizeLoops
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        fn iter_fold<I: Iterator, B, F: FnMut(B, I::Item) -> B>(
            mut iter: I,
            init: B,
            mut f: F,
        ) -> B {
            let mut accum = init;
            while let Option::Some(x) = iter.next() {
                accum = f(accum, x);
            }
            accum
        }

        // opaque: while-let loop is not supported by hax FunctionalizeLoops
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        fn iter_all<I: Iterator, F: FnMut(I::Item) -> bool>(iter: &mut I, mut f: F) -> bool {
            while let Option::Some(x) = iter.next() {
                if !f(x) {
                    return false;
                }
            }
            true
        }

        // opaque: while-let loop is not supported by hax FunctionalizeLoops
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        fn iter_any<I: Iterator, F: FnMut(I::Item) -> bool>(iter: &mut I, mut f: F) -> bool {
            while let Option::Some(x) = iter.next() {
                if f(x) {
                    return true;
                }
            }
            false
        }

        // opaque: while-let loop is not supported by hax FunctionalizeLoops
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        fn iter_find<I: Iterator, P: FnMut(&I::Item) -> bool>(
            iter: &mut I,
            mut predicate: P,
        ) -> Option<I::Item> {
            while let Option::Some(x) = iter.next() {
                if predicate(&x) {
                    return Option::Some(x);
                }
            }
            Option::None
        }

        // opaque: while-let loop is not supported by hax FunctionalizeLoops
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        fn iter_find_map<I: Iterator, B, F: FnMut(I::Item) -> Option<B>>(
            iter: &mut I,
            mut f: F,
        ) -> Option<B> {
            while let Option::Some(x) = iter.next() {
                if let Option::Some(v) = f(x) {
                    return Option::Some(v);
                }
            }
            Option::None
        }

        // opaque: while-let loop is not supported by hax FunctionalizeLoops
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        fn iter_position<I: Iterator, P: FnMut(I::Item) -> bool>(
            iter: &mut I,
            mut predicate: P,
        ) -> Option<usize> {
            let mut i: usize = 0;
            while let Option::Some(x) = iter.next() {
                if predicate(x) {
                    return Option::Some(i);
                }
                i += 1;
            }
            Option::None
        }

        // opaque: while-let loop is not supported by hax FunctionalizeLoops
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        fn iter_count<I: Iterator>(mut iter: I) -> usize {
            let mut n: usize = 0;
            while let Option::Some(_) = iter.next() {
                n += 1;
            }
            n
        }

        // opaque: for-loop generates Rust_primitives.Hax.Folds, causing F* dependency cycle
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        #[cfg_attr(hax_backend_lean, hax_lib::exclude)] // forward reference in lean (`core.Usize.Insts.CoreIterRangeStep`)
        fn iter_nth<I: Iterator>(mut iter: I, n: usize) -> Option<I::Item> {
            for _ in 0..n {
                if let Option::None = iter.next() {
                    return Option::None;
                }
            }
            iter.next()
        }

        // opaque: while-let loop is not supported by hax FunctionalizeLoops
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        fn iter_last<I: Iterator>(mut iter: I) -> Option<I::Item> {
            let mut last = Option::None;
            while let Option::Some(x) = iter.next() {
                last = Option::Some(x);
            }
            last
        }

        // opaque: while-let loop is not supported by hax FunctionalizeLoops
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        fn iter_for_each<I: Iterator, F: FnMut(I::Item)>(mut iter: I, mut f: F) {
            while let Option::Some(x) = iter.next() {
                f(x);
            }
        }

        // opaque: while-let loop is not supported by hax FunctionalizeLoops
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        fn iter_reduce<I: Iterator, F: FnMut(I::Item, I::Item) -> I::Item>(
            mut iter: I,
            mut f: F,
        ) -> Option<I::Item> {
            let mut accum = match iter.next() {
                Option::Some(x) => x,
                Option::None => return Option::None,
            };
            while let Option::Some(x) = iter.next() {
                accum = f(accum, x);
            }
            Option::Some(accum)
        }

        // opaque: while-let loop is not supported by hax FunctionalizeLoops
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        fn iter_min<I: Iterator>(mut iter: I) -> Option<I::Item>
        where
            I::Item: crate::cmp::Ord,
        {
            let mut min = match iter.next() {
                Option::Some(x) => x,
                Option::None => return Option::None,
            };
            while let Option::Some(x) = iter.next() {
                if let crate::cmp::Ordering::Less = crate::cmp::Ord::cmp(&x, &min) {
                    min = x;
                }
            }
            Option::Some(min)
        }

        // opaque: while-let loop is not supported by hax FunctionalizeLoops
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        fn iter_max<I: Iterator>(mut iter: I) -> Option<I::Item>
        where
            I::Item: crate::cmp::Ord,
        {
            let mut max = match iter.next() {
                Option::Some(x) => x,
                Option::None => return Option::None,
            };
            while let Option::Some(x) = iter.next() {
                if let crate::cmp::Ordering::Greater | crate::cmp::Ordering::Equal =
                    crate::cmp::Ord::cmp(&x, &max)
                {
                    max = x;
                }
            }
            Option::Some(max)
        }

        #[hax_lib::attributes]
        #[cfg_attr(hax_backend_lean, hax_lib::exclude)]
        impl<I: Iterator> IteratorMethods for I {
            fn fold<B, F: FnMut(B, I::Item) -> B>(self, init: B, f: F) -> B {
                iter_fold(self, init, f)
            }

            fn all<F: FnMut(I::Item) -> bool>(mut self, f: F) -> bool {
                iter_all(&mut self, f)
            }

            fn map<O, F: FnMut(I::Item) -> O>(self, f: F) -> Map<I, F> {
                Map::new(self, f)
            }

            fn enumerate(self) -> Enumerate<I> {
                Enumerate::new(self)
            }

            #[hax_lib::requires(step > 0)]
            fn step_by(self, step: usize) -> StepBy<I> {
                StepBy::new(self, step)
            }

            fn take(self, n: usize) -> Take<I> {
                Take::new(self, n)
            }

            fn skip(self, n: usize) -> Skip<I> {
                Skip::new(self, n)
            }

            fn filter<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> Filter<Self, P> {
                Filter::new(self, predicate)
            }

            fn filter_map<B, F: FnMut(Self::Item) -> Option<B>>(self, f: F) -> FilterMap<Self, F> {
                FilterMap::new(self, f)
            }

            fn take_while<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> TakeWhile<Self, P> {
                TakeWhile::new(self, predicate)
            }

            fn skip_while<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> SkipWhile<Self, P> {
                SkipWhile::new(self, predicate)
            }

            fn map_while<B, F: FnMut(Self::Item) -> Option<B>>(self, f: F) -> MapWhile<Self, F> {
                MapWhile::new(self, f)
            }

            fn inspect<F: FnMut(&Self::Item)>(self, f: F) -> Inspect<Self, F> {
                Inspect::new(self, f)
            }

            fn fuse(self) -> Fuse<Self> {
                Fuse::new(self)
            }

            fn flat_map<U: Iterator, F: FnMut(I::Item) -> U>(self, f: F) -> FlatMap<I, U, F> {
                FlatMap::new(self, f)
            }

            fn flatten(self) -> Flatten<I>
            where
                I::Item: Iterator,
            {
                Flatten::new(self)
            }

            fn zip<I2: Iterator>(self, it2: I2) -> Zip<Self, I2> {
                Zip::new(self, it2)
            }

            fn chain<U: Iterator<Item = Self::Item>>(self, other: U) -> Chain<Self, U> {
                Chain::new(self, other)
            }

            fn any<F: FnMut(Self::Item) -> bool>(mut self, f: F) -> bool {
                iter_any(&mut self, f)
            }

            fn find<P: FnMut(&Self::Item) -> bool>(mut self, predicate: P) -> Option<Self::Item> {
                iter_find(&mut self, predicate)
            }

            fn find_map<B, F: FnMut(Self::Item) -> Option<B>>(mut self, f: F) -> Option<B> {
                iter_find_map(&mut self, f)
            }

            fn position<P: FnMut(Self::Item) -> bool>(mut self, predicate: P) -> Option<usize> {
                iter_position(&mut self, predicate)
            }

            fn count(self) -> usize {
                iter_count(self)
            }

            fn nth(self, n: usize) -> Option<Self::Item> {
                iter_nth(self, n)
            }

            fn last(self) -> Option<Self::Item> {
                iter_last(self)
            }

            fn for_each<F: FnMut(Self::Item)>(self, f: F) {
                iter_for_each(self, f)
            }

            fn reduce<F: FnMut(Self::Item, Self::Item) -> Self::Item>(
                self,
                f: F,
            ) -> Option<Self::Item> {
                iter_reduce(self, f)
            }

            fn min(self) -> Option<Self::Item>
            where
                Self::Item: crate::cmp::Ord,
            {
                iter_min(self)
            }

            fn max(self) -> Option<Self::Item>
            where
                Self::Item: crate::cmp::Ord,
            {
                iter_max(self)
            }

            fn collect<B: super::super::traits::collect::FromIterator<Self::Item>>(self) -> B {
                super::super::traits::collect::FromIterator::from_iter(self)
            }
        }

        #[hax_lib::attributes]
        impl<I: Iterator> super::collect::IntoIterator for I {
            type Item = I::Item;
            type IntoIter = Self;
            fn into_iter(self) -> Self {
                self
            }
        }
    }
    pub mod double_ended {
        use super::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::DoubleEndedIterator`]
        ///
        /// Modelled only for the Lean/charon backend (the `rev` provided method that
        /// drives it is `#[cfg(not(hax_backend_fstar))]`); F* keeps the original
        /// `IteratorMethods` workaround, which never needs `rev`.
        #[cfg(not(hax_backend_fstar))]
        #[hax_lib::attributes]
        pub trait DoubleEndedIterator: Iterator {
            #[hax_lib::requires(true)]
            fn next_back(&mut self) -> Option<Self::Item>;
        }
    }
    pub mod exact_size {
        use super::iterator::Iterator;
        /// See [`std::iter::ExactSizeIterator`]
        ///
        /// `len` is modelled as a required method (real Rust makes it a provided method
        /// over `size_hint`). Used by `Enumerate::next_back` to recover the yielded
        /// element's original index. Lean/charon backend only (see `DoubleEndedIterator`).
        #[cfg(not(hax_backend_fstar))]
        #[hax_lib::attributes]
        pub trait ExactSizeIterator: Iterator {
            #[hax_lib::requires(true)]
            fn len(&self) -> usize;
        }
    }
    pub mod collect {
        /// See [`std::iter::IntoIterator`]
        pub trait IntoIterator {
            // The bound carries the `Iterator` super-instance that
            // `FromIterator::from_iter` needs to fold with, as in Aeneas.Std.
            // Keeping the provided methods standalone keeps it out of the
            // recursive field resolution `impl_def` chokes on.
            type Item;
            type IntoIter: super::iterator::Iterator<Item = Self::Item>;
            fn into_iter(self) -> Self::IntoIter;
        }
        /// See [`std::iter::FromIterator`]
        #[hax_lib::attributes]
        pub trait FromIterator<A>: Sized {
            // `Item = A` pins the iterated element to the collection's element
            // type (as in std and Aeneas.Std). Without it a real `from_iter`
            // fold yields a free `List Clause0_Item` that can't be built into a
            // `Vec<A>`, forcing an opaque stub. It requires the `Result`
            // `FromIterator` impl (result.rs) to be phrased so it still compiles.
            #[hax_lib::requires(true)]
            fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
        }
    }
}

pub mod adapters {
    pub mod enumerate {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::Enumerate`]
        pub struct Enumerate<I> {
            iter: I,
            count: usize,
        }
        #[hax_lib::attributes]
        impl<I> Enumerate<I> {
            pub fn new(iter: I) -> Enumerate<I> {
                Enumerate { iter, count: 0 }
            }
        }
        #[hax_lib::attributes]
        impl<I: Iterator> Iterator for Enumerate<I> {
            type Item = (usize, <I as Iterator>::Item);

            fn next(&mut self) -> Option<(usize, <I as Iterator>::Item)> {
                match self.iter.next() {
                    Option::Some(a) => {
                        let i = self.count;
                        // TODO check what to do here. It would be bad to have an iterator with
                        // more than usize::MAX elements, this could be a requirement (but hard to formulate).
                        // F* only: the Lean library has no `hax_lib::assume` model.
                        #[cfg(hax_backend_fstar)]
                        hax_lib::assume!(self.count < crate::num::usize::MAX);
                        self.count += 1;
                        Option::Some((i, a))
                    }
                    Option::None => Option::None,
                }
            }
        }
        // After the inner `next_back`, `I::len()` is the count still remaining,
        // which is that element's index relative to `count`. Hence the
        // double-ended AND exact-size bounds.
        //
        // Both bound orders are wrong, differently: this one extracts but hands
        // `.enumerate().rev()` its dictionaries in the wrong positions (std
        // declares them the other way), while std's order makes `Enumerate<I>`
        // reach `Iterator` by two parent paths and aeneas rejects it. See
        // `iter_rev_enumerate` in `tests/client_test/src/lib.rs`.
        #[cfg(not(hax_backend_fstar))]
        impl<
            I: crate::iter::traits::double_ended::DoubleEndedIterator
                + crate::iter::traits::exact_size::ExactSizeIterator,
        > crate::iter::traits::double_ended::DoubleEndedIterator for Enumerate<I>
        {
            fn next_back(&mut self) -> Option<(usize, <I as Iterator>::Item)> {
                match self.iter.next_back() {
                    Option::Some(a) => {
                        let len = self.iter.len();
                        // No `assume!`: Lean has no model for it, and this impl is
                        // Lean-only. The extracted add is checked, so it panics on
                        // overflow rather than wrapping.
                        Option::Some((self.count + len, a))
                    }
                    Option::None => Option::None,
                }
            }
        }
    }
    pub mod rev {
        // The trait is Lean/charon-only (see `double_ended`), so its import must be
        // gated to match — otherwise F* extraction hits an unresolved import (E0432).
        #[cfg(not(hax_backend_fstar))]
        use super::super::traits::double_ended::DoubleEndedIterator;
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::Rev`]
        pub struct Rev<I> {
            iter: I,
        }
        // Gated like the `Iterator` impl below and `rev` itself: `Rev` is
        // unreachable on the F* lane, so an ungated constructor is dead code there.
        #[cfg(not(hax_backend_fstar))]
        impl<I> Rev<I> {
            pub fn new(iter: I) -> Rev<I> {
                Rev { iter }
            }
        }
        // `next` delegates to the inner `next_back`, as in Aeneas.Std.
        #[cfg(not(hax_backend_fstar))]
        impl<I: DoubleEndedIterator> Iterator for Rev<I> {
            type Item = <I as Iterator>::Item;
            fn next(&mut self) -> Option<<I as Iterator>::Item> {
                self.iter.next_back()
            }
        }
    }
    pub mod step_by {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::StepBy`]
        pub struct StepBy<I> {
            iter: I,
            step: usize,
        }

        #[hax_lib::attributes]
        impl<I> StepBy<I> {
            // std panics in `Iterator::step_by`, which is this constructor's only caller.
            #[hax_lib::requires(step > 0)]
            pub fn new(iter: I, step: usize) -> Self {
                if step == 0 {
                    crate::panicking::internal::panic()
                }
                StepBy { iter, step }
            }
        }

        #[hax_lib::attributes]
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        impl<I: Iterator> Iterator for StepBy<I> {
            type Item = <I as Iterator>::Item;

            // Yields indices 0, step, 2*step, …, so the first call must not skip.
            fn next(&mut self) -> Option<<I as Iterator>::Item> {
                let current = self.iter.next();
                // No early exit: Aeneas can't translate `break`, and `next` on an
                // exhausted iterator is a no-op.
                for _ in 1..self.step {
                    self.iter.next();
                }
                current
            }
        }
    }
    pub mod map {
        /// See [`std::iter::Map`]
        pub struct Map<I, F> {
            iter: I,
            f: F,
        }

        #[hax_lib::attributes]
        impl<I, F> Map<I, F> {
            pub fn new(iter: I, f: F) -> Self {
                Self { iter, f }
            }
        }
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        // `FnMut`, as in std: a downstream `.map(closure)` yields an `FnMut`
        // instance, and `Fn` would reject it. Pinned by `iter_map` in
        // `tests/client_test/src/lib.rs`.
        #[hax_lib::attributes]
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        impl<I: Iterator, O, F: FnMut(I::Item) -> O> Iterator for Map<I, F> {
            type Item = O;

            fn next(&mut self) -> Option<O> {
                match self.iter.next() {
                    Option::Some(v) => Option::Some((self.f)(v)),
                    Option::None => Option::None,
                }
            }
        }
    }
    pub mod take {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::Take`]
        pub struct Take<I> {
            iter: I,
            n: usize,
        }
        #[hax_lib::attributes]
        impl<I> Take<I> {
            pub fn new(iter: I, n: usize) -> Take<I> {
                Take { iter, n }
            }
        }
        #[hax_lib::attributes]
        impl<I: Iterator> Iterator for Take<I> {
            type Item = <I as Iterator>::Item;

            fn next(&mut self) -> Option<<I as Iterator>::Item> {
                if self.n != 0 {
                    self.n -= 1;
                    self.iter.next()
                } else {
                    Option::None
                }
            }
        }
    }
    pub mod flat_map {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::FlatMap`]
        pub struct FlatMap<I, U, F> {
            it: I,
            f: F,
            current: Option<U>,
        }
        #[hax_lib::attributes]
        impl<I: Iterator, U: Iterator, F: FnMut(I::Item) -> U> FlatMap<I, U, F> {
            pub fn new(it: I, f: F) -> Self {
                Self {
                    it,
                    f,
                    current: Option::None,
                }
            }
        }
        #[hax_lib::attributes]
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        impl<I: Iterator, U: Iterator, F: FnMut(I::Item) -> U> Iterator for FlatMap<I, U, F> {
            type Item = U::Item;
            fn next(&mut self) -> Option<U::Item> {
                loop {
                    if let Option::Some(current_it) = &mut self.current
                        && let Option::Some(v) = current_it.next()
                    {
                        return Option::Some(v);
                    } else {
                        match self.it.next() {
                            Option::Some(c) => self.current = Option::Some((self.f)(c)),
                            Option::None => return Option::None,
                        }
                    }
                }
            }
        }
    }
    pub mod flatten {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::Flatten`]
        #[hax_lib::fstar::before("noeq")] // https://github.com/pereq-dev/hax/issues/1810
        pub struct Flatten<I: Iterator>
        where
            I::Item: Iterator,
        {
            it: I,
            current: Option<I::Item>,
        }
        #[hax_lib::attributes]
        impl<I: Iterator> Flatten<I>
        where
            I::Item: Iterator,
        {
            pub fn new(it: I) -> Self {
                Self {
                    it,
                    current: Option::None,
                }
            }
        }
        #[hax_lib::attributes]
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        impl<I: Iterator> Iterator for Flatten<I>
        where
            I::Item: Iterator,
        {
            type Item = <<I as Iterator>::Item as Iterator>::Item;
            fn next(&mut self) -> Option<<<I as Iterator>::Item as Iterator>::Item> {
                loop {
                    if let Option::Some(current_it) = &mut self.current
                        && let Option::Some(v) = current_it.next()
                    {
                        return Option::Some(v);
                    } else {
                        match self.it.next() {
                            Option::Some(c) => self.current = Option::Some(c),
                            Option::None => return Option::None,
                        }
                    }
                }
            }
        }
    }
    pub mod zip {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::Zip`]
        pub struct Zip<I1, I2> {
            it1: I1,
            it2: I2,
        }
        #[hax_lib::attributes]
        impl<I1: Iterator, I2: Iterator> Zip<I1, I2> {
            pub fn new(it1: I1, it2: I2) -> Self {
                Self { it1, it2 }
            }
        }
        #[hax_lib::attributes]
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        impl<I1: Iterator, I2: Iterator> Iterator for Zip<I1, I2> {
            type Item = (I1::Item, I2::Item);
            fn next(&mut self) -> Option<Self::Item> {
                match self.it1.next() {
                    Option::Some(v1) => match self.it2.next() {
                        Option::Some(v2) => Option::Some((v1, v2)),
                        Option::None => Option::None,
                    },
                    Option::None => Option::None,
                }
            }
        }
    }
    pub mod filter {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::Filter`]
        pub struct Filter<I, P> {
            iter: I,
            predicate: P,
        }
        #[hax_lib::attributes]
        impl<I, P> Filter<I, P> {
            pub fn new(iter: I, predicate: P) -> Self {
                Self { iter, predicate }
            }
        }
        // opaque: loop + Fn output projection not provably bool in F* (opaque still
        // extracts a real computable loop for the Lean backend).
        #[hax_lib::attributes]
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        impl<I: Iterator, P: FnMut(&I::Item) -> bool> Iterator for Filter<I, P> {
            type Item = I::Item;
            fn next(&mut self) -> Option<I::Item> {
                loop {
                    match self.iter.next() {
                        Option::Some(v) => {
                            if (self.predicate)(&v) {
                                return Option::Some(v);
                            }
                        }
                        Option::None => return Option::None,
                    }
                }
            }
        }
    }
    pub mod chain {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::Chain`]
        pub struct Chain<A, B> {
            a: Option<A>,
            b: B,
        }
        #[hax_lib::attributes]
        impl<A: Iterator, B: Iterator<Item = A::Item>> Chain<A, B> {
            pub fn new(a: A, b: B) -> Self {
                Self {
                    a: Option::Some(a),
                    b,
                }
            }
        }
        #[hax_lib::attributes]
        // opaque: `ref mut` pattern in if-let is not supported by hax
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        impl<A: Iterator, B: Iterator<Item = A::Item>> Iterator for Chain<A, B> {
            type Item = A::Item;
            fn next(&mut self) -> Option<A::Item> {
                if let Option::Some(ref mut a) = self.a {
                    match a.next() {
                        Option::Some(v) => return Option::Some(v),
                        Option::None => self.a = Option::None,
                    }
                }
                self.b.next()
            }
        }
    }
    pub mod skip {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::Skip`]
        pub struct Skip<I> {
            iter: I,
            n: usize,
        }
        #[hax_lib::attributes]
        impl<I> Skip<I> {
            pub fn new(iter: I, n: usize) -> Self {
                Self { iter, n }
            }
        }
        #[hax_lib::attributes]
        // opaque: while-loop generates Rust_primitives.Hax.while_loop, causing F* dependency cycle
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        impl<I: Iterator> Iterator for Skip<I> {
            type Item = I::Item;
            fn next(&mut self) -> Option<I::Item> {
                while self.n > 0 {
                    self.n -= 1;
                    if let Option::None = self.iter.next() {
                        return Option::None;
                    }
                }
                self.iter.next()
            }
        }
    }
    pub mod filter_map {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::FilterMap`]
        pub struct FilterMap<I, F> {
            iter: I,
            f: F,
        }
        impl<I, F> FilterMap<I, F> {
            pub fn new(iter: I, f: F) -> Self {
                Self { iter, f }
            }
        }
        // opaque: loop + Fn output projection, as with filter/flat_map.
        #[hax_lib::opaque]
        impl<I: Iterator, B, F: FnMut(I::Item) -> Option<B>> Iterator for FilterMap<I, F> {
            type Item = B;
            fn next(&mut self) -> Option<B> {
                loop {
                    match self.iter.next() {
                        Option::Some(x) => {
                            if let Option::Some(y) = (self.f)(x) {
                                return Option::Some(y);
                            }
                        }
                        Option::None => return Option::None,
                    }
                }
            }
        }
    }
    pub mod take_while {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::TakeWhile`]
        pub struct TakeWhile<I, P> {
            iter: I,
            flag: bool,
            predicate: P,
        }
        impl<I, P> TakeWhile<I, P> {
            pub fn new(iter: I, predicate: P) -> Self {
                Self {
                    iter,
                    flag: false,
                    predicate,
                }
            }
        }
        #[hax_lib::opaque]
        impl<I: Iterator, P: FnMut(&I::Item) -> bool> Iterator for TakeWhile<I, P> {
            type Item = I::Item;
            fn next(&mut self) -> Option<I::Item> {
                if self.flag {
                    Option::None
                } else {
                    match self.iter.next() {
                        Option::Some(x) => {
                            if (self.predicate)(&x) {
                                Option::Some(x)
                            } else {
                                self.flag = true;
                                Option::None
                            }
                        }
                        Option::None => Option::None,
                    }
                }
            }
        }
    }
    pub mod skip_while {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::SkipWhile`]
        pub struct SkipWhile<I, P> {
            iter: I,
            flag: bool,
            predicate: P,
        }
        impl<I, P> SkipWhile<I, P> {
            pub fn new(iter: I, predicate: P) -> Self {
                Self {
                    iter,
                    flag: false,
                    predicate,
                }
            }
        }
        // opaque: loop + Fn output projection, as with filter/skip.
        #[hax_lib::opaque]
        impl<I: Iterator, P: FnMut(&I::Item) -> bool> Iterator for SkipWhile<I, P> {
            type Item = I::Item;
            fn next(&mut self) -> Option<I::Item> {
                loop {
                    match self.iter.next() {
                        Option::Some(x) => {
                            if self.flag || !(self.predicate)(&x) {
                                self.flag = true;
                                return Option::Some(x);
                            }
                        }
                        Option::None => return Option::None,
                    }
                }
            }
        }
    }
    pub mod map_while {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::MapWhile`]
        pub struct MapWhile<I, F> {
            iter: I,
            f: F,
        }
        impl<I, F> MapWhile<I, F> {
            pub fn new(iter: I, f: F) -> Self {
                Self { iter, f }
            }
        }
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        impl<I: Iterator, B, F: FnMut(I::Item) -> Option<B>> Iterator for MapWhile<I, F> {
            type Item = B;
            fn next(&mut self) -> Option<B> {
                match self.iter.next() {
                    Option::Some(x) => (self.f)(x),
                    Option::None => Option::None,
                }
            }
        }
    }
    pub mod inspect {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::Inspect`]
        pub struct Inspect<I, F> {
            iter: I,
            f: F,
        }
        impl<I, F> Inspect<I, F> {
            pub fn new(iter: I, f: F) -> Self {
                Self { iter, f }
            }
        }
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        impl<I: Iterator, F: FnMut(&I::Item)> Iterator for Inspect<I, F> {
            type Item = I::Item;
            fn next(&mut self) -> Option<I::Item> {
                match self.iter.next() {
                    Option::Some(x) => {
                        (self.f)(&x);
                        Option::Some(x)
                    }
                    Option::None => Option::None,
                }
            }
        }
    }
    pub mod fuse {
        use super::super::traits::iterator::Iterator;
        use crate::option::Option;
        /// See [`std::iter::Fuse`] — once the inner iterator returns `None`, always `None`.
        pub struct Fuse<I> {
            iter: I,
            done: bool,
        }
        impl<I> Fuse<I> {
            pub fn new(iter: I) -> Self {
                Self { iter, done: false }
            }
        }
        #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
        impl<I: Iterator> Iterator for Fuse<I> {
            type Item = I::Item;
            fn next(&mut self) -> Option<I::Item> {
                if self.done {
                    Option::None
                } else {
                    match self.iter.next() {
                        Option::Some(x) => Option::Some(x),
                        Option::None => {
                            self.done = true;
                            Option::None
                        }
                    }
                }
            }
        }
    }
}

pub mod range {
    use crate::clone::Clone;
    // // We cannot use core model's PartialOrd because its instances currently have an
    // // `hax_lib::exclude` attribute.
    // use crate::cmp::PartialOrd;
    use crate::option::Option;
    use crate::result::Result;
    /// See [`std::iter::Step`]
    pub trait Step: Clone + PartialOrd<Self> + Sized {
        fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>);
        fn forward_checked(start: Self, count: usize) -> Option<Self>;
        fn backward_checked(start: Self, count: usize) -> Option<Self>;

        fn forward(start: Self, count: usize) -> Self {
            Step::forward_checked(start, count).expect("overflow in `Step::forward`")
        }

        unsafe fn forward_unchecked(start: Self, count: usize) -> Self {
            Step::forward(start, count)
        }

        fn backward(start: Self, count: usize) -> Self {
            Step::backward_checked(start, count).expect("overflow in `Step::backward`")
        }

        unsafe fn backward_unchecked(start: Self, count: usize) -> Self {
            Step::backward(start, count)
        }
    }

    macro_rules! step_signed_methods {
        ($Name:ty, $u:ty) => {
            unsafe fn forward_unchecked(start: Self, n: usize) -> Self {
                unsafe { <$Name>::checked_add_unsigned(start, n as $u).unwrap() }
            }

            unsafe fn backward_unchecked(start: Self, n: usize) -> Self {
                unsafe { <$Name>::checked_sub_unsigned(start, n as $u).unwrap() }
            }
        };
    }

    macro_rules! step_unsigned_methods {
        ($Name:ty) => {
            unsafe fn forward_unchecked(start: Self, n: usize) -> Self {
                unsafe { <$Name>::unchecked_add(start, n as Self) }
            }

            unsafe fn backward_unchecked(start: Self, n: usize) -> Self {
                unsafe { <$Name>::unchecked_sub(start, n as Self) }
            }
        };
    }

    macro_rules! step_identical_methods {
        ($Name:ty) => {
            fn forward(start: Self, n: usize) -> Self {
                Self::forward_checked(start, n).unwrap()
            }

            fn backward(start: Self, n: usize) -> Self {
                Self::backward_checked(start, n).unwrap()
            }
        };
    }

    macro_rules! step_integer_impls {
        {
            narrower than or same width as usize:
                $( [ $UName:ty, $u_narrower:ty, $IName:ty, $i_narrower:ty ] ),+;
            wider than usize:
                $( [ $UName_wide:ty, $u_wider:ty, $IName_wide:ty, $i_wider:ty ] ),+;
        } => {
            $(
                #[hax_lib::attributes]
                impl Step for $u_narrower {
                    step_identical_methods!($UName);
                    step_unsigned_methods!($UName);

                    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
                        if *start <= *end {
                            // This relies on $u_narrower <= usize
                            let steps = (*end - *start) as usize;
                            (steps, Option::Some(steps))
                        } else {
                            (0, Option::None)
                        }
                    }

                    fn forward_checked(start: Self, n: usize) -> Option<Self> {
                        match <Self as crate::convert::TryFrom<usize>>::try_from(n) {
                            Result::Ok(n) => <$UName>::checked_add(start, n),
                            Result::Err(_) => Option::None, // if n is out of range, `unsigned_start + n` is too
                        }
                    }

                    fn backward_checked(start: Self, n: usize) -> Option<Self> {
                        match <Self as crate::convert::TryFrom<usize>>::try_from(n) {
                            Result::Ok(n) => <$UName>::checked_sub(start, n),
                            Result::Err(_) => Option::None, // if n is out of range, `unsigned_start - n` is too
                        }
                    }
                }

                #[hax_lib::attributes]
                impl Step for $i_narrower {
                    step_identical_methods!($IName);
                    step_signed_methods!($IName, $u_narrower);

                    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
                        if *start <= *end {
                            // This relies on $i_narrower <= usize.
                            // Casting to isize extends the width but preserves the sign.
                            // Use wrapping_sub in isize space and cast to usize to compute
                            // the difference that might not fit inside the range of isize.
                            let steps = crate::num::isize::wrapping_sub(*end as isize, *start as isize) as usize;
                            (steps, Option::Some(steps))
                        } else {
                            (0, Option::None)
                        }
                    }

                    fn forward_checked(start: Self, n: usize) -> Option<Self> {
                        match <$u_narrower as crate::convert::TryFrom<usize>>::try_from(n) {
                            Result::Ok(n) => {
                                // Wrapping handles cases like
                                // `Step::forward(-120_i8, 200) == Some(80_i8)`,
                                // even though 200 is out of range for i8.
                                let wrapped = <$IName>::wrapping_add(start, n as Self);
                                if wrapped >= start {
                                    Option::Some(wrapped)
                                } else {
                                    Option::None // Addition overflowed
                                }
                            }
                            // If n is out of range of e.g. u8,
                            // then it is bigger than the entire range for i8 is wide
                            // so `any_i8 + n` necessarily overflows i8.
                            Result::Err(_) => Option::None,
                        }
                    }

                    fn backward_checked(start: Self, n: usize) -> Option<Self> {
                        match <$u_narrower as crate::convert::TryFrom<usize>>::try_from(n) {
                            Result::Ok(n) => {
                                // Wrapping handles cases like
                                // `Step::forward(-120_i8, 200) == Some(80_i8)`,
                                // even though 200 is out of range for i8.
                                let wrapped = <$IName>::wrapping_sub(start, n as Self);
                                if wrapped <= start {
                                    Option::Some(wrapped)
                                } else {
                                    Option::None // Subtraction overflowed
                                }
                            }
                            // If n is out of range of e.g. u8,
                            // then it is bigger than the entire range for i8 is wide
                            // so `any_i8 - n` necessarily overflows i8.
                            Result::Err(_) => Option::None,
                        }
                    }
                }
            )+

            $(
                #[hax_lib::attributes]
                impl Step for $u_wider {
                    step_identical_methods!($UName_wide);
                    step_unsigned_methods!($UName_wide);

                    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
                        if *start <= *end {
                            match <usize as crate::convert::TryFrom<Self>>::try_from(*end - *start) {
                                Result::Ok(steps) => (steps, Option::Some(steps)),
                                Result::Err(_) => (usize::MAX, Option::None),
                            }
                        } else {
                            (0, Option::None)
                        }
                    }

                    fn forward_checked(start: Self, n: usize) -> Option<Self> {
                        <$UName_wide>::checked_add(start, n as Self)
                    }

                    fn backward_checked(start: Self, n: usize) -> Option<Self> {
                        <$UName_wide>::checked_sub(start, n as Self)
                    }
                }

                #[hax_lib::attributes]
                impl Step for $i_wider {
                    step_identical_methods!($IName_wide);
                    step_signed_methods!($IName_wide, $u_wider);

                    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
                        if *start <= *end {
                            match <$IName_wide>::checked_sub(*end, *start) {
                                Option::Some(result) => {
                                    match <usize as crate::convert::TryFrom<$i_wider>>::try_from(result) {
                                        Result::Ok(steps) => (steps, Option::Some(steps)),
                                        Result::Err(_) => (usize::MAX, Option::None),
                                    }
                                }
                                // If the difference is too big for e.g. i128,
                                // it's also gonna be too big for usize with fewer bits.
                                Option::None => (usize::MAX, Option::None),
                            }
                        } else {
                            (0, Option::None)
                        }
                    }

                    fn forward_checked(start: Self, n: usize) -> Option<Self> {
                        <$IName_wide>::checked_add(start, n as Self)
                    }

                    fn backward_checked(start: Self, n: usize) -> Option<Self> {
                        <$IName_wide>::checked_sub(start, n as Self)
                    }
                }
            )+
        };
    }

    // Assuming usize to be 64 bits
    step_integer_impls! {
        narrower than or same width as usize:
            [crate::num::u8, core::primitive::u8, crate::num::i8, core::primitive::i8],
            [crate::num::u16, core::primitive::u16, crate::num::i16, core::primitive::i16],
            [crate::num::u32, core::primitive::u32, crate::num::i32, core::primitive::i32],
            [crate::num::u64, core::primitive::u64, crate::num::i64, core::primitive::i64],
            [crate::num::usize, core::primitive::usize, crate::num::isize, core::primitive::isize];
        wider than usize:
            [crate::num::u128, core::primitive::u128, crate::num::i128, core::primitive::i128];
    }
}

#[cfg(test)]
mod tests {
    use super::traits::iterator::{Iterator, IteratorMethods};
    use crate::option::Option;
    use crate::testing::Inject;
    use proptest::prelude::*;

    /// A simple iterator over a Vec, used to test IteratorMethods.
    /// `Clone` so that a `VecIter<VecIter<_>>` can be built for `flatten`.
    #[derive(Clone)]
    struct VecIter<T> {
        data: Vec<T>,
        pos: usize,
    }

    impl<T> VecIter<T> {
        fn new(data: Vec<T>) -> Self {
            Self { data, pos: 0 }
        }
    }

    impl<T: Clone> Iterator for VecIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<T> {
            if self.pos < self.data.len() {
                let v = self.data[self.pos].clone();
                self.pos += 1;
                Option::Some(v)
            } else {
                Option::None
            }
        }
    }

    // Front cursor is `pos`; the back is consumed by popping `data`. Elements still
    // pending are `data[pos..]`, so `len` is `data.len() - pos`. Lets us differential-
    // test `next_back` / `Rev` / `Enumerate::next_back` against std.
    #[cfg(not(hax_backend_fstar))]
    impl<T: Clone> crate::iter::traits::double_ended::DoubleEndedIterator for VecIter<T> {
        fn next_back(&mut self) -> Option<T> {
            if self.pos < self.data.len() {
                // `pos < len` guarantees a back element, so `pop` cannot be `None`.
                Option::Some(self.data.pop().unwrap())
            } else {
                Option::None
            }
        }
    }
    #[cfg(not(hax_backend_fstar))]
    impl<T: Clone> crate::iter::traits::exact_size::ExactSizeIterator for VecIter<T> {
        fn len(&self) -> usize {
            self.data.len() - self.pos
        }
    }

    proptest! {
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_range_next_back(lo in 0usize..50, len in 0usize..50) {
            use crate::iter::traits::double_ended::DoubleEndedIterator;
            let hi = lo + len;
            let mut r = crate::ops::range::Range { start: lo, end: hi };
            let mut model: Vec<usize> = Vec::new();
            while let Option::Some(x) = r.next_back() {
                model.push(x);
            }
            let std_result: Vec<usize> = (lo..hi).rev().collect();
            prop_assert_eq!(model, std_result);
        }

        #[cfg(not(hax_backend_fstar))]

        #[test]
        fn test_rev_next_back(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            use crate::iter::traits::double_ended::DoubleEndedIterator;
            let mut it = VecIter::new(v.clone());
            let mut model: Vec<i32> = Vec::new();
            while let Option::Some(x) = it.next_back() {
                model.push(x);
            }
            let std_result: Vec<i32> = v.iter().rev().copied().collect();
            prop_assert_eq!(model, std_result);
        }

        #[cfg(not(hax_backend_fstar))]

        #[test]
        fn test_enumerate_next_back(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            use crate::iter::traits::double_ended::DoubleEndedIterator;
            let mut it = crate::iter::adapters::enumerate::Enumerate::new(VecIter::new(v.clone()));
            let mut model: Vec<(usize, i32)> = Vec::new();
            while let Option::Some(x) = it.next_back() {
                model.push(x);
            }
            let std_result: Vec<(usize, i32)> = v.iter().copied().enumerate().rev().collect();
            prop_assert_eq!(model, std_result);
        }


        // Closure-driven adapters.
        #[test]
        fn test_filter_map(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            let mut it = VecIter::new(v.clone()).filter_map(
                |x: i32| if x > 0 { Option::Some(x.wrapping_mul(2)) } else { Option::None });
            let mut model: Vec<i32> = Vec::new();
            while let Option::Some(x) = it.next() {
                model.push(x);
            }
            let std_result: Vec<i32> = v.iter().copied()
                .filter_map(|x| if x > 0 { Some(x.wrapping_mul(2)) } else { None }).collect();
            prop_assert_eq!(model, std_result);
        }

        #[test]
        fn test_take_while(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            let mut it = VecIter::new(v.clone()).take_while(|x: &i32| *x >= 0);
            let mut model: Vec<i32> = Vec::new();
            while let Option::Some(x) = it.next() {
                model.push(x);
            }
            let std_result: Vec<i32> = v.iter().copied().take_while(|x| *x >= 0).collect();
            prop_assert_eq!(model, std_result);
        }

        #[test]
        fn test_skip_while(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            let mut it = VecIter::new(v.clone()).skip_while(|x: &i32| *x >= 0);
            let mut model: Vec<i32> = Vec::new();
            while let Option::Some(x) = it.next() {
                model.push(x);
            }
            let std_result: Vec<i32> = v.iter().copied().skip_while(|x| *x >= 0).collect();
            prop_assert_eq!(model, std_result);
        }

        #[test]
        fn test_map_while(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            let mut it = VecIter::new(v.clone()).map_while(
                |x: i32| if x > 0 { Option::Some(x.wrapping_mul(2)) } else { Option::None });
            let mut model: Vec<i32> = Vec::new();
            while let Option::Some(x) = it.next() {
                model.push(x);
            }
            let std_result: Vec<i32> = v.iter().copied()
                .map_while(|x| if x > 0 { Some(x.wrapping_mul(2)) } else { None }).collect();
            prop_assert_eq!(model, std_result);
        }

        // `inspect` passes through and `fuse` is the identity over a well-behaved
        // iterator: both reproduce the input unchanged.
        #[test]
        fn test_inspect(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            let mut it = VecIter::new(v.clone()).inspect(|_x: &i32| {});
            let mut model: Vec<i32> = Vec::new();
            while let Option::Some(x) = it.next() {
                model.push(x);
            }
            prop_assert_eq!(model, v);
        }

        #[test]
        fn test_fuse(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            let mut it = VecIter::new(v.clone()).fuse();
            let mut model: Vec<i32> = Vec::new();
            while let Option::Some(x) = it.next() {
                model.push(x);
            }
            prop_assert_eq!(model, v);
        }

        #[test]
        fn test_fold_sum(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            let std_result = v.iter().fold(0i32, |acc, &x| acc.wrapping_add(x));
            let model_result = VecIter::new(v).fold(0i32, |acc: i32, x: i32| acc.wrapping_add(x));
            prop_assert_eq!(model_result, std_result);
        }

        #[test]
        fn test_fold_collect(v in prop::collection::vec(any::<i32>(), 0..=10)) {
            let std_result: Vec<i32> = v.iter().fold(Vec::new(), |mut acc, &x| { acc.push(x); acc });
            let model_result: Vec<i32> = VecIter::new(v).fold(Vec::new(), |mut acc: Vec<i32>, x: i32| { acc.push(x); acc });
            prop_assert_eq!(model_result, std_result);
        }

        // One test each, not one per outcome: `iter_all`/`iter_any` are generic in
        // the predicate, so a second test would be a second instantiation that
        // only ever takes one of the two exits. `bound` is biased toward the
        // extreme that makes the predicate uniformly true (resp. false).
        #[test]
        fn test_all(
            v in prop::collection::vec(any::<i32>(), 0..=20),
            bound in prop_oneof![Just(i32::MIN), any::<i32>()],
        ) {
            let std_result = v.iter().all(|x| *x > bound);
            let model_result = VecIter::new(v).all(|x: i32| x > bound);
            prop_assert_eq!(model_result, std_result);
        }

        #[test]
        fn test_any(
            v in prop::collection::vec(any::<i32>(), 0..=20),
            bound in prop_oneof![Just(i32::MAX), any::<i32>()],
        ) {
            let std_result = v.iter().any(|x| *x > bound);
            let model_result = VecIter::new(v).any(|x: i32| x > bound);
            prop_assert_eq!(model_result, std_result);
        }

        #[test]
        fn test_find(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            let std_result = v.iter().find(|x| **x > 10).copied();
            let model_result = VecIter::new(v).find(|x: &i32| *x > 10);
            prop_assert_eq!(model_result, std_result.inject());
        }

        #[test]
        fn test_find_map(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            let std_result: std::option::Option<u32> = v.iter().find_map(|&x| if x > 0 { Some(x as u32) } else { None });
            let model_result: Option<u32> = VecIter::new(v).find_map(|x: i32| if x > 0 { Option::Some(x as u32) } else { Option::None });
            prop_assert_eq!(model_result, std_result.inject());
        }

        #[test]
        fn test_position(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            let std_result = v.iter().position(|x| *x > 10);
            let model_result = VecIter::new(v).position(|x: i32| x > 10);
            prop_assert_eq!(model_result, std_result.inject());
        }

        #[test]
        fn test_count(v in prop::collection::vec(any::<u8>(), 0..=20)) {
            let std_result = v.iter().count();
            let model_result = VecIter::new(v).count();
            prop_assert_eq!(model_result, std_result);
        }

        #[test]
        fn test_nth(v in prop::collection::vec(any::<i32>(), 0..=20), n in 0usize..25) {
            let std_result = v.iter().nth(n).copied();
            let model_result = VecIter::new(v).nth(n);
            prop_assert_eq!(model_result, std_result.inject());
        }

        #[test]
        fn test_last(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            let std_result = v.iter().last().copied();
            let model_result = VecIter::new(v).last();
            prop_assert_eq!(model_result, std_result.inject());
        }

        #[test]
        fn test_for_each(v in prop::collection::vec(any::<i32>(), 0..=5)) {
            // Use a side-effect-free test: for_each with an empty body should consume the iterator
            // without error. We verify it doesn't panic and processes all elements.
            let std_count = std::cell::Cell::new(0usize);
            v.iter().for_each(|_| { std_count.set(std_count.get() + 1); });
            let model_count = std::cell::Cell::new(0usize);
            VecIter::new(v).for_each(|_: i32| { model_count.set(model_count.get() + 1); });
            prop_assert_eq!(model_count.get(), std_count.get());
        }

        #[test]
        fn test_reduce(v in prop::collection::vec(any::<i32>(), 0..=10)) {
            let std_result = v.iter().copied().reduce(|a, b| a.wrapping_add(b));
            let model_result = VecIter::new(v).reduce(|a: i32, b: i32| a.wrapping_add(b));
            prop_assert_eq!(model_result, std_result.inject());
        }

        #[test]
        fn test_min(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            let std_result = v.iter().copied().min();
            let model_result = VecIter::new(v).min();
            prop_assert_eq!(model_result, std_result.inject());
        }

        #[test]
        fn test_max(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            let std_result = v.iter().copied().max();
            let model_result = VecIter::new(v).max();
            prop_assert_eq!(model_result, std_result.inject());
        }
    }

    /// The model's `FromIterator::from_iter` gets no bound relating the
    /// iterator's items to `A`, so a collector can consume the iterator but not
    /// read it; `Consumed` records that `collect` reached `from_iter`.
    #[derive(PartialEq, Debug)]
    struct Consumed;

    impl super::traits::collect::FromIterator<u8> for Consumed {
        fn from_iter<T: super::traits::collect::IntoIterator>(iter: T) -> Self {
            let _ = super::traits::collect::IntoIterator::into_iter(iter);
            Consumed
        }
    }

    /// Unlike `Consumed`, this one DRIVES the iterator, so it observes the
    /// items the `Result` shunt buffered (and covers `SeqIter::next`).
    #[derive(PartialEq, Debug)]
    struct Total(u32);

    impl super::traits::collect::FromIterator<u8> for Total {
        // Declares the trait's `Item = u8` bound (unlike `Consumed`, which omits
        // it and so cannot read the items).
        fn from_iter<T: super::traits::collect::IntoIterator<Item = u8>>(iter: T) -> Self {
            let mut it = super::traits::collect::IntoIterator::into_iter(iter);
            let mut acc = 0u32;
            while let Option::Some(x) = Iterator::next(&mut it) {
                acc += x as u32;
            }
            Total(acc)
        }
    }

    /// The adapters are lazy; draining them is what observes them.
    fn drain<I: Iterator>(mut it: I) -> Vec<I::Item> {
        let mut out = Vec::new();
        while let Option::Some(x) = it.next() {
            out.push(x);
        }
        out
    }

    proptest! {
        /// The `Rev` adapter itself (its `next` delegates to the inner `next_back`).
        /// Gated like the `DoubleEndedIterator` impls it relies on.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_rev_adapter(v in prop::collection::vec(any::<i32>(), 0..=20)) {
            prop_assert_eq!(
                drain(crate::iter::adapters::rev::Rev::new(VecIter::new(v.clone()))),
                v.iter().copied().rev().collect::<Vec<_>>()
            );
        }

        /// `take_while`'s four outcomes, all through ONE closure type: llvm-cov
        /// scores regions per monomorphisation, so splitting these across
        /// closures leaves an arm unreached in each instantiation.
        #[test]
        fn test_take_while_arms(bound in any::<i32>()) {
            let pred = |x: &i32| *x < bound;
            let below = bound.saturating_sub(1);

            // Predicate fails partway: yield, then latch, then stay latched.
            let stops = vec![below, bound.saturating_add(1), below];
            let mut it = VecIter::new(stops.clone()).take_while(pred);
            let mut got = Vec::new();
            for _ in 0..stops.len() + 1 {
                if let Option::Some(x) = it.next() {
                    got.push(x);
                }
            }
            prop_assert_eq!(
                got,
                stops.iter().copied().take_while(|x| *x < bound).collect::<Vec<_>>()
            );

            // Predicate always holds: the inner iterator exhausts with the flag
            // still unset, which is the remaining arm.
            let all_pass = vec![below; 3];
            prop_assert_eq!(
                drain(VecIter::new(all_pass.clone()).take_while(pred)),
                all_pass.iter().copied().take_while(|x| *x < bound).collect::<Vec<_>>()
            );
        }

        /// `fuse` latches: `next` past exhaustion stays `None`.
        #[test]
        fn test_fuse_latches(v in prop::collection::vec(any::<i32>(), 0..=5)) {
            let mut it = VecIter::new(v.clone()).fuse();
            let mut got = Vec::new();
            // Two extra `next` calls past the end exercise the `done` branch.
            for _ in 0..v.len() + 2 {
                if let Option::Some(x) = it.next() {
                    got.push(x);
                }
            }
            prop_assert_eq!(got, v);
        }

        #[test]
        fn test_map(v in prop::collection::vec(any::<u8>(), 0..=20), table in any::<[u8; 256]>()) {
            let f = |x: u8| table[x as usize];
            prop_assert_eq!(
                drain(VecIter::new(v.clone()).map(f)),
                v.iter().map(|&x| f(x)).collect::<Vec<_>>()
            );
        }

        #[test]
        fn test_filter(v in prop::collection::vec(any::<u8>(), 0..=20), bound in any::<u8>()) {
            prop_assert_eq!(
                drain(VecIter::new(v.clone()).filter(|x: &u8| *x > bound)),
                v.iter().copied().filter(|x| *x > bound).collect::<Vec<_>>()
            );
        }

        #[test]
        fn test_enumerate(v in prop::collection::vec(any::<u8>(), 0..=20)) {
            prop_assert_eq!(
                drain(VecIter::new(v.clone()).enumerate()),
                v.iter().copied().enumerate().collect::<Vec<_>>()
            );
        }

        #[test]
        fn test_skip(v in prop::collection::vec(any::<u8>(), 0..=20), n in 0usize..=25) {
            prop_assert_eq!(
                drain(VecIter::new(v.clone()).skip(n)),
                v.iter().copied().skip(n).collect::<Vec<_>>()
            );
        }

        #[test]
        fn test_take(v in prop::collection::vec(any::<u8>(), 0..=20), n in 0usize..=25) {
            prop_assert_eq!(
                drain(VecIter::new(v.clone()).take(n)),
                v.iter().copied().take(n).collect::<Vec<_>>()
            );
        }

        #[test]
        fn test_chain(
            a in prop::collection::vec(any::<u8>(), 0..=10),
            b in prop::collection::vec(any::<u8>(), 0..=10),
        ) {
            prop_assert_eq!(
                drain(VecIter::new(a.clone()).chain(VecIter::new(b.clone()))),
                a.iter().copied().chain(b.iter().copied()).collect::<Vec<_>>()
            );
        }

        // Unequal lengths: `zip` must stop at the shorter side.
        #[test]
        fn test_zip(
            a in prop::collection::vec(any::<u8>(), 0..=10),
            b in prop::collection::vec(any::<u8>(), 0..=15),
        ) {
            prop_assert_eq!(
                drain(VecIter::new(a.clone()).zip(VecIter::new(b.clone()))),
                a.iter().copied().zip(b.iter().copied()).collect::<Vec<_>>()
            );
        }

        #[test]
        fn test_flat_map(
            vs in prop::collection::vec(prop::collection::vec(any::<u8>(), 0..=5), 0..=6),
        ) {
            prop_assert_eq!(
                drain(VecIter::new(vs.clone()).flat_map(|v: Vec<u8>| VecIter::new(v))),
                vs.iter().flat_map(|v| v.iter().copied()).collect::<Vec<_>>()
            );
        }

        #[test]
        fn test_flatten(
            vs in prop::collection::vec(prop::collection::vec(any::<u8>(), 0..=5), 0..=6),
        ) {
            let inner: Vec<VecIter<u8>> = vs.iter().cloned().map(VecIter::new).collect();
            prop_assert_eq!(
                drain(VecIter::new(inner).flatten()),
                vs.iter().flat_map(|v| v.iter().copied()).collect::<Vec<_>>()
            );
        }

        // `IntoIterator for I: Iterator` is the identity.
        #[test]
        fn test_iterator_into_iter(v in prop::collection::vec(any::<u8>(), 0..=20)) {
            use super::traits::collect::IntoIterator;
            prop_assert_eq!(drain(IntoIterator::into_iter(VecIter::new(v.clone()))), v);
        }

        #[test]
        fn test_collect(v in prop::collection::vec(any::<u8>(), 0..=10)) {
            prop_assert_eq!(VecIter::new(v).collect::<Consumed>(), Consumed);
        }

        // `Result<V, E>: FromIterator<Result<A, E>>` delegates to `V`'s own
        // `from_iter`, which is all its (opaque) body claims to do.
        #[test]
        fn test_collect_into_result(v in prop::collection::vec(any::<u8>(), 0..=10)) {
            let it = VecIter::new(v).map(crate::result::Result::<u8, u8>::Ok);
            let collected: crate::result::Result<Consumed, u8> = it.collect();
            prop_assert_eq!(collected, crate::result::Result::Ok(Consumed));
        }

        /// Collecting into a collector that actually drains: the `Ok`s buffered by
        /// the `Result` shunt must reach `V::from_iter` in order.
        #[test]
        fn test_collect_into_result_total(v in prop::collection::vec(any::<u8>(), 0..=10)) {
            let it = VecIter::new(v.clone()).map(crate::result::Result::<u8, u8>::Ok);
            let collected: crate::result::Result<Total, u8> = it.collect();
            let expected: u32 = v.iter().map(|&x| x as u32).sum();
            prop_assert_eq!(collected, crate::result::Result::Ok(Total(expected)));
        }

        /// The `Err` case the previous axiomatised `from_iter` could not model:
        /// collecting stops at the FIRST `Err` and yields it, and still reaches
        /// `Ok` when every item is `Ok`. (`VecIter` needs `T: Clone`, and the
        /// model's `Result` is not `Clone`, so the `Result`s come from a `map`.)
        #[test]
        fn test_collect_into_result_err(
            v in prop::collection::vec(any::<u8>(), 0..=10),
            bound in any::<u8>(),
        ) {
            let expected_err = v.iter().copied().find(|&x| x > bound);
            let collected: crate::result::Result<Consumed, u8> = VecIter::new(v.clone())
                .map(|x: u8| if x > bound {
                    crate::result::Result::Err(x)
                } else {
                    crate::result::Result::Ok(x)
                })
                .collect();
            match expected_err {
                Some(e) => prop_assert_eq!(collected, crate::result::Result::Err(e)),
                None => prop_assert_eq!(collected, crate::result::Result::Ok(Consumed)),
            }
        }

        #[test]
        fn test_step_by(v in prop::collection::vec(any::<u8>(), 0..=20), step in 1usize..=5) {
            prop_assert_eq!(
                drain(VecIter::new(v.clone()).step_by(step)),
                v.iter().copied().step_by(step).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn test_step_by_zero_panics() {
        crate::testing::panics_like_core(
            || VecIter::new(vec![1u8, 2, 3]).step_by(0),
            || [1u8, 2, 3].iter().step_by(0),
        );
    }

    macro_rules! step_tests {
        ($mod_name:ident, $T:ty) => {
            mod $mod_name {
                use super::super::range::Step as ModelStep;
                use crate::testing::Inject;
                use proptest::prelude::*;
                use std::iter::Step as StdStep;

                proptest! {
                    #[test]
                    fn forward(x: $T, y: $T) {
                        if let Some(n) = <$T as StdStep>::steps_between(&x, &y).1 {
                            prop_assert_eq!(
                                <$T as ModelStep>::forward(x, n),
                                <$T as StdStep>::forward(x, n).inject(),
                            );
                        }
                    }

                    #[test]
                    fn backward(x: $T, y: $T) {
                        if let Some(n) = <$T as StdStep>::steps_between(&x, &y).1 {
                            prop_assert_eq!(
                                <$T as ModelStep>::backward(y, n),
                                <$T as StdStep>::backward(y, n).inject(),
                            );
                        }
                    }

                    #[test]
                    fn forward_unchecked(x: $T, y: $T) {
                        if let Some(n) = <$T as StdStep>::steps_between(&x, &y).1 {
                            prop_assert_eq!(
                                unsafe { <$T as ModelStep>::forward_unchecked(x, n) },
                                unsafe { <$T as StdStep>::forward_unchecked(x, n) }.inject(),
                            );
                        }
                    }

                    #[test]
                    fn backward_unchecked(x: $T, y: $T) {
                        if let Some(n) = <$T as StdStep>::steps_between(&x, &y).1 {
                            prop_assert_eq!(
                                unsafe { <$T as ModelStep>::backward_unchecked(y, n) },
                                unsafe { <$T as StdStep>::backward_unchecked(y, n) }.inject(),
                            );
                        }
                    }

                    #[test]
                    fn steps_between(a: $T, b: $T) {
                        let (model_lower, model_exact) = <$T as ModelStep>::steps_between(&a, &b);
                        let (std_lower, std_exact) = <$T as StdStep>::steps_between(&a, &b);
                        prop_assert_eq!(model_lower, std_lower);
                        prop_assert_eq!(model_exact, std_exact.inject());
                    }

                    // For `u128`/`i128` a random pair never fits in a `usize`, so
                    // the tests above never run these bodies; a small step does.
                    #[test]
                    fn forward_close(x: $T, d in 0usize..=1000) {
                        if <$T as StdStep>::forward_checked(x, d).is_some() {
                            prop_assert_eq!(
                                <$T as ModelStep>::forward(x, d),
                                <$T as StdStep>::forward(x, d).inject(),
                            );
                            prop_assert_eq!(
                                unsafe { <$T as ModelStep>::forward_unchecked(x, d) },
                                unsafe { <$T as StdStep>::forward_unchecked(x, d) }.inject(),
                            );
                        }
                    }

                    #[test]
                    fn backward_close(x: $T, d in 0usize..=1000) {
                        if <$T as StdStep>::backward_checked(x, d).is_some() {
                            prop_assert_eq!(
                                <$T as ModelStep>::backward(x, d),
                                <$T as StdStep>::backward(x, d).inject(),
                            );
                            prop_assert_eq!(
                                unsafe { <$T as ModelStep>::backward_unchecked(x, d) },
                                unsafe { <$T as StdStep>::backward_unchecked(x, d) }.inject(),
                            );
                        }
                    }

                    // For `u128`/`i128` a random pair never fits in a `usize`;
                    // stepping forward by a small amount does.
                    #[test]
                    fn steps_between_close(a: $T, d in 0usize..=1000) {
                        if let Some(b) = <$T as StdStep>::forward_checked(a, d) {
                            let (model_lower, model_exact) = <$T as ModelStep>::steps_between(&a, &b);
                            let (std_lower, std_exact) = <$T as StdStep>::steps_between(&a, &b);
                            prop_assert_eq!(model_lower, std_lower);
                            prop_assert_eq!(model_exact, std_exact.inject());
                        }
                    }

                    #[test]
                    fn forward_checked(x: $T, n: usize) {
                        let model = <$T as ModelStep>::forward_checked(x, n);
                        let std_result = <$T as StdStep>::forward_checked(x, n);
                        prop_assert_eq!(model, std_result.inject());
                    }

                    // A full-range `n` never gets past the `TryFrom<usize>` guard
                    // in the narrow-integer impls, leaving their wrapping-overflow
                    // arm unreached; a narrow `n` reaches both.
                    #[test]
                    fn forward_checked_narrow(x: $T, n in 0usize..=255) {
                        prop_assert_eq!(
                            <$T as ModelStep>::forward_checked(x, n),
                            <$T as StdStep>::forward_checked(x, n).inject(),
                        );
                    }

                    #[test]
                    fn backward_checked_narrow(x: $T, n in 0usize..=255) {
                        prop_assert_eq!(
                            <$T as ModelStep>::backward_checked(x, n),
                            <$T as StdStep>::backward_checked(x, n).inject(),
                        );
                    }

                    #[test]
                    fn backward_checked(x: $T, n: usize) {
                        let model = <$T as ModelStep>::backward_checked(x, n);
                        let std_result = <$T as StdStep>::backward_checked(x, n);
                        prop_assert_eq!(model, std_result.inject());
                    }
                }
            }
        };
    }

    /// Every integer `Step` impl overrides `forward`/`backward` and their
    /// `_unchecked` variants, so the trait's default bodies need a type that
    /// implements only the three required methods.
    mod step_defaults {
        use super::super::range::Step as ModelStep;
        use crate::option::Option;
        use crate::testing::Inject;
        use proptest::prelude::*;
        use std::iter::Step as StdStep;

        #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
        struct Wrap(u8);

        // Under the F* cfg `crate::clone::Clone` has a blanket impl.
        #[cfg(not(hax_backend_fstar))]
        impl crate::clone::Clone for Wrap {
            fn clone(self) -> Self {
                self
            }
        }

        impl ModelStep for Wrap {
            fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
                <u8 as ModelStep>::steps_between(&start.0, &end.0)
            }
            fn forward_checked(start: Self, count: usize) -> Option<Self> {
                match <u8 as ModelStep>::forward_checked(start.0, count) {
                    Option::Some(v) => Option::Some(Wrap(v)),
                    Option::None => Option::None,
                }
            }
            fn backward_checked(start: Self, count: usize) -> Option<Self> {
                match <u8 as ModelStep>::backward_checked(start.0, count) {
                    Option::Some(v) => Option::Some(Wrap(v)),
                    Option::None => Option::None,
                }
            }
        }

        proptest! {
            #[test]
            fn steps_between(a in any::<u8>(), b in any::<u8>()) {
                // `Step: Clone` is only a bound, never called by the defaults.
                prop_assert_eq!(crate::clone::Clone::clone(Wrap(a)), Wrap(a));
                let (model_lower, model_exact) = <Wrap as ModelStep>::steps_between(&Wrap(a), &Wrap(b));
                let (std_lower, std_exact) = <u8 as StdStep>::steps_between(&a, &b);
                prop_assert_eq!(model_lower, std_lower);
                prop_assert_eq!(model_exact, std_exact.inject());
            }

            #[test]
            fn forward(x in any::<u8>(), n in 0usize..=255) {
                prop_assume!(<u8 as StdStep>::forward_checked(x, n).is_some());
                prop_assert_eq!(<Wrap as ModelStep>::forward(Wrap(x), n).0, <u8 as StdStep>::forward(x, n));
                prop_assert_eq!(
                    unsafe { <Wrap as ModelStep>::forward_unchecked(Wrap(x), n) }.0,
                    unsafe { <u8 as StdStep>::forward_unchecked(x, n) }
                );
            }

            #[test]
            fn backward(x in any::<u8>(), n in 0usize..=255) {
                prop_assume!(<u8 as StdStep>::backward_checked(x, n).is_some());
                prop_assert_eq!(<Wrap as ModelStep>::backward(Wrap(x), n).0, <u8 as StdStep>::backward(x, n));
                prop_assert_eq!(
                    unsafe { <Wrap as ModelStep>::backward_unchecked(Wrap(x), n) }.0,
                    unsafe { <u8 as StdStep>::backward_unchecked(x, n) }
                );
            }

            #[test]
            fn forward_checked(x in any::<u8>(), n in any::<usize>()) {
                let model = <Wrap as ModelStep>::forward_checked(Wrap(x), n);
                let expected = <u8 as StdStep>::forward_checked(x, n);
                match (model, expected) {
                    (Option::Some(m), Some(e)) => prop_assert_eq!(m.0, e),
                    (Option::None, None) => {},
                    _ => prop_assert!(false, "forward_checked disagrees"),
                }
            }

            #[test]
            fn backward_checked(x in any::<u8>(), n in any::<usize>()) {
                let model = <Wrap as ModelStep>::backward_checked(Wrap(x), n);
                let expected = <u8 as StdStep>::backward_checked(x, n);
                match (model, expected) {
                    (Option::Some(m), Some(e)) => prop_assert_eq!(m.0, e),
                    (Option::None, None) => {},
                    _ => prop_assert!(false, "backward_checked disagrees"),
                }
            }
        }
    }

    step_tests!(step_u8, u8);
    step_tests!(step_i8, i8);
    step_tests!(step_u16, u16);
    step_tests!(step_i16, i16);
    step_tests!(step_u32, u32);
    step_tests!(step_i32, i32);
    step_tests!(step_u64, u64);
    step_tests!(step_i64, i64);
    step_tests!(step_usize, usize);
    step_tests!(step_isize, isize);
    step_tests!(step_u128, u128);
    step_tests!(step_i128, i128);
}
