//! Equivalence tests for `core::result::Result::*`.

use crate::helpers::{Bumped, Keyed, keyed};

// Local helpers: function-level return-type annotations survive Aeneas
// extraction even when the variant only pins one of the two type params.
// Without these, `Ok(0u8)` leaves `E` unknown and `Err(0u8)` leaves `T`
// unknown, producing Lean output that fails to elaborate.
fn ok_u8_u8(v: u8) -> Result<u8, u8> {
    Ok(v)
}
fn err_u8_u8(e: u8) -> Result<u8, u8> {
    Err(e)
}

// ----- is_ok / is_err --------------------------------------------------------

#[hax_test::equiv]
pub fn test_is_ok_ok_zero() -> bool {
    ok_u8_u8(0).is_ok() == true
}

#[hax_test::equiv]
pub fn test_is_ok_ok_max() -> bool {
    ok_u8_u8(u8::MAX).is_ok() == true
}

#[hax_test::equiv]
pub fn test_is_ok_err_zero() -> bool {
    err_u8_u8(0).is_ok() == false
}

#[hax_test::equiv]
pub fn test_is_ok_err_max() -> bool {
    err_u8_u8(u8::MAX).is_ok() == false
}

#[hax_test::equiv]
pub fn test_is_err_ok_zero() -> bool {
    ok_u8_u8(0).is_err() == false
}

#[hax_test::equiv]
pub fn test_is_err_err_zero() -> bool {
    err_u8_u8(0).is_err() == true
}

#[hax_test::equiv]
pub fn test_is_err_err_max() -> bool {
    err_u8_u8(u8::MAX).is_err() == true
}

// ----- is_ok_and -------------------------------------------------------------

// ----- is_err_and ------------------------------------------------------------

// ----- as_ref ----------------------------------------------------------------

// TODO(as_ref): as_ref returns Result<&T, &E> which involves references-to-references
// through extraction; skip until references are exercised.

// ----- expect ----------------------------------------------------------------

#[hax_test::equiv]
pub fn test_expect_ok_zero() -> bool {
    ok_u8_u8(0).expect("msg") == 0
}

#[hax_test::equiv]
pub fn test_expect_ok_max() -> bool {
    ok_u8_u8(u8::MAX).expect("msg") == u8::MAX
}

#[hax_test::equiv]
pub fn test_expect_ok_mid() -> bool {
    ok_u8_u8(42).expect("msg") == 42
}

// ----- unwrap ----------------------------------------------------------------

#[hax_test::equiv]
pub fn test_unwrap_ok_zero() -> bool {
    ok_u8_u8(0).unwrap() == 0
}

#[hax_test::equiv]
pub fn test_unwrap_ok_max() -> bool {
    ok_u8_u8(u8::MAX).unwrap() == u8::MAX
}

#[hax_test::equiv]
pub fn test_unwrap_ok_mid() -> bool {
    ok_u8_u8(7).unwrap() == 7
}

// ----- unwrap_err ------------------------------------------------------------

#[hax_test::equiv]
pub fn test_unwrap_err_err_zero() -> bool {
    err_u8_u8(0).unwrap_err() == 0
}

#[hax_test::equiv]
pub fn test_unwrap_err_err_max() -> bool {
    err_u8_u8(u8::MAX).unwrap_err() == u8::MAX
}

#[hax_test::equiv]
pub fn test_unwrap_err_err_mid() -> bool {
    err_u8_u8(7).unwrap_err() == 7
}

// ----- unwrap_or -------------------------------------------------------------

#[hax_test::equiv]
pub fn test_unwrap_or_ok_zero() -> bool {
    ok_u8_u8(0).unwrap_or(99) == 0
}

#[hax_test::equiv]
pub fn test_unwrap_or_ok_max() -> bool {
    ok_u8_u8(u8::MAX).unwrap_or(0) == u8::MAX
}

#[hax_test::equiv]
pub fn test_unwrap_or_err_default_zero() -> bool {
    err_u8_u8(99).unwrap_or(0) == 0
}

#[hax_test::equiv]
pub fn test_unwrap_or_err_default_max() -> bool {
    err_u8_u8(0).unwrap_or(u8::MAX) == u8::MAX
}

// ----- unwrap_or_else --------------------------------------------------------

// ----- unwrap_or_default -----------------------------------------------------

#[hax_test::equiv]
pub fn test_unwrap_or_default_ok_zero() -> bool {
    ok_u8_u8(0).unwrap_or_default() == 0
}

#[hax_test::equiv]
pub fn test_unwrap_or_default_ok_max() -> bool {
    ok_u8_u8(u8::MAX).unwrap_or_default() == u8::MAX
}

#[hax_test::equiv]
pub fn test_unwrap_or_default_err() -> bool {
    err_u8_u8(u8::MAX).unwrap_or_default() == 0
}

// ----- map -------------------------------------------------------------------

// ----- map_or ----------------------------------------------------------------

// ----- map_or_else -----------------------------------------------------------

// ----- map_or_default --------------------------------------------------------

// TODO(result-method-missing: `Result::map_or_default` missing from extracted
// Lean (see map).)

