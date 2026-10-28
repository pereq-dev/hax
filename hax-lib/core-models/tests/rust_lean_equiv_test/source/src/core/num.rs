//! Equivalence tests for `core::num::*` (integer primitive methods).
//!
//! Mirrors the proptest blocks in `core-models/src/core/num/mod.rs`,
//! which exercise (for each `{u,i}{8,16,32,64,128,size}`):
//!   - `MIN`, `MAX`, `BITS` constants,
//!   - `wrapping_{add,sub,mul}`, `saturating_{add,sub,mul}`,
//!     `overflowing_{add,sub,mul}`,
//!   - `rem_euclid`, `pow`, `count_ones`,
//!   - `rotate_left`, `rotate_right`, `leading_zeros`,
//!   - `from_{be,le}_bytes`, `to_{be,le}_bytes`,
//!   - `checked_div`, `checked_rem`,
//!   - `is_power_of_two` (unsigned), `abs`, `unsigned_abs` and `signum` (signed),
//!   - `ilog2`,
//!   - `Default::default()`.
//!
//! We pick a handful of representative widths and a few corner cases
//! per method (zero, MIN, MAX, overflow boundaries, sign flips). The
//! upstream proptest omits `checked_{add,sub,mul}` (see the comment
//! there about `to_int()` stub returning 0); we do the same.

use crate::helpers::{none_i8, none_i16, none_i32, none_u8, none_u16, none_u32};
use rust_lean_test_macro::rust_lean_test;

// =============================================================================
// Constants: MIN / MAX / BITS
// =============================================================================

#[rust_lean_test]
pub fn test_u8_min() -> bool {
    u8::MIN == 0u8
}

#[rust_lean_test]
pub fn test_u8_max() -> bool {
    u8::MAX == 255u8
}

#[rust_lean_test]
pub fn test_u8_bits() -> bool {
    u8::BITS == 8u32
}

#[rust_lean_test]
pub fn test_u16_min() -> bool {
    u16::MIN == 0u16
}

#[rust_lean_test]
pub fn test_u16_max() -> bool {
    u16::MAX == 65535u16
}

#[rust_lean_test]
pub fn test_u16_bits() -> bool {
    u16::BITS == 16u32
}

#[rust_lean_test]
pub fn test_u32_min() -> bool {
    u32::MIN == 0u32
}

#[rust_lean_test]
pub fn test_u32_max() -> bool {
    u32::MAX == 4294967295u32
}

#[rust_lean_test]
pub fn test_u32_bits() -> bool {
    u32::BITS == 32u32
}

#[rust_lean_test]
pub fn test_i8_min() -> bool {
    i8::MIN == -128i8
}

#[rust_lean_test]
pub fn test_i8_max() -> bool {
    i8::MAX == 127i8
}

#[rust_lean_test]
pub fn test_i8_bits() -> bool {
    i8::BITS == 8u32
}

#[rust_lean_test]
pub fn test_i16_min() -> bool {
    i16::MIN == -32768i16
}

#[rust_lean_test]
pub fn test_i16_max() -> bool {
    i16::MAX == 32767i16
}

#[rust_lean_test]
pub fn test_i32_min() -> bool {
    i32::MIN == -2147483648i32
}

#[rust_lean_test]
pub fn test_i32_max() -> bool {
    i32::MAX == 2147483647i32
}

// The pointer-sized widths, which the model pins at 64 bits.
#[rust_lean_test]
pub fn test_usize_bits() -> bool {
    usize::BITS == 64u32
}

#[rust_lean_test]
pub fn test_isize_bits() -> bool {
    isize::BITS == 64u32
}

// =============================================================================
// wrapping_add
// =============================================================================

#[rust_lean_test]
pub fn test_u8_wrapping_add_zero() -> bool {
    0u8.wrapping_add(0u8) == 0u8
}

#[rust_lean_test]
pub fn test_u8_wrapping_add_no_overflow() -> bool {
    100u8.wrapping_add(50u8) == 150u8
}

#[rust_lean_test]
pub fn test_u8_wrapping_add_at_max() -> bool {
    u8::MAX.wrapping_add(1u8) == 0u8
}

#[rust_lean_test]
pub fn test_u8_wrapping_add_overflow() -> bool {
    200u8.wrapping_add(100u8) == 44u8
}

