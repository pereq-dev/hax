#![allow(dead_code)]
#![feature(try_trait_v2)]
#![feature(allocator_api)]

extern crate alloc;
/* This is a dummy Rust file. Every path used in this file will be
 * exported and made available automatically in OCaml. */

mod crypto_abstractions;

fn dummy_hax_concrete_ident_wrapper<I: core::iter::Iterator<Item = u8>>(x: I, mut y: I) {
    let _: core::result::Result<u8, u8> = core::result::Result::Ok(0);
    let _: core::result::Result<u8, u8> = core::result::Result::Err(0);
    let _ = x.fold(0, |a, b| a + b);
    let _ = y.next();
    let _: core::ops::ControlFlow<u8> = core::ops::ControlFlow::Break(0);
    let _: core::ops::ControlFlow<u8> = core::ops::ControlFlow::Continue(());
    let mut v = vec![()];
    v[0];
    v[0] = ();
    let _ = v.as_slice().to_vec();
    use std::ops::FromResidual;
    let _ = Result::<String, i64>::from_residual(Err(3u8));
    let _ = Box::new(());
    let _: Option<alloc::alloc::Global> = None;
    let _: Option<()> = Some(());
    let _: Option<()> = None;
    let _ = Option::<()>::None.is_some();
    let _: Result<(), u32> = Result::Err(3u8).map_err(u32::from);
    assert!(true);
    assert_eq!(1, 1);
    hax_lib::assert!(true);
    hax_lib::_internal_loop_invariant(|_: usize| true);
    hax_lib::_internal_while_loop_invariant(hax_lib::Prop::from(true));
    hax_lib::_internal_loop_decreases(hax_lib::Int::_unsafe_from_str("0"));

    fn props() {
        use hax_lib::prop::*;
        let x = Prop::from_bool(true);
        constructors::from_bool(true);
        constructors::and(x, x);
        constructors::or(x, x);
        constructors::not(x);
        constructors::eq(x, x);
        constructors::ne(x, x);
        constructors::implies(x, x);
        constructors::forall(|_: ()| x);
        constructors::exists(|_: ()| x);

        Prop::from_bool(true);
        Prop::and(x, x);
        Prop::or(x, x);
        Prop::not(x);
        Prop::eq(x, x);
        Prop::ne(x, x);
        Prop::implies(x, x);

        true.to_prop();

        forall(|_: ()| x);
        exists(|_: ()| x);
        implies(x, x);
    }

    let _ = [()].into_iter();
    let _: u16 = 6u8.into();
    let _ = 1..2;
    let _ = 1..=2;
    let _ = 1..;
    let _ = ..;
    let _ = ..1;

    let _ = [
        std::ops::ControlFlow::Break(()),
        std::ops::ControlFlow::Continue(()),
    ];

    fn iterator_functions<It: Iterator + Clone>(it: It) {
        let _ = it.clone().step_by(2);
        let _ = it.clone().enumerate();
        let _ = [()].chunks_exact(2);
        let _ = [()].iter();
        let _ = (&[()] as &[()]).iter();
    }

    {
        use hax_lib::*;
        let a: Int = 3u8.lift();
        let _: Int = 3u8.to_int();
        let _ = a.clone().pow2();
        let _ = Int::_unsafe_from_str("1");
        let _: u32 = a.concretize();
    }

    fn index_mut<I, T: std::ops::IndexMut<I>>(mut x: T, index: I) {
        x.index_mut(index);
    }

    fn question_mark_result<A, B: From<A>>(x: A) -> Result<(), B> {
        Err(x)?;
        Ok(())
    }

    let _ = hax_lib::inline("");
    let _: () = hax_lib::inline_unsafe("");
    let _: () = hax_lib::any_to_unit(());
    use hax_lib::{RefineAs, Refinement};

    fn refinements<T: Refinement + Clone, U: RefineAs<T>>(x: T, y: U) -> T {
        let _ = x.clone().get_mut();
        T::new(x.get());
        y.into_checked()
    }

    const _: () = {
        use core::{cmp::*, ops::*};
        fn arith<
            X: Add<Output = X>
                + Sub<Output = X>
                + Mul<Output = X>
                + Div<Output = X>
                + Rem<Output = X>
                + BitXor<Output = X>
                + BitAnd<Output = X>
                + BitOr<Output = X>
                + Shl<Output = X>
                + Shr<Output = X>
                + Neg<Output = X>
                + Not<Output = X>
                + PartialOrd
                + Eq
                + Ord
                + Copy,
        >(
            x: X,
        ) -> X {
            let _ = x < x && x > x && x <= x && x >= x && x == x && x != x;
            (x ^ x & !x) + x / x * x - x | (-x) % x << x >> x
        }
    };

    fn dummy<T: core::ops::Try<Output = ()>>(z: T) {
        let _ = T::from_output(());
        let _ = z.branch();
    }

    let s: &str = "123";
    let ptr: *const u8 = s.as_ptr();

    unsafe {
        let _ = *ptr.offset(1) as char;
    }

    const _: () = {
        use std::ops::DerefMut;
        fn f<T: DerefMut>(x: T) {
            let _: &mut _ = { x }.deref_mut();
        }
        use std::ops::Deref;
        fn g<T: Deref>(x: T) {
            let _: &_ = { x }.deref();
        }
    };

    #[derive(PartialEq)]
    struct Foo();
}

