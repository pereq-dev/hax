// Tests on traits
#![allow(dead_code)]
#![allow(unused_variables)]

// Simple trait
mod basic {
    trait T1 {
        fn f1(&self) -> usize;
        fn f2(&self, other: &Self) -> usize;
    }

    // Simple Impl
    struct S;

    impl T1 for S {
        fn f1(&self) -> usize {
            42
        }

        fn f2(&self, other: &Self) -> usize {
            43
        }
    }

    // Simple ImplExpr
    fn f<T: T1>(x: T) -> usize {
        x.f1() + x.f2(&x)
    }
}

// Bounds on parameters and on self
mod bounds {
    trait T1 {
        fn f1(&self) -> usize;
    }
    trait T2 {
        fn f2(&self) -> usize;
    }

    trait Test<T: T1>: T2 {
        fn f_test(&self, x: &T) -> usize;
    }

    struct S1;
    impl T1 for S1 {
        fn f1(&self) -> usize {
            0
        }
    }

    struct S2;
    impl T2 for S2 {
        fn f2(&self) -> usize {
            1
        }
    }

    impl Test<S1> for S2 {
        fn f_test(&self, x: &S1) -> usize {
            x.f1() + self.f2() + 1
        }
    }

    fn test(x1: S1, x2: S2) -> usize {
        x2.f_test(&x1) + x1.f1()
    }
}

mod associated_types {
    trait T1 {
        type T;
        fn f(&self, x: Self::T) -> Self::T;
    }

    trait T2 {
        type T: T1;
        fn f(&self, x: Self::T) -> usize;
    }

    trait Foo<T> {}
    trait Bar {}

    trait T3 {
        type T: Bar;
        type Tp<A: Bar>: Foo<Self::T>;
        fn f<A: Bar>(&self, x: Self::T, y: Self::Tp<A>) -> usize;
    }

    struct S {}
    impl T1 for S {
        type T = i32;

        fn f(&self, x: Self::T) -> Self::T {
            2121
        }
    }
    impl T2 for S {
        type T = S;

        fn f(&self, x: Self::T) -> usize {
            21
        }
    }

    impl Bar for i16 {}
    impl<A> Foo<i16> for (u32, A) {}

    impl T3 for S {
        type T = i16;

        type Tp<A: Bar> = (u32, A);

        fn f<A: Bar>(&self, x: Self::T, y: Self::Tp<A>) -> usize {
            12
        }
    }

    trait Chain0 {}

    trait Chain1 {
        type A: Chain0;
        type B: Chain0;
    }

    trait Chain2: Chain1 {}

    trait Chain3: Chain2 {
        fn f() -> Self::A;
    }

    impl Chain0 for u8 {}
    impl Chain1 for u8 {
        type A = u8;
        type B = u8;
    }
    impl Chain2 for u8 {}
    impl Chain3 for u8 {
        fn f() -> u8 {
            0
        }
    }
}

mod overlapping_methods {

    trait T1 {
        fn f(&self) -> usize;
    }
    trait T2 {
        fn f(&self) -> usize;
    }
    trait T3 {
        fn f(&self) -> usize;
    }
    impl T1 for u32 {
        fn f(&self) -> usize {
            0
        }
    }
    impl T2 for u32 {
        fn f(&self) -> usize {
            1
        }
    }
    impl T3 for u32 {
        fn f(&self) -> usize {
            2
        }
    }
    fn test() -> usize {
        let x: u32 = 9;
        T1::f(&x) + T2::f(&x) + T3::f(&x)
    }
}

mod inheritance {
    trait T1 {
        fn f1(&self) -> usize;
    }
    trait T2 {
        fn f2(&self) -> usize;
    }
    trait T3: T2 + T1 {
        fn f3(&self) -> usize;
    }
    trait Tp1 {
        fn f1(&self) -> usize;
    }
    trait Tp2: Tp1 + T3 {
        fn fp2(&self) -> usize;
    }

    struct S {}
    impl T1 for S {
        fn f1(&self) -> usize {
            1
        }
    }
    impl T2 for S {
        fn f2(&self) -> usize {
            2
        }
    }
    impl T3 for S {
        fn f3(&self) -> usize {
            3
        }
    }