#[rust_lean_test]
pub fn test_i8_wrapping_add_at_max() -> bool {
    i8::MAX.wrapping_add(1i8) == i8::MIN
}

#[rust_lean_test]
pub fn test_i8_wrapping_add_neg() -> bool {
    (-1i8).wrapping_add(1i8) == 0i8
}

#[rust_lean_test]
pub fn test_u32_wrapping_add_at_max() -> bool {
    u32::MAX.wrapping_add(1u32) == 0u32
}

// =============================================================================
// wrapping_sub
// =============================================================================

#[rust_lean_test]
pub fn test_u8_wrapping_sub_zero() -> bool {
    0u8.wrapping_sub(0u8) == 0u8
}

#[rust_lean_test]
pub fn test_u8_wrapping_sub_underflow() -> bool {
    0u8.wrapping_sub(1u8) == u8::MAX
}

#[rust_lean_test]
pub fn test_u8_wrapping_sub_no_underflow() -> bool {
    100u8.wrapping_sub(50u8) == 50u8
}

#[rust_lean_test]
pub fn test_i8_wrapping_sub_at_min() -> bool {
    i8::MIN.wrapping_sub(1i8) == i8::MAX
}

#[rust_lean_test]
pub fn test_u32_wrapping_sub_underflow() -> bool {
    0u32.wrapping_sub(1u32) == u32::MAX
}

// =============================================================================
// wrapping_mul
// =============================================================================

#[rust_lean_test]
pub fn test_u8_wrapping_mul_zero() -> bool {
    0u8.wrapping_mul(42u8) == 0u8
}

#[rust_lean_test]
pub fn test_u8_wrapping_mul_one() -> bool {
    42u8.wrapping_mul(1u8) == 42u8
}

#[rust_lean_test]
pub fn test_u8_wrapping_mul_overflow() -> bool {
    // 16 * 16 == 256 -> wraps to 0
    16u8.wrapping_mul(16u8) == 0u8
}

#[rust_lean_test]
pub fn test_u8_wrapping_mul_max() -> bool {
    // 255 * 255 == 65025 mod 256 == 1
    u8::MAX.wrapping_mul(u8::MAX) == 1u8
}

#[rust_lean_test]
pub fn test_i8_wrapping_mul_neg() -> bool {
    (-1i8).wrapping_mul(2i8) == -2i8
}

// =============================================================================
// saturating_add
// =============================================================================

#[rust_lean_test]
pub fn test_u8_saturating_add_zero() -> bool {
    0u8.saturating_add(0u8) == 0u8
}

#[rust_lean_test]
pub fn test_u8_saturating_add_no_overflow() -> bool {
    100u8.saturating_add(50u8) == 150u8
}

#[rust_lean_test]
pub fn test_u8_saturating_add_at_max() -> bool {
    u8::MAX.saturating_add(1u8) == u8::MAX
}

#[rust_lean_test]
pub fn test_u8_saturating_add_overflow() -> bool {
    200u8.saturating_add(100u8) == u8::MAX
}

#[rust_lean_test]
pub fn test_i8_saturating_add_at_max() -> bool {
    i8::MAX.saturating_add(1i8) == i8::MAX
}

#[rust_lean_test]
pub fn test_i8_saturating_add_at_min() -> bool {
    i8::MIN.saturating_add(-1i8) == i8::MIN
}

// =============================================================================
// saturating_sub
// =============================================================================

#[rust_lean_test]
pub fn test_u8_saturating_sub_zero() -> bool {
    0u8.saturating_sub(0u8) == 0u8
}

#[rust_lean_test]
pub fn test_u8_saturating_sub_no_underflow() -> bool {
    100u8.saturating_sub(50u8) == 50u8
}

#[rust_lean_test]
pub fn test_u8_saturating_sub_at_min() -> bool {
    0u8.saturating_sub(1u8) == 0u8
}

#[rust_lean_test]
pub fn test_i8_saturating_sub_at_min() -> bool {
    i8::MIN.saturating_sub(1i8) == i8::MIN
}

// =============================================================================
// saturating_mul
// =============================================================================

#[rust_lean_test]
pub fn test_u8_saturating_mul_zero() -> bool {
    0u8.saturating_mul(42u8) == 0u8
}

#[rust_lean_test]
pub fn test_u8_saturating_mul_no_overflow() -> bool {
    10u8.saturating_mul(10u8) == 100u8
}

