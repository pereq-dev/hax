//! Exhaustive equivalence tests for the `rust_primitives::arithmetic` ops.
//!
//! ## Why this file exists
//!
//! Everything `core_models::num` exposes bottoms out in
//! `rust_primitives::arithmetic::*`. Those functions have *two* independent
//! definitions: the Rust one (a call straight into std) and a hand-written Lean
//! one in `../proof-libs/lean/CoreModels/RustPrimitives/Funs.lean`. The property
//! tests in the model crate cannot see the Lean side at all — on the Rust side
//! `wrapping_add_u8(x, y)` *is* `x.wrapping_add(y)`, so comparing the two is a
//! tautology. Only an equivalence test reaches the Lean definition.
//!
//! ## How the oracle works
//!
//! Each test recomputes the operation from primitives that do *not* route
//! through `rust_primitives::arithmetic` — shifts, masks, comparisons and
//! wider-integer arithmetic — and compares. That reference is independent on
//! both sides, so the check is meaningful in Lean (hand-written primitive vs.
//! extracted reference) as well as in Rust (std vs. reference).
//!
//! `u8`/`i8` are swept exhaustively: small enough to evaluate inside a `#guard`,
//! wide enough to cover every sign, carry and overflow boundary. The wider types
//! share the same Lean definitions (all are generic over the scalar type), so a
//! bug at 8 bits is a bug everywhere -- except for byte order, which one byte
//! cannot distinguish, so `to_be_bytes`/`to_le_bytes` get 16-bit probes too.

use rust_lean_test_macro::rust_lean_test;

/// Second operands for the binary sweeps: 0/1 and the boundaries, plus a value
/// with no special structure. Sweeping `x` fully against these covers the carry
/// and overflow edges without the cost of a full 65536-case product.
const U8_PROBES: [u8; 6] = [0, 1, 2, 127, 128, 255];
const I8_PROBES: [i8; 7] = [0, 1, -1, 127, -128, 42, -42];
/// Byte-order probes. Deliberately 16 bits: at one byte big and little endian
/// agree, so an 8-bit sweep cannot tell them apart.
const U16_PROBES: [u16; 7] = [0, 1, 255, 256, 4660, 65280, 65535];

// ----- helpers: references built without `rust_primitives::arithmetic` -------

/// `x`'s bit pattern widened to `u32` (0..=255).
fn u8_bits(x: u8) -> u32 {
    x as u32
}

/// `x`'s two's-complement bit pattern as `u32` (0..=255).
fn i8_bits(x: i8) -> u32 {
    (x as u8) as u32
}

/// Reduce a `u32` bit pattern back to `u8`.
fn to_u8(v: u32) -> u8 {
    (v % 256) as u8
}

/// Reduce a `u32` bit pattern back to `i8` (two's complement).
fn to_i8(v: u32) -> i8 {
    let m = v % 256;
    if m > 127 { (m as u8) as i8 } else { m as i8 }
}

/// `x`'s bit pattern widened to `u32` (0..=65535).
fn u16_bits(x: u16) -> u32 {
    x as u32
}

/// `x`'s two's-complement bit pattern as `u32` (0..=65535).
fn i16_bits(x: i16) -> u32 {
    (x as u16) as u32
}

/// Reduce a `u32` bit pattern back to `i16` (two's complement).
fn to_i16(v: u32) -> i16 {
    let m = v % 65536;
    if m > 32767 {
        (m as u16) as i16
    } else {
        m as i16
    }
}

/// Exact value of an `i8`, widened.
fn i8_val(x: i8) -> i32 {
    x as i32
}

/// Clamp a widened value into the `i8` range.
fn clamp_i8(v: i32) -> i32 {
    if v > 127 {
        127
    } else if v < -128 {
        -128
    } else {
        v
    }
}

// ----- count_ones ------------------------------------------------------------

#[rust_lean_test]
pub fn test_u8_count_ones_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        let x = i as u8;
        let mut expected: u32 = 0;
        let mut b: u32 = 0;
        while b < 8 {
            expected += (u8_bits(x) >> b) & 1;
            b += 1;
        }
        if x.count_ones() != expected {
            ok = false;
        }
    }
    ok
}

#[rust_lean_test]
pub fn test_i8_count_ones_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        let x = to_i8(i as u32);
        let mut expected: u32 = 0;
        let mut b: u32 = 0;
        while b < 8 {
            expected += (i8_bits(x) >> b) & 1;
            b += 1;
        }
        if x.count_ones() != expected {
            ok = false;
        }
    }
    ok
}

// ----- leading_zeros ---------------------------------------------------------

