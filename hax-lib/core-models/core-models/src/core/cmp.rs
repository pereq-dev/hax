use crate::option::Option;

/// See [`std::cmp::PartialEq`]
#[hax_lib::attributes]
pub trait PartialEq<Rhs>
where
    Rhs: ?Sized,
{
    /// See [`std::cmp::PartialEq::eq`]
    #[hax_lib::requires(true)]
    fn eq(&self, other: &Rhs) -> bool;

    /// See [`std::cmp::PartialEq::ne`]. Provided method: the aeneas Lean backend
    /// synthesises `#[derive(PartialEq)]` instances as `{ eq := …, ne := …PartialEq.ne.default … }`,
    /// so the model must carry `ne` or those instances fail with "`ne` is not a field of
    /// `core.cmp.PartialEq`". Modelled exactly like the `PartialOrd::{lt,le,gt,ge}` defaults
    /// below (cfg-guarded off F*; `== false` avoids negation for the F* lib).
    #[cfg(not(hax_backend_fstar))]
    #[hax_lib::requires(true)]
    fn ne(&self, other: &Rhs) -> bool {
        self.eq(other) == false
    }
}

/// See [`std::cmp::Eq`]
pub trait Eq: PartialEq<Self> {}

/// See [`std::cmp::Ordering`]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Ordering {
    /// See [`std::cmp::Ordering::Less`]
    Less = -1,
    /// See [`std::cmp::Ordering::Equal`]
    Equal = 0,
    /// See [`std::cmp::Ordering::Greater`]
    Greater = 1,
}

/// See [`std::fmt::Debug`] for [`Ordering`]
#[cfg(not(hax_backend_fstar))]
impl crate::fmt::Debug for Ordering {
    fn fmt(&self, f: &mut crate::fmt::Formatter) -> crate::fmt::Result {
        crate::fmt::Result::Ok(())
    }
}

/// See [`std::cmp::PartialOrd`]
#[hax_lib::attributes]
pub trait PartialOrd<Rhs>: PartialEq<Rhs>
where
    Rhs: ?Sized,
{
    /// See [`std::cmp::PartialOrd::partial_cmp`]
    #[hax_lib::requires(true)]
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    // hax/F* does not support default methods. We work around that using the `PartialOrdDefaults`
    // trait below.
    #[cfg(not(hax_backend_fstar))]
    #[hax_lib::requires(true)]
    fn lt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Option::Some(Ordering::Less))
    }
    #[cfg(not(hax_backend_fstar))]
    #[hax_lib::requires(true)]
    fn le(&self, other: &Rhs) -> bool {
        matches!(
            self.partial_cmp(other),
            Option::Some(Ordering::Less | Ordering::Equal)
        )
    }
    #[cfg(not(hax_backend_fstar))]
    #[hax_lib::requires(true)]
    fn gt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Option::Some(Ordering::Greater))
    }
    #[cfg(not(hax_backend_fstar))]
    #[hax_lib::requires(true)]
    fn ge(&self, other: &Rhs) -> bool {
        matches!(
            self.partial_cmp(other),
            Option::Some(Ordering::Greater | Ordering::Equal)
        )
    }
}

#[hax_lib::attributes]
trait Neq<Rhs> {
    #[hax_lib::requires(true)]
    fn neq(&self, y: &Rhs) -> bool;
}

impl<T: PartialEq<T>> Neq<T> for T {
    fn neq(&self, y: &T) -> bool {
        // Not using negation is a workaround for the F* lib
        self.eq(y) == false
    }
}

// These methods in core are provided using trait defaults, but this is not supported by hax/F*
// so we have to define them in a different way.
#[cfg(any(hax_backend_fstar, test))]
#[hax_lib::attributes]
trait PartialOrdDefaults<Rhs> {
    #[hax_lib::requires(true)]
    fn lt(&self, y: &Rhs) -> bool
    where
        Self: PartialOrd<Rhs>;
    #[hax_lib::requires(true)]
    fn le(&self, y: &Rhs) -> bool
    where
        Self: PartialOrd<Rhs>;
    #[hax_lib::requires(true)]
    fn gt(&self, y: &Rhs) -> bool
    where
        Self: PartialOrd<Rhs>;
    #[hax_lib::requires(true)]
    fn ge(&self, y: &Rhs) -> bool
    where
        Self: PartialOrd<Rhs>;
}