#[rust_lean_test]
pub fn test_u8_saturating_mul_overflow() -> bool {
    16u8.saturating_mul(16u8) == u8::MAX
}

#[rust_lean_test]
pub fn test_i8_saturating_mul_neg_overflow() -> bool {
    i8::MIN.saturating_mul(2i8) == i8::MIN
}

// =============================================================================
// overflowing_add
// =============================================================================

#[rust_lean_test]
pub fn test_u8_overflowing_add_zero() -> bool {
    0u8.overflowing_add(0u8) == (0u8, false)
}

#[rust_lean_test]
pub fn test_u8_overflowing_add_no_overflow() -> bool {
    100u8.overflowing_add(50u8) == (150u8, false)
}

#[rust_lean_test]
pub fn test_u8_overflowing_add_at_max() -> bool {
    u8::MAX.overflowing_add(1u8) == (0u8, true)
}

#[rust_lean_test]
pub fn test_u8_overflowing_add_overflow() -> bool {
    200u8.overflowing_add(100u8) == (44u8, true)
}

#[rust_lean_test]
pub fn test_i8_overflowing_add_at_max() -> bool {
    i8::MAX.overflowing_add(1i8) == (i8::MIN, true)
}

// =============================================================================
// overflowing_sub
// =============================================================================

#[rust_lean_test]
pub fn test_u8_overflowing_sub_zero() -> bool {
    0u8.overflowing_sub(0u8) == (0u8, false)
}

#[rust_lean_test]
pub fn test_u8_overflowing_sub_no_underflow() -> bool {
    100u8.overflowing_sub(50u8) == (50u8, false)
}

#[rust_lean_test]
pub fn test_u8_overflowing_sub_underflow() -> bool {
    0u8.overflowing_sub(1u8) == (u8::MAX, true)
}

#[rust_lean_test]
pub fn test_i8_overflowing_sub_at_min() -> bool {
    i8::MIN.overflowing_sub(1i8) == (i8::MAX, true)
}

// =============================================================================
// overflowing_mul
// =============================================================================

#[rust_lean_test]
pub fn test_u8_overflowing_mul_zero() -> bool {
    0u8.overflowing_mul(42u8) == (0u8, false)
}

#[rust_lean_test]
pub fn test_u8_overflowing_mul_no_overflow() -> bool {
    10u8.overflowing_mul(10u8) == (100u8, false)
}

#[rust_lean_test]
pub fn test_u8_overflowing_mul_overflow() -> bool {
    16u8.overflowing_mul(16u8) == (0u8, true)
}

#[rust_lean_test]
pub fn test_u8_overflowing_mul_max() -> bool {
    u8::MAX.overflowing_mul(u8::MAX) == (1u8, true)
}

// =============================================================================
// rem_euclid
// =============================================================================

#[rust_lean_test]
pub fn test_u8_rem_euclid_basic() -> bool {
    10u8.rem_euclid(3u8) == 1u8
}

#[rust_lean_test]
pub fn test_u8_rem_euclid_divisible() -> bool {
    12u8.rem_euclid(3u8) == 0u8
}

#[rust_lean_test]
pub fn test_u8_rem_euclid_zero_dividend() -> bool {
    0u8.rem_euclid(5u8) == 0u8
}

#[rust_lean_test]
pub fn test_i8_rem_euclid_pos() -> bool {
    10i8.rem_euclid(3i8) == 1i8
}

#[rust_lean_test]
pub fn test_i8_rem_euclid_neg() -> bool {
    (-7i8).rem_euclid(3i8) == 2i8
}

// =============================================================================
// pow
// =============================================================================

#[rust_lean_test]
pub fn test_u8_pow_zero_exp() -> bool {
    2u8.pow(0u32) == 1u8
}

#[rust_lean_test]
pub fn test_u8_pow_one_exp() -> bool {
    2u8.pow(1u32) == 2u8
}

#[rust_lean_test]
pub fn test_u8_pow_two_exp() -> bool {
    2u8.pow(2u32) == 4u8
}

#[rust_lean_test]
pub fn test_u8_pow_zero_base() -> bool {
    0u8.pow(2u32) == 0u8
}

#[rust_lean_test]
pub fn test_i8_pow_neg_base() -> bool {
    (-2i8).pow(2u32) == 4i8
}

// =============================================================================
// overflowing_pow
// =============================================================================

