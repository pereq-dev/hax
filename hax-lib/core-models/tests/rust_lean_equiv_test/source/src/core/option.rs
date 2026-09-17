//! Equivalence tests for `core::option::Option::*`.

use crate::helpers::none_u8;
use crate::helpers::{Bumped, Keyed, keyed};

// ----- is_some / is_none -----------------------------------------------------

#[hax_test::equiv]
pub fn test_is_some_none_u8() -> bool {
    none_u8().is_some() == false
}

#[hax_test::equiv]
pub fn test_is_some_some_zero() -> bool {
    Some(0u8).is_some() == true
}

#[hax_test::equiv]
pub fn test_is_some_some_max() -> bool {
    Some(u8::MAX).is_some() == true
}

#[hax_test::equiv]
pub fn test_is_none_none_u8() -> bool {
    none_u8().is_none() == true
}

#[hax_test::equiv]
pub fn test_is_none_some_zero() -> bool {
    Some(0u8).is_none() == false
}

#[hax_test::equiv]
pub fn test_is_none_some_max() -> bool {
    Some(u8::MAX).is_none() == false
}

// ----- is_some_and -----------------------------------------------------------

#[hax_test::equiv]
pub fn test_is_some_and_some_true() -> bool {
    Some(4u8).is_some_and(|x| x > 0) == true
}

#[hax_test::equiv]
pub fn test_is_some_and_some_false() -> bool {
    Some(0u8).is_some_and(|x| x > 0) == false
}

#[hax_test::equiv]
pub fn test_is_some_and_none() -> bool {
    none_u8().is_some_and(|x| x > 0) == false
}

// ----- is_none_or ------------------------------------------------------------

#[hax_test::equiv]
pub fn test_is_none_or_some_true() -> bool {
    Some(4u8).is_none_or(|x| x > 0) == true
}

#[hax_test::equiv]
pub fn test_is_none_or_some_false() -> bool {
    Some(0u8).is_none_or(|x| x > 0) == false
}

#[hax_test::equiv]
pub fn test_is_none_or_none() -> bool {
    none_u8().is_none_or(|x| x > 0) == true
}

// ----- as_ref ----------------------------------------------------------------

// TODO(as_ref): as_ref returns Option<&T> which involves references-to-references
// through extraction; skip until references are exercised.

// ----- expect ----------------------------------------------------------------

#[hax_test::equiv]
pub fn test_expect_some_zero() -> bool {
    Some(0u8).expect("msg") == 0
}

#[hax_test::equiv]
pub fn test_expect_some_max() -> bool {
    Some(u8::MAX).expect("msg") == u8::MAX
}

#[hax_test::equiv]
pub fn test_expect_some_mid() -> bool {
    Some(42u8).expect("msg") == 42
}

// ----- unwrap ----------------------------------------------------------------

#[hax_test::equiv]
pub fn test_unwrap_some_zero() -> bool {
    Some(0u8).unwrap() == 0
}

#[hax_test::equiv]
pub fn test_unwrap_some_max() -> bool {
    Some(u8::MAX).unwrap() == u8::MAX
}

#[hax_test::equiv]
pub fn test_unwrap_some_mid() -> bool {
    Some(7u8).unwrap() == 7
}

// ----- unwrap_or -------------------------------------------------------------

#[hax_test::equiv]
pub fn test_unwrap_or_some_zero() -> bool {
    Some(0u8).unwrap_or(42) == 0
}

#[hax_test::equiv]
pub fn test_unwrap_or_some_max() -> bool {
    Some(u8::MAX).unwrap_or(0) == u8::MAX
}

#[hax_test::equiv]
pub fn test_unwrap_or_none_default_zero() -> bool {
    none_u8().unwrap_or(0) == 0
}

#[hax_test::equiv]
pub fn test_unwrap_or_none_default_max() -> bool {
    none_u8().unwrap_or(u8::MAX) == u8::MAX
}

// ----- unwrap_or_else --------------------------------------------------------

