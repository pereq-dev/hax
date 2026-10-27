#![allow(unused_variables)]

/// See [`std::fmt::Error`]
pub struct Error;

/// See [`std::fmt::Result`]
pub type Result = super::result::Result<(), Error>;

/// See [`std::fmt::Formatter`]
///
/// DEVIATION(std): the model's formatter has no output, so `Debug` and
/// `Display` only model whether formatting succeeds.
pub struct Formatter;

// `Formatter::debug_struct_field{1..5}_finish`: one arity per field count, as
// real `core` spells them out.
macro_rules! debug_struct_field_finish {
    ($( $name:ident : $( ($T:ident, $key:ident, $value:ident) ),+ );+ $(;)?) => {$(
        #[doc = concat!("See [`std::fmt::Formatter::", stringify!($name), "`]")]
        pub fn $name<$($T: Debug),+>(
            &mut self,
            struct_name: &str,
            $($key: &str, $value: &$T),+
        ) -> Result {
            Result::Ok(())
        }
    )+};
}

impl Formatter {
    // The `debug_*_finish` family is what `#[derive(Debug)]` expands to. The
    // model's `Formatter` renders nothing, so they all succeed without reading
    // their arguments. Real `core` takes the values as `&dyn Debug`; the model
    // takes one generic reference per field, since `dyn` has no F* counterpart.

    debug_struct_field_finish! {
        debug_struct_field1_finish: (T1, name1, value1);
        debug_struct_field2_finish: (T1, name1, value1), (T2, name2, value2);
        debug_struct_field3_finish:
            (T1, name1, value1), (T2, name2, value2), (T3, name3, value3);
        debug_struct_field4_finish:
            (T1, name1, value1), (T2, name2, value2), (T3, name3, value3),
            (T4, name4, value4);
        debug_struct_field5_finish:
            (T1, name1, value1), (T2, name2, value2), (T3, name3, value3),
            (T4, name4, value4), (T5, name5, value5);
    }

    /// See [`std::fmt::Formatter::debug_struct_fields_finish`]
    ///
    /// Real `core` asserts that the two slices have the same length; the model
    /// keeps that panic, since it is the only observable behaviour left.
    pub fn debug_struct_fields_finish<T: Debug>(
        &mut self,
        struct_name: &str,
        names: &[&str],
        values: &[&T],
    ) -> Result {
        if rust_primitives::slice::slice_length(names)
            != rust_primitives::slice::slice_length(values)
        {
            crate::panicking::internal::panic()
        }
        Result::Ok(())
    }

    /// See [`std::fmt::Formatter::debug_tuple_field1_finish`]
    pub fn debug_tuple_field1_finish<T1: Debug>(
        &mut self,
        struct_name: &str,
        value1: &T1,
    ) -> Result {
        Result::Ok(())
    }
}

/// See [`std::fmt::Display`]
pub trait Display {
    /// See [`std::fmt::Display::fmt`]
    fn fmt(&self, f: &mut Formatter) -> Result;
}

/// See [`std::fmt::Debug`]
pub trait Debug {
    /// See [`std::fmt::Debug::fmt`]
    #[cfg(not(hax_backend_fstar))]
    fn fmt(&self, f: &mut Formatter) -> Result;
    #[cfg(hax_backend_fstar)]
    fn dbg_fmt(&self, f: &mut Formatter) -> Result;
}

/// See [`std::fmt::Arguments`]
pub struct Arguments<'a>(&'a ());

// F*-only. Real `core` has no blanket `Debug`, and keeping one here is what
// made the concrete impls below impossible to add (`E0119`). The F* model keeps
// it: its `Clone`/`TrivialClone`/`UseCloned` are blanket too (see
// `crate::clone`), and nothing in the F* lane needs the per-type names.
#[cfg(hax_backend_fstar)]
impl<T> Debug for T {
    fn dbg_fmt(&self, f: &mut Formatter) -> Result {
        Result::Ok(())
    }
}

/// See [`std::fmt::Debug`] for `&T`
#[cfg(not(hax_backend_fstar))]
impl<T: Debug + ?Sized> Debug for &T {
    fn fmt(&self, f: &mut Formatter) -> Result {
        (**self).fmt(f)
    }
}

/// See [`std::fmt::Debug`] for [`Error`]
#[cfg(not(hax_backend_fstar))]
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Result::Ok(())
    }
}

/// See [`std::fmt::Debug`] for `bool`
#[cfg(not(hax_backend_fstar))]
impl Debug for core::primitive::bool {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Result::Ok(())
    }
}

/// See [`std::fmt::Debug`] for `()`
#[cfg(not(hax_backend_fstar))]
impl Debug for () {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Result::Ok(())
    }
}