#[rust_lean_test]
pub fn test_u8_overflowing_pow_no_overflow() -> bool {
    2u8.overflowing_pow(3u32) == (8u8, false)
}

#[rust_lean_test]
pub fn test_u8_overflowing_pow_overflow() -> bool {
    // 4^4 = 256 wraps to 0.
    4u8.overflowing_pow(4u32) == (0u8, true)
}

#[rust_lean_test]
pub fn test_u8_overflowing_pow_zero_exp() -> bool {
    u8::MAX.overflowing_pow(0u32) == (1u8, false)
}

#[rust_lean_test]
pub fn test_i8_overflowing_pow_neg_base() -> bool {
    (-2i8).overflowing_pow(2u32) == (4i8, false)
}

#[rust_lean_test]
pub fn test_i8_overflowing_pow_overflow() -> bool {
    // (-2)^7 = -128 fits; (-2)^8 = 256 wraps to 0 with overflow.
    (-2i8).overflowing_pow(8u32) == (0i8, true)
}

// =============================================================================
// checked_pow
// =============================================================================

#[rust_lean_test]
pub fn test_u8_checked_pow_basic() -> bool {
    2u8.checked_pow(3u32).unwrap_or(0) == 8u8
}

#[rust_lean_test]
pub fn test_u8_checked_pow_zero_exp() -> bool {
    u8::MAX.checked_pow(0u32).unwrap_or(0) == 1u8
}

#[rust_lean_test]
pub fn test_u8_checked_pow_zero_base() -> bool {
    0u8.checked_pow(3u32).unwrap_or(1) == 0u8
}

#[rust_lean_test]
pub fn test_u8_checked_pow_at_max() -> bool {
    // 2^7 = 128 fits, 2^8 = 256 does not.
    2u8.checked_pow(7u32).unwrap_or(0) == 128u8
}

#[rust_lean_test]
pub fn test_u8_checked_pow_overflow() -> bool {
    2u8.checked_pow(8u32).is_none()
}

#[rust_lean_test]
pub fn test_u8_checked_pow_max_base_overflow() -> bool {
    u8::MAX.checked_pow(2u32).is_none()
}

#[rust_lean_test]
pub fn test_u32_checked_pow_basic() -> bool {
    10u32.checked_pow(9u32).unwrap_or(0) == 1_000_000_000u32
}

#[rust_lean_test]
pub fn test_u32_checked_pow_overflow() -> bool {
    10u32.checked_pow(10u32).is_none()
}

#[rust_lean_test]
pub fn test_i8_checked_pow_neg_base() -> bool {
    (-2i8).checked_pow(2u32).unwrap_or(0) == 4i8
}

#[rust_lean_test]
pub fn test_i8_checked_pow_neg_base_odd_exp() -> bool {
    (-2i8).checked_pow(3u32).unwrap_or(0) == -8i8
}

#[rust_lean_test]
pub fn test_i8_checked_pow_at_min() -> bool {
    // (-2)^7 = -128 = i8::MIN fits exactly.
    (-2i8).checked_pow(7u32).unwrap_or(0) == i8::MIN
}

#[rust_lean_test]
pub fn test_i8_checked_pow_overflow() -> bool {
    // (-2)^8 = 256 > i8::MAX.
    (-2i8).checked_pow(8u32).is_none()
}

// =============================================================================
// count_ones
// =============================================================================

#[rust_lean_test]
pub fn test_u8_count_ones_zero() -> bool {
    0u8.count_ones() == 0u32
}

#[rust_lean_test]
pub fn test_u8_count_ones_max() -> bool {
    u8::MAX.count_ones() == 8u32
}

#[rust_lean_test]
pub fn test_u8_count_ones_one() -> bool {
    1u8.count_ones() == 1u32
}

#[rust_lean_test]
pub fn test_u8_count_ones_pattern() -> bool {
    // 0b10101010 -> 4
    170u8.count_ones() == 4u32
}

#[rust_lean_test]
pub fn test_u32_count_ones_max() -> bool {
    u32::MAX.count_ones() == 32u32
}

#[rust_lean_test]
pub fn test_u64_count_ones_max() -> bool {
    u64::MAX.count_ones() == 64u32
}

#[rust_lean_test]
pub fn test_u128_count_ones_max() -> bool {
    u128::MAX.count_ones() == 128u32
}

