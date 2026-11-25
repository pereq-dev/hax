mod abstraction;
pub use abstraction::Concretization;

pub mod prop;
pub use prop::*;

pub use int::*;

#[cfg(feature = "macros")]
pub use crate::proc_macros::*;

#[macro_export]
macro_rules! debug_assert {
    ($($arg:tt)*) => {
        ::core::debug_assert!($($arg)*);
    };
}

#[macro_export]
macro_rules! assert {
    ($($arg:tt)*) => {
        ::core::assert!($($arg)*);
    };
}

#[macro_export]
macro_rules! assert_prop {
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! assume {
    ($formula:expr) => {
        ()
    };
}

#[doc(hidden)]
pub fn inline(_: &str) {}

#[doc(hidden)]
pub fn inline_unsafe<T>(_: &str) -> T {
    unreachable!()
}

#[doc(hidden)]
pub const fn _internal_loop_invariant<T, R: Into<Prop>, P: FnOnce(T) -> R>(_: &P) {}

#[doc(hidden)]
pub const fn _internal_while_loop_invariant(_: Prop) {}

#[doc(hidden)]
pub const fn _internal_loop_decreases(_: int::Int) {}

pub trait Refinement {
    type InnerType;
    fn new(x: Self::InnerType) -> Self;
    fn get(self) -> Self::InnerType;
    fn get_mut(&mut self) -> &mut Self::InnerType;
    fn invariant(value: Self::InnerType) -> crate::Prop;
}

pub trait RefineAs<RefinedType> {
    fn into_checked(self) -> RefinedType;
}

pub mod int;