/// See [`std::fmt::Debug`] for `str`
#[cfg(not(hax_backend_fstar))]
impl Debug for core::primitive::str {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Result::Ok(())
    }
}

/// See [`std::fmt::Debug`] for `[T]`
#[cfg(not(hax_backend_fstar))]
impl<T: Debug> Debug for [T] {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Real `core` stops at the first element that fails to format.
        let mut res = Result::Ok(());
        let mut i = 0;
        while i < rust_primitives::slice::slice_length(self) {
            if let Result::Ok(()) = res {
                res = Debug::fmt(rust_primitives::slice::slice_index(self, i), f);
            }
            i += 1;
        }
        res
    }
}

/// `core::fmt::num` — where real `core` puts the integer `Debug`/`Display`
/// impls. The model mirrors the nesting so that the extracted names match the
/// `core::fmt::num::{core::fmt::Debug<u8>}` paths a client references.
#[cfg(not(hax_backend_fstar))]
pub mod num {
    use super::{Debug, Formatter, Result};

    macro_rules! impl_debug_for_int {
        ($($t:ty),*) => {
            $(
                impl Debug for $t {
                    fn fmt(&self, f: &mut Formatter) -> Result {
                        Result::Ok(())
                    }
                }
            )*
        };
    }

    impl_debug_for_int!(
        core::primitive::u8,
        core::primitive::u16,
        core::primitive::u32,
        core::primitive::u64,
        core::primitive::u128,
        core::primitive::usize,
        core::primitive::i8,
        core::primitive::i16,
        core::primitive::i32,
        core::primitive::i64,
        core::primitive::i128,
        core::primitive::isize
    );
}

// No blanket `Display` (unlike `Debug` above); spell out the integer impls,
// stubbed to `Ok(())` like the rest of this module.
macro_rules! impl_display_for_int {
    ($($t:ty),*) => {
        $(
            impl Display for $t {
                fn fmt(&self, f: &mut Formatter) -> Result {
                    Result::Ok(())
                }
            }
        )*
    };
}

impl_display_for_int!(
    core::primitive::u8,
    core::primitive::u16,
    core::primitive::u32,
    core::primitive::u64,
    core::primitive::u128,
    core::primitive::usize,
    core::primitive::i8,
    core::primitive::i16,
    core::primitive::i32,
    core::primitive::i64,
    core::primitive::i128,
    core::primitive::isize
);

impl<'a> Arguments<'a> {}
impl<'a> Arguments<'a> {}
impl<'a> Arguments<'a> {}
impl<'a> Arguments<'a> {}
impl<'a> Arguments<'a> {}
impl<'a> Arguments<'a> {}
impl<'a> Arguments<'a> {}
impl<'a> Arguments<'a> {}
impl<'a> Arguments<'a> {}
impl<'a> Formatter {
    /// See [`std::fmt::Formatter::write_str`]
    pub fn write_str(&mut self, data: &str) -> Result {
        Result::Ok(())
    }
}
impl<'a> Arguments<'a> {
    fn write_fmt(f: &mut Formatter, args: Arguments) -> Result {
        Result::Ok(())
    }
    /// Not a real `std::fmt::Arguments` method: the carve lowers panic/assert
    /// messages to `Arguments::from_str(msg)` and discards the result, so this
    /// exists to make those call sites resolve.
    ///
    /// Opaque: real `core` builds an `Arguments` that carries the formatted
    /// message, while the model's is a payload-free phantom, so the body is not
    /// a model of what `core` does — only enough to have a value.
    #[hax_lib::opaque]
    pub fn from_str(_s: &str) -> Arguments<'a> {
        Arguments(&())
    }

    /// See [`std::fmt::Arguments::new`]: `format_args!`'s non-literal entry
    /// point. The model's `Arguments` has no payload, so neither argument is
    /// read.
    ///
    /// Excluded from Lean like the other `Arguments` constructors ("There
    /// should be no bottoms in the value"); hand-written in `FunsPrologue.lean`.
    /// Excluded from F* too: it is the only `fmt` item mentioning `fmt::rt`,
    /// whose module cycle makes hax rename `Core_models.Fmt{,.Rt}`.
    #[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
    #[cfg_attr(hax_backend_lean, hax_lib::exclude)]
    fn new<const N: usize, const M: usize>(
        template: &'a [core::primitive::u8; N],
        args: &'a [rt::Argument<'a>; M],
    ) -> Arguments<'a> {
        Arguments(&())
    }
}