#[rust_lean_test]
pub fn test_u8_leading_zeros_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        let x = i as u8;
        let mut expected: u32 = 0;
        let mut seen = false;
        let mut b: u32 = 0;
        while b < 8 {
            if (u8_bits(x) >> (7 - b)) & 1 == 1 {
                seen = true;
            }
            if !seen {
                expected += 1;
            }
            b += 1;
        }
        if x.leading_zeros() != expected {
            ok = false;
        }
    }
    ok
}

#[rust_lean_test]
pub fn test_i8_leading_zeros_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        let x = to_i8(i as u32);
        let mut expected: u32 = 0;
        let mut seen = false;
        let mut b: u32 = 0;
        while b < 8 {
            if (i8_bits(x) >> (7 - b)) & 1 == 1 {
                seen = true;
            }
            if !seen {
                expected += 1;
            }
            b += 1;
        }
        if x.leading_zeros() != expected {
            ok = false;
        }
    }
    ok
}

// ----- ilog2 (defined for x > 0) --------------------------------------------

#[rust_lean_test]
pub fn test_u8_ilog2_exhaustive() -> bool {
    let mut ok = true;
    for i in 1..256usize {
        let x = i as u8;
        // Largest `e` with `2^e <= x`, counted rather than assigned: aeneas
        // cannot find a fixed point for a loop that writes the loop variable.
        // `2^0 <= x` always holds here (x >= 1), so start the count at `e = 1`.
        let mut expected: u32 = 0;
        let mut e: u32 = 1;
        while e < 8 {
            if (1u32 << e) <= u8_bits(x) {
                expected += 1;
            }
            e += 1;
        }
        if x.ilog2() != expected {
            ok = false;
        }
    }
    ok
}

#[rust_lean_test]
pub fn test_i8_ilog2_exhaustive() -> bool {
    let mut ok = true;
    for i in 1..128usize {
        let x = i as i8;
        let mut expected: u32 = 0;
        let mut e: u32 = 1;
        while e < 8 {
            if (1i32 << e) <= i8_val(x) {
                expected += 1;
            }
            e += 1;
        }
        if x.ilog2() != expected {
            ok = false;
        }
    }
    ok
}

// ----- abs (signed, defined for x > MIN) ------------------------------------

#[rust_lean_test]
pub fn test_i8_abs_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        // `i == 128` is `MIN`, where `abs` overflows and both sides must panic.
        if i != 128 {
            let x = to_i8(i as u32);
            let expected = if i8_val(x) < 0 { 0 - x } else { x };
            if x.abs() != expected {
                ok = false;
            }
        }
    }
    ok
}

// ----- unsigned_abs (signed, total) -----------------------------------------

#[rust_lean_test]
pub fn test_i8_unsigned_abs_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        let x = to_i8(i as u32);
        let v = i8_val(x);
        let expected = if v < 0 { (0 - v) as u8 } else { v as u8 };
        if x.unsigned_abs() != expected {
            ok = false;
        }
    }
    ok
}

// ----- rotate_left / rotate_right -------------------------------------------

#[rust_lean_test]
pub fn test_u8_rotate_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        for k in 0..8usize {
            let x = i as u8;
            let n = k as u32;
            let bits = u8_bits(x);
            let left = if n == 0 {
                x
            } else {
                to_u8((bits << n) | (bits >> (8 - n)))
            };
            let right = if n == 0 {
                x
            } else {
                to_u8((bits >> n) | (bits << (8 - n)))
            };
            if x.rotate_left(n) != left || x.rotate_right(n) != right {
                ok = false;
            }
        }
    }
    ok
}

#[rust_lean_test]
pub fn test_i8_rotate_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        for k in 0..8usize {
            let x = to_i8(i as u32);
            let n = k as u32;
            let bits = i8_bits(x);
            let left = if n == 0 {
                x
            } else {
                to_i8((bits << n) | (bits >> (8 - n)))
            };
            let right = if n == 0 {
                x
            } else {
                to_i8((bits >> n) | (bits << (8 - n)))
            };
            if x.rotate_left(n) != left || x.rotate_right(n) != right {
                ok = false;
            }
        }
    }
    ok
}

// ----- add -------------------------------------------------------------------

#[rust_lean_test]
pub fn test_u8_add_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        for j in 0..6usize {
            let x = i as u8;
            let y = U8_PROBES[j];
            let sum = u8_bits(x) + u8_bits(y);
            let over = sum > 255;
            let wrapped = to_u8(sum);
            if x.wrapping_add(y) != wrapped {
                ok = false;
            }
            if x.saturating_add(y) != (if over { 255 } else { wrapped }) {
                ok = false;
            }
            let (r, o) = x.overflowing_add(y);
            if !(r == wrapped && o == over) {
                ok = false;
            }
        }
    }
    ok
}

