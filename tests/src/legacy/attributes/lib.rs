//! @fail(tc): legacy-lean(1)
//! @fail(extraction): ssprove(HAX0001)
//! @fail(tc): fstar(47)
use hax_lib as hax;

// dummy max value
const u32_max: u32 = 90000;

/// A doc comment on `add3`
#[doc = "another doc comment on add3"]
#[hax::requires(x > 10 && y > 10 && z > 10 && x + y + z < u32_max)]
#[hax::ensures(|result| hax_lib::implies(true, result > 32))]
fn add3(x: u32, y: u32, z: u32) -> u32 {
    x + y + z
}

#[hax::requires(*x < 40 && *y < 300)]
#[hax::ensures(|result| *future(x) == *y && *future(y) == *x && result == *x + *y)]
fn swap_and_mut_req_ens(x: &mut u32, y: &mut u32) -> u32 {
    let x0 = *x;
    *x = *y;
    *y = x0;
    *x + *y
}

#[hax_lib::ensures(|_| true)]
fn issue_844(_x: &mut u8) {}

// From issue #845
mod ensures_on_arity_zero_fns {
    #[hax_lib::requires(true)]
    #[hax_lib::ensures(|_x| true)]
    fn doing_nothing() {}
    #[hax_lib::requires(true)]
    #[hax_lib::ensures(|x| x > 100)]
    fn basically_a_constant() -> u8 {
        127
    }
}

#[hax::lemma]
fn add3_lemma(x: u32) -> Proof<{ x <= 10 || x >= u32_max / 3 || add3(x, x, x) == x * 3 }> {}

fn dummy_function(x: u32) -> u32 {
    x
}

#[hax::lemma]
#[hax::fstar::smt_pat(x)]
fn apply_dummy_function_lemma(x: u32) -> Proof<{ x == dummy_function(x) }> {}

mod postprocess_with {
    #[hax_lib::fstar::postprocess_with("fun _ -> FStar.Tactics.trefl ()")]
    fn f() {}

    pub mod somewhere {
        pub fn some_hypothetical_tactic(some_param: u8) {}
    }
    use somewhere::some_hypothetical_tactic;

    #[hax_lib::fstar::postprocess_with(|()| some_hypothetical_tactic(12))]
    fn g() {}
}

#[hax::exclude]
pub fn f<'a, T>(c: bool, x: &'a mut T, y: &'a mut T) -> &'a mut T {
    if c {
        x
    } else {
        y
    }
}

#[hax::decreases(x)]
fn fib(x: usize) -> usize {
    if x <= 2 {
        x
    } else {
        fib(x - 1).wrapping_add(fib(x - 2))
    }
}

#[hax::attributes]
pub struct Foo {
    pub x: u32,
    #[refine(y > 3)]
    pub y: u32,
    #[refine(y + x + z > 3)]
    pub z: u32,
}

/// Regression test for https://github.com/pereq-dev/hax/issues/899: a refinement
/// on a struct field may mention a const generic (here `LEN`), both in a field
/// type and in the refinement formula itself.
#[hax::attributes]
pub struct FooConstGeneric<const LEN: usize> {
    #[hax_lib::refine(hax_lib::forall(|i: usize| hax_lib::implies(
        i < indices.len(),
        indices[i] as (usize) < 2
    )))]
    pub indices: [u8; LEN],
    #[refine(length < LEN)]
    pub length: usize,
}

#[hax::exclude]
impl Foo {
    fn g(&self) {}
}

impl Foo {
    #[hax::exclude]
    fn h(&self) {}
}

fn props() {
    hax_lib::assume!(hax_lib::fstar::prop!("True"));
    hax_lib::assert_prop!(hax_lib::fstar::prop!("True"));
}

#[hax::attributes]
mod refined_arithmetic {
    use core::ops::{Add, Mul};

    struct Foo(u8);

    impl Add for Foo {
        type Output = Foo;
        #[requires(self.0 < 255 - rhs.0)]
        fn add(self, rhs: Foo) -> Foo {
            Foo(self.0 + rhs.0)
        }
    }