#[rust_lean_test]
pub fn test_usize_count_ones_max() -> bool {
    usize::MAX.count_ones() == 64u32
}

#[rust_lean_test]
pub fn test_usize_count_ones_zero() -> bool {
    0usize.count_ones() == 0u32
}

// The signed models shift arithmetically, so a negative argument exercises the
// sign-extended high bits.
#[rust_lean_test]
pub fn test_i32_count_ones_minus_one() -> bool {
    (-1i32).count_ones() == 32u32
}

#[rust_lean_test]
pub fn test_i32_count_ones_min() -> bool {
    i32::MIN.count_ones() == 1u32
}

#[rust_lean_test]
pub fn test_i64_count_ones_minus_one() -> bool {
    (-1i64).count_ones() == 64u32
}

#[rust_lean_test]
pub fn test_isize_count_ones_minus_one() -> bool {
    (-1isize).count_ones() == 64u32
}

// =============================================================================
// rotate_left / rotate_right
// =============================================================================

#[rust_lean_test]
pub fn test_u8_rotate_left_zero() -> bool {
    0b10000001u8.rotate_left(0u32) == 0b10000001u8
}

#[rust_lean_test]
pub fn test_u8_rotate_left_one() -> bool {
    0b10000001u8.rotate_left(1u32) == 0b00000011u8
}

#[rust_lean_test]
pub fn test_u8_rotate_left_full() -> bool {
    0b10000001u8.rotate_left(8u32) == 0b10000001u8
}

#[rust_lean_test]
pub fn test_u8_rotate_right_zero() -> bool {
    0b10000001u8.rotate_right(0u32) == 0b10000001u8
}

#[rust_lean_test]
pub fn test_u8_rotate_right_one() -> bool {
    0b10000001u8.rotate_right(1u32) == 0b11000000u8
}

#[rust_lean_test]
pub fn test_u8_rotate_right_full() -> bool {
    0b10000001u8.rotate_right(8u32) == 0b10000001u8
}

#[rust_lean_test]
pub fn test_u8_rotate_left_wrapping() -> bool {
    0b10000001u8.rotate_left(9u32) == 0b00000011u8
}

#[rust_lean_test]
pub fn test_u8_rotate_right_wrapping() -> bool {
    0b10000001u8.rotate_right(9u32) == 0b11000000u8
}

#[rust_lean_test]
pub fn test_usize_rotate_left_zero() -> bool {
    1usize.rotate_left(0u32) == 1usize
}

#[rust_lean_test]
pub fn test_usize_rotate_left_one() -> bool {
    1usize.rotate_left(1u32) == 2usize
}

#[rust_lean_test]
pub fn test_usize_rotate_right_one() -> bool {
    2usize.rotate_right(1u32) == 1usize
}

#[rust_lean_test]
pub fn test_usize_rotate_left_wrapping() -> bool {
    1usize.rotate_left(65u32) == 2usize
}

#[rust_lean_test]
pub fn test_usize_rotate_right_wrapping() -> bool {
    2usize.rotate_right(65u32) == 1usize
}

// =============================================================================
// leading_zeros
// =============================================================================

#[rust_lean_test]
pub fn test_u8_leading_zeros_zero() -> bool {
    0u8.leading_zeros() == 8u32
}

#[rust_lean_test]
pub fn test_u8_leading_zeros_max() -> bool {
    u8::MAX.leading_zeros() == 0u32
}

#[rust_lean_test]
pub fn test_u8_leading_zeros_one() -> bool {
    1u8.leading_zeros() == 7u32
}

#[rust_lean_test]
pub fn test_u32_leading_zeros_max() -> bool {
    u32::MAX.leading_zeros() == 0u32
}

// =============================================================================
// ilog2 (skipped for x == 0; that case panics)
// =============================================================================

#[rust_lean_test]
pub fn test_u8_ilog2_one() -> bool {
    1u8.ilog2() == 0u32
}

#[rust_lean_test]
pub fn test_u8_ilog2_max() -> bool {
    u8::MAX.ilog2() == 7u32
}

#[rust_lean_test]
pub fn test_u8_ilog2_two() -> bool {
    2u8.ilog2() == 1u32
}

#[rust_lean_test]
pub fn test_u32_ilog2_max() -> bool {
    u32::MAX.ilog2() == 31u32
}