#[hax_test::equiv]
pub fn test_unwrap_or_else_some_zero() -> bool {
    Some(0u8).unwrap_or_else(|| 42) == 0
}

#[hax_test::equiv]
pub fn test_unwrap_or_else_some_max() -> bool {
    Some(u8::MAX).unwrap_or_else(|| 0) == u8::MAX
}

#[hax_test::equiv]
pub fn test_unwrap_or_else_none() -> bool {
    none_u8().unwrap_or_else(|| 7) == 7
}

// ----- unwrap_or_default -----------------------------------------------------

#[hax_test::equiv]
pub fn test_unwrap_or_default_some_zero() -> bool {
    Some(0u8).unwrap_or_default() == 0
}

#[hax_test::equiv]
pub fn test_unwrap_or_default_some_max() -> bool {
    Some(u8::MAX).unwrap_or_default() == u8::MAX
}

#[hax_test::equiv]
pub fn test_unwrap_or_default_none() -> bool {
    none_u8().unwrap_or_default() == 0
}

// ----- map -------------------------------------------------------------------

// TODO(closure-extraction-map): `Option::map` returns `Option<U>`; observing the
// result requires either `Option::PartialEq` (blocked by option-eq-extraction)
// or destructuring through `match`, but the latter goes back through
// `Option::PartialEq` once we compare the inner value. Revisit alongside
// option-eq-extraction.

#[hax_test::equiv]
pub fn test_map_some_via_unwrap_or() -> bool {
    Some(3u8).map(|x| x + 1).unwrap_or(0) == 4
}

#[hax_test::equiv]
pub fn test_map_none_via_unwrap_or() -> bool {
    none_u8().map(|x| x + 1).unwrap_or(99) == 99
}

#[hax_test::equiv]
pub fn test_map_some_is_some() -> bool {
    Some(3u8).map(|x| x + 1).is_some()
}

#[hax_test::equiv]
pub fn test_map_none_is_none() -> bool {
    none_u8().map(|x| x + 1).is_none()
}

// ----- map_or ----------------------------------------------------------------

#[hax_test::equiv]
pub fn test_map_or_some_zero() -> bool {
    Some(3u8).map_or(0u8, |x| x + 1) == 4
}

#[hax_test::equiv]
pub fn test_map_or_some_max() -> bool {
    Some(0u8).map_or(99u8, |x| x + 1) == 1
}

#[hax_test::equiv]
pub fn test_map_or_none() -> bool {
    none_u8().map_or(99u8, |x| x + 1) == 99
}

// ----- map_or_else -----------------------------------------------------------

// TODO(closure-extraction-order): `Option::map_or_else` takes (default: D, f: F)
// in source order but the model lists the trait instances in (F, D) order, so
// Aeneas emits them swapped and the Lean elaboration fails for the `None`
// branch. Revisit once the closure-instance ordering is fixed.

// ----- map_or_default --------------------------------------------------------

// TODO(map_or_default-unstable): `Option::map_or_default` is gated behind
// `result_option_map_or_default` on stable. Revisit once it stabilises.

// ----- ok_or -----------------------------------------------------------------

#[hax_test::equiv]
pub fn test_ok_or_some_zero() -> bool {
    matches!(Some(0u8).ok_or(99u8), Ok(0))
}

#[hax_test::equiv]
pub fn test_ok_or_some_max() -> bool {
    matches!(Some(u8::MAX).ok_or(0u8), Ok(v) if v == u8::MAX)
}

#[hax_test::equiv]
pub fn test_ok_or_none_err_zero() -> bool {
    matches!(none_u8().ok_or(0u8), Err(0))
}

#[hax_test::equiv]
pub fn test_ok_or_none_err_max() -> bool {
    matches!(none_u8().ok_or(u8::MAX), Err(v) if v == u8::MAX)
}

// ----- ok_or_else ------------------------------------------------------------

