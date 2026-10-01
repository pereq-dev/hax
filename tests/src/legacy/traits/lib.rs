//! @fail(extraction): proverif(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001)
//! @fail(tc): legacy-lean(1)
//! @fail(tc): fstar(72)

#![allow(dead_code)]

pub trait SuperTrait: Clone {
    fn function_of_super_trait(self) -> u32;
}

pub trait Foo {
    type AssocType: SuperTrait;
    const N: usize;
    fn assoc_f() -> ();
    fn method_f(&self) -> ();
    fn assoc_type(_: Self::AssocType) -> ()
    where
        Self::AssocType: Copy;
}

impl SuperTrait for i32 {
    fn function_of_super_trait(self) -> u32 {
        self.abs() as u32
    }
}

impl Foo for () {
    type AssocType = i32;
    const N: usize = 32;
    fn assoc_f() {
        ()
    }
    fn method_f(&self) {
        Self::assoc_f()
    }
    fn assoc_type(_: Self::AssocType) -> () {}
}

fn f<T: Foo>(x: T) {
    T::assoc_f();
    x.method_f()
}

fn g<T: Foo>(x: T::AssocType) -> u32 {
    x.function_of_super_trait()
}

struct Struct;

trait Bar<'a> {
    fn bar(self);
}

impl<'a> Struct {
    fn method<T: Bar<'a>>(x: T) {
        x.bar()
    }
}

pub fn closure_impl_expr<I: Iterator<Item = ()>>(it: I) -> Vec<()> {
    it.map(|x| x).collect()
}

/// @fail(extraction): legacy-lean(HAX0001)
pub fn closure_impl_expr_fngen<I: Iterator<Item = ()>, F: FnMut(()) -> ()>(it: I, f: F) -> Vec<()> {
    it.map(f).collect()
}

// From issue #523
pub trait Lang: Sized {
    type Var;
    fn s(self, _: i32) -> (Self, Self::Var);
}

pub enum Error {
    Fail,
}

// From issue #474
impl Error {
    pub fn for_application_callback() -> impl FnOnce() -> Self {
        || Self::Fail
    }
}

// Trickier case.
fn iter_option<'a, T>(x: &'a Option<T>) -> impl Iterator<Item = &'a T> {
    x.as_ref().into_iter()
}

// Issue #684
fn use_impl_trait() {
    let mut iter = iter_option(&Some(false));
    let _ = iter.next();
}

mod for_clauses {
    trait Foo<T> {
        fn to_t(&self) -> T;
    }

    fn _f<X: for<'a> Foo<&'a u8>>(x: X) {
        x.to_t();
    }

    // From issue #495
    mod issue_495 {
        use core::iter::Filter;
        use core::ops::Range;

        fn original_function_from_495(list: Vec<u8>) {
            let _indices: Vec<_> = (0..5).filter(|i| list.iter().any(|n| n == i)).collect();
        }

        fn minimized_1(list: Vec<u8>) -> Vec<u8> {
            (0..5).filter(|_| true).collect()
        }
        fn minimized_2(it: Filter<Range<u8>, for<'a> fn(&'a u8) -> bool>) {
            let _indices: Vec<_> = it.collect();
        }
        mod minimized_3 {
            pub trait Trait {}

            /// @fail(extraction): legacy-lean(HAX0001)
            impl<P: FnMut(&u8) -> bool> Trait for P {}
        }
    }
}

mod unconstrainted_types_issue_677 {
    trait PolyOp {
        fn op(x: u32, y: u32) -> u32;
    }
    struct Plus;
    impl PolyOp for Plus {
        fn op(x: u32, y: u32) -> u32 {
            x + y
        }
    }

    struct Times;
    impl PolyOp for Times {
        fn op(x: u32, y: u32) -> u32 {
            x * y
        }
    }

    fn twice<OP: PolyOp>(x: u32) -> u32 {
        OP::op(x, x)
    }

    fn both(x: u32) -> (u32, u32) {
        (twice::<Plus>(x), twice::<Times>(x))
    }

    #[test]
    fn test() {
        assert!(both(10) == (20, 100));
    }
}

// From issue_667
mod implicit_dependencies_issue_667 {
    mod trait_definition {
        pub trait MyTrait {
            fn my_method(self);
        }
    }
    mod define_type {
        pub struct MyType;
    }
    mod impl_type {
        impl super::trait_definition::MyTrait for super::define_type::MyType {
            fn my_method(self) {}
        }
    }
    mod use_type {
        fn some_function(x: super::define_type::MyType) {
            use super::trait_definition::MyTrait;
            x.my_method()
        }
    }
}

// Related to issue 719
mod interlaced_consts_types {
    struct Bar<const FooConst: usize, FooType>([FooType; FooConst]);

    trait Foo<const FooConst: usize, FooType> {
        fn fun<const FunConst: usize, FunType>(x: [FooType; FooConst], y: [FunType; FunConst]);
    }

    impl<const FooConst: usize, FooType, SelfType> Foo<FooConst, FooType> for SelfType {
        fn fun<const FunConst: usize, FunType>(x: [FooType; FooConst], y: [FunType; FunConst]) {}
    }
}

// Related to issue #719 (after reopen)
mod implicit_explicit_calling_conventions {
    struct Type<TypeArg, const ConstArg: usize> {
        field: [TypeArg; ConstArg],
    }