// =============================================================================
// is_power_of_two (unsigned only)
// =============================================================================

#[rust_lean_test]
pub fn test_u8_is_power_of_two_zero() -> bool {
    0u8.is_power_of_two() == false
}

#[rust_lean_test]
pub fn test_u8_is_power_of_two_one() -> bool {
    1u8.is_power_of_two() == true
}

#[rust_lean_test]
pub fn test_u8_is_power_of_two_two() -> bool {
    2u8.is_power_of_two() == true
}

#[rust_lean_test]
pub fn test_u8_is_power_of_two_three() -> bool {
    3u8.is_power_of_two() == false
}

#[rust_lean_test]
pub fn test_u8_is_power_of_two_128() -> bool {
    128u8.is_power_of_two() == true
}

#[rust_lean_test]
pub fn test_u8_is_power_of_two_max() -> bool {
    u8::MAX.is_power_of_two() == false
}

// =============================================================================
// abs (signed only)
// =============================================================================

#[rust_lean_test]
pub fn test_i8_abs_zero() -> bool {
    0i8.abs() == 0i8
}

#[rust_lean_test]
pub fn test_i8_abs_pos() -> bool {
    42i8.abs() == 42i8
}

#[rust_lean_test]
pub fn test_i8_abs_neg() -> bool {
    (-42i8).abs() == 42i8
}

#[rust_lean_test]
pub fn test_i8_abs_max() -> bool {
    i8::MAX.abs() == i8::MAX
}

#[rust_lean_test]
pub fn test_i8_abs_min_plus_one() -> bool {
    (i8::MIN + 1).abs() == i8::MAX
}

#[rust_lean_test]
pub fn test_i16_abs_neg() -> bool {
    (-100i16).abs() == 100i16
}

#[rust_lean_test]
pub fn test_i16_abs_min_plus_one() -> bool {
    (i16::MIN + 1).abs() == i16::MAX
}

#[rust_lean_test]
pub fn test_i32_abs_neg() -> bool {
    (-100000i32).abs() == 100000i32
}

#[rust_lean_test]
pub fn test_i32_abs_min_plus_one() -> bool {
    (i32::MIN + 1).abs() == i32::MAX
}

#[rust_lean_test]
pub fn test_i64_abs_neg() -> bool {
    (-100000i64).abs() == 100000i64
}

#[rust_lean_test]
pub fn test_i128_abs_neg() -> bool {
    (-100000i128).abs() == 100000i128
}

#[rust_lean_test]
pub fn test_isize_abs_neg() -> bool {
    (-100000isize).abs() == 100000isize
}

// =============================================================================
// unsigned_abs (signed only)
// =============================================================================

#[rust_lean_test]
pub fn test_i8_unsigned_abs_zero() -> bool {
    0i8.unsigned_abs() == 0u8
}

#[rust_lean_test]
pub fn test_i8_unsigned_abs_neg() -> bool {
    (-42i8).unsigned_abs() == 42u8
}

#[rust_lean_test]
pub fn test_i8_unsigned_abs_min() -> bool {
    i8::MIN.unsigned_abs() == 128u8
}

#[rust_lean_test]
pub fn test_i32_unsigned_abs_min() -> bool {
    i32::MIN.unsigned_abs() == 2147483648u32
}

#[rust_lean_test]
pub fn test_i64_unsigned_abs_neg() -> bool {
    (-100000i64).unsigned_abs() == 100000u64
}

// =============================================================================
// signum (signed only)
// =============================================================================

#[rust_lean_test]
pub fn test_i8_signum_zero() -> bool {
    0i8.signum() == 0i8
}

#[rust_lean_test]
pub fn test_i8_signum_pos() -> bool {
    42i8.signum() == 1i8
}

#[rust_lean_test]
pub fn test_i8_signum_neg() -> bool {
    (-42i8).signum() == -1i8
}

#[rust_lean_test]
pub fn test_i8_signum_max() -> bool {
    i8::MAX.signum() == 1i8
}

#[rust_lean_test]
pub fn test_i8_signum_min() -> bool {
    i8::MIN.signum() == -1i8
}

// =============================================================================
// checked_div / checked_rem
// =============================================================================

#[rust_lean_test]
pub fn test_u8_checked_div_basic() -> bool {
    10u8.checked_div(3u8) == Some(3u8)
}

