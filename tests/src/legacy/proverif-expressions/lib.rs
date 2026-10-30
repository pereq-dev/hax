//! @fail(extraction): proverif(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001)
//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! Expressions and items the uniform-bitstring model has to encode: array
//! literals, indexing and slicing, field and nested-field assignment through
//! `&mut`, casts, literals of every kind, operators, associated constants,
//! trait default methods and `impl Trait` parameters.
#![allow(dead_code)]

fn array_literal(a: u8, b: u8) -> [u8; 3] {
    [a, b, a]
}

fn array_repeat(a: u8) -> [u8; 4] {
    [a; 4]
}

fn index(xs: [u8; 4], i: usize) -> u8 {
    xs[i]
}

fn slice(xs: &[u8]) -> &[u8] {
    &xs[1..3]
}

fn length(xs: &[u8]) -> usize {
    xs.len()
}

struct Inner {
    b: u8,
}

struct Outer {
    a: Inner,
    c: u8,
}

fn nested_projection(o: Outer) -> u8 {
    o.a.b
}

fn field_assign(mut o: Outer, x: u8) -> Outer {
    o.c = x;
    o
}

fn nested_field_assign(mut o: Outer, x: u8) -> Outer {
    o.a.b = x;
    o
}

fn set_c(o: &mut Outer, x: u8) {
    o.c = x;
}

fn through_mut_ref(mut o: Outer) -> u8 {
    set_c(&mut o, 7);
    o.c
}

struct Pair(u8, u8);

fn swap(p: Pair) -> Pair {
    Pair(p.1, p.0)
}

enum Code {
    A = 1,
    B = 20,
}

fn discriminant(c: Code) -> u8 {
    c as u8
}

fn cast(x: u32) -> u8 {
    x as u8
}

fn literals() -> (u128, i64, char, &'static str, f32, &'static [u8; 2], u8) {
    (
        340282366920938463463374607431768211455,
        -9223372036854775808,
        '\'',
        "quote \" (* not a comment *)",
        -1.5,
        b"ab",
        b'z',
    )
}

fn operators(a: u8, b: u8, c: bool) -> bool {
    let x = (a & b) | (a << 1) ^ !b;
    let y = a % 3 == b / 2;
    !c && (x != a || y) && a <= b
}

fn neg(x: i8) -> i8 {
    -x
}

fn struct_eq(a: Pair, b: Pair) -> bool
where
    Pair: PartialEq,
{
    a == b
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

trait Tagged {
    const TAG: u8;
    fn tag(&self) -> u8 {
        Self::TAG
    }
    fn double_tag(&self) -> u8 {
        self.tag() + self.tag()
    }
}

impl Tagged for Pair {
    const TAG: u8 = 42;
}

impl Tagged for Inner {
    const TAG: u8 = 7;
    fn tag(&self) -> u8 {
        self.b
    }
}

fn default_method(p: Pair) -> u8 {
    p.double_tag()
}

fn overridden_default(i: Inner) -> u8 {
    i.double_tag()
}

fn impl_trait_param(t: impl Tagged) -> u8 {
    t.tag()
}

fn generic_const<const N: usize>(xs: [u8; N]) -> usize {
    N
}

fn call_generic_const() -> usize {
    generic_const([1, 2, 3])
}

fn shadow_literal_name(nat_3: u8) -> u8 {
    nat_3 + 3
}

static GLOBAL: u8 = 5;

fn read_static() -> u8 {
    GLOBAL
}

// Const generics are erased, so their uses are rejected: `label::<16>()` and
// `label::<32>()` would otherwise be equal.
fn label<const N: usize>() -> usize {
    N
}

fn labels_differ() -> bool {
    label::<16>() == label::<32>()
}

struct Buf<const N: usize>;

impl<const N: usize> Buf<N> {
    const LEN: usize = N;
}

fn buf_len() -> usize {
    Buf::<4>::LEN
}

// A widening cast keeps the value, so it is the identity.
fn widen(x: u8) -> u64 {
    x as u64
}

// A narrowing or sign-changing cast may change the value: it is still the
// identity, with a warning.
fn narrow_roundtrip(x: u16) -> bool {
    (x as u8) as u16 == x
}

fn sign_change(x: i8) -> bool {
    (x as u8) == 255
}

// The model declares `Range`'s fields, so reading them gives back the bounds.
fn range_bounds(a: u8, b: u8) -> (u8, u8) {
    let r = a..b;
    (r.start, r.end)
}

#[hax_lib::opaque]
struct Hidden {
    field: u8,
}

// An opaque struct has no field accessors in the model: rejected.
fn read_hidden(h: Hidden) -> u8 {
    h.field
}

// `Range`'s fields are reordered to its declaration order: `(a, b)`.
fn range_literal(a: u8, b: u8) -> core::ops::Range<u8> {
    core::ops::Range { end: b, start: a }
}

// Assigning a tuple field rebuilds the whole tuple: `(t.0, x)`.
fn tuple_field_assign(mut t: (u8, u8), x: u8) -> (u8, u8) {
    t.1 = x;
    t
}

// Assignments through an index or a range are modelled.
fn update_index(mut xs: [u8; 4], x: u8) -> [u8; 4] {
    xs[0] = x;
    xs
}

fn update_range_to(mut xs: [u8; 4], ys: [u8; 2]) -> [u8; 4] {
    xs[..2].copy_from_slice(&ys);
    xs
}

// `size_of` depends on its type argument only, which the model erases: both
// calls are the same symbol, with a warning.
fn sizes() -> bool {
    core::mem::size_of::<u8>() == core::mem::size_of::<u64>()
}
