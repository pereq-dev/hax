use crate::abstraction::*;
use core::ops::*;

/// Represent a logical proposition, that may be not computable.
#[derive(Clone, Copy, Debug)]
pub struct Prop(bool);

/// This module provides monomorphic constructors for `Prop`.
/// Hax rewrite more elaborated versions (see `forall` or `AndBit` below) to those monomorphic constructors.
pub mod constructors {
    use super::Prop;
    pub const fn from_bool(b: bool) -> Prop {
        Prop(b)
    }
    pub fn and(lhs: Prop, other: Prop) -> Prop {
        Prop(lhs.0 && other.0)
    }
    pub fn or(lhs: Prop, other: Prop) -> Prop {
        Prop(lhs.0 || other.0)
    }
    pub fn not(lhs: Prop) -> Prop {
        Prop(!lhs.0)
    }

    /// Logical equality between two value of *any* type
    pub fn eq<T>(_lhs: T, _rhs: T) -> Prop {
        Prop(true)
    }

    pub fn ne<T>(_lhs: T, _rhs: T) -> Prop {
        Prop(true)
    }

    pub fn implies(lhs: Prop, other: Prop) -> Prop {
        Prop(!lhs.0 || other.0)
    }

    pub fn forall<A, F: Fn(A) -> Prop>(_pred: F) -> Prop {
        Prop(true)
    }

    pub fn exists<A, F: Fn(A) -> Prop>(_pred: F) -> Prop {
        Prop(true)
    }
}

impl Prop {
    /// Lifts a boolean to a logical proposition.
    pub const fn from_bool(b: bool) -> Self {
        constructors::from_bool(b)
    }
    /// Conjunction of two propositions.
    pub fn and(self, other: impl Into<Self>) -> Self {
        constructors::and(self, other.into())
    }
    /// Disjunction of two propositions.
    pub fn or(self, other: impl Into<Self>) -> Self {
        constructors::or(self, other.into())
    }
    /// Negation of a proposition.
    pub fn not(self) -> Self {
        constructors::not(self)
    }
    /// Equality between two propositions.
    pub fn eq(self, other: impl Into<Self>) -> Self {
        constructors::eq(self, other.into())
    }
    /// Equality between two propositions.
    pub fn ne(self, other: impl Into<Self>) -> Self {
        constructors::ne(self, other.into())
    }
    /// Logical implication.
    pub fn implies(self, other: impl Into<Self>) -> Self {
        constructors::implies(self, other.into())
    }
}

impl Abstraction for bool {
    type AbstractType = Prop;
    fn lift(self) -> Self::AbstractType {
        Prop(self)
    }
}

pub trait ToProp {
    fn to_prop(self) -> Prop;
}
impl ToProp for bool {
    fn to_prop(self) -> Prop {
        self.lift()
    }
}

impl From<bool> for Prop {
    fn from(value: bool) -> Self {
        Prop(value)
    }
}

impl<T: Into<Prop>> BitAnd<T> for Prop {
    type Output = Prop;
    fn bitand(self, rhs: T) -> Self::Output {
        Prop(self.0 & rhs.into().0)
    }
}

impl<T: Into<Prop>> BitOr<T> for Prop {
    type Output = Prop;
    fn bitor(self, rhs: T) -> Self::Output {
        Prop(self.0 | rhs.into().0)
    }
}

impl Not for Prop {
    type Output = Prop;
    fn not(self) -> Self::Output {
        Prop(!self.0)
    }
}

/// The universal quantifier. This should be used only for Hax code: in
/// Rust, this is always true.
///
/// # Example:
///
/// The Rust expression `forall(|x: T| phi(x))` corresponds to `∀ (x: T), phi(x)`.
pub fn forall<T, U: Into<Prop>>(f: impl Fn(T) -> U) -> Prop {
    constructors::forall(|x| f(x).into())
}

/// The existential quantifier. This should be used only for Hax code: in
/// Rust, this is always true.
///
/// # Example:
///
/// The Rust expression `exists(|x: T| phi(x))` corresponds to `∃ (x: T), phi(x)`.
pub fn exists<T, U: Into<Prop>>(f: impl Fn(T) -> U) -> Prop {
    constructors::exists(|x| f(x).into())
}

/// The logical implication `a ==> b`.
///
/// Both sides are evaluated eagerly. For lazy evaluation of the
/// right-hand side, use [`implies!`](crate::implies!) instead.
pub fn implies(lhs: impl Into<Prop>, rhs: impl Into<Prop>) -> Prop {
    constructors::implies(lhs.into(), rhs.into())
}

/// The logical implication `a ==> b`, where `b` is evaluated only
/// when `a` holds.
///
/// Unlike the function [`implies`], `implies!(a, b)` does not evaluate
/// `b` when `a` is false. This matters when `b` is only well-defined
/// under `a`, e.g. when `a` is a bounds check for an index in `b`.
///
/// `implies!(a, b)` expands to `if a { b } else { true }`, lifted to
/// [`Prop`]: `a` is a `bool`, and `b` is anything that converts into a
/// [`Prop`].
///
/// # Example:
///
/// ```
/// use hax_lib::*;
/// let a: &[u8] = &[1, 2];
/// // Never indexes `a` out of bounds.
/// let _ = implies!(a.len() > 2, a[2] == 0);
/// let _ = forall(|k: usize| implies!(k < a.len(), a[k] <= 2));
/// ```
#[macro_export]
macro_rules! implies {
    ($lhs:expr, $rhs:expr $(,)?) => {
        if $lhs {
            $crate::Prop::from($rhs)
        } else {
            $crate::Prop::from(true)
        }
    };
}

pub use constructors::eq;

#[cfg(test)]
mod tests {
    use super::*;

    const TRUTH_TABLE: [(bool, bool, bool); 4] = [
        (false, false, true),
        (false, true, true),
        (true, false, false),
        (true, true, true),
    ];

    #[test]
    fn implies_truth_table() {
        for (lhs, rhs, expected) in TRUTH_TABLE {
            assert_eq!(implies(lhs, rhs).0, expected, "implies({lhs}, {rhs})");
            assert_eq!(implies!(lhs, rhs).0, expected, "implies!({lhs}, {rhs})");
        }
    }

    /// A right-hand side that panics when evaluated.
    fn rhs() -> bool {
        panic!("the right-hand side is evaluated")
    }

    #[test]
    fn implies_macro_is_lazy() {
        assert!(implies!(false, rhs()).0);
    }

    #[test]
    #[should_panic(expected = "the right-hand side is evaluated")]
    fn implies_macro_evaluates_rhs_when_lhs_holds() {
        let _ = implies!(true, rhs());
    }
}