#[rust_lean_test]
pub fn test_u8_checked_div_zero_divisor() -> bool {
    10u8.checked_div(0u8) == none_u8()
}

#[rust_lean_test]
pub fn test_u8_checked_div_zero_dividend() -> bool {
    0u8.checked_div(5u8) == Some(0u8)
}

#[rust_lean_test]
pub fn test_i8_checked_div_min_by_neg_one() -> bool {
    // i8::MIN / -1 would overflow -> None.
    i8::MIN.checked_div(-1i8) == none_i8()
}

#[rust_lean_test]
pub fn test_i8_checked_div_zero_divisor() -> bool {
    10i8.checked_div(0i8) == none_i8()
}

#[rust_lean_test]
pub fn test_u8_checked_rem_basic() -> bool {
    10u8.checked_rem(3u8) == Some(1u8)
}

#[rust_lean_test]
pub fn test_u8_checked_rem_zero_divisor() -> bool {
    10u8.checked_rem(0u8) == none_u8()
}

#[rust_lean_test]
pub fn test_i8_checked_rem_min_by_neg_one() -> bool {
    i8::MIN.checked_rem(-1i8) == none_i8()
}

#[rust_lean_test]
pub fn test_i8_checked_rem_zero_divisor() -> bool {
    10i8.checked_rem(0i8) == none_i8()
}

// =============================================================================
// from_be_bytes / from_le_bytes / to_be_bytes / to_le_bytes
// =============================================================================

#[rust_lean_test]
pub fn test_u16_from_be_bytes_basic() -> bool {
    u16::from_be_bytes([0x12, 0x34]) == 0x1234u16
}

#[rust_lean_test]
pub fn test_u16_from_le_bytes_basic() -> bool {
    u16::from_le_bytes([0x34, 0x12]) == 0x1234u16
}

#[rust_lean_test]
pub fn test_u16_from_be_bytes_zero() -> bool {
    u16::from_be_bytes([0u8, 0u8]) == 0u16
}

#[rust_lean_test]
pub fn test_u16_from_be_bytes_max() -> bool {
    u16::from_be_bytes([0xff, 0xff]) == u16::MAX
}

#[rust_lean_test]
pub fn test_u32_from_be_bytes_basic() -> bool {
    u32::from_be_bytes([0x12, 0x34, 0x56, 0x78]) == 0x12345678u32
}

#[rust_lean_test]
pub fn test_u32_from_le_bytes_basic() -> bool {
    u32::from_le_bytes([0x78, 0x56, 0x34, 0x12]) == 0x12345678u32
}

#[rust_lean_test]
pub fn test_u16_to_be_bytes_basic() -> bool {
    0x1234u16.to_be_bytes() == [0x12u8, 0x34u8]
}

#[rust_lean_test]
pub fn test_u16_to_le_bytes_basic() -> bool {
    0x1234u16.to_le_bytes() == [0x34u8, 0x12u8]
}

#[rust_lean_test]
pub fn test_u16_to_be_bytes_zero() -> bool {
    0u16.to_be_bytes() == [0u8, 0u8]
}

#[rust_lean_test]
pub fn test_u16_to_be_bytes_max() -> bool {
    u16::MAX.to_be_bytes() == [0xffu8, 0xffu8]
}

#[rust_lean_test]
pub fn test_u32_to_be_bytes_basic() -> bool {
    0x12345678u32.to_be_bytes() == [0x12u8, 0x34u8, 0x56u8, 0x78u8]
}

#[rust_lean_test]
pub fn test_u32_to_le_bytes_basic() -> bool {
    0x12345678u32.to_le_bytes() == [0x78u8, 0x56u8, 0x34u8, 0x12u8]
}

#[rust_lean_test]
pub fn test_u8_to_le_bytes_basic() -> bool {
    0xabu8.to_le_bytes() == [0xabu8]
}

#[rust_lean_test]
pub fn test_u64_to_le_bytes_basic() -> bool {
    0x0123456789abcdefu64.to_le_bytes()
        == [
            0xefu8, 0xcdu8, 0xabu8, 0x89u8, 0x67u8, 0x45u8, 0x23u8, 0x01u8,
        ]
}

#[rust_lean_test]
pub fn test_u64_to_le_bytes_max() -> bool {
    u64::MAX.to_le_bytes() == [0xffu8; 8]
}