    impl Mul for Foo {
        type Output = Foo;
        #[requires(rhs.0 == 0 || self.0 < 255 / rhs.0)]
        fn mul(self, rhs: Foo) -> Foo {
            Foo(self.0 * rhs.0)
        }
    }
}

mod refined_indexes {
    use hax_lib as hax;
    const MAX: usize = 10;
    struct MyArray(pub [u8; MAX]);

    #[hax::attributes]
    impl std::ops::Index<usize> for MyArray {
        type Output = u8;
        #[requires(index < MAX)]
        fn index(&self, index: usize) -> &Self::Output {
            &self[index]
        }
    }

    #[hax::exclude]
    impl std::ops::IndexMut<usize> for MyArray {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self[index]
        }
    }

    /// Triple dash comment
    /** Multiline double star comment Maecenas blandit accumsan feugiat.
    Done vitae ullamcorper est.
    Curabitur id dui eget sem viverra interdum. */
    fn mutation_example(
        use_generic_update_at: &mut MyArray,
        use_specialized_update_at: &mut [u8],
        specialized_as_well: &mut Vec<u8>,
    ) {
        use_generic_update_at[2] = 0;
        use_specialized_update_at[2] = 0;
        specialized_as_well[2] = 0;
    }
}
mod newtype_pattern {
    use hax_lib as hax;

    const MAX: usize = 10;
    #[hax::attributes]
    struct SafeIndex {
        #[refine(i < MAX)]
        i: usize,
    }
    impl SafeIndex {
        fn new(i: usize) -> Option<Self> {
            if i < MAX {
                Some(Self { i })
            } else {
                None
            }
        }
        fn as_usize(&self) -> usize {
            self.i
        }
    }

    impl<T> std::ops::Index<SafeIndex> for [T; MAX] {
        type Output = T;
        fn index(&self, index: SafeIndex) -> &Self::Output {
            &self[index.i]
        }
    }
}

#[hax::fstar::before(r#"let before_inlined_code = "example before""#)]
#[hax::fstar::after(r#"let inlined_code_after = "example after""#)]
fn inlined_code(foo: Foo) {
    const V: u8 = 12;
    let v_a = 13;
    hax::fstar!(
        r"let x = ${foo.x} in
          let $?{Foo {y, ..}} = $foo in
          $add3 ((fun _ -> 3ul) $foo) $v_a $V y
        "
    );
}

#[hax::fstar::before(r#"let before_1 = "example before 1""#)]
#[hax::fstar::before(r#"let before_2 = "example before 2""#)]
#[hax::fstar::before(r#"let before_3 = "example before 3""#)]
#[hax::fstar::after(r#"let after 1 = "example after 1""#)]
#[hax::fstar::after(r#"let after 2 = "example after 2""#)]
#[hax::fstar::after(r#"let after 3 = "example after 3""#)]
fn mutliple_before_after() {}

#[hax::fstar::replace(r#"unfold let $some_function _ = "hello from F*""#)]
fn some_function() -> String {
    String::from("hello from Rust")
}

mod future_self {
    #[derive(Eq, PartialEq)]
    struct Dummy;

    #[hax_lib::attributes]
    impl Dummy {
        #[hax_lib::ensures(|_| future(self) == self)]
        fn f(&mut self) {}
    }
}

/// The extra tuple an `ensures` receives carries the `future(_)` values and the
/// result. The engine lanes order it `(futures..., result)`; the aeneas/Lean lane
/// emits `(result, futures...)`, matching the tuple an Aeneas-extracted function
/// returns. Only the engine lanes run here, so these cases guard against that
/// ordering leaking into them. Each gives its futures and result *distinct* types,
/// which is what makes the order visible: `swap_and_mut_req_ens` above is
/// all-`u32`, so a permutation there renames binders and nothing else.
mod future_and_result_order {
    /// One `&mut` input, non-unit result: `(u32, bool)` in the engine lanes,
    /// `(bool, u32)` in the aeneas/Lean lane.
    #[hax_lib::ensures(|result| *future(x) == x.wrapping_add(1) && result == (*future(x) == 0))]
    fn one_mut_and_result(x: &mut u32) -> bool {
        *x = x.wrapping_add(1);
        *x == 0
    }