    trait Trait<TypeArg, const ConstArg: usize> {
        fn method<MethodTypeArg, const MethodConstArg: usize>(
            self,
            value_TypeArg: TypeArg,
            value_Type: Type<TypeArg, ConstArg>,
        );
        fn associated_function<MethodTypeArg, const MethodConstArg: usize>(
            _self: Self,
            value_TypeArg: TypeArg,
            value_Type: Type<TypeArg, ConstArg>,
        );
    }

    impl<TypeArg, const ConstArg: usize> Trait<TypeArg, ConstArg> for () {
        fn method<MethodTypeArg, const MethodConstArg: usize>(
            self,
            value_TypeArg: TypeArg,
            value_Type: Type<TypeArg, ConstArg>,
        ) {
        }
        fn associated_function<MethodTypeArg, const MethodConstArg: usize>(
            _self: Self,
            value_TypeArg: TypeArg,
            value_Type: Type<TypeArg, ConstArg>,
        ) {
        }
    }

    trait SubTrait<TypeArg, const ConstArg: usize>: Trait<TypeArg, ConstArg> {
        type AssocType: Trait<TypeArg, ConstArg>;
    }

    fn method_caller<
        MethodTypeArg,
        TypeArg,
        const ConstArg: usize,
        const MethodConstArg: usize,
        ImplTrait: Trait<TypeArg, ConstArg>,
    >(
        x: ImplTrait,
        value_TypeArg: TypeArg,
        value_Type: Type<TypeArg, ConstArg>,
    ) {
        x.method::<MethodTypeArg, MethodConstArg>(value_TypeArg, value_Type);
    }

    fn associated_function_caller<
        MethodTypeArg,
        TypeArg,
        const ConstArg: usize,
        const MethodConstArg: usize,
        ImplTrait: Trait<TypeArg, ConstArg>,
    >(
        x: ImplTrait,
        value_TypeArg: TypeArg,
        value_Type: Type<TypeArg, ConstArg>,
    ) {
        ImplTrait::associated_function::<MethodTypeArg, MethodConstArg>(
            x,
            value_TypeArg,
            value_Type,
        );
    }
}

mod type_alias_bounds_issue_707 {
    struct StructWithGenericBounds<T: Clone>(T);
    type SynonymA<T> = StructWithGenericBounds<T>;
    type SynonymB<T> = StructWithGenericBounds<(T, T)>;
}

// Related to PR 730
mod block_size {
    pub trait BlockSizeUser {
        type BlockSize;
    }
    pub trait ParBlocksSizeUser: BlockSizeUser {}

    pub trait BlockBackend: ParBlocksSizeUser {
        fn proc_block(block: Vec<<Self as BlockSizeUser>::BlockSize>);
    }
}

// issue 692
mod recursive_trait_with_assoc_type {
    pub trait Trait1 {
        type T: Trait1;
    }

    pub trait Trait2: Trait1 {
        type U;
    }
}

// issue 310
mod default_traits_parameters {
    trait Foo: Bar {
        type U;
    }
    trait Bar<T = <Self as Foo>::U> {}
}

// issue 1218
mod impl_expr_in_goal {
    trait T1 {
        type Assoc;
    }

    trait T2 {}

    impl<U: T1> T2 for U where U::Assoc: T2 {}
}

// issue 1290
mod implement_arithmetic_trait {
    struct Wrapped(i32);

    impl std::ops::Add for Wrapped {
        type Output = Wrapped;
        fn add(self, rhs: Self) -> Self::Output {
            Wrapped(self.0 + rhs.0)
        }
    }

    fn test(x: Wrapped, y: Wrapped) -> Wrapped {
        x + y
    }
}

// issue 1566
mod typenum_perf {
    use typenum::{IsLess, UInt, UTerm, B1};

    type I20 = UInt<I19, B1>;
    type I19 = UInt<I18, B1>;
    type I18 = UInt<I17, B1>;
    type I17 = UInt<I16, B1>;
    type I16 = UInt<I15, B1>;
    type I15 = UInt<I14, B1>;
    type I14 = UInt<I13, B1>;
    type I13 = UInt<I12, B1>;
    type I12 = UInt<I11, B1>;
    type I11 = UInt<I10, B1>;
    type I10 = UInt<I9, B1>;
    type I9 = UInt<I8, B1>;
    type I8 = UInt<I7, B1>;
    type I7 = UInt<I6, B1>;
    type I6 = UInt<I5, B1>;
    type I5 = UInt<I4, B1>;
    type I4 = UInt<I3, B1>;
    type I3 = UInt<I2, B1>;
    type I2 = UInt<I1, B1>;
    type I1 = UInt<I0, B1>;
    type I0 = UTerm;

    fn _f<T: IsLess<I20>>() {}
}

/// The bound on a generic associated type resolves in the impl. The backends still print the
/// associated type without its own parameters (issue 972), so the output does not type-check.
mod gat_bounds_issue_1907 {
    trait Bound {}
    trait WithGat {
        type Assoc<A: Bound>: Bound;
    }

    impl Bound for u8 {}
    impl<A: Bound> Bound for (u8, A) {}

    struct S;
    impl WithGat for S {
        type Assoc<A: Bound> = (u8, A);
    }
}