    impl Tp1 for S {
        fn f1(&self) -> usize {
            10
        }
    }

    impl Tp2 for S {
        fn fp2(&self) -> usize {
            Tp1::f1(self) + T1::f1(self) + T2::f2(self) + T3::f3(self)
        }
    }
    fn test() -> usize {
        let s = S {};
        s.f3() + 1
    }
}

mod default {

    /// @fail(extraction): ssprove(HAX0008), coq(HAX0008), fstar(HAX0008), proverif(HAX0008)
    trait Easy {
        fn dft(&self) -> usize {
            32
        }
    }

    impl Easy for usize {
        fn dft(&self) -> usize {
            self + 1
        }
    }

    impl Easy for u32 {}

    /// @fail(extraction): ssprove(HAX0008), coq(HAX0008), proverif(HAX0008), fstar(HAX0008)
    trait T1 {
        fn f1(&self) -> usize;
        fn f2(&self) -> usize {
            1
        }
        fn f3<A>(&self, x: &A) -> usize {
            1
        }
        fn f4<A: Easy>(&self, x: &A) -> usize {
            x.dft() + 1
        }
    }

    struct S<A>(usize, A);

    // Override
    impl T1 for S<usize> {
        fn f1(&self) -> usize {
            self.0 + self.1
        }

        fn f2(&self) -> usize {
            self.1
        }
    }

    impl T1 for S<bool> {
        fn f1(&self) -> usize {
            if self.1 { self.0 } else { 9 }
        }

        fn f2(&self) -> usize {
            self.0 + 1
        }
    }

    // No override
    impl T1 for S<String> {
        fn f1(&self) -> usize {
            0
        }
    }
}

mod trait_level_args {
    trait T1<A, B> {
        fn f1<C, D>(&self) -> (); // A and B do not appear
        fn f2<C, D>(&self, x: &A) -> (); // A appears
        fn f3<C, D>(&self, x: &A, y: &B) -> (); // Both appear
    }

    impl T1<u32, u64> for usize {
        fn f1<C, D>(&self) {}
        fn f2<C, D>(&self, x: &u32) {}
        fn f3<C, D>(&self, x: &u32, y: &u64) {}
    }

    fn test<A, B, C, D, U: T1<A, B>>(x: U, a: &A, b: &B) -> () {
        x.f1::<C, D>();
        x.f2::<C, D>(a);
        x.f3::<C, D>(a, b);
    }
}

mod trait_with_constraints {

    trait T1 {}

    trait T2 {
        fn func(&self) -> bool
        where
            Self: T1;
    }

    impl<A: T1> T2 for A {
        fn func(&self) -> bool
        where
            A: T1,
        {
            true
        }
    }
}

mod associated_constant {
    /// @fail(extraction): proverif(HAX0008)
    pub trait Foo {
        const f: bool;
        const x: u8 = 0;
    }

    pub struct Bar;

    impl Foo for Bar {
        const f: bool = true;
        const x: u8 = 1 + 1;
    }

    // https://github.com/pereq-dev/hax/issues/1940
    /// @fail(extraction): proverif(HAX0008)
    trait Baz {
        const One: u32 = 1;
    }

    fn foo<F: Baz>(n: u32) -> u32 {
        n + F::One
    }
}

mod methods_hoisting {
    trait T1<U>: Sized {
        type A;
        fn f<T: T0, const C: u8>(x: T, y: U) -> (T, U, Self, Self::A, u16);
        fn g<T>(x: T) -> T;
        const AC: u16;
    }

    trait T0 {}

    impl<U: T0> T1<U> for u8 {
        type A = u64;
        fn f<T: T0, const C: u8>(x: T, y: U) -> (T, U, u8, Self::A, u16) {
            (<Self as T1<U>>::g(x), y, 2u8 + C, 3u64, <Self as T1<U>>::AC)
        }
        fn g<T>(x: T) -> T {
            x
        }
        const AC: u16 = 5u16;
    }
}
