pub mod arith {
    /// See [`std::ops::Add`]
    pub trait Add<Rhs = Self> {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    }
    /// See [`std::ops::Sub`]
    pub trait Sub<Rhs = Self> {
        type Output;
        fn sub(self, rhs: Rhs) -> Self::Output;
    }
    /// See [`std::ops::Mul`]
    pub trait Mul<Rhs = Self> {
        type Output;
        fn mul(self, rhs: Rhs) -> Self::Output;
    }
    /// See [`std::ops::Div`]
    pub trait Div<Rhs = Self> {
        type Output;
        fn div(self, rhs: Rhs) -> Self::Output;
    }
    /// See [`std::ops::Neg`]
    pub trait Neg {
        type Output;
        fn neg(self) -> Self::Output;
    }
    /// See [`std::ops::Rem`]
    pub trait Rem<Rhs = Self> {
        type Output;
        fn rem(self, rhs: Rhs) -> Self::Output;
    }
    /// See [`std::ops::AddAssign`]
    pub trait AddAssign<Rhs = Self> {
        fn add_assign(&mut self, rhs: Rhs);
    }
    /// See [`std::ops::SubAssign`]
    pub trait SubAssign<Rhs = Self> {
        fn sub_assign(&mut self, rhs: Rhs);
    }
    /// See [`std::ops::MulAssign`]
    pub trait MulAssign<Rhs = Self> {
        fn mul_assign(&mut self, rhs: Rhs);
    }
    /// See [`std::ops::DivAssign`]
    pub trait DivAssign<Rhs = Self> {
        fn div_assign(&mut self, rhs: Rhs);
    }
    /// See [`std::ops::RemAssign`]
    pub trait RemAssign<Rhs = Self> {
        fn rem_assign(&mut self, rhs: Rhs);
    }

    macro_rules! int_trait_impls {
        ($($Self:ty)*) => {
            use hax_lib::ToInt;
            $(
            #[hax_lib::attributes]
            #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
            impl crate::ops::arith::AddAssign<$Self> for $Self {
                #[hax_lib::requires(self.to_int() + rhs.to_int() <= $Self::MAX.to_int())]
                fn add_assign(&mut self, rhs: $Self) {
                    *self = *self + rhs
                }
            }
            #[hax_lib::attributes]
            #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
            impl crate::ops::arith::SubAssign<$Self> for $Self {
                #[hax_lib::requires(self.to_int() - rhs.to_int() >= 0.to_int())]
                fn sub_assign(&mut self, rhs: $Self) {
                    *self = *self - rhs
                }
            })*
        }
    }

    int_trait_impls!(u8 u16 u32 u64);
}

pub mod bit {
    /// See [`std::ops::Shr`]
    pub trait Shr<Rhs = Self> {
        type Output;
        fn shr(self, rhs: Rhs) -> Self::Output;
    }
    /// See [`std::ops::Shl`]
    pub trait Shl<Rhs = Self> {
        type Output;
        fn shl(self, rhs: Rhs) -> Self::Output;
    }
    /// See [`std::ops::BitXor`]
    pub trait BitXor<Rhs = Self> {
        type Output;
        fn bitxor(self, rhs: Rhs) -> Self::Output;
    }
    /// See [`std::ops::BitAnd`]
    pub trait BitAnd<Rhs = Self> {
        type Output;
        fn bitand(self, rhs: Rhs) -> Self::Output;
    }
    /// See [`std::ops::BitOr`]
    pub trait BitOr<Rhs = Self> {
        type Output;
        fn bitor(self, rhs: Rhs) -> Self::Output;
    }
    /// See [`std::ops::Not`]
    pub trait Not {
        type Output;
        fn not(self) -> Self::Output;
    }
    /// See [`std::ops::ShrAssign`]
    pub trait ShrAssign<Rhs = Self> {
        fn shr_assign(&mut self, rhs: Rhs);
    }
    /// See [`std::ops::ShlAssign`]
    pub trait ShlAssign<Rhs = Self> {
        fn shl_assign(&mut self, rhs: Rhs);
    }
    /// See [`std::ops::BitXorAssign`]
    pub trait BitXorAssign<Rhs = Self> {
        fn bitxor_assign(&mut self, rhs: Rhs);
    }
    /// See [`std::ops::BitAndAssign`]
    pub trait BitAndAssign<Rhs = Self> {
        fn bitand_assign(&mut self, rhs: Rhs);
    }
    /// See [`std::ops::BitOrAssign`]
    pub trait BitOrAssign<Rhs = Self> {
        fn bitor_assign(&mut self, rhs: Rhs);
    }
}