#[cfg(any(hax_backend_fstar, test))]
impl<T: PartialOrd<T>> PartialOrdDefaults<T> for T {
    fn lt(&self, y: &T) -> bool
    where
        T: PartialOrd<T>,
    {
        matches!(self.partial_cmp(y), Option::Some(Ordering::Less))
    }
    fn le(&self, y: &T) -> bool
    where
        T: PartialOrd<T>,
    {
        matches!(
            self.partial_cmp(y),
            Option::Some(Ordering::Less | Ordering::Equal)
        )
    }
    fn gt(&self, y: &T) -> bool
    where
        T: PartialOrd<T>,
    {
        matches!(self.partial_cmp(y), Option::Some(Ordering::Greater))
    }
    fn ge(&self, y: &T) -> bool
    where
        T: PartialOrd<T>,
    {
        matches!(
            self.partial_cmp(y),
            Option::Some(Ordering::Greater | Ordering::Equal)
        )
    }
}

/// See [`std::cmp::Ord`]
#[hax_lib::attributes]
pub trait Ord: Eq + PartialOrd<Self> {
    /// See [`std::cmp::Ord::cmp`]
    #[hax_lib::requires(true)]
    fn cmp(&self, other: &Self) -> Ordering;

    // `max`, `min` and `clamp` compare through `PartialOrd`, as std's do; an
    // impl may make it disagree with `cmp`.
    /// See [`std::cmp::Ord::max`]
    #[cfg(not(hax_backend_fstar))]
    #[hax_lib::requires(true)]
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if PartialOrd::lt(&other, &self) {
            self
        } else {
            other
        }
    }
    /// See [`std::cmp::Ord::min`]
    #[cfg(not(hax_backend_fstar))]
    #[hax_lib::requires(true)]
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if PartialOrd::lt(&other, &self) {
            other
        } else {
            self
        }
    }
    /// See [`std::cmp::Ord::clamp`]
    #[cfg(not(hax_backend_fstar))]
    #[hax_lib::requires(true)]
    fn clamp(self, lo: Self, hi: Self) -> Self
    where
        Self: Sized,
    {
        if !PartialOrd::le(&lo, &hi) {
            crate::panicking::internal::panic()
        }
        if PartialOrd::lt(&self, &lo) {
            lo
        } else if PartialOrd::gt(&self, &hi) {
            hi
        } else {
            self
        }
    }
}

/// See [`std::cmp::max`]
#[cfg(not(hax_backend_fstar))]
pub fn max<T: Ord>(v1: T, v2: T) -> T {
    Ord::max(v1, v2)
}

/// See [`std::cmp::min`]
#[cfg(not(hax_backend_fstar))]
pub fn min<T: Ord>(v1: T, v2: T) -> T {
    Ord::min(v1, v2)
}

/// See [`std::cmp::max`]
#[cfg(hax_backend_fstar)]
pub fn max<T: Ord>(v1: T, v2: T) -> T {
    match v1.cmp(&v2) {
        Ordering::Greater => v1,
        _ => v2,
    }
}

/// See [`std::cmp::min`]
#[cfg(hax_backend_fstar)]
pub fn min<T: Ord>(v1: T, v2: T) -> T {
    match v1.cmp(&v2) {
        Ordering::Greater => v2,
        _ => v1,
    }
}

/// See [`std::cmp::Reverse`]
pub struct Reverse<T>(pub T);