#[hax_test::equiv]
pub fn test_ok_or_else_some() -> bool {
    matches!(Some(7u8).ok_or_else(|| 99u8), Ok(7))
}

#[hax_test::equiv]
pub fn test_ok_or_else_none() -> bool {
    matches!(none_u8().ok_or_else(|| 42u8), Err(42))
}

// ----- and_then --------------------------------------------------------------

#[hax_test::equiv]
pub fn test_and_then_some_to_some() -> bool {
    Some(3u8).and_then(|x| Some(x + 1)).unwrap_or(0) == 4
}

#[hax_test::equiv]
pub fn test_and_then_some_to_none() -> bool {
    Some(3u8).and_then(|_| none_u8()).is_none()
}

#[hax_test::equiv]
pub fn test_and_then_none() -> bool {
    none_u8().and_then(|x| Some(x + 1)).is_none()
}

// ----- filter ----------------------------------------------------------------

// TODO(closure-by-ref-extraction): `Option::filter` takes `FnOnce(&T) -> bool`;
// Aeneas trips on the borrow with "Region ids should not be visited directly".
// Revisit once closures whose parameter is a reference extract.

// ----- or --------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_or_some_some() -> bool {
    Some(0u8).or(Some(99u8)).unwrap_or(7) == 0
}

#[hax_test::equiv]
pub fn test_or_some_none() -> bool {
    Some(u8::MAX).or(none_u8()).unwrap_or(0) == u8::MAX
}

#[hax_test::equiv]
pub fn test_or_none_some() -> bool {
    none_u8().or(Some(42u8)).unwrap_or(0) == 42
}

#[hax_test::equiv]
pub fn test_or_none_none() -> bool {
    none_u8().or(none_u8()).is_none()
}

// ----- or_else ---------------------------------------------------------------

#[hax_test::equiv]
pub fn test_or_else_some_kept() -> bool {
    Some(0u8).or_else(|| Some(99u8)).unwrap_or(7) == 0
}

#[hax_test::equiv]
pub fn test_or_else_some_max_kept() -> bool {
    Some(u8::MAX).or_else(|| none_u8()).unwrap_or(0) == u8::MAX
}

#[hax_test::equiv]
pub fn test_or_else_none_to_some() -> bool {
    none_u8().or_else(|| Some(42u8)).unwrap_or(0) == 42
}

#[hax_test::equiv]
pub fn test_or_else_none_to_none() -> bool {
    none_u8().or_else(|| none_u8()).is_none()
}

// ----- xor -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_xor_some_none() -> bool {
    Some(7u8).xor(none_u8()).unwrap_or(0) == 7
}

#[hax_test::equiv]
pub fn test_xor_none_some() -> bool {
    none_u8().xor(Some(u8::MAX)).unwrap_or(0) == u8::MAX
}

#[hax_test::equiv]
pub fn test_xor_some_some() -> bool {
    Some(0u8).xor(Some(99u8)).is_none()
}

#[hax_test::equiv]
pub fn test_xor_none_none() -> bool {
    none_u8().xor(none_u8()).is_none()
}

// ----- zip -------------------------------------------------------------------

#[hax_test::equiv]
pub fn test_zip_some_some_zero() -> bool {
    match Some(0u8).zip(Some(0u8)) {
        Some((a, b)) => a == 0 && b == 0,
        None => false,
    }
}

#[hax_test::equiv]
pub fn test_zip_some_some_max() -> bool {
    match Some(u8::MAX).zip(Some(0u8)) {
        Some((a, b)) => a == u8::MAX && b == 0,
        None => false,
    }
}

#[hax_test::equiv]
pub fn test_zip_some_none() -> bool {
    Some(7u8).zip(none_u8()).is_none()
}

#[hax_test::equiv]
pub fn test_zip_none_some() -> bool {
    none_u8().zip(Some(7u8)).is_none()
}

#[hax_test::equiv]
pub fn test_zip_none_none() -> bool {
    none_u8().zip(none_u8()).is_none()
}

// ----- inspect ---------------------------------------------------------------