pub mod control_flow {
    /// See [`std::ops::ControlFlow`]
    pub enum ControlFlow<B, C> {
        /// See [`std::ops::ControlFlow::Continue`]
        Continue(C),
        /// See [`std::ops::ControlFlow::Break`]
        Break(B),
    }
}

pub mod index {
    /// See [`std::ops::Index`]
    pub trait Index<Idx> {
        type Output: ?Sized;
        fn index(&self, i: Idx) -> &Self::Output;
    }
    /// See [`std::ops::IndexMut`]
    //
    // Lean-only. The impls delegate to the mutable slice accessors
    // (`SliceIndex::get_mut`), which model `&mut` returns; the F* backend does
    // not use those (it lowers indexed assignment to `Slice.update` /
    // `Array.update`), so `IndexMut` is excluded there.
    #[cfg(not(hax_backend_fstar))]
    pub trait IndexMut<Idx>: Index<Idx> {
        fn index_mut(&mut self, i: Idx) -> &mut Self::Output;
    }
}

pub mod function {
    /// See [`std::ops::FnOnce`]
    #[hax_lib::attributes]
    pub trait FnOnce<Args> {
        type Output;
        #[hax_lib::requires(true)]
        fn call_once(&self, args: Args) -> Self::Output;
    }

    /// See [`std::ops::Fn`]
    #[hax_lib::attributes]
    pub trait FnMut<Args>: FnOnce<Args> {
        #[hax_lib::requires(true)]
        fn call_mut(&self, args: Args) -> Self::Output;
    }

    /// See [`std::ops::Fn`]
    /* Instances of the `Fn*` classes for F* arrows (arities 1 to 3), so that a
    closure can be passed where a `Fn*` bound is expected. Hand-written rather
    than extracted from Rust impls on `fn(..) -> _`: hax emits
    `_super_i0 = FStar.Tactics.Typeclasses.solve`, which F* cannot relate to the
    arrow's return type. Writing them out also gives the post-conditions (`res == x0 x1`). */
    #[cfg_attr(
        all(not(test), hax_backend_fstar),
        hax_lib::fstar::after(
            "unfold instance fnonce_arrow_binder t u
  : t_FnOnce (_:t -> u) t = {
    f_Output = u;
    f_call_once_pre = (fun _ _ -> true);
    f_call_once_post = (fun (x0: (_:t -> u)) (x1: t) (res: u) -> res == x0 x1);
    f_call_once = (fun (x0: (_:t -> u)) (x1: t) -> x0 x1);
  }

unfold instance fnmut_arrow_binder t u
  : t_FnMut (_:t -> u) t = {
    _super_i0 = fnonce_arrow_binder t u;
    f_call_mut_pre = (fun _ _ -> true);
    f_call_mut_post = (fun (x0: (_:t -> u)) (x1: t) (res: u) -> res == x0 x1);
    f_call_mut = (fun (x0: (_:t -> u)) (x1: t) -> x0 x1);
  }

unfold instance fn_arrow_binder t u
  : t_Fn (_:t -> u) t = {
    _super_i0 = fnmut_arrow_binder t u;
    f_call_pre = (fun _ _ -> true);
    f_call_post = (fun (x0: (_:t -> u)) (x1: t) (res: u) -> res == x0 x1);
    f_call = (fun (x0: (_:t -> u)) (x1: t) -> x0 x1);
  }

unfold instance fnonce_arrow_binder2 t1 t2 u
  : t_FnOnce (t1 -> t2 -> u) (t1 & t2) = {
    f_Output = u;
    f_call_once_pre = (fun _ _ -> true);
    f_call_once_post = (fun (x0: (t1 -> t2 -> u)) (x1: (t1 & t2)) (res: u) -> res == x0 x1._1 x1._2);
    f_call_once = (fun (x0: (t1 -> t2 -> u)) (x1: (t1 & t2)) -> x0 x1._1 x1._2);
  }

unfold instance fnmut_arrow_binder2 t1 t2 u
  : t_FnMut (t1 -> t2 -> u) (t1 & t2) = {
    _super_i0 = fnonce_arrow_binder2 t1 t2 u;
    f_call_mut_pre = (fun _ _ -> true);
    f_call_mut_post = (fun (x0: (t1 -> t2 -> u)) (x1: (t1 & t2)) (res: u) -> res == x0 x1._1 x1._2);
    f_call_mut = (fun (x0: (t1 -> t2 -> u)) (x1: (t1 & t2)) -> x0 x1._1 x1._2);
  }

unfold instance fn_arrow_binder2 t1 t2 u
  : t_Fn (t1 -> t2 -> u) (t1 & t2) = {
    _super_i0 = fnmut_arrow_binder2 t1 t2 u;
    f_call_pre = (fun _ _ -> true);
    f_call_post = (fun (x0: (t1 -> t2 -> u)) (x1: (t1 & t2)) (res: u) -> res == x0 x1._1 x1._2);
    f_call = (fun (x0: (t1 -> t2 -> u)) (x1: (t1 & t2)) -> x0 x1._1 x1._2);
  }

unfold instance fnonce_arrow_binder3 t1 t2 t3 u
  : t_FnOnce (t1 -> t2 -> t3 -> u) (t1 & t2 & t3) = {
    f_Output = u;
    f_call_once_pre = (fun _ _ -> true);
    f_call_once_post = (fun (x0: (t1 -> t2 -> t3 -> u)) (x1: (t1 & t2 & t3)) (res: u) -> res == x0 x1._1 x1._2 x1._3);
    f_call_once = (fun (x0: (t1 -> t2 -> t3 -> u)) (x1: (t1 & t2 & t3)) -> x0 x1._1 x1._2 x1._3);
  }

unfold instance fnmut_arrow_binder3 t1 t2 t3 u
  : t_FnMut (t1 -> t2 -> t3 -> u) (t1 & t2 & t3) = {
    _super_i0 = fnonce_arrow_binder3 t1 t2 t3 u;
    f_call_mut_pre = (fun _ _ -> true);
    f_call_mut_post = (fun (x0: (t1 -> t2 -> t3 -> u)) (x1: (t1 & t2 & t3)) (res: u) -> res == x0 x1._1 x1._2 x1._3);
    f_call_mut = (fun (x0: (t1 -> t2 -> t3 -> u)) (x1: (t1 & t2 & t3)) -> x0 x1._1 x1._2 x1._3);
  }

unfold instance fn_arrow_binder3 t1 t2 t3 u
  : t_Fn (t1 -> t2 -> t3 -> u) (t1 & t2 & t3) = {
    _super_i0 = fnmut_arrow_binder3 t1 t2 t3 u;
    f_call_pre = (fun _ _ -> true);
    f_call_post = (fun (x0: (t1 -> t2 -> t3 -> u)) (x1: (t1 & t2 & t3)) (res: u) -> res == x0 x1._1 x1._2 x1._3);
    f_call = (fun (x0: (t1 -> t2 -> t3 -> u)) (x1: (t1 & t2 & t3)) -> x0 x1._1 x1._2 x1._3);
  }"
        )
    )]
    #[hax_lib::attributes]
    pub trait Fn<Args>: FnMut<Args> {
        #[hax_lib::requires(true)]
        fn call(&self, args: Args) -> Self::Output;
    }
}