impl<T: PartialOrd<T>> PartialOrd<Reverse<T>> for Reverse<T> {
    fn partial_cmp(&self, other: &Reverse<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: PartialEq<T>> PartialEq<Reverse<T>> for Reverse<T> {
    #[cfg(not(hax_backend_fstar))]
    fn ne(&self, other: &Reverse<T>) -> bool {
        self.eq(other) == false
    }
    fn eq(&self, other: &Reverse<T>) -> bool {
        other.0.eq(&self.0)
    }
}

impl<T: Eq> Eq for Reverse<T> {}

impl<T: Ord> Ord for Reverse<T> {
    fn cmp(&self, other: &Reverse<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

macro_rules! int_impls {
    ($($t:ty)*) => ($(
        #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
        #[hax_lib::attributes]
        #[cfg_attr(hax_backend_lean, hax_lib::exclude)]
        impl PartialOrd<$t> for $t {
            #[hax_lib::ensures(|res| {
                match res {
                    Option::Some(Ordering::Less) => self < other,
                    Option::Some(Ordering::Equal) => self == other,
                    Option::Some(Ordering::Greater) => self > other,
                    Option::None => false
                }
            })]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                if self < other {Option::Some(Ordering::Less)}
                else if self > other {Option::Some(Ordering::Greater)}
                else {Option::Some(Ordering::Equal)}
            }
        }
        #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
        #[hax_lib::attributes]
        #[cfg_attr(hax_backend_lean, hax_lib::exclude)]
        impl Ord for $t {
            #[hax_lib::ensures(|res| {
                match res {
                    Ordering::Less => self < other,
                    Ordering::Equal => self == other,
                    Ordering::Greater => self > other,
                }
            })]
            fn cmp(&self, other: &Self) -> Ordering {
                if self < other {Ordering::Less}
                else if self > other {Ordering::Greater}
                else {Ordering::Equal}
            }
        }
        #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
        #[cfg_attr(hax_backend_lean, hax_lib::exclude)]
        impl PartialEq<$t> for $t {
            fn eq(&self, other: &Self) -> bool {
                self == other
            }
        }
        #[cfg_attr(hax_backend_lean, hax_lib::exclude)]
        #[cfg_attr(hax_backend_legacy_lean, hax_lib::exclude)]
        impl Eq for $t {}
    )*)
}

int_impls! { u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize }

