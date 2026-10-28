//! Tests on monadic encoding
#![allow(dead_code)]
#![allow(unused_variables)]

struct S {
    f: u32,
}

fn test() {
    let _ = 9; // value
    let _ = 9 + 9; // computation
    let _ = S { f: 9 }; // constructors are values
    let _ = S { f: 9 + 9 }; // computation within a value
    let _ = (S { f: 9 + 9 }).f; // projections are values
    let _ = (S { f: 9 + 9 }).f + 9; // projections are values
    let _ = if true { 3 + 4 } else { 3 - 4 }; // ite expects value for condition
    let _ = if 9 + 9 == 0 { 3 + 4 } else { 3 - 4 }; // ite expects value for condition
    let _ = if true {
        let x = 9;
        3 + x;
    } else {
        let y = 19;
        3 + y - 4;
    };
}

mod trait_constants {
    // https://github.com/pereq-dev/hax/issues/1928
    trait Foo {
        const F : u32;
    }

    trait Bar {
        const B : u32;
    }

    struct Baz;

    impl Foo for Baz {
        const F : u32 = 1;
    }                                                                                                                                      
                                                                                                                                        
    impl Bar for Baz {
        const B : u32 = Self::F - 1;
    }
}