    /// Two `&mut` inputs: pins the futures' order relative to each other as
    /// well as relative to the result. `(u8, u64, bool)` in the engine lanes,
    /// `(bool, u8, u64)` in the aeneas/Lean lane.
    #[hax_lib::ensures(|result| *future(a) == a.wrapping_add(1)
        && *future(b) == b.wrapping_add(2)
        && result == (*future(a) == 0))]
    fn two_mut_and_result(a: &mut u8, b: &mut u64) -> bool {
        *a = a.wrapping_add(1);
        *b = b.wrapping_add(2);
        *a == 0
    }

    /// One `&mut` input, unit result: the result binder is dropped altogether,
    /// so what is left is futures-only and the reordering is a no-op — both
    /// lanes agree here.
    #[hax_lib::ensures(|_| *future(x) == x.wrapping_add(1))]
    fn one_mut_no_result(x: &mut u32) {
        *x = x.wrapping_add(1);
    }
}

mod replace_body {
    #[hax_lib::fstar::replace_body("magic ${x}")]
    fn f(x: u8, y: u8) -> u8 {
        1 + 2
    }
    struct Foo;
    impl Foo {
        #[hax_lib::fstar::replace_body("(magic (${self} <: $:{Self})) ${x}")]
        fn assoc_fn(&self, x: u8) {}
    }
    impl ToString for Foo {
        #[hax_lib::fstar::replace_body(r#""The type was $:{Self}""#)]
        fn to_string(&self) -> String {
            "Hello".into()
        }
    }
}

mod pre_post_on_traits_and_impls {
    use hax_lib::*;

    #[hax_lib::attributes]
    trait Operation {
        // we allow `hax_lib`, `::hax_lib` or no path at all
        #[hax_lib::requires(x.lift() <= int!(127))]
        #[ensures(|result| x.lift() * int!(2) == result.lift())]
        fn double(x: u8) -> u8;
    }

    struct ViaAdd;
    struct ViaMul;

    #[hax_lib::attributes]
    impl Operation for ViaAdd {
        #[::hax_lib::requires(x.lift() <= int!(127))]
        #[ensures(|result| x.lift() * int!(2) == result.lift())]
        fn double(x: u8) -> u8 {
            x + x
        }
    }

    #[hax_lib::attributes]
    impl Operation for ViaMul {
        #[requires(x.lift() <= int!(127))]
        #[::hax_lib::ensures(|result| x.lift() * int!(2) == result.lift())]
        fn double(x: u8) -> u8 {
            x * 2
        }
    }

    #[hax_lib::attributes]
    trait TraitWithRequiresAndEnsures {
        #[requires(x < 100)]
        #[ensures(|r| r > 88)]
        fn method(&self, x: u8) -> u8;
    }

    fn test<T: TraitWithRequiresAndEnsures>(x: T) -> u8 {
        x.method(99) - 88
    }
}

/// An minimal example of a model of math integers for F*
mod int_model {
    use super::hax;
    #[hax::fstar::replace(r#"unfold type $:{Int} = int"#)]
    #[derive(Copy, Clone)]
    struct Int(u128);

    #[hax::fstar::replace(r#"unfold let ${add} x y = x + y"#)]
    fn add(x: Int, y: Int) -> Int {
        Int(x.0 + y.0)
    }

    use std::ops::Sub;
    #[hax::fstar::replace(
        r#"
unfold instance impl: Core.Ops.Arith.t_Sub $:Int $:Int =
  {
    f_Output = $:Int;
    f_sub_pre = (fun (self: $:Int) (other: $:Int) -> true);
    f_sub_post = (fun (self: $:Int) (other: $:Int) (out: $:Int) -> true);
    f_sub = fun (self: $:Int) (other: $:Int) -> self + other
  }
"#
    )]
    impl Sub for Int {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Self(self.0 + other.0)
        }
    }
}

/// Illustration of the `refinement_type` macro that helps creating refinement types via thin newtype wrappers.
mod refinement_types {
    use hax_lib::*;