// TODO(closure-by-ref-extraction): `Option::inspect` takes `FnOnce(&T)`;
// Aeneas trips on the borrow with "Region ids should not be visited directly".
// Revisit once closures whose parameter is a reference extract.

// ----- flatten ---------------------------------------------------------------

#[hax_test::equiv]
pub fn test_flatten_some_some_zero() -> bool {
    Some(Some(0u8)).flatten().unwrap_or(99) == 0
}

#[hax_test::equiv]
pub fn test_flatten_some_some_max() -> bool {
    Some(Some(u8::MAX)).flatten().unwrap_or(0) == u8::MAX
}

#[hax_test::equiv]
pub fn test_flatten_some_none() -> bool {
    Some(none_u8()).flatten().is_none()
}

// ----- Default ---------------------------------------------------------------

// TODO(default-trait): testing Default for Option<T> via the model trait requires
// importing the model's Default trait; revisit alongside Default coverage.

// ----------------------------------------------------------------------------
// `take` carries `#[cfg_attr(hax_backend_lean, hax_lib::exclude)]`, so Aeneas
// does not extract its body and the Lean side resolves to a hand-written
// definition in `lean/CoreModels/Core/FunsPrologue.lean`. The sections below
// widen the element types the other three methods are exercised at.
// ----------------------------------------------------------------------------

use crate::helpers::{none_i32, none_u32};

// ----- is_some (further element types) --------------------------------------

#[hax_test::equiv]
pub fn test_is_some_some_u8_zero() -> bool {
    Some(0u8).is_some() == true
}

#[hax_test::equiv]
pub fn test_is_some_some_u8_max() -> bool {
    Some(u8::MAX).is_some() == true
}

#[hax_test::equiv]
pub fn test_is_some_some_u32() -> bool {
    Some(123u32).is_some() == true
}

#[hax_test::equiv]
pub fn test_is_some_none_u32() -> bool {
    none_u32().is_some() == false
}

#[hax_test::equiv]
pub fn test_is_some_some_bool() -> bool {
    Some(true).is_some() == true
}

#[hax_test::equiv]
pub fn test_is_some_none_bool() -> bool {
    crate::helpers::none_bool().is_some() == false
}

// ----- is_none (further element types) --------------------------------------

#[hax_test::equiv]
pub fn test_is_none_some_u8_zero() -> bool {
    Some(0u8).is_none() == false
}

#[hax_test::equiv]
pub fn test_is_none_some_u8_max() -> bool {
    Some(u8::MAX).is_none() == false
}

#[hax_test::equiv]
pub fn test_is_none_some_i32() -> bool {
    Some(-1i32).is_none() == false
}

#[hax_test::equiv]
pub fn test_is_none_none_i32() -> bool {
    none_i32().is_none() == true
}

#[hax_test::equiv]
pub fn test_is_none_some_bool() -> bool {
    Some(false).is_none() == false
}

#[hax_test::equiv]
pub fn test_is_none_none_bool() -> bool {
    crate::helpers::none_bool().is_none() == true
}

// ----- unwrap_or (further element types) ------------------------------------

#[hax_test::equiv]
pub fn test_unwrap_or_some_u8_zero() -> bool {
    Some(0u8).unwrap_or(42) == 0
}

#[hax_test::equiv]
pub fn test_unwrap_or_some_u8_max() -> bool {
    Some(u8::MAX).unwrap_or(0) == u8::MAX
}

#[hax_test::equiv]
pub fn test_unwrap_or_none_u8_small_default() -> bool {
    none_u8().unwrap_or(7) == 7
}

#[hax_test::equiv]
pub fn test_unwrap_or_none_u8_max_default() -> bool {
    none_u8().unwrap_or(u8::MAX) == u8::MAX
}

#[hax_test::equiv]
pub fn test_unwrap_or_some_u32_small() -> bool {
    Some(5u32).unwrap_or(99) == 5
}