// ----- map_err ---------------------------------------------------------------
// `Result::map_err` is now modeled and extracted, but can't be exercised here:
// the model takes CoreModels' `FnOnce` while the equiv call site supplies
// Aeneas's `BuiltinFnOnce`, and the two don't unify.
// TODO(closure-extraction): re-enable once core-models closures extract as
// `core.ops.function.FnOnce` in the equiv pipeline.

// ----- inspect / inspect_err -------------------------------------------------

// ----- ok --------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_ok_ok_zero() -> bool {
    ok_u8_u8(0).ok().unwrap_or(99) == 0
}

#[hax_test::equiv]
pub fn test_ok_ok_max() -> bool {
    ok_u8_u8(u8::MAX).ok().unwrap_or(0) == u8::MAX
}

#[hax_test::equiv]
pub fn test_ok_err() -> bool {
    err_u8_u8(7).ok().is_none()
}

// ----- err -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_err_err_zero() -> bool {
    err_u8_u8(0).err().unwrap_or(99) == 0
}

#[hax_test::equiv]
pub fn test_err_err_max() -> bool {
    err_u8_u8(u8::MAX).err().unwrap_or(0) == u8::MAX
}

#[hax_test::equiv]
pub fn test_err_ok() -> bool {
    ok_u8_u8(7).err().is_none()
}

// ----- and -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_and_ok_ok() -> bool {
    ok_u8_u8(0).and(ok_u8_u8(7)).unwrap_or(99) == 7
}

#[hax_test::equiv]
pub fn test_and_ok_err() -> bool {
    ok_u8_u8(0).and(err_u8_u8(42)).unwrap_err() == 42
}

#[hax_test::equiv]
pub fn test_and_err_ok() -> bool {
    err_u8_u8(99).and(ok_u8_u8(0)).unwrap_err() == 99
}

#[hax_test::equiv]
pub fn test_and_err_err() -> bool {
    err_u8_u8(u8::MAX).and(err_u8_u8(0)).unwrap_err() == u8::MAX
}

// ----- and_then --------------------------------------------------------------

// ----- or --------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_or_ok_ok() -> bool {
    ok_u8_u8(0).or(ok_u8_u8(99)).unwrap_or(7) == 0
}

#[hax_test::equiv]
pub fn test_or_ok_err() -> bool {
    ok_u8_u8(u8::MAX).or(err_u8_u8(42)).unwrap_or(0) == u8::MAX
}

#[hax_test::equiv]
pub fn test_or_err_ok() -> bool {
    err_u8_u8(99).or(ok_u8_u8(42)).unwrap_or(0) == 42
}

#[hax_test::equiv]
pub fn test_or_err_err() -> bool {
    err_u8_u8(99).or(err_u8_u8(7)).unwrap_err() == 7
}

// ----- or_else ---------------------------------------------------------------

// ----- cloned ----------------------------------------------------------------

// TODO(result-cloned-shape): the model's `cloned` takes `self` and returns
// `Result<T, E>` (an identity over our clone-by-value `Clone`). Std's
// `Result::cloned` lives on `Result<&T, E>` and is unstable, so calling
// `.cloned()` directly from the Rust side does not type-check on stable.
// Revisit when references/shared semantics get a typed test surface.

// ----- transpose -------------------------------------------------------------

// Helpers for Result<Option<u8>, u8>: typed via function return type.
fn ok_some_u8(v: u8) -> Result<Option<u8>, u8> {
    Ok(Some(v))
}
fn ok_none_u8() -> Result<Option<u8>, u8> {
    let mut x: Option<u8> = Some(0);
    x.take();
    Ok(x)
}
fn err_outer_u8(e: u8) -> Result<Option<u8>, u8> {
    Err(e)
}