macro_rules! impl_arith {
    ($name:ident$(,)?) => {
        mod $name {
            fn add() {}
            fn sub() {}
            fn mul() {}
            fn div() {}
            fn rem() {}
            fn neg() {}
            fn bit_xor() {}
            fn bit_and() {}
            fn bit_or() {}
            fn shl() {}
            fn shr() {}
            fn eq() {}
            fn lt() {}
            fn le() {}
            fn ne() {}
            fn ge() {}
            fn gt() {}
        }
    };
    ($name:ident,$($rest:tt)*) => {
        impl_arith!($name);
        impl_arith!($($rest)*);
    }
}

impl_arith!(u8, u16, u32, u64, u128, usize);
impl_arith!(i8, i16, i32, i64, i128, isize);

fn offset() {}

fn unsize() {}

/// Hax additions
mod hax {
    fn failure() {}
    struct Failure;
    enum Never {}

    // Only useful when HAX_CORE_EXTRACTION_MODE in `on`
    enum MutRef {}

    struct Tuple2(u8, u8);
    fn deref_op() {}
    fn cast_op() {}
    fn logical_op_and() {}
    fn logical_op_or() {}

    fn while_loop() {}
    fn while_loop_cf() {}
    fn while_loop_return() {}
    fn repeat() {}
    fn update_at() {}
    mod monomorphized_update_at {
        fn update_at_usize() {}
        fn update_at_range() {}
        fn update_at_range_from() {}
        fn update_at_range_to() {}
        fn update_at_range_full() {}
    }
    // TODO: Should that live here? (this is F* specific)
    fn array_of_list() {}
    fn never_to_any() {}

    mod folds {
        fn fold_range() {}
        fn fold_range_cf() {}
        fn fold_range_return() {}
        fn fold_range_inclusive() {}
        fn fold_range_inclusive_cf() {}
        fn fold_range_inclusive_return() {}
        fn fold_range_step_by() {}
        fn fold_range_step_by_cf() {}
        fn fold_range_step_by_return() {}
        fn fold_enumerated_slice() {}
        fn fold_enumerated_slice_cf() {}
        fn fold_enumerated_slice_return() {}
        fn fold_enumerated_chunked_slice() {}
        fn fold_enumerated_chunked_slice_cf() {}
        fn fold_enumerated_chunked_slice_return() {}
        fn fold_chunked_slice() {}
        fn fold_chunked_slice_cf() {}
        fn fold_chunked_slice_return() {}
        fn fold_cf() {}
        fn fold_return() {}
    }

    /// The engine uses this `dropped_body` symbol as a marker value
    /// to signal that a item was extracted without body.
    fn dropped_body() {}

    mod int {
        fn add() {}
        fn sub() {}
        fn div() {}
        fn mul() {}
        fn rem() {}
        fn neg() {}

        fn le() {}
        fn lt() {}
        fn ge() {}
        fn gt() {}

        fn eq() {}
        fn ne() {}

        fn from_machine() {}
        fn into_machine() {}
    }

    mod machine_int {
        fn add() {}
        fn sub() {}
        fn div() {}
        fn mul() {}
        fn rem() {}

        fn not() {}
        fn bitxor() {}
        fn bitor() {}
        fn bitand() {}
        fn shl() {}
        fn shr() {}

        fn eq() {}
        fn ne() {}
        fn le() {}
        fn lt() {}
        fn ge() {}
        fn gt() {}

        fn add_with_overflow() {}
        fn sub_with_overflow() {}
        fn mul_with_overflow() {}
        fn cmp() {}
    }

    mod control_flow_monad {
        trait ControlFlowMonad {
            fn lift() {}
        }
        mod mexception {
            fn run() {}
        }
        mod mresult {
            fn run() {}
        }
        mod moption {
            fn run() {}
        }
    }
    fn box_new() {}

    mod explicit_monadic {
        fn lift() {}
        fn pure() {}
    }
}

mod arithmetic {
    fn neg() {}
}
