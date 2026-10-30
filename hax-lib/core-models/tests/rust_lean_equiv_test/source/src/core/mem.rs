//! Equivalence tests for `core::mem::swap` and `core::mem::replace`.
//!
//! Both items carry `#[cfg_attr(hax_backend_lean, hax_lib::exclude)]` so Aeneas
//! does not extract the Rust std bodies; instead the name map routes them to
//! manually-written Lean definitions in `lean/CoreModels/FunsExternal.lean`
//! and friends. The Rust side of each test calls std directly — if the
//! manual Lean def disagrees with std on a given input, the generated
//! `#guard` fails the Lean build.

// ----- mem::swap (manually defined in Lean, not extracted) ------------------

#[hax_test::equiv]
pub fn test_swap_u8_distinct() -> bool {
    let mut a: u8 = 1;
    let mut b: u8 = 2;
    core::mem::swap(&mut a, &mut b);
    a == 2 && b == 1
}

#[hax_test::equiv]
pub fn test_swap_u8_equal() -> bool {
    let mut a: u8 = 7;
    let mut b: u8 = 7;
    core::mem::swap(&mut a, &mut b);
    a == 7 && b == 7
}

#[hax_test::equiv]
pub fn test_swap_u8_min_max() -> bool {
    let mut a: u8 = u8::MIN;
    let mut b: u8 = u8::MAX;
    core::mem::swap(&mut a, &mut b);
    a == u8::MAX && b == u8::MIN
}

#[hax_test::equiv]
pub fn test_swap_u32_distinct() -> bool {
    let mut a: u32 = 100;
    let mut b: u32 = 200;
    core::mem::swap(&mut a, &mut b);
    a == 200 && b == 100
}

#[hax_test::equiv]
pub fn test_swap_u32_min_max() -> bool {
    let mut a: u32 = u32::MIN;
    let mut b: u32 = u32::MAX;
    core::mem::swap(&mut a, &mut b);
    a == u32::MAX && b == u32::MIN
}

#[hax_test::equiv]
pub fn test_swap_tuple_u32() -> bool {
    let mut a: (u32, u32) = (1, 2);
    let mut b: (u32, u32) = (3, 4);
    core::mem::swap(&mut a, &mut b);
    a.0 == 3 && a.1 == 4 && b.0 == 1 && b.1 == 2
}

// ----- mem::replace (manually defined in Lean, not extracted) ---------------

#[hax_test::equiv]
pub fn test_replace_u8_returns_old() -> bool {
    let mut dst: u8 = 5;
    let old = core::mem::replace(&mut dst, 9);
    old == 5
}

#[hax_test::equiv]
pub fn test_replace_u8_leaves_new() -> bool {
    let mut dst: u8 = 5;
    let _ = core::mem::replace(&mut dst, 9);
    dst == 9
}

#[hax_test::equiv]
pub fn test_replace_u8_min_max() -> bool {
    let mut dst: u8 = u8::MIN;
    let old = core::mem::replace(&mut dst, u8::MAX);
    old == u8::MIN && dst == u8::MAX
}

#[hax_test::equiv]
pub fn test_replace_u8_equal_values() -> bool {
    let mut dst: u8 = 3;
    let old = core::mem::replace(&mut dst, 3);
    old == 3 && dst == 3
}

#[hax_test::equiv]
pub fn test_replace_tuple_returns_old() -> bool {
    let mut dst: (u32, u32) = (1, 2);
    let old = core::mem::replace(&mut dst, (3, 4));
    old.0 == 1 && old.1 == 2
}

#[hax_test::equiv]
pub fn test_replace_tuple_leaves_new() -> bool {
    let mut dst: (u32, u32) = (1, 2);
    let _ = core::mem::replace(&mut dst, (3, 4));
    dst.0 == 3 && dst.1 == 4
}

#[hax_test::equiv]
pub fn test_replace_option_some_with_none() -> bool {
    let mut dst: Option<u8> = Some(7);
    let _old = core::mem::replace(&mut dst, crate::helpers::none_u8());
    dst.is_none()
}

#[hax_test::equiv]
pub fn test_replace_option_none_with_some() -> bool {
    let mut dst: Option<u8> = crate::helpers::none_u8();
    let old = core::mem::replace(&mut dst, Some(42));
    old.is_none() && dst == Some(42)
}

#[hax_test::equiv]
pub fn test_replace_option_some_with_some() -> bool {
    let mut dst: Option<u8> = Some(1);
    let old = core::mem::replace(&mut dst, Some(2));
    old == Some(1) && dst == Some(2)
}