#[hax_test::equiv]
pub fn test_unwrap_or_none_u32_max_default() -> bool {
    none_u32().unwrap_or(u32::MAX) == u32::MAX
}

#[hax_test::equiv]
pub fn test_unwrap_or_some_bool() -> bool {
    Some(true).unwrap_or(false) == true
}

#[hax_test::equiv]
pub fn test_unwrap_or_none_bool() -> bool {
    crate::helpers::none_bool().unwrap_or(true) == true
}

// ----- take (manually defined in Lean, not extracted) -----------------------

#[hax_test::equiv]
pub fn test_manual_take_some_u8_returns_old() -> bool {
    let mut x: Option<u8> = Some(9);
    let old = x.take();
    old == Some(9)
}

#[hax_test::equiv]
pub fn test_manual_take_some_u8_replaces_with_none() -> bool {
    let mut x: Option<u8> = Some(9);
    let _ = x.take();
    x.is_some() == false
}

#[hax_test::equiv]
pub fn test_manual_take_some_u8_max_returns_old() -> bool {
    let mut x: Option<u8> = Some(u8::MAX);
    let old = x.take();
    old == Some(u8::MAX) && x.is_none()
}

#[hax_test::equiv]
pub fn test_manual_take_none_u8_returns_none() -> bool {
    let mut x: Option<u8> = none_u8();
    let old = x.take();
    old.is_none() && x.is_none()
}

#[hax_test::equiv]
pub fn test_manual_take_some_u32() -> bool {
    let mut x: Option<u32> = Some(123);
    let old = x.take();
    old == Some(123) && x.is_none()
}

#[hax_test::equiv]
pub fn test_manual_take_none_u32() -> bool {
    let mut x: Option<u32> = none_u32();
    let old = x.take();
    old.is_none() && x.is_none()
}

#[hax_test::equiv]
pub fn test_manual_take_some_bool() -> bool {
    let mut x: Option<bool> = Some(true);
    let old = x.take();
    old == Some(true) && x.is_none()
}

// ----- PartialEq: `Option<T> == Option<T>` -----------------------------------

#[hax_test::equiv]
pub fn test_option_eq_some_same() -> bool {
    (Some(5u8) == Some(5u8)) == true
}

#[hax_test::equiv]
pub fn test_option_eq_some_diff() -> bool {
    (Some(5u8) == Some(6u8)) == false
}

#[hax_test::equiv]
pub fn test_option_eq_none_none() -> bool {
    (none_u8() == none_u8()) == true
}

#[hax_test::equiv]
pub fn test_option_eq_some_none() -> bool {
    (Some(5u8) == none_u8()) == false
}

#[hax_test::equiv]
pub fn test_option_eq_none_some() -> bool {
    (none_u8() == Some(5u8)) == false
}

// ----- dictionary-applying tests ---------------------------------------------

#[hax_test::equiv]
pub fn test_clone_some_applies_element_clone() -> bool {
    let x = Some(Bumped(1));
    match x.clone() {
        Some(b) => b.0 == 2,
        None => false,
    }
}

#[hax_test::equiv]
pub fn test_clone_none_of_a_cloneable_element() -> bool {
    let x: Option<Bumped> = None;
    x.clone().is_none()
}

#[hax_test::equiv]
pub fn test_eq_some_goes_through_the_dictionary() -> bool {
    Some(keyed(5, 1)) == Some(keyed(5, 2))
}

#[hax_test::equiv]
pub fn test_ne_some_goes_through_the_dictionary() -> bool {
    (Some(keyed(5, 1)) != Some(keyed(5, 2))) == false
}

#[hax_test::equiv]
pub fn test_eq_some_differing_keys() -> bool {
    (Some(keyed(5, 1)) == Some(keyed(6, 1))) == false
}

// The mismatch arm never reaches the element dictionary.
#[hax_test::equiv]
pub fn test_eq_some_vs_none() -> bool {
    let n: Option<Keyed> = None;
    (Some(keyed(5, 1)) == n) == false
}