#[hax_test::equiv]
pub fn test_transpose_ok_some_zero() -> bool {
    match ok_some_u8(0).transpose() {
        Some(Ok(v)) => v == 0,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn test_transpose_ok_some_max() -> bool {
    match ok_some_u8(u8::MAX).transpose() {
        Some(Ok(v)) => v == u8::MAX,
        _ => false,
    }
}

#[hax_test::equiv]
pub fn test_transpose_ok_none() -> bool {
    ok_none_u8().transpose().is_none()
}

#[hax_test::equiv]
pub fn test_transpose_err() -> bool {
    match err_outer_u8(7).transpose() {
        Some(Err(e)) => e == 7,
        _ => false,
    }
}

// ----- flatten ---------------------------------------------------------------

// TODO(result-flatten-unstable): `Result::flatten` is gated behind the
// `result_flattening` feature on stable std. The model defines `flatten`
// directly so the Lean side works, but the Rust side cannot call
// `r.flatten()` on stable. Revisit once `result_flattening` stabilises.

// ----- `?` operator (Try::branch + FromResidual::from_residual) --------------
// `?` is stable even though `Try`/`FromResidual` aren't, so it's the only way to
// drive the whole desugaring against our model end-to-end.

fn question_identity(a: Result<u8, u8>) -> Result<u8, u8> {
    let x = a?;
    Ok(x)
}

#[hax_test::equiv]
pub fn test_question_propagates_ok() -> bool {
    match question_identity(ok_u8_u8(7)) {
        Ok(v) => v == 7,
        Err(_) => false,
    }
}

#[hax_test::equiv]
pub fn test_question_propagates_err() -> bool {
    match question_identity(err_u8_u8(3)) {
        Ok(_) => false,
        Err(e) => e == 3,
    }
}

#[hax_test::equiv]
pub fn test_expect_err() -> bool {
    err_u8_u8(4).expect_err("expected an error") == 4
}

// ----- methods newly reachable from Lean -------------------------------------

#[hax_test::equiv]
pub fn test_is_ok_and_true() -> bool {
    ok_u8_u8(7).is_ok_and(|v| v == 7)
}

#[hax_test::equiv]
pub fn test_is_ok_and_false_on_err() -> bool {
    !err_u8_u8(7).is_ok_and(|v| v == 7)
}

#[hax_test::equiv]
pub fn test_is_err_and_true() -> bool {
    err_u8_u8(3).is_err_and(|e| e == 3)
}

#[hax_test::equiv]
pub fn test_map_ok() -> bool {
    match ok_u8_u8(4).map(|v| v + 1) {
        Ok(v) => v == 5,
        Err(_) => false,
    }
}

#[hax_test::equiv]
pub fn test_map_leaves_err() -> bool {
    match err_u8_u8(4).map(|v| v + 1) {
        Ok(_) => false,
        Err(e) => e == 4,
    }
}

#[hax_test::equiv]
pub fn test_map_or_ok() -> bool {
    ok_u8_u8(4).map_or(0, |v| v + 1) == 5
}

#[hax_test::equiv]
pub fn test_map_or_err_uses_default() -> bool {
    err_u8_u8(4).map_or(9, |v| v + 1) == 9
}

#[hax_test::equiv]
pub fn test_map_or_else_err_branch() -> bool {
    err_u8_u8(4).map_or_else(|e| e + 1, |v| v) == 5
}

#[hax_test::equiv]
pub fn test_unwrap_or_else_uses_err() -> bool {
    err_u8_u8(4).unwrap_or_else(|e| e + 1) == 5
}

#[hax_test::equiv]
pub fn test_and_then_chains() -> bool {
    match ok_u8_u8(4).and_then(|v| ok_u8_u8(v + 1)) {
        Ok(v) => v == 5,
        Err(_) => false,
    }
}

#[hax_test::equiv]
pub fn test_and_then_short_circuits() -> bool {
    match err_u8_u8(4).and_then(|v| ok_u8_u8(v + 1)) {
        Ok(_) => false,
        Err(e) => e == 4,
    }
}

#[hax_test::equiv]
pub fn test_or_else_recovers() -> bool {
    match err_u8_u8(4).or_else(|e| ok_u8_u8(e + 1)) {
        Ok(v) => v == 5,
        Err(_) => false,
    }
}

#[hax_test::equiv]
pub fn test_inspect_returns_self() -> bool {
    match ok_u8_u8(4).inspect(|_| ()) {
        Ok(v) => v == 4,
        Err(_) => false,
    }
}

#[hax_test::equiv]
pub fn test_inspect_err_returns_self() -> bool {
    match err_u8_u8(4).inspect_err(|_| ()) {
        Ok(_) => false,
        Err(e) => e == 4,
    }
}

// ----- dictionary-applying tests ---------------------------------------------

#[hax_test::equiv]
pub fn test_eq_ok_goes_through_the_dictionary() -> bool {
    let a: Result<Keyed, u8> = Ok(keyed(5, 1));
    let b: Result<Keyed, u8> = Ok(keyed(5, 2));
    a == b
}

#[hax_test::equiv]
pub fn test_eq_err_goes_through_the_dictionary() -> bool {
    let a: Result<u8, Keyed> = Err(keyed(5, 1));
    let b: Result<u8, Keyed> = Err(keyed(5, 2));
    a == b
}

#[hax_test::equiv]
pub fn test_eq_ok_differing_keys() -> bool {
    let a: Result<Keyed, u8> = Ok(keyed(5, 1));
    let b: Result<Keyed, u8> = Ok(keyed(6, 1));
    (a == b) == false
}

// The mismatch arm never reaches either dictionary.
#[hax_test::equiv]
pub fn test_eq_ok_vs_err() -> bool {
    let a: Result<Keyed, Keyed> = Ok(keyed(5, 1));
    let b: Result<Keyed, Keyed> = Err(keyed(5, 1));
    (a == b) == false
}

#[hax_test::equiv]
pub fn test_clone_ok_applies_element_clone() -> bool {
    let a: Result<Bumped, Bumped> = Ok(Bumped(1));
    match a.clone() {
        Ok(b) => b.0 == 2,
        Err(_) => false,
    }
}

#[hax_test::equiv]
pub fn test_clone_err_applies_element_clone() -> bool {
    let a: Result<Bumped, Bumped> = Err(Bumped(1));
    match a.clone() {
        Err(b) => b.0 == 2,
        Ok(_) => false,
    }
}