mod rt {
    // F*-only: `charon::opaque` drops the declaration too, and `Argument`'s
    // field still names this type, so Lean would not elaborate.
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    // The internals of this are not important in this model
    enum ArgumentType<'a> {
        Placeholder {
            /* value: NonNull<()>,
            formatter: unsafe fn(NonNull<()>, &mut Formatter<'_>) -> Result, */
            _lifetime: std::marker::PhantomData<&'a ()>,
        },
        /* Count(u16), */
    }

    pub struct Argument<'a> {
        ty: ArgumentType<'a>,
    }

    // The formatting arguments carry no observable payload in this model, so
    // every constructor below yields the single placeholder value. Opaque like
    // `ArgumentType` itself: charon drops that enum's variants, so a body
    // building one would leave aeneas without the fields.
    #[hax_lib::opaque]
    fn placeholder<'a>() -> ArgumentType<'a> {
        ArgumentType::Placeholder {
            _lifetime: std::marker::PhantomData,
        }
    }

    impl Argument<'_> {
        #[hax_lib::opaque]
        fn new_display<T>(x: &T) -> Self {
            Argument { ty: placeholder() }
        }
        #[hax_lib::opaque]
        fn new_debug<T>(x: &T) -> Self {
            Argument { ty: placeholder() }
        }
        #[hax_lib::opaque]
        fn new_lower_hex<T>(x: &T) -> Self {
            Argument { ty: placeholder() }
        }
    }
    impl<'a> Argument<'a> {
        #[hax_lib::opaque]
        fn new_binary<T>(x: &T) -> Self {
            Argument { ty: placeholder() }
        }
        #[hax_lib::opaque]
        fn new_const<T, U>(x: &T, y: &U) -> super::Arguments<'a> {
            super::Arguments(&())
        }
        #[hax_lib::opaque]
        fn new_v1<T, U, V, W>(x: &T, y: &U, z: &V, t: &W) -> super::Arguments<'a> {
            super::Arguments(&())
        }
        fn none() -> [Self; 0] {
            []
        }
        #[hax_lib::opaque]
        fn new_v1_formatted<T, U, V>(x: &T, y: &U, z: &V) -> super::Arguments<'a> {
            super::Arguments(&())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{Argument, ArgumentType};

        /// The `rt` constructors are placeholders (core's real ones build
        /// type-erased trait objects), so all a test can check is that each one
        /// returns the placeholder rather than diverging.
        fn is_placeholder(a: &Argument<'_>) -> bool {
            matches!(a.ty, ArgumentType::Placeholder { .. })
        }

        #[test]
        fn test_argument_constructors() {
            assert!(is_placeholder(&Argument::new_display(&1u8)));
            assert!(is_placeholder(&Argument::new_debug(&1u8)));
            assert!(is_placeholder(&Argument::new_lower_hex(&1u8)));
            assert!(is_placeholder(&Argument::new_binary(&1u8)));
            assert!(Argument::none().is_empty());
        }

        #[test]
        fn test_arguments_constructors() {
            // `Arguments` has no observable content either; construction is all
            // there is to check.
            let _ = Argument::new_const(&1u8, &2u8);
            let _ = Argument::new_v1(&1u8, &2u8, &3u8, &4u8);
            let _ = Argument::new_v1_formatted(&1u8, &2u8, &3u8);
        }
    }

    enum Count {
        Is(u16),
        Param(u16),
        Implied,
    }

    struct Placeholder {
        position: usize,
        flags: u32,
        precision: Count,
        width: Count,
    }

    struct UnsafeArg;
}

#[cfg(test)]
mod tests {
    use super::{Arguments, Display, Formatter, Result};

    // Everything in this module is a stub returning `Ok(())`: the model has no
    // output buffer, so `Ok(())` is the whole observable behaviour.
    // `fmt::Error` has no `PartialEq`, hence `is_ok` rather than `assert_eq!`.
    #[test]
    fn test_write_str() {
        let mut f = Formatter;
        assert!(f.write_str("hello").is_ok());
    }

    // `Arguments` is a phantom, so reaching the constructor is the whole
    // observable behaviour.
    #[test]
    fn test_arguments_from_str() {
        let mut f = Formatter;
        let args = Arguments::from_str("boom");
        assert!(super::Arguments::write_fmt(&mut f, args).is_ok());
    }

    #[test]
    fn test_debug_fmt() {
        let mut f = Formatter;
        #[cfg(not(hax_backend_fstar))]
        assert!(super::Debug::fmt(&1u8, &mut f).is_ok());
        #[cfg(hax_backend_fstar)]
        assert!(super::Debug::dbg_fmt(&1u8, &mut f).is_ok());
    }

    /// `Arguments::new` is `format_args!`'s non-literal entry point; the
    /// model's `Arguments` has no payload, so construction is all there is.
    #[test]
    fn test_arguments_new() {
        let template: [u8; 0] = [];
        let args: [super::rt::Argument; 0] = [];
        let _ = Arguments::new(&template, &args);
    }

