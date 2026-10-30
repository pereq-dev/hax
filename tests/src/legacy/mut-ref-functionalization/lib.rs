//! @fail(tc): legacy-lean(1)
//! @fail(tc): fstar(72)

#![allow(dead_code)]

struct S {
    b: [u8; 5],
}

fn foo(mut lhs: S, rhs: &S) -> S {
    for i in 0..1 {
        lhs.b[i] += rhs.b[i];
    }

    lhs
}

impl S {
    fn update(&mut self, x: u8) {
        self.b[0] = x;
    }
}

fn index_mutation(x: core::ops::Range<usize>, a: &'static [u8]) {
    let mut v = vec![1];
    v[x].copy_from_slice(a);
    v[1] = 3;
}

fn index_mutation_unsize(mut x: [u8; 12]) -> u8 {
    x[4..5].copy_from_slice(&[1, 2]);
    42
}

fn build_vec() -> Vec<u8> {
    vec![1, 2, 3]
}

fn test_append() -> Vec<u8> {
    let mut vec1 = Vec::new();
    let mut vec2 = vec![1u8, 2, 3];
    vec1.append(&mut vec2);
    vec1.append(&mut build_vec());
    vec1
}

fn f() -> Vec<u8> {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.swap(0, 1);

    // `vec.swap(0, 1)` is desugared into:
    use std::ops::DerefMut;
    (&mut *(vec.deref_mut())).swap(0, 1);

    vec
}

struct Foo {
    field: Vec<u8>,
}
struct Pair<T> {
    a: T,
    b: Foo,
}

fn g(x: Pair<Vec<u8>>) -> Vec<u8> {
    let mut x = x;
    for i in 1..10 {
        x.a.push(i);
    }
    x.a.swap(0, 1);
    x.b.field.swap(0, 1);
    x.a
}

fn h(x: &mut u8) {
    *x += 10;
}

struct Bar {
    a: u8,
    b: u8,
}

fn i(bar: &mut Bar) -> u8 {
    (*bar).b += bar.a;
    h(&mut bar.a);
    bar.a + bar.b
}

fn j(x: &mut Bar) -> u8 {
    let out = 123;
    i(x) + out
}

fn k(
    vec: &mut Vec<u8>,
    _: &mut u16,
    /*test var shadowing*/ arg_1_wild: u8,
    _: &mut (),
) -> u64 {
    // test variable shadowing
    let arg_1_wild2 = vec[1];
    let arg_3_wild = vec[2];
    let arg_1_wild1 = vec[3];
    let arg_3_wild1 = vec[4];
    vec[0] = arg_1_wild + arg_3_wild + arg_1_wild1 + arg_3_wild1 + arg_1_wild;
    12345
}

trait FooTrait {
    fn z(&mut self);
}
impl FooTrait for Foo {
    fn z(&mut self) {}
}

fn array(x: &mut [u8; 10]) {
    x[1] = x[2];
}