pub mod try_trait {
    /// See [`std::ops::FromResidual`]
    pub trait FromResidual<R> {
        fn from_residual(x: R) -> Self;
    }

    /// See [`std::ops::Try`]
    pub trait Try {
        type Output;
        type Residual;
        fn from_output(x: Self::Output) -> Self;
        fn branch(self) -> super::control_flow::ControlFlow<Self::Residual, Self::Output>;
    }
}

mod deref {
    /// See [`std::ops::Deref`]
    pub trait Deref {
        type Target: ?Sized;

        fn deref(&self) -> &Self::Target;
    }

    impl<T> Deref for &T {
        type Target = T;
        fn deref(&self) -> &T {
            &self
        }
    }

    /// See [`std::ops::DerefMut`]
    // The `&mut Self::Target` return trips hax's `&mut` restriction (HAX0003,
    // pereq-dev/hax#420), so this is excluded from the F* backend; aeneas/lean/native
    // keep it. The `impl DerefMut for Vec` in alloc is guarded to match.
    #[cfg(not(hax_backend_fstar))]
    pub trait DerefMut: Deref {
        fn deref_mut(&mut self) -> &mut Self::Target;
    }
}

pub mod drop {
    /// See [`std::ops::Drop`]
    pub trait Drop {
        /// See [`std::ops::Drop::drop`]
        #[cfg(not(hax_backend_fstar))]
        fn drop(&mut self) {}
        #[cfg(hax_backend_fstar)]
        fn drop(&mut self);
    }
}

