//! @fail(tc): fstar(2)
#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

// The following implementation of a function called from a `yield` statement
// (apparently requiring the Result and the `String` type or constructor)
// creates conditions where the `coroutine::StateTransform` MIR transform will
// drop all `Counter` `Coverage` statements from a MIR. `simplify.rs` has logic
// to handle this condition, and still report dead block coverage.
fn get_u32(val: bool) -> Result<u32, String> {
    if val {
        Ok(1) //
    } else {
        Err(String::from("some error")) //
    }
}

/// @fail(extraction): coq(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001), ssprove(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001), legacy-lean(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001), fstar(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001)
/// @fail(extraction): proverif(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001)
fn main() {
    let is_true = std::env::args().len() == 1;
    let mut coroutine = #[coroutine]
    || {
        yield get_u32(is_true);
        return "foo";
    };

    match Pin::new(&mut coroutine).resume(()) {
        CoroutineState::Yielded(Ok(1)) => {}
        _ => panic!("unexpected return from resume"),
    }
    match Pin::new(&mut coroutine).resume(()) {
        CoroutineState::Complete("foo") => {}
        _ => panic!("unexpected return from resume"),
    }
}
