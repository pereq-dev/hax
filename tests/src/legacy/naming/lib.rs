//! @fail(tc): legacy-lean(1)
//! @fail(extraction): ssprove(HAX0001)
//! @fail(tc): fstar(228)

#![allow(dead_code)]
#![allow(non_camel_case_types)]

enum Foo {
    A,
    B { x: usize },
}
enum Foo2 {
    A,
    B { x: usize },
}
struct B;

struct C {
    x: usize,
}

struct X {}

fn mk_c() -> C {
    let _ = Foo::B { x: 3 };
    let _ = X {};
    C { x: 3 }
}

impl Foo {
    fn f(self) -> Foo {
        Foo::A
    }
}
impl B {
    fn f(self) -> B {
        B
    }
}

struct Foobar {
    a: Foo,
}

fn f(x: Foobar) -> usize {
    fn g() {
        impl B {
            fn g(self) -> usize {
                enum Foo {
                    A,
                    B { x: usize },
                }
                0usize
            }
        }
        impl Foo {
            fn g(self) -> usize {
                mod hello {
                    fn h() {}
                }
                1usize
            }
        }
    }
    x.a.g()
}

fn reserved_names(val: u8, noeq: u8, of: u8) -> u8 {
    val + noeq + of
}

struct Arity1<T>(T);

trait T1 {}
impl T1 for Foo {}
impl T1 for (Foo, u8) {}

trait T2_for_a {}
impl T2_for_a for Arity1<(Foo, u8)> {}
trait T3_e_for_a {}
impl T3_e_for_a for Foo {}

struct StructA {
    a: usize,
}
struct StructB {
    a: usize,
    b: usize,
}
struct StructC {
    a: usize,
}
struct StructD {
    a: usize,
    b: usize,
}
fn construct_structs(a: usize, b: usize) {
    let _ = StructA { a };
    let _ = StructB { a, b };
    let _ = StructC { a };
    let _ = StructD { a, b };
}

const INHERENT_CONSTANT: usize = 3;
trait FooTrait {
    const ASSOCIATED_CONSTANT: usize;
}

fn constants<T: FooTrait>() -> usize {
    <T as FooTrait>::ASSOCIATED_CONSTANT + INHERENT_CONSTANT
}

/// Test for ambiguous local names renaming: when two local vars are
/// ambiguous by name but not by their internal IDs.
/// Such situation can occur playing with *hygenic* macros.
/// Also, this happens with some internal Rustc rewrite. (e.g. assignment of tuples)
mod ambiguous_names {
/// @fail(extraction): ssprove(HAX0001)
    fn debug(label: u32, value: u32) {
        println!("[{}] a={}", label, value)
    }

    /// This macro surround a given expression with a let binding for
    /// an identifier `a` and a print of that `a`.
    macro_rules! introduce_binding_to_new_name_a {
        ($label:expr, $value:expr, $($e:tt)*) => {
            let a = $value;
            $($e)*
            debug($label, a)
        };
    }

    /// `f` stacks mutliple let bindings declaring different `a`s.
    fn f() {
        introduce_binding_to_new_name_a!(1, 104,
               introduce_binding_to_new_name_a!(2, 205,
                      introduce_binding_to_new_name_a!(3, 306, let a = 123;);
               );
        );
        debug(4, a)
    }

    /// `f` is expanded into `f_expand` below, while the execution of `f` gives:
    ///
    /// ```plaintext
    ///  [3] a=306
    ///  [2] a=205
    ///  [1] a=104
    ///  [last] a=123
    /// ```
    #[allow(unused)]
    fn f_expand() {
        let a = 104;
        let a = 205;
        let a = 306;
        let a = 123;
        debug(3, a);
        debug(2, a);
        debug(1, a);
        debug(0, a)
    }
}

/// From issue https://github.com/pereq-dev/hax/issues/839
fn string_shadows(string: &str, n: &str) {}

/// From issue https://github.com/pereq-dev/hax/issues/1411
mod functions_defined_in_trait_impls {
    struct A;

    impl PartialEq for A {
        fn eq(&self, other: &Self) -> bool {
            panic!()
        }
    }

    struct B;

    impl PartialEq for B {
        fn eq(&self, other: &Self) -> bool {
            panic!()
        }
    }
}

/// From issue https://github.com/pereq-dev/hax/issues/1450
fn items_under_closures() {
    let _: fn() -> () = || {
        fn nested_function() {}
        struct NestedStruct;
    };
    fn nested_function() {}
    struct NestedStruct;
}
