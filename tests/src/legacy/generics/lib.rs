//! @fail(extraction): proverif(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001)
//! @fail(tc): legacy-lean(1)

#![allow(dead_code)]

fn dup<T: Clone>(x: T) -> (T, T) {
    (x.clone(), x.clone())
}

fn foo<const LEN: usize>(arr: [usize; LEN]) -> usize {
    let mut acc = LEN + 9;
    for i in 0..LEN {
        acc += arr[i];
    }
    acc
}

fn repeat<const LEN: usize, T: Copy>(x: T) -> [T; LEN] {
    [x; LEN]
}

fn call_f() -> usize {
    f::<10>(3) + 3
}
fn f<const N: usize>(x: usize) -> usize {
    N + N + x
}

fn call_g() -> usize {
    g::<3, [usize; 3]>([42, 3, 49]) + 3
}
fn g<const N: usize, T: Into<[usize; N]>>(arr: T) -> usize {
    arr.into().into_iter().max().unwrap_or(N) + N
}

trait Foo {
    fn const_add<const N: usize>(self) -> usize;
}

impl Foo for usize {
    fn const_add<const N: usize>(self) -> usize {
        self + N
    }
}

struct Bar;

impl Bar {
    fn inherent_impl_generics<T, const N: usize>(x: [T; N]) {}
}

/// Test defaults types and constants
mod defaults_generics {
    struct Defaults<T = (), const N: usize = 2>([T; N]);
    fn f(_: Defaults) {}
}

/// See https://github.com/pereq-dev/hax/issues/1176
mod impl_generics {
    struct Test();

    impl Test {
        fn set_ciphersuites<S>(&self, ciphers: impl IntoIterator<Item = S>) -> Result<(), ()>
        where
            S: AsRef<str>,
        {
            Ok(())
        }

        fn set_alpn_protocols<S>(&self, _protocols: impl IntoIterator<Item = S>) -> Result<(), ()>
        where
            S: AsRef<str>,
        {
            Ok(())
        }
    }
}

/// See https://github.com/pereq-dev/hax/issues/1289
mod assoc_const_param {
    struct Test<const N: usize>();

    impl<const N: usize> Test<N> {
        const A: Self = Self();
    }

    fn test() -> Test<1> {
        Test::<1>::A
    }
}