#[rust_lean_test]
pub fn test_usize_to_le_bytes_basic() -> bool {
    1usize.to_le_bytes() == [1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]
}

// Signed: the bytes are the two's-complement ones, reached through an arithmetic
// shift in the model.
#[rust_lean_test]
pub fn test_i32_to_le_bytes_minus_one() -> bool {
    (-1i32).to_le_bytes() == [0xffu8, 0xffu8, 0xffu8, 0xffu8]
}

#[rust_lean_test]
pub fn test_i32_to_le_bytes_min() -> bool {
    i32::MIN.to_le_bytes() == [0u8, 0u8, 0u8, 0x80u8]
}

#[rust_lean_test]
pub fn test_i8_to_le_bytes_minus_one() -> bool {
    (-1i8).to_le_bytes() == [0xffu8]
}

// =============================================================================
// Default
// =============================================================================

#[rust_lean_test]
pub fn test_u8_default() -> bool {
    <u8 as Default>::default() == 0u8
}

#[rust_lean_test]
pub fn test_u16_default() -> bool {
    <u16 as Default>::default() == 0u16
}

#[rust_lean_test]
pub fn test_u32_default() -> bool {
    <u32 as Default>::default() == 0u32
}

#[rust_lean_test]
pub fn test_i8_default() -> bool {
    <i8 as Default>::default() == 0i8
}

#[rust_lean_test]
pub fn test_i16_default() -> bool {
    <i16 as Default>::default() == 0i16
}

#[rust_lean_test]
pub fn test_i32_default() -> bool {
    <i32 as Default>::default() == 0i32
}

#[rust_lean_test]
pub fn test_bool_default() -> bool {
    <bool as Default>::default() == false
}

// =============================================================================
// Cross-type sanity (suppress unused-import warning in case some specific
// helpers aren't otherwise exercised in this file).
// =============================================================================

#[rust_lean_test]
pub fn test_u16_checked_div_zero() -> bool {
    10u16.checked_div(0u16) == none_u16()
}

#[rust_lean_test]
pub fn test_u32_checked_div_zero() -> bool {
    10u32.checked_div(0u32) == none_u32()
}

#[rust_lean_test]
pub fn test_i16_checked_div_zero() -> bool {
    10i16.checked_div(0i16) == none_i16()
}

#[rust_lean_test]
pub fn test_i32_checked_div_zero() -> bool {
    10i32.checked_div(0i32) == none_i32()
}

// =============================================================================
// div_ceil (unsigned)
// =============================================================================
// Unsigned only: std's signed `div_ceil` is unstable (can't be called here); the
// signed model is covered by the `num` proptests.

#[rust_lean_test]
pub fn test_u8_div_ceil_exact() -> bool {
    8u8.div_ceil(4u8) == 2u8
}

#[rust_lean_test]
pub fn test_u8_div_ceil_round_up() -> bool {
    7u8.div_ceil(2u8) == 4u8
}

#[rust_lean_test]
pub fn test_u8_div_ceil_by_one() -> bool {
    200u8.div_ceil(1u8) == 200u8
}

#[rust_lean_test]
pub fn test_u8_div_ceil_zero_dividend() -> bool {
    0u8.div_ceil(7u8) == 0u8
}

#[rust_lean_test]
pub fn test_u32_div_ceil_round_up() -> bool {
    1000u32.div_ceil(3u32) == 334u32
}

// =============================================================================
// is_multiple_of (unsigned)
// =============================================================================

#[rust_lean_test]
pub fn test_u8_is_multiple_of_true() -> bool {
    8u8.is_multiple_of(4u8) == true
}

#[rust_lean_test]
pub fn test_u8_is_multiple_of_false() -> bool {
    7u8.is_multiple_of(4u8) == false
}

#[rust_lean_test]
pub fn test_u8_is_multiple_of_one() -> bool {
    7u8.is_multiple_of(1u8) == true
}

// 0 divides only 0
#[rust_lean_test]
pub fn test_u8_is_multiple_of_zero_rhs_zero() -> bool {
    0u8.is_multiple_of(0u8) == true
}

#[rust_lean_test]
pub fn test_u8_is_multiple_of_zero_rhs_nonzero() -> bool {
    5u8.is_multiple_of(0u8) == false
}

#[rust_lean_test]
pub fn test_u32_is_multiple_of_true() -> bool {
    1000u32.is_multiple_of(8u32) == true
}