    #[hax_lib::refinement_type(|x| x >= MIN && x <= MAX)]
    pub struct BoundedU8<const MIN: u8, const MAX: u8>(u8);

    pub fn bounded_u8(x: BoundedU8<12, 15>, y: BoundedU8<10, 11>) -> BoundedU8<1, 23> {
        BoundedU8::new(x.get() + y.get())
    }

    /// Even `u8` numbers. Constructing pub Even values triggers static
    /// proofs in the extraction.
    #[hax_lib::refinement_type(|x| x % 2 == 0)]
    pub struct Even(u8);

    #[hax_lib::requires(x < 127)]
    pub fn double(x: u8) -> Even {
        Even::new(x + x)
    }

    #[hax_lib::requires(x < 127)]
    pub fn double_refine(x: u8) -> Even {
        (x + x).into_checked()
    }

    /// A string that contains no space.
    #[hax_lib::refinement_type(|x| !x.chars().any(|ch| ch == ' '))]
    pub struct NoE(String);

    /// A modular mutliplicative inverse
    #[hax_lib::refinement_type(|n| (n as u128 * MOD as u128) % MOD as u128 == 1)]
    pub struct ModInverse<const MOD: u32>(u32);

    /// A field element
    #[hax_lib::refinement_type(|x| x <= 2347)]
    pub struct FieldElement(u16);

    /// Example of a specific constraint on a value
    #[hax_lib::refinement_type(|x| x == 4 || x == 5 || x == 10 || x == 11)]
    pub struct CompressionFactor(u8);

    use hax_lib::int::*;
    /// Example of a refined int, that derives all common arithmetic operations
    hax_bounded_integers::refinement_int!(
        BoundedAbsI16<const B: usize>(i16, 2, |x| B.lift() < int!(32768) && x.lift() >= -B.lift() && x.lift() <= B.lift())
    );

    #[hax_lib::requires(M.lift() < int!(32768) && M.lift() == N.lift() * int!(2))]
    fn double_abs_i16<const N: usize, const M: usize>(x: BoundedAbsI16<N>) -> BoundedAbsI16<M> {
        (x * 2).into_checked()
    }
}
mod nested_refinement_elim {
    use hax_lib::*;
    #[refinement_type(|x| true)]
    pub struct DummyRefinement(u16);

    fn elim_twice(x: DummyRefinement) -> u16 {
        (DummyRefinement::new(x.get())).get()
    }
}

/// `ensures` and `requires` with inlined code (issue #825)
mod inlined_code_ensures_requires {
    #[hax_lib::requires(fstar!("forall i. FStar.Seq.index $v i <. ${254u8}"))]
    #[hax_lib::ensures(|()| {
        let future_v = future(v);
        fstar!("forall i. FStar.Seq.index ${future_v} i >. ${0u8}")
    })]
    fn increment_array(v: &mut [u8; 4]) {
        v[0] += 1;
        v[1] += 1;
        v[2] += 1;
        v[3] += 1;
    }
}

mod verifcation_status {
    #[hax_lib::fstar::verification_status(lax)]
    fn a_function_which_only_laxes() {
        assert!(/*very complicated stuff*/ false)
    }

    #[hax_lib::fstar::verification_status(panic_free)]
    #[hax_lib::ensures(|x|/*very complicated stuff*/false)]
    fn a_panicfree_function() -> u8 {
        let a = 3;
        let b = 6;
        a + b
    }

    #[hax_lib::fstar::verification_status(panic_free)]
    #[hax_lib::ensures(|x|/*very complicated stuff*/false)]
    fn another_panicfree_function() {
        let not_much = 0;
        let nothing = 0;
        let still_not_much = not_much + nothing;
    }
}

mod requires_mut {
    use hax_lib::*;

    #[hax_lib::attributes]
    trait Foo {
        #[hax_lib::requires(x.lift() + y.lift() < int!(254))]
        #[hax_lib::ensures(|output_variable| output_variable == *future(y))]
        fn f(x: u8, y: &mut u8) -> u8;

        fn g(x: u8, y: u8) -> u8;
        fn h(x: u8, y: u8);
        fn i(x: u8, y: &mut u8);
    }