pub mod range {
    use crate::cmp::PartialOrd;
    /// See [`std::ops::RangeTo`]
    pub struct RangeTo<T> {
        pub end: T,
    }
    /// See [`std::ops::RangeFrom`]
    pub struct RangeFrom<T> {
        pub start: T,
    }
    /// See [`std::ops::Range`]
    pub struct Range<T> {
        pub start: T,
        pub end: T,
    }
    /// See [`std::ops::RangeFull`]
    pub struct RangeFull;
    /// See [`std::ops::RangeInclusive`]
    // Not `start`/`end` as in std (where they are private): Lean would then
    // name the `start`/`end` methods apart from the ones clients call.
    pub struct RangeInclusive<T> {
        pub start_: T,
        pub end_: T,
        pub exhausted: bool,
    }
    /// See [`std::ops::RangeToInclusive`]
    pub struct RangeToInclusive<T> {
        pub end: T,
    }

    macro_rules! impl_iterator_range_int {
        ($($int_type: ident)*) => {
            use crate::option::Option;
            $(
                #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
                impl crate::iter::traits::iterator::Iterator for Range<$int_type> {
                    type Item = $int_type;
                    fn next(&mut self) -> Option<$int_type> {
                        if self.start >= self.end {
                            Option::None
                        } else {
                            let res = self.start;
                            self.start += 1;
                            Option::Some(res)
                        }
                    }
                }
                // `next_back` yields from the high end: decrement `end`, yield the new
                // `end`. Makes `(lo..hi).rev()` iterate. Lean/charon backend only.
                #[cfg(not(hax_backend_fstar))]
                #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
                impl crate::iter::traits::double_ended::DoubleEndedIterator for Range<$int_type> {
                    fn next_back(&mut self) -> Option<$int_type> {
                        if self.start >= self.end {
                            Option::None
                        } else {
                            self.end -= 1;
                            Option::Some(self.end)
                        }
                    }
                }
            )*
        }
    }

    impl_iterator_range_int!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

