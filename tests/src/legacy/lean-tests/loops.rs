//! Tests on loops
#![allow(dead_code)]
#![allow(unused_variables)]

// Simple for-loop
fn loop1() -> u32 {
    let mut x: u32 = 0;
    for i in 1..10 {
        x = x + i
    }
    x
}

// For-loop with a return
fn loop2() -> u32 {
    let mut x: u32 = 0;
    for i in 1..10 {
        if i == 5 {
            return x;
        }
        x = x + i;
    }
    x
}

/// For-loop with a spec
#[hax_lib::requires(y > 0)]
#[hax_lib::ensures(|res| res > 0)]
fn for_loop_with_spec(y: u64) -> u64 {
    let mut x: u64 = y;
    for i in 0..y {
        hax_lib::loop_invariant!(|i: u64| x > 0);
        if x % 5 == 0 {
            x = 200;
        } else {
            x = x % 5;
        }
    }
    x
}

/// while-loop
#[hax_lib::ensures(|r| r == 0)]
#[hax_lib::legacy_lean::proof_method::grind]
/// @fail(extraction): coq(HAX0001, HAX0001), ssprove(HAX0001)
fn while_loop1(s: u32) -> u32 {
    let mut x: u32 = s;
    while x > 0 {
        hax_lib::loop_decreases!(x);
        x = x - 1;
    }
    x
}

mod errors {
    enum Error {
        Foo,
        Bar(u32),
    }

    fn loop3() -> Result<u32, Error> {
        let mut x = 0;
        let end: u32 = 10;
        for i in 1..end {
            if i == 5 {
                return Err(Error::Foo);
            }
            x = x + 5
        }
        Ok(x)
    }

    fn loop4() -> Result<(u32, u32), Error> {
        let mut e = 0;
        let f = |()| 42;

        for i in 0..(f(())) {
            // verify degree bound
            if i > 10 {
                return Err(Error::Bar(e));
            }
            e = e + i
        }

        Ok((e, e))
    }
}