    #[hax_lib::attributes]
    impl Foo for () {
        #[hax_lib::requires(x.lift() + y.lift() < int!(254))]
        #[hax_lib::ensures(|output_variable| output_variable == *future(y))]
        fn f(x: u8, y: &mut u8) -> u8 {
            *y += x;
            *y
        }

        #[hax_lib::requires(true)]
        #[hax_lib::ensures(|output_variable| output_variable == y)]
        fn g(x: u8, y: u8) -> u8 {
            y
        }

        #[hax_lib::requires(true)]
        #[hax_lib::ensures(|output_variable| output_variable == ())]
        fn h(x: u8, y: u8) {
            ()
        }

        #[hax_lib::requires(true)]
        #[hax_lib::ensures(|out| *future(y) == *y)]
        fn i(x: u8, y: &mut u8) {
            ()
        }
    }
}

mod issue_1266 {
    #[hax_lib::attributes]
    trait T {
        #[hax_lib::ensures(|_|true)]
        fn v(x: &mut Self);
    }
}

// Specifications on methods whose signature mentions an associated type of
// `Self`. Only `impl` blocks are supported: on a trait declaration, and for
// associated types the block does not define, the macro errors out.
mod issue_2089 {
    pub trait Super {
        type B;
    }

    #[hax_lib::attributes]
    pub trait T<X>: Super {
        type A;
        const C: u8;

        // No specification here: the signatures mention `Self::A`.
        fn f(x: &Self::A) -> u8;
        fn g(&self, x: Self::A) -> Self::A;
        fn h<Y: Into<Self::A>>(x: Self::B, y: Y, z: X) -> Self::A;

        #[hax_lib::requires(x > 0)]
        fn plain(x: u8) -> u8;
    }

    pub struct S;
    impl Super for S {
        type B = u8;
    }

    #[hax_lib::attributes]
    impl T<u16> for S {
        type A = u32;
        const C: u8 = 1;

        // `Self::A` in an argument type
        #[hax_lib::requires(true)]
        fn f(x: &Self::A) -> u8 {
            0
        }

        // `Self` and `Self::A` in argument and return types
        #[hax_lib::ensures(|result| true)]
        fn g(&self, x: Self::A) -> Self::A {
            x
        }

        // `Self::A` in the bounds of the generics of the method
        #[hax_lib::requires(true)]
        #[hax_lib::ensures(|result| true)]
        fn h<Y: Into<Self::A>>(x: u8, y: Y, z: u16) -> Self::A {
            y.into()
        }

        #[hax_lib::requires(x > 0)]
        fn plain(x: u8) -> u8 {
            x
        }
    }
}

mod props {
    use hax_lib::*;

    fn f(x: Prop, y: bool) -> Prop {
        let xprop: Prop = y.into();
        let p = y.lift() & xprop & y & y.to_prop();
        !(p | y).implies(forall(|x: u8| x <= u8::MAX) & exists(|x: u16| x > 300))
    }
}

mod reorder {
    #[hax_lib::attributes]
    struct Foo {
        #[order(40)]
        pub field_1: u8,
        #[hax_lib::order(31)]
        pub field_2: u8,
        pub field_3: u8,
        pub field_4: u8,
    }

    #[hax_lib::attributes]
    enum Bar {
        A {
            a_field_1: u8,
            a_field_2: u8,
            #[hax_lib::order(-42)]
            a_field_3: u8,
        },
        B {
            b_field_1: u8,
            #[hax_lib::order(42)]
            b_field_2: u8,
            b_field_3: u8,
        },
    }
}

mod issue_1276 {
    struct S(pub u8);

    #[hax_lib::attributes]
    impl S {
        #[hax_lib::requires(self.0 == 0 && self_ == self_1 && self_2 == 9)]
        fn f(&self, self_: u8, self_0: u8, self_1: u8, self_2: u8) {}
    }
}

mod issue_evit_57 {
    struct Foo;

    #[hax_lib::attributes]
    impl Foo {
        #[hax_lib::requires(true)]
        fn f(mut self) {}
    }
}