#[rust_lean_test]
pub fn test_i8_add_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        for j in 0..7usize {
            let x = to_i8(i as u32);
            let y = I8_PROBES[j];
            let sum = i8_val(x) + i8_val(y);
            let over = clamp_i8(sum) != sum;
            let wrapped = to_i8((i8_bits(x) + i8_bits(y)) % 256);
            if x.wrapping_add(y) != wrapped {
                ok = false;
            }
            let (r, o) = x.overflowing_add(y);
            if !(r == wrapped && o == over) {
                ok = false;
            }
        }
    }
    ok
}

// ----- sub -------------------------------------------------------------------

#[rust_lean_test]
pub fn test_u8_sub_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        for j in 0..6usize {
            let x = i as u8;
            let y = U8_PROBES[j];
            let wrapped = to_u8(u8_bits(x) + 256 - u8_bits(y));
            let over = x < y;
            if x.wrapping_sub(y) != wrapped {
                ok = false;
            }
            if x.saturating_sub(y) != (if over { 0 } else { wrapped }) {
                ok = false;
            }
            let (r, o) = x.overflowing_sub(y);
            if !(r == wrapped && o == over) {
                ok = false;
            }
        }
    }
    ok
}

#[rust_lean_test]
pub fn test_i8_sub_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        for j in 0..7usize {
            let x = to_i8(i as u32);
            let y = I8_PROBES[j];
            let diff = i8_val(x) - i8_val(y);
            let over = clamp_i8(diff) != diff;
            let wrapped = to_i8((i8_bits(x) + 256 - i8_bits(y)) % 256);
            if x.wrapping_sub(y) != wrapped {
                ok = false;
            }
            let (r, o) = x.overflowing_sub(y);
            if !(r == wrapped && o == over) {
                ok = false;
            }
        }
    }
    ok
}

// ----- mul -------------------------------------------------------------------

#[rust_lean_test]
pub fn test_u8_mul_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        for j in 0..6usize {
            let x = i as u8;
            let y = U8_PROBES[j];
            let prod = u8_bits(x) * u8_bits(y);
            let over = prod > 255;
            let wrapped = to_u8(prod);
            if x.wrapping_mul(y) != wrapped {
                ok = false;
            }
            if x.saturating_mul(y) != (if over { 255 } else { wrapped }) {
                ok = false;
            }
            let (r, o) = x.overflowing_mul(y);
            if !(r == wrapped && o == over) {
                ok = false;
            }
        }
    }
    ok
}

#[rust_lean_test]
pub fn test_i8_mul_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        for j in 0..7usize {
            let x = to_i8(i as u32);
            let y = I8_PROBES[j];
            let prod = i8_val(x) * i8_val(y);
            let over = clamp_i8(prod) != prod;
            // `prod` may be negative, so fold it into range before reducing.
            let wrapped = to_i8((((prod % 256) + 256) % 256) as u32);
            if x.wrapping_mul(y) != wrapped {
                ok = false;
            }
            let (r, o) = x.overflowing_mul(y);
            if !(r == wrapped && o == over) {
                ok = false;
            }
        }
    }
    ok
}

// ----- rem_euclid ------------------------------------------------------------

#[rust_lean_test]
pub fn test_u8_rem_euclid_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        for j in 0..6usize {
            let x = i as u8;
            let y = U8_PROBES[j];
            if y != 0 && x.rem_euclid(y) != to_u8(u8_bits(x) % u8_bits(y)) {
                ok = false;
            }
        }
    }
    ok
}

#[rust_lean_test]
pub fn test_i8_rem_euclid_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        for j in 0..7usize {
            let x = to_i8(i as u32);
            let y = I8_PROBES[j];
            // Skips the panicking inputs: `y == 0`, and `MIN % -1`.
            if y != 0 && !(i == 128 && y == -1) {
                let r = i8_val(x) % i8_val(y);
                let magnitude = if i8_val(y) < 0 {
                    0 - i8_val(y)
                } else {
                    i8_val(y)
                };
                let expected = if r < 0 { r + magnitude } else { r };
                if i8_val(x.rem_euclid(y)) != expected {
                    ok = false;
                }
            }
        }
    }
    ok
}

// ----- saturating (signed) ---------------------------------------------------

#[rust_lean_test]
pub fn test_i8_saturating_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        for j in 0..7usize {
            let x = to_i8(i as u32);
            let y = I8_PROBES[j];
            let sum = i8_val(x) + i8_val(y);
            let diff = i8_val(x) - i8_val(y);
            let prod = i8_val(x) * i8_val(y);
            if i8_val(x.saturating_add(y)) != clamp_i8(sum) {
                ok = false;
            }
            if i8_val(x.saturating_sub(y)) != clamp_i8(diff) {
                ok = false;
            }
            if i8_val(x.saturating_mul(y)) != clamp_i8(prod) {
                ok = false;
            }
        }
    }
    ok
}