#[hax_lib::attributes]
impl Ordering {
    /// See [`std::cmp::Ordering::is_eq`]
    pub fn is_eq(self) -> bool {
        matches!(self, Ordering::Equal)
    }
    /// See [`std::cmp::Ordering::is_ne`]
    pub fn is_ne(self) -> bool {
        matches!(self, Ordering::Less | Ordering::Greater)
    }
    /// See [`std::cmp::Ordering::is_lt`]
    pub fn is_lt(self) -> bool {
        matches!(self, Ordering::Less)
    }
    /// See [`std::cmp::Ordering::is_gt`]
    pub fn is_gt(self) -> bool {
        matches!(self, Ordering::Greater)
    }
    /// See [`std::cmp::Ordering::is_le`]
    pub fn is_le(self) -> bool {
        matches!(self, Ordering::Less | Ordering::Equal)
    }
    /// See [`std::cmp::Ordering::is_ge`]
    pub fn is_ge(self) -> bool {
        matches!(self, Ordering::Greater | Ordering::Equal)
    }
    /// See [`std::cmp::Ordering::reverse`]
    pub fn reverse(self) -> Ordering {
        match self {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
    /// See [`std::cmp::Ordering::then`]
    pub fn then(self, other: Ordering) -> Ordering {
        match self {
            Ordering::Equal => other,
            _ => self,
        }
    }
    /// See [`std::cmp::Ordering::then_with`]
    pub fn then_with<F: FnOnce() -> Ordering>(self, f: F) -> Ordering {
        match self {
            Ordering::Equal => f(),
            _ => self,
        }
    }
}

/// See [`std::cmp::clamp`]
#[hax_lib::requires(min.cmp(&max).is_le())]
pub fn clamp<T: Ord>(value: T, min: T, max: T) -> T {
    if !min.cmp(&max).is_le() {
        crate::panicking::internal::panic()
    }
    match value.cmp(&min) {
        Ordering::Less => min,
        Ordering::Equal => value,
        Ordering::Greater => match value.cmp(&max) {
            Ordering::Greater => max,
            _ => value,
        },
    }
}

// `PartialEq`/`Eq`/`PartialOrd`/`Ord` for the unit type, which real `core`
// spells out in `cmp::impls`.

/// See [`std::cmp::PartialEq`] for `()`
impl PartialEq<()> for () {
    fn eq(&self, other: &()) -> bool {
        true
    }
    #[cfg(not(hax_backend_fstar))]
    fn ne(&self, other: &()) -> bool {
        false
    }
}

/// See [`std::cmp::Eq`] for `()`
impl Eq for () {}

/// See [`std::cmp::PartialOrd`] for `()`
impl PartialOrd<()> for () {
    fn partial_cmp(&self, other: &()) -> Option<Ordering> {
        Option::Some(Ordering::Equal)
    }
}

/// See [`std::cmp::Ord`] for `()`
impl Ord for () {
    fn cmp(&self, other: &()) -> Ordering {
        Ordering::Equal
    }
}

/// See [`std::cmp::PartialEq`] for [`Ordering`]
#[cfg(not(hax_backend_fstar))]
impl PartialEq<Ordering> for Ordering {
    fn eq(&self, other: &Ordering) -> bool {
        match (self, other) {
            (Ordering::Less, Ordering::Less) => true,
            (Ordering::Equal, Ordering::Equal) => true,
            (Ordering::Greater, Ordering::Greater) => true,
            _ => false,
        }
    }
}

/// See [`std::cmp::Eq`] for [`Ordering`]
#[cfg(not(hax_backend_fstar))]
impl Eq for Ordering {}

#[cfg(test)]
mod tests {
    use super::{Ord, Ordering, PartialEq, PartialOrd};
    use crate::testing::Inject;
    use proptest::prelude::*;

    /// `Debug` for `Ordering` renders nothing, like every other `Debug` in the
    /// model.
    #[cfg(not(hax_backend_fstar))]
    #[test]
    fn test_ordering_debug() {
        let mut f = crate::fmt::Formatter;
        for o in [Ordering::Less, Ordering::Equal, Ordering::Greater] {
            assert!(crate::fmt::Debug::fmt(&o, &mut f).is_ok());
        }
    }

    #[cfg(not(hax_backend_fstar))]
    mod skew {
        use super::super::Ordering;

        /// `PartialOrd` reversed against `Ord`: `max`, `min` and `clamp` must
        /// follow the former.
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub(super) struct Skew(pub(super) u8);
        impl std::cmp::PartialOrd for Skew {
            fn partial_cmp(&self, other: &Skew) -> Option<std::cmp::Ordering> {
                Some(std::cmp::Ord::cmp(&other.0, &self.0))
            }
        }
        impl std::cmp::Ord for Skew {
            fn cmp(&self, other: &Skew) -> std::cmp::Ordering {
                std::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        impl super::super::PartialEq<Skew> for Skew {
            fn eq(&self, other: &Skew) -> bool {
                self.0 == other.0
            }
        }
        impl super::super::Eq for Skew {}
        impl super::super::PartialOrd<Skew> for Skew {
            fn partial_cmp(&self, other: &Skew) -> crate::option::Option<Ordering> {
                crate::option::Option::Some(super::super::Ord::cmp(&other.0, &self.0))
            }
        }
        impl super::super::Ord for Skew {
            fn cmp(&self, other: &Skew) -> Ordering {
                super::super::Ord::cmp(&self.0, &other.0)
            }
        }
    }
    #[cfg(not(hax_backend_fstar))]
    use skew::Skew;

    #[cfg(not(hax_backend_fstar))]
    #[test]
    #[should_panic]
    fn test_ord_clamp_default_panics() {
        super::Ord::clamp(1u8, 3, 2);
    }

    proptest! {
        // Ints don't override `max`/`min`/`clamp`, so these exercise the
        // trait's defaults.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_ord_defaults(x in any::<u8>(), y in any::<u8>(), z in any::<u8>()) {
            let (lo, hi) = (std::cmp::min(y, z), std::cmp::max(y, z));
            prop_assert_eq!(super::Ord::max(x, y), std::cmp::Ord::max(x, y));
            prop_assert_eq!(super::Ord::min(x, y), std::cmp::Ord::min(x, y));
            prop_assert_eq!(super::Ord::clamp(x, lo, hi), std::cmp::Ord::clamp(x, lo, hi));
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_ord_defaults_skewed(x in any::<u8>(), y in any::<u8>(), z in any::<u8>()) {
            let (lo, hi) = (std::cmp::max(y, z), std::cmp::min(y, z));
            prop_assert_eq!(
                super::Ord::clamp(Skew(x), Skew(lo), Skew(hi)),
                std::cmp::Ord::clamp(Skew(x), Skew(lo), Skew(hi))
            );
            prop_assert_eq!(super::Ord::max(Skew(x), Skew(y)), std::cmp::Ord::max(Skew(x), Skew(y)));
            prop_assert_eq!(super::Ord::min(Skew(x), Skew(y)), std::cmp::Ord::min(Skew(x), Skew(y)));
            prop_assert_eq!(super::max(Skew(x), Skew(y)), std::cmp::max(Skew(x), Skew(y)));
            prop_assert_eq!(super::min(Skew(x), Skew(y)), std::cmp::min(Skew(x), Skew(y)));
            // The two sides' `Ord` and model `PartialEq` agree with `u8`'s.
            prop_assert_eq!(std::cmp::Ord::cmp(&Skew(x), &Skew(y)), std::cmp::Ord::cmp(&x, &y));
            prop_assert_eq!(super::Ord::cmp(&Skew(x), &Skew(y)), std::cmp::Ord::cmp(&x, &y).inject());
            prop_assert_eq!(super::PartialEq::eq(&Skew(x), &Skew(y)), x == y);
        }

        // Ints don't override `ne`, so this exercises the trait's default.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_partial_eq_ne_default(x in 0u8..4, y in 0u8..4) {
            prop_assert_eq!(PartialEq::ne(&x.inject(), &y.inject()), x != y);
        }

        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_reverse_ne(x in 0u8..4, y in 0u8..4) {
            let a = std::cmp::Reverse(x);
            let b = std::cmp::Reverse(y);
            prop_assert_eq!(PartialEq::ne(&a.inject(), &b.inject()), a != b);
        }

        #[test]
        fn test_ordering_is_eq(x in any::<u8>(), y in any::<u8>()) {
            let model_ord = <u8 as Ord>::cmp(&x.inject(), &y.inject());
            let std_ord = std::cmp::Ord::cmp(&x, &y);
            prop_assert_eq!(model_ord.is_eq(), std_ord.is_eq());
        }

        #[test]
        fn test_ordering_is_ne(x in any::<u8>(), y in any::<u8>()) {
            let model_ord = <u8 as Ord>::cmp(&x.inject(), &y.inject());
            let std_ord = std::cmp::Ord::cmp(&x, &y);
            prop_assert_eq!(model_ord.is_ne(), std_ord.is_ne());
        }

        #[test]
        fn test_ordering_is_lt(x in any::<u8>(), y in any::<u8>()) {
            let model_ord = <u8 as Ord>::cmp(&x.inject(), &y.inject());
            let std_ord = std::cmp::Ord::cmp(&x, &y);
            prop_assert_eq!(model_ord.is_lt(), std_ord.is_lt());
        }

        #[test]
        fn test_ordering_is_gt(x in any::<u8>(), y in any::<u8>()) {
            let model_ord = <u8 as Ord>::cmp(&x.inject(), &y.inject());
            let std_ord = std::cmp::Ord::cmp(&x, &y);
            prop_assert_eq!(model_ord.is_gt(), std_ord.is_gt());
        }

        #[test]
        fn test_ordering_is_le(x in any::<u8>(), y in any::<u8>()) {
            let model_ord = <u8 as Ord>::cmp(&x.inject(), &y.inject());
            let std_ord = std::cmp::Ord::cmp(&x, &y);
            prop_assert_eq!(model_ord.is_le(), std_ord.is_le());
        }

        #[test]
        fn test_ordering_is_ge(x in any::<u8>(), y in any::<u8>()) {
            let model_ord = <u8 as Ord>::cmp(&x.inject(), &y.inject());
            let std_ord = std::cmp::Ord::cmp(&x, &y);
            prop_assert_eq!(model_ord.is_ge(), std_ord.is_ge());
        }

        #[test]
        fn test_ordering_reverse(x in any::<u8>(), y in any::<u8>()) {
            let model_ord = <u8 as Ord>::cmp(&x.inject(), &y.inject());
            let std_ord = std::cmp::Ord::cmp(&x, &y);
            prop_assert_eq!(model_ord.reverse(), std_ord.reverse().inject());
        }

        #[test]
        fn test_ordering_then(x in any::<u8>(), y in any::<u8>(), a in any::<u8>(), b in any::<u8>()) {
            let model_ord1 = <u8 as Ord>::cmp(&x.inject(), &y.inject());
            let model_ord2 = <u8 as Ord>::cmp(&a.inject(), &b.inject());
            let std_ord1 = std::cmp::Ord::cmp(&x, &y);
            let std_ord2 = std::cmp::Ord::cmp(&a, &b);
            prop_assert_eq!(model_ord1.then(model_ord2), std_ord1.then(std_ord2).inject());
        }

        // The `Equal` arms of `reverse`/`then`/`then_with`/`clamp` need both sides
        // of a comparison to agree, which a pair of independent draws never does.
        // Two independent draws are essentially never equal, so the `Equal` input
        // of each predicate needs a reflexive comparison. `Ordering` is not
        // `Copy`, hence the closure.
        #[test]
        fn test_ordering_predicates_on_equal(x in any::<u8>()) {
            let m = || <u8 as Ord>::cmp(&x.inject(), &x.inject());
            let s = std::cmp::Ord::cmp(&x, &x);
            prop_assert_eq!(m().is_eq(), s.is_eq());
            prop_assert_eq!(m().is_ne(), s.is_ne());
            prop_assert_eq!(m().is_lt(), s.is_lt());
            prop_assert_eq!(m().is_gt(), s.is_gt());
            prop_assert_eq!(m().is_le(), s.is_le());
            prop_assert_eq!(m().is_ge(), s.is_ge());
        }

        #[test]
        fn test_ordering_reverse_equal(x in any::<u8>()) {
            let model_ord = <u8 as Ord>::cmp(&x.inject(), &x.inject());
            let std_ord = std::cmp::Ord::cmp(&x, &x);
            prop_assert_eq!(model_ord.reverse(), std_ord.reverse().inject());
        }

        #[test]
        fn test_ordering_then_equal(x in any::<u8>(), a in any::<u8>(), b in any::<u8>()) {
            let model_ord2 = <u8 as Ord>::cmp(&a.inject(), &b.inject());
            let std_ord2 = std::cmp::Ord::cmp(&a, &b);
            prop_assert_eq!(
                <u8 as Ord>::cmp(&x.inject(), &x.inject()).then(model_ord2),
                std::cmp::Ord::cmp(&x, &x).then(std_ord2).inject()
            );
        }

        // One test, not one per arm: `then_with` is generic in the closure, so a
        // second test would be a second instantiation reaching only one arm.
        // `equal` decides whether the first ordering is `Equal`.
        #[test]
        fn test_ordering_then_with(
            x in any::<u8>(),
            y in any::<u8>(),
            equal in any::<bool>(),
            a in any::<u8>(),
            b in any::<u8>(),
        ) {
            let y = if equal { x } else { y };
            let model_ord2 = <u8 as Ord>::cmp(&a.inject(), &b.inject());
            let std_ord2 = std::cmp::Ord::cmp(&a, &b);
            prop_assert_eq!(
                <u8 as Ord>::cmp(&x.inject(), &y.inject()).then_with(|| model_ord2),
                std::cmp::Ord::cmp(&x, &y).then_with(|| std_ord2).inject()
            );
        }

        #[test]
        fn test_clamp_at_min(x in any::<u8>(), hi in any::<u8>()) {
            let hi = std::cmp::max(x, hi);
            prop_assert_eq!(super::clamp(x.inject(), x.inject(), hi.inject()), std::cmp::Ord::clamp(x, x, hi));
        }

        #[test]
        fn test_clamp(x in any::<u8>(), a in any::<u8>(), b in any::<u8>()) {
            let lo = std::cmp::min(a, b);
            let hi = std::cmp::max(a, b);
            prop_assert_eq!(
                super::clamp(x.inject(), lo.inject(), hi.inject()),
                std::cmp::Ord::clamp(x, lo, hi)
            );
        }

        #[test]
        fn test_max(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(super::max(x.inject(), y.inject()).inject(), std::cmp::max(x, y));
        }

        #[test]
        fn test_min(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(super::min(x.inject(), y.inject()).inject(), std::cmp::min(x, y));
        }

        #[test]
        fn test_reverse_partial_cmp(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                super::Reverse(x.inject()).partial_cmp(&super::Reverse(y.inject())),
                std::cmp::Reverse(x).partial_cmp(&std::cmp::Reverse(y)).inject()
            );
        }

        #[test]
        fn test_reverse_inject(x in any::<u8>()) {
            prop_assert_eq!(std::cmp::Reverse(x).inject().0, x);
        }

        #[test]
        fn test_reverse_eq(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                super::Reverse(x.inject()).eq(&super::Reverse(y.inject())),
                std::cmp::Reverse(x).eq(&std::cmp::Reverse(y))
            );
        }

        #[test]
        fn test_reverse_cmp(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                super::Reverse(x.inject()).cmp(&super::Reverse(y.inject())),
                std::cmp::Reverse(x).cmp(&std::cmp::Reverse(y)).inject()
            );
        }

        #[test]
        fn test_int_partial_cmp(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                <u8 as PartialOrd<u8>>::partial_cmp(&x.inject(), &y.inject()),
                std::cmp::PartialOrd::partial_cmp(&x, &y).inject()
            );
        }