    /// See [`std::ops::Bound`]
    pub enum Bound<T> {
        Included(T),
        Excluded(T),
        Unbounded,
    }
    /// See [`std::ops::RangeBounds`]
    #[hax_lib::attributes]
    pub trait RangeBounds<T> {
        #[hax_lib::requires(true)]
        fn start_bound(&self) -> Bound<&T>;
        #[hax_lib::requires(true)]
        fn end_bound(&self) -> Bound<&T>;
        /// See [`std::ops::RangeBounds::contains`]
        // F* has no default methods: there, `RangeBoundsDefaults` provides it.
        #[cfg(not(hax_backend_fstar))]
        #[hax_lib::requires(true)]
        fn contains<U>(&self, item: &U) -> bool
        where
            T: PartialOrd<U>,
            U: ?Sized + PartialOrd<T>,
        {
            after_start(self.start_bound(), item) && before_end(self.end_bound(), item)
        }
    }
    // Like std, `end_bound` is only called once the start check passes, via
    // `after_start(..) && before_end(..)`.
    fn after_start<T, U: ?Sized>(start: Bound<&T>, item: &U) -> bool
    where
        T: PartialOrd<U>,
    {
        match start {
            Bound::Included(start) => bound_le(start, item),
            Bound::Excluded(start) => bound_lt(start, item),
            Bound::Unbounded => true,
        }
    }
    fn before_end<T, U: ?Sized>(end: Bound<&T>, item: &U) -> bool
    where
        U: PartialOrd<T>,
    {
        match end {
            Bound::Included(end) => bound_le(item, end),
            Bound::Excluded(end) => bound_lt(item, end),
            Bound::Unbounded => true,
        }
    }
    // std compares with `<=` and `<`, so a type's own `le` and `lt` are used.
    #[cfg(not(hax_backend_fstar))]
    fn bound_le<A: ?Sized + PartialOrd<B>, B: ?Sized>(a: &A, b: &B) -> bool {
        a.le(b)
    }
    #[cfg(not(hax_backend_fstar))]
    fn bound_lt<A: ?Sized + PartialOrd<B>, B: ?Sized>(a: &A, b: &B) -> bool {
        a.lt(b)
    }
    // F* has no `le`/`lt` between two different types: these ignore overrides.
    #[cfg(hax_backend_fstar)]
    fn bound_le<A: ?Sized + PartialOrd<B>, B: ?Sized>(a: &A, b: &B) -> bool {
        matches!(
            a.partial_cmp(b),
            Option::Some(crate::cmp::Ordering::Less | crate::cmp::Ordering::Equal)
        )
    }
    #[cfg(hax_backend_fstar)]
    fn bound_lt<A: ?Sized + PartialOrd<B>, B: ?Sized>(a: &A, b: &B) -> bool {
        matches!(a.partial_cmp(b), Option::Some(crate::cmp::Ordering::Less))
    }
    impl<T> RangeBounds<T> for RangeFull {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Unbounded
        }
        fn end_bound(&self) -> Bound<&T> {
            Bound::Unbounded
        }
    }
    impl<T> RangeBounds<T> for RangeFrom<T> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Included(&self.start)
        }
        fn end_bound(&self) -> Bound<&T> {
            Bound::Unbounded
        }
    }
    impl<T: PartialOrd<T>> Range<T> {
        /// See [`std::ops::Range::contains`]
        pub fn contains<U>(&self, item: &U) -> bool
        where
            T: PartialOrd<U>,
            U: ?Sized + PartialOrd<T>,
        {
            bound_le(&self.start, item) && bound_lt(item, &self.end)
        }
        /// See [`std::ops::Range::is_empty`]
        pub fn is_empty(&self) -> bool
        where
            T: PartialOrd<T>,
        {
            !bound_lt(&self.start, &self.end)
        }
    }
    impl<T> RangeBounds<T> for RangeTo<T> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Unbounded
        }
        fn end_bound(&self) -> Bound<&T> {
            Bound::Excluded(&self.end)
        }
    }
    impl<T: PartialOrd<T>> RangeFrom<T> {
        /// See [`std::ops::RangeFrom::contains`]
        pub fn contains<U>(&self, item: &U) -> bool
        where
            T: PartialOrd<U>,
            U: ?Sized + PartialOrd<T>,
        {
            bound_le(&self.start, item)
        }
    }
    impl<T> RangeBounds<T> for Range<T> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Included(&self.start)
        }
        fn end_bound(&self) -> Bound<&T> {
            Bound::Excluded(&self.end)
        }
    }
    impl<T: PartialOrd<T>> RangeTo<T> {
        /// See [`std::ops::RangeTo::contains`]
        pub fn contains<U>(&self, item: &U) -> bool
        where
            T: PartialOrd<U>,
            U: ?Sized + PartialOrd<T>,
        {
            bound_lt(item, &self.end)
        }
    }
    impl<T> RangeInclusive<T> {
        /// See [`std::ops::RangeInclusive::new`]
        pub fn new(start: T, end: T) -> Self {
            RangeInclusive {
                start_: start,
                end_: end,
                exhausted: false,
            }
        }
        /// See [`std::ops::RangeInclusive::start`]
        pub fn start(&self) -> &T {
            &self.start_
        }
        /// See [`std::ops::RangeInclusive::end`]
        pub fn end(&self) -> &T {
            &self.end_
        }
        /// See [`std::ops::RangeInclusive::into_inner`]
        pub fn into_inner(self) -> (T, T) {
            (self.start_, self.end_)
        }
    }
    // Stand-in for real core's `impl RangeInclusive<usize>`.
    impl RangeInclusive<usize> {}
    // std may leave any bounds once exhausted: `start..start` is always empty.
    impl<T> RangeBounds<T> for RangeInclusive<T> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Included(&self.start_)
        }
        fn end_bound(&self) -> Bound<&T> {
            if self.exhausted {
                Bound::Excluded(&self.start_)
            } else {
                Bound::Included(&self.end_)
            }
        }
    }
    impl<T: PartialOrd<T>> RangeInclusive<T> {
        /// See [`std::ops::RangeInclusive::contains`]
        pub fn contains<U>(&self, item: &U) -> bool
        where
            T: PartialOrd<U>,
            U: ?Sized + PartialOrd<T>,
        {
            after_start(self.start_bound(), item) && before_end(self.end_bound(), item)
        }
        /// See [`std::ops::RangeInclusive::is_empty`]
        // The bound repeats the impl's, as in core: clients pass both.
        pub fn is_empty(&self) -> bool
        where
            T: PartialOrd<T>,
        {
            self.exhausted || !bound_le(&self.start_, &self.end_)
        }
    }
    impl<T> RangeBounds<T> for RangeToInclusive<T> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Unbounded
        }
        fn end_bound(&self) -> Bound<&T> {
            Bound::Included(&self.end)
        }
    }
    impl<T: PartialOrd<T>> RangeToInclusive<T> {
        /// See [`std::ops::RangeToInclusive::contains`]
        pub fn contains<U>(&self, item: &U) -> bool
        where
            T: PartialOrd<U>,
            U: ?Sized + PartialOrd<T>,
        {
            bound_le(item, &self.end)
        }
    }
    impl<T> RangeBounds<T> for (Bound<T>, Bound<T>) {
        fn start_bound(&self) -> Bound<&T> {
            bound_as_ref(&self.0)
        }
        fn end_bound(&self) -> Bound<&T> {
            bound_as_ref(&self.1)
        }
    }
    fn bound_as_ref<T>(bound: &Bound<T>) -> Bound<&T> {
        match bound {
            Bound::Included(x) => Bound::Included(x),
            Bound::Excluded(x) => Bound::Excluded(x),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    // `RangeBounds::contains` for F*, where a trait cannot provide it: this
    // blanket impl gives it to every `RangeBounds`, including clients' own.
    // Last in the module, so that it does not shift the `impl_N` names above.
    #[cfg(any(hax_backend_fstar, test))]
    #[hax_lib::attributes]
    pub(crate) trait RangeBoundsDefaults<T> {
        #[hax_lib::requires(true)]
        fn contains<U>(&self, item: &U) -> bool
        where
            Self: RangeBounds<T>,
            T: PartialOrd<U>,
            U: ?Sized + PartialOrd<T>;
    }
    #[cfg(any(hax_backend_fstar, test))]
    impl<T, R> RangeBoundsDefaults<T> for R {
        fn contains<U>(&self, item: &U) -> bool
        where
            Self: RangeBounds<T>,
            T: PartialOrd<U>,
            U: ?Sized + PartialOrd<T>,
        {
            after_start(self.start_bound(), item) && before_end(self.end_bound(), item)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::Inject;
    use pastey::paste;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_range_inclusive_new(start in any::<u8>(), end in any::<u8>()) {
            let model = super::range::RangeInclusive::new(start, end);
            let std_range = start..=end;
            prop_assert_eq!(*model.start(), *std_range.start());
            prop_assert_eq!(*model.end(), *std_range.end());
            prop_assert_eq!(model.into_inner(), std_range.into_inner());
        }

        #[test]
        fn test_range_inclusive_contains(
            start in any::<u8>(),
            end in any::<u8>(),
            same in any::<bool>(),
            item in any::<u8>(),
        ) {
            // Independent draws are almost never equal: that edge is `same`.
            let end = if same { start } else { end };
            let model = super::range::RangeInclusive::new(start, end);
            let std_range = start..=end;
            prop_assert_eq!(model.contains(&item), std_range.contains(&item));
            prop_assert_eq!(model.is_empty(), std_range.is_empty());
            if start <= end {
                let mut std_range = std_range;
                std_range.nth((end - start) as usize);
                let iterated = std_range.inject();
                prop_assert_eq!(iterated.contains(&item), std_range.contains(&item));
                prop_assert_eq!(iterated.is_empty(), std_range.is_empty());
            }
        }

        #[test]
        fn test_range_contains(
            start in any::<u8>(),
            end in any::<u8>(),
            same in any::<bool>(),
            item in any::<u8>(),
        ) {
            // Independent draws are almost never equal: that edge is `same`.
            let end = if same { start } else { end };
            let model = super::range::Range { start, end };
            let std_range = start..end;
            prop_assert_eq!(model.contains(&item), std_range.contains(&item));
            prop_assert_eq!(model.is_empty(), std_range.is_empty());
        }

        #[test]
        fn test_one_sided_range_contains(
            bound in any::<u8>(),
            same in any::<bool>(),
            item in any::<u8>(),
        ) {
            // Independent draws are almost never equal: that edge is `same`.
            let item = if same { bound } else { item };
            let from = super::range::RangeFrom { start: bound };
            prop_assert_eq!(from.contains(&item), (bound..).contains(&item));
            let to = super::range::RangeTo { end: bound };
            prop_assert_eq!(to.contains(&item), (..bound).contains(&item));
            let to_inclusive = super::range::RangeToInclusive { end: bound };
            prop_assert_eq!(to_inclusive.contains(&item), (..=bound).contains(&item));
        }

        #[test]
        fn test_range_inclusive_exhausted(
            start in any::<u8>(),
            end in any::<u8>(),
            item in any::<u8>(),
        ) {
            // Whatever bounds std leaves, an exhausted range holds nothing.
            let model = super::range::RangeInclusive { start_: start, end_: end, exhausted: true };
            prop_assert!(!model.contains(&item));
            prop_assert!(model.is_empty());
            // Iterated up to the integer bounds, where some std versions wrap.
            let mut forward = start..=u8::MAX;
            forward.nth((u8::MAX - start) as usize);
            let mut backward = u8::MIN..=end;
            backward.nth_back(end as usize);
            for std_range in [forward, backward] {
                let model = std_range.inject();
                prop_assert_eq!(model.contains(&item), std_range.contains(&item));
                prop_assert_eq!(model.is_empty(), std_range.is_empty());
            }
        }

        #[test]
        fn test_range_bounds_contains(
            start in crate::testing::range_endpoint(),
            end in crate::testing::range_endpoint(),
            item in crate::testing::range_endpoint(),
        ) {
            use std::ops::RangeBounds as _;
            for (model, real) in crate::testing::range_forms(start, end) {
                let expected = real.contains(&item);
                #[cfg(not(hax_backend_fstar))]
                prop_assert_eq!(
                    crate::ops::range::RangeBounds::<usize>::contains(&model, &item),
                    expected
                );
                prop_assert_eq!(
                    crate::ops::range::RangeBoundsDefaults::<usize>::contains(&model, &item),
                    expected
                );
            }
        }
    }

    /// std does not compare against the end once the start rules the item out.
    #[test]
    fn test_contains_skips_end_below_start() {
        use crate::ops::range::{Bound, RangeBoundsDefaults, RangeInclusive};
        use crate::testing::Tripwire;
        let item = Tripwire(1);
        assert!(!(Tripwire(2)..=Tripwire(u8::MAX)).contains(&item));
        let model = RangeInclusive::new(Tripwire(2), Tripwire(u8::MAX));
        assert!(!model.contains(&item));
        assert!(!(Tripwire(2)..Tripwire(u8::MAX)).contains(&item));
        let model = crate::ops::range::Range {
            start: Tripwire(2),
            end: Tripwire(u8::MAX),
        };
        assert!(!model.contains(&item));
        let model = (
            Bound::Excluded(Tripwire(1)),
            Bound::Included(Tripwire(u8::MAX)),
        );
        #[cfg(not(hax_backend_fstar))]
        assert!(!crate::ops::range::RangeBounds::contains(&model, &item));
        assert!(!RangeBoundsDefaults::contains(&model, &item));
    }

    /// std does not call `end_bound` once the start rules the item out.
    #[test]
    fn test_contains_skips_end_bound_below_start() {
        use crate::ops::range::{Bound, RangeBounds, RangeBoundsDefaults};
        struct PanickyEnd(u8);
        impl std::ops::RangeBounds<u8> for PanickyEnd {
            fn start_bound(&self) -> std::ops::Bound<&u8> {
                std::ops::Bound::Included(&self.0)
            }
            // Never called: the test checks exactly that.
            #[cfg_attr(coverage_nightly, coverage(off))]
            fn end_bound(&self) -> std::ops::Bound<&u8> {
                panic!("end_bound called")
            }
        }
        impl RangeBounds<u8> for PanickyEnd {
            fn start_bound(&self) -> Bound<&u8> {
                Bound::Included(&self.0)
            }
            // Never called: the test checks exactly that.
            #[cfg_attr(coverage_nightly, coverage(off))]
            fn end_bound(&self) -> Bound<&u8> {
                panic!("end_bound called")
            }
        }
        assert!(!std::ops::RangeBounds::contains(&PanickyEnd(2), &1));
        #[cfg(not(hax_backend_fstar))]
        assert!(!RangeBounds::contains(&PanickyEnd(2), &1));
        assert!(!RangeBoundsDefaults::contains(&PanickyEnd(2), &1));
    }

    // F* compares with `partial_cmp`, having no `le`/`lt` to override.
    #[cfg(not(hax_backend_fstar))]
    mod contains_uses_le_and_lt {
        /// Incomparable, yet `le` holds and `lt` does not: std goes by the latter.
        struct Skewed;
        impl std::cmp::PartialEq for Skewed {
            fn eq(&self, _: &Self) -> bool {
                false
            }
        }
        impl std::cmp::PartialOrd for Skewed {
            fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
                None
            }
            fn le(&self, _: &Self) -> bool {
                true
            }
            fn lt(&self, _: &Self) -> bool {
                false
            }
        }
        impl crate::cmp::PartialEq<Skewed> for Skewed {
            fn eq(&self, _: &Self) -> bool {
                false
            }
        }
        impl crate::cmp::PartialOrd<Skewed> for Skewed {
            fn partial_cmp(&self, _: &Self) -> crate::option::Option<crate::cmp::Ordering> {
                crate::option::Option::None
            }
            fn le(&self, _: &Self) -> bool {
                true
            }
            fn lt(&self, _: &Self) -> bool {
                false
            }
        }

        #[test]
        fn test_contains_uses_le_and_lt() {
            use crate::ops::range::{
                Bound, Range, RangeBounds, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive,
            };
            assert!(Skewed.partial_cmp(&Skewed).is_none());
            assert!(Skewed != Skewed);
            assert!(crate::cmp::PartialOrd::partial_cmp(&Skewed, &Skewed).is_none());
            assert!(!crate::cmp::PartialEq::eq(&Skewed, &Skewed));
            assert!((Skewed..=Skewed).contains(&Skewed));
            assert!(RangeInclusive::new(Skewed, Skewed).contains(&Skewed));
            assert!(!(Skewed..Skewed).contains(&Skewed));
            assert!((Skewed..).contains(&Skewed));
            assert!(RangeFrom { start: Skewed }.contains(&Skewed));
            assert!(!(..Skewed).contains(&Skewed));
            assert!(!RangeTo { end: Skewed }.contains(&Skewed));
            assert!((..=Skewed).contains(&Skewed));
            assert!(RangeToInclusive { end: Skewed }.contains(&Skewed));
            let range = Range {
                start: Skewed,
                end: Skewed,
            };
            assert!(!range.contains(&Skewed));
            assert!((Skewed..Skewed).is_empty());
            assert!(range.is_empty());
            let model = (Bound::Included(Skewed), Bound::Excluded(Skewed));
            assert!(!RangeBounds::contains(&model, &Skewed));
            assert!(!(Skewed..=Skewed).is_empty());
            assert!(!RangeInclusive::new(Skewed, Skewed).is_empty());
        }
    }

    // `int_trait_impls!` covers u8..u64. The `requires` rules out wrapping, so
    // the domain is every non-overflowing pair, edges included.
    macro_rules! assign_test {
        ($($t:ty)*) => {
            paste! {
                $(
                    proptest! {
                        #[test]
                        fn [<test_ $t _add_assign>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assume!(x.checked_add(y).is_some());
                            let mut model = x.inject();
                            super::arith::AddAssign::add_assign(&mut model, y.inject());
                            let mut std_value = x;
                            std::ops::AddAssign::add_assign(&mut std_value, y);
                            prop_assert_eq!(model, std_value);
                        }

                        #[test]
                        fn [<test_ $t _sub_assign>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assume!(x.checked_sub(y).is_some());
                            let mut model = x.inject();
                            super::arith::SubAssign::sub_assign(&mut model, y.inject());
                            let mut std_value = x;
                            std::ops::SubAssign::sub_assign(&mut std_value, y);
                            prop_assert_eq!(model, std_value);
                        }

                        #[test]
                        fn [<test_ $t _add_assign_at_max>](x in any::<$t>()) {
                            let y = <$t>::MAX - x;
                            let mut model = x.inject();
                            super::arith::AddAssign::add_assign(&mut model, y.inject());
                            let mut std_value = x;
                            std::ops::AddAssign::add_assign(&mut std_value, y);
                            prop_assert_eq!(model, std_value);
                        }
                    }
                )*
            }
        }
    }

    assign_test! { u8 u16 u32 u64 }

    macro_rules! range_iter_test {
        ($($t:ident)*) => {
            paste! {
                $(
                    proptest! {
                        // `len` is kept small and added saturatingly so the range
                        // stays inside `$t` for every type.
                        #[test]
                        fn [<test_ $t _range_iter>](start in any::<$t>(), len in 0u8..=20) {
                            let end = start.saturating_add(len as $t);
                            let mut model = super::range::Range { start, end };
                            let mut collected = std::vec::Vec::new();
                            while let crate::option::Option::Some(x) =
                                crate::iter::traits::iterator::Iterator::next(&mut model)
                            {
                                collected.push(x);
                            }
                            prop_assert_eq!(collected, (start..end).collect::<std::vec::Vec<$t>>());
                        }
                    }
                )*
            }
        }
    }

    range_iter_test! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

    proptest! {
        #[test]
        fn test_deref_ref(x in any::<u8>()) {
            let r = &x;
            prop_assert_eq!(
                *super::deref::Deref::deref(&r),
                *core::ops::Deref::deref(&r)
            );
        }
    }

    /// `Drop::drop`'s provided body. Real `core` has no default there (the
    /// method is required), so there is nothing to compare against: all the
    /// model's body does is leave the receiver alone.
    #[cfg(not(hax_backend_fstar))]
    #[test]
    fn test_drop_default() {
        struct Guard(u8);
        impl crate::ops::drop::Drop for Guard {}

        let mut g = Guard(7);
        crate::ops::drop::Drop::drop(&mut g);
        assert_eq!(g.0, 7);
    }
}