// ----- pow / overflowing_pow -------------------------------------------------
//
// The Lean side computes these in closed form (`x^n mod 2^bits`, plus an
// out-of-range flag) rather than by exponentiation-by-squaring as std does; this
// sweep is what pins the two together.

#[rust_lean_test]
pub fn test_u8_pow_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        for e in 0..9usize {
            let x = i as u8;
            let exp = e as u32;
            // Reference: repeated multiplication, tracking exactness separately.
            let mut wrapped: u32 = 1;
            let mut exact: u32 = 1;
            let mut over = false;
            let mut k = 0usize;
            while k < e {
                wrapped = (wrapped * u8_bits(x)) % 256;
                // Stop tracking the exact value once it has left the range;
                // continuing would overflow `u32` for large `x` and `e`.
                if !over {
                    exact = exact * u8_bits(x);
                    if exact > 255 {
                        over = true;
                    }
                }
                k += 1;
            }
            let (r, o) = x.overflowing_pow(exp);
            if !(r == to_u8(wrapped) && o == over) {
                ok = false;
            }
            if !over && x.pow(exp) != to_u8(wrapped) {
                ok = false;
            }
        }
    }
    ok
}

/// The signed counterpart. `ioverflowing_pow` is a distinct Lean definition
/// with a two-sided bound, so the `u8` sweep above says nothing about it.
#[rust_lean_test]
pub fn test_i8_pow_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        for e in 0..9usize {
            let x = to_i8(i as u32);
            let exp = e as u32;
            // Reference: repeated multiplication, tracking exactness separately.
            let mut wrapped: u32 = 1;
            let mut exact: i32 = 1;
            let mut over = false;
            let mut k = 0usize;
            while k < e {
                wrapped = (wrapped * i8_bits(x)) % 256;
                // Stop tracking the exact value once it has left the range; both
                // ends matter here, unlike the unsigned case.
                if !over {
                    exact = exact * i8_val(x);
                    if exact > 127 || exact < -128 {
                        over = true;
                    }
                }
                k += 1;
            }
            let (r, o) = x.overflowing_pow(exp);
            if !(r == to_i8(wrapped) && o == over) {
                ok = false;
            }
            if !over && x.pow(exp) != to_i8(wrapped) {
                ok = false;
            }
        }
    }
    ok
}

// ----- byte conversions ------------------------------------------------------

#[rust_lean_test]
pub fn test_u8_byte_conversions_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        let x = i as u8;
        if x.to_be_bytes()[0] != x || x.to_le_bytes()[0] != x {
            ok = false;
        }
        if u8::from_be_bytes([x]) != x || u8::from_le_bytes([x]) != x {
            ok = false;
        }
    }
    ok
}

#[rust_lean_test]
pub fn test_i8_byte_conversions_exhaustive() -> bool {
    let mut ok = true;
    for i in 0..256usize {
        let x = to_i8(i as u32);
        let b = i as u8;
        if x.to_be_bytes()[0] != b || x.to_le_bytes()[0] != b {
            ok = false;
        }
        if i8::from_be_bytes([b]) != x || i8::from_le_bytes([b]) != x {
            ok = false;
        }
    }
    ok
}

#[rust_lean_test]
pub fn test_u16_byte_order() -> bool {
    let mut ok = true;
    for j in 0..7usize {
        let x = U16_PROBES[j];
        let hi = to_u8(u16_bits(x) / 256);
        let lo = to_u8(u16_bits(x) % 256);
        if x.to_be_bytes()[0] != hi || x.to_be_bytes()[1] != lo {
            ok = false;
        }
        if x.to_le_bytes()[0] != lo || x.to_le_bytes()[1] != hi {
            ok = false;
        }
        if u16::from_be_bytes([hi, lo]) != x || u16::from_le_bytes([lo, hi]) != x {
            ok = false;
        }
    }
    ok
}

#[rust_lean_test]
pub fn test_i16_byte_order() -> bool {
    let mut ok = true;
    for j in 0..7usize {
        let x = to_i16(u16_bits(U16_PROBES[j]));
        let hi = to_u8(i16_bits(x) / 256);
        let lo = to_u8(i16_bits(x) % 256);
        if x.to_be_bytes()[0] != hi || x.to_be_bytes()[1] != lo {
            ok = false;
        }
        if x.to_le_bytes()[0] != lo || x.to_le_bytes()[1] != hi {
            ok = false;
        }
        if i16::from_be_bytes([hi, lo]) != x || i16::from_le_bytes([lo, hi]) != x {
            ok = false;
        }
    }
    ok
}