        #[test]
        fn test_int_cmp(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                <u8 as Ord>::cmp(&x.inject(), &y.inject()),
                std::cmp::Ord::cmp(&x, &y).inject()
            );
        }

        #[test]
        fn test_int_eq(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(<u8 as PartialEq<u8>>::eq(&x.inject(), &y.inject()), std::cmp::PartialEq::eq(&x, &y));
        }

        #[test]
        fn test_int_neq(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                super::Neq::neq(&x.inject(), &y.inject()),
                x != y
            );
        }

        // `PartialOrd::lt` only exists off the F* backend; the workaround
        // trait is covered by `test_defaults_lt`.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_int_lt(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                <u8 as PartialOrd<u8>>::lt(&x.inject(), &y.inject()),
                x < y
            );
        }

        // `PartialOrd::le` only exists off the F* backend; the workaround
        // trait is covered by `test_defaults_le`.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_int_le(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                <u8 as PartialOrd<u8>>::le(&x.inject(), &y.inject()),
                x <= y
            );
        }

        // `PartialOrd::gt` only exists off the F* backend; the workaround
        // trait is covered by `test_defaults_gt`.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_int_gt(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                <u8 as PartialOrd<u8>>::gt(&x.inject(), &y.inject()),
                x > y
            );
        }

        // `PartialOrd::ge` only exists off the F* backend; the workaround
        // trait is covered by `test_defaults_ge`.
        #[cfg(not(hax_backend_fstar))]
        #[test]
        fn test_int_ge(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                <u8 as PartialOrd<u8>>::ge(&x.inject(), &y.inject()),
                x >= y
            );
        }

        #[test]
        fn test_defaults_lt(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                super::PartialOrdDefaults::lt(&x.inject(), &y.inject()),
                x < y
            );
        }

        #[test]
        fn test_defaults_le(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                super::PartialOrdDefaults::le(&x.inject(), &y.inject()),
                x <= y
            );
        }

        #[test]
        fn test_defaults_gt(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                super::PartialOrdDefaults::gt(&x.inject(), &y.inject()),
                x > y
            );
        }

        #[test]
        fn test_defaults_ge(x in any::<u8>(), y in any::<u8>()) {
            prop_assert_eq!(
                super::PartialOrdDefaults::ge(&x.inject(), &y.inject()),
                x >= y
            );
        }
    }

    #[test]
    fn test_clamp_min_above_max_panics() {
        crate::testing::panics_like_core(
            || super::clamp(5u8, 7u8, 3u8),
            || std::cmp::Ord::clamp(5u8, 7u8, 3u8),
        );
    }

    /// The unit type's comparison impls, against std's.
    #[test]
    fn test_unit_cmp() {
        assert_eq!(
            <() as PartialEq<()>>::eq(&(), &()),
            std::cmp::PartialEq::eq(&(), &())
        );
        assert_eq!(
            <() as PartialOrd<()>>::partial_cmp(&(), &()),
            std::cmp::PartialOrd::partial_cmp(&(), &()).inject()
        );
        assert_eq!(
            <() as Ord>::cmp(&(), &()),
            std::cmp::Ord::cmp(&(), &()).inject()
        );
    }

    #[cfg(not(hax_backend_fstar))]
    #[test]
    fn test_unit_ne() {
        assert_eq!(
            <() as PartialEq<()>>::ne(&(), &()),
            std::cmp::PartialEq::ne(&(), &())
        );
    }

    #[cfg(not(hax_backend_fstar))]
    #[test]
    fn test_ordering_eq() {
        let all = [
            std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal,
            std::cmp::Ordering::Greater,
        ];
        for a in all {
            for b in all {
                assert_eq!(
                    <Ordering as PartialEq<Ordering>>::eq(&a.inject(), &b.inject()),
                    a == b
                );
                assert_eq!(
                    <Ordering as PartialEq<Ordering>>::ne(&a.inject(), &b.inject()),
                    a != b
                );
            }
        }
    }
}