    /// The `debug_*_finish` family: `#[derive(Debug)]`'s entry points. Like
    /// everything else here they render nothing and succeed.
    #[test]
    fn test_debug_finish_family() {
        let mut f = Formatter;
        assert!(f.debug_struct_field1_finish("S", "a", &1u8).is_ok());
        assert!(
            f.debug_struct_field2_finish("S", "a", &1u8, "b", &2u16)
                .is_ok()
        );
        assert!(
            f.debug_struct_field3_finish("S", "a", &1u8, "b", &2u16, "c", &3u32)
                .is_ok()
        );
        assert!(
            f.debug_struct_field4_finish("S", "a", &1u8, "b", &2u16, "c", &3u32, "d", &4u64)
                .is_ok()
        );
        assert!(
            f.debug_struct_field5_finish(
                "S", "a", &1u8, "b", &2u16, "c", &3u32, "d", &4u64, "e", &5u128
            )
            .is_ok()
        );
        assert!(f.debug_tuple_field1_finish("S", &1u8).is_ok());

        let names: [&str; 2] = ["a", "b"];
        let (v1, v2) = (1u8, 2u8);
        let values: [&u8; 2] = [&v1, &v2];
        assert!(f.debug_struct_fields_finish("S", &names, &values).is_ok());
    }

    /// `debug_struct_fields_finish` asserts that the two slices agree in
    /// length; real `core` panics too, which is what the std side below shows
    /// (its version is only reachable from inside a `Debug` impl).
    #[test]
    fn test_debug_struct_fields_finish_length_mismatch() {
        struct Mismatch;
        impl std::fmt::Debug for Mismatch {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let names: [&str; 2] = ["a", "b"];
                let values: [&dyn std::fmt::Debug; 1] = [&1u8];
                f.debug_struct_fields_finish("S", &names, &values)
            }
        }

        let names: [&str; 2] = ["a", "b"];
        let v = 1u8;
        let values: [&u8; 1] = [&v];
        crate::testing::panics_like_core(
            || Formatter.debug_struct_fields_finish("S", &names, &values),
            || format!("{:?}", Mismatch),
        );
    }

    /// The concrete `Debug` impls: `&T`, `bool`, `()` and, through
    /// `fmt::num`, every integer type. Only in the default configuration --
    /// under `hax_backend_fstar` the blanket impl covers everything and the
    /// method is called `dbg_fmt`.
    #[cfg(not(hax_backend_fstar))]
    #[test]
    fn test_concrete_debug_impls() {
        fn check<T: super::Debug>(x: &T) {
            let mut f = Formatter;
            assert!(super::Debug::fmt(x, &mut f).is_ok());
        }
        check(&true);
        check(&());
        check(&super::Error);
        check(&1u8);
        // `&T` goes through the reference impl, which forwards to `T`'s.
        check(&&1u8);
        check(&1u16);
        check(&1u32);
        check(&1u64);
        check(&1u128);
        check(&1usize);
        check(&-1i8);
        check(&-1i16);
        check(&-1i32);
        check(&-1i64);
        check(&-1i128);
        check(&-1isize);
        // `str` and `[T]` are unsized, so they cannot go through `check`.
        let mut f = Formatter;
        let s: &[u8] = &[1, 2, 3];
        assert!(super::Debug::fmt(s, &mut f).is_ok());
        assert!(super::Debug::fmt("hello", &mut f).is_ok());
    }

    /// `Debug` for `[T]` fails as soon as any element does.
    #[cfg(not(hax_backend_fstar))]
    #[test]
    fn test_slice_debug_forwards() {
        use crate::testing::DebugWitness;
        let mut f = Formatter;
        let empty: &[DebugWitness] = &[];
        assert!(super::Debug::fmt(empty, &mut f).is_ok());
        for n in 0..3 {
            let s: Vec<DebugWitness> = (0..3).map(|i| DebugWitness { fails: i == n }).collect();
            assert!(super::Debug::fmt(&s[..], &mut f).is_err());
        }
        let s = [DebugWitness { fails: false }, DebugWitness { fails: false }];
        assert!(super::Debug::fmt(&s[..], &mut f).is_ok());
    }

    macro_rules! display_tests {
        ($($t:ident),*) => {
            pastey::paste! { $(
                #[test]
                fn [<test_display_ $t>]() {
                    let mut f = Formatter;
                    assert!(Display::fmt(&(0 as $t), &mut f).is_ok());
                }
            )* }
        };
    }

    display_tests!(
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    );

    #[test]
    fn test_write_fmt() {
        let mut f = Formatter;
        assert!(Arguments::write_fmt(&mut f, Arguments(&())).is_ok());
    }

    // `Arguments` can only be built inside this module, so `panicking::panic_fmt`
    // is exercised here rather than next to the other `panicking` tests.
    #[test]
    #[should_panic]
    fn test_panic_fmt() {
        crate::panicking::panic_fmt(Arguments(&()));
    }
}
