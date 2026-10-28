#![allow(non_camel_case_types, unused_variables)]

use crate::option::Option;
use crate::result::Result;
use pastey::paste;

pub mod error;

use rust_primitives::arithmetic::*;

// Bounds must be spelled `<$Name>::MAX`/`MIN` (referring to core models), not
// `$Self::MAX`/`MIN` (real `core`): both print the same, but only the former is a
// dependency hax sees, and cycles it misses become recursive backend modules.
macro_rules! uint_impl {
    (
        $Self: ty,
        $Name: ty,
        $Max: expr,
        $Bits: expr,
        $Bytes: expr,
    ) => {
        #[hax_lib::attributes]
        impl $Name {
            /// See [`std::primitive::u8::MIN`] (and similar for other unsigned integer types)
            pub const MIN: $Self = 0;
            /// See [`std::primitive::u8::MAX`] (and similar for other unsigned integer types)
            pub const MAX: $Self = $Max;
            /// See [`std::primitive::u8::BITS`] (and similar for other unsigned integer types)
            pub const BITS: core::primitive::u32 = $Bits;
            /// See [`std::primitive::u8::wrapping_add`] (and similar for other unsigned integer types)
            pub fn wrapping_add(x: $Self, y: $Self) -> $Self {
                paste! { [<wrapping_add_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
            pub fn saturating_add(x: $Self, y: $Self) -> $Self {
                paste! { [<saturating_add_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
            pub fn overflowing_add(x: $Self, y: $Self) -> ($Self, bool) {
                paste! { [<overflowing_add_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
            pub fn checked_add(x: $Self, y: $Self) -> Option<$Self> {
                let (result, overflowed) = Self::overflowing_add(x, y);
                if overflowed {
                    Option::None
                } else {
                    Option::Some(result)
                }
            }
            /// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
            #[hax_lib::requires(x.to_int() + y.to_int() <= <$Name>::MAX.to_int())]
            pub unsafe fn unchecked_add(x: $Self, y: $Self) -> $Self {
                x + y
            }
            /// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
            pub fn wrapping_sub(x: $Self, y: $Self) -> $Self {
                paste! { [<wrapping_sub_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
            pub fn saturating_sub(x: $Self, y: $Self) -> $Self {
                paste! { [<saturating_sub_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
            pub fn overflowing_sub(x: $Self, y: $Self) -> ($Self, bool) {
                paste! { [<overflowing_sub_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
            pub fn checked_sub(x: $Self, y: $Self) -> Option<$Self> {
                let (result, overflowed) = Self::overflowing_sub(x, y);
                if overflowed {
                    Option::None
                } else {
                    Option::Some(result)
                }
            }
            /// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
            #[hax_lib::requires(x >= y)]
            pub unsafe fn unchecked_sub(x: $Self, y: $Self) -> $Self {
                x - y
            }
            /// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
            pub fn wrapping_mul(x: $Self, y: $Self) -> $Self {
                paste! { [<wrapping_mul_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
            pub fn saturating_mul(x: $Self, y: $Self) -> $Self {
                paste! { [<saturating_mul_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
            pub fn overflowing_mul(x: $Self, y: $Self) -> ($Self, bool) {
                paste! { [<overflowing_mul_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
            pub fn checked_mul(x: $Self, y: $Self) -> Option<$Self> {
                let (result, overflowed) = Self::overflowing_mul(x, y);
                if overflowed {
                    Option::None
                } else {
                    Option::Some(result)
                }
            }
            /// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
            #[hax_lib::requires(x.to_int() * y.to_int() <= <$Name>::MAX.to_int())]
            pub unsafe fn unchecked_mul(x: $Self, y: $Self) -> $Self {
                x * y
            }
            /// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
            #[hax_lib::requires(y != 0)]
            pub fn rem_euclid(x: $Self, y: $Self) -> $Self {
                paste! { [<rem_euclid_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::pow`] (and similar for other integer types)
            #[hax_lib::requires(<$Name>::overflowing_pow(x, exp).1 == false)]
            pub fn pow(x: $Self, exp: core::primitive::u32) -> $Self {
                paste! { [<pow_ $Name>](x, exp) }
            }
            /// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
            pub fn overflowing_pow(x: $Self, exp: core::primitive::u32) -> ($Self, bool) {
                paste! { [<overflowing_pow_ $Name>](x, exp) }
            }
            /// See [`std::primitive::u8::checked_pow`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::exclude)] //avoid cyclic dependency
            pub fn checked_pow(x: $Self, exp: core::primitive::u32) -> Option<$Self> {
                let (result, overflowed) = Self::overflowing_pow(x, exp);
                if overflowed {
                    Option::None
                } else {
                    Option::Some(result)
                }
            }
            /// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
            pub fn count_ones(x: $Self) -> core::primitive::u32 {
                let mut n = 0u32;
                for i in 0u32..$Bits {
                    // Rules out the `n + 1` overflow; F* alone needs it.
                    #[cfg(hax_backend_fstar)]
                    hax_lib::loop_invariant!(|i: core::primitive::u32| n <= i);
                    if (x >> i) & 1 == 1 {
                        n += 1;
                    }
                }
                n
            }
            /// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::fstar::before("[@@ \"opaque_to_smt\"]"))]
            pub fn rotate_right(x: $Self, n: core::primitive::u32) -> $Self {
                let m = n % $Bits;
                if m == 0 {
                    x
                } else {
                    (x >> m) ^ (x << ($Bits - m))
                }
            }
            /// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::fstar::before("[@@ \"opaque_to_smt\"]"))]
            pub fn rotate_left(x: $Self, n: core::primitive::u32) -> $Self {
                let m = n % $Bits;
                if m == 0 {
                    x
                } else {
                    (x << m) ^ (x >> ($Bits - m))
                }
            }
            /// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
            pub fn leading_zeros(x: $Self) -> core::primitive::u32 {
                paste! { [<leading_zeros_ $Name>](x) }
            }
            /// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
            #[hax_lib::requires(x > 0)]
            pub fn ilog2(x: $Self) -> core::primitive::u32 {
                paste! { [<ilog2_ $Name>](x) }
            }
            /// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
            #[hax_lib::opaque]
            #[hax_lib::requires(radix >= 2 && radix <= 36)]
            pub fn from_str_radix(
                src: &str,
                radix: core::primitive::u32,
            ) -> Result<$Self, error::ParseIntError> {
                let (parsed, value) = paste! { [<from_str_radix_ $Name>](src, radix) };
                if parsed {
                    Result::Ok(value)
                } else {
                    // The model's `ParseIntError` carries no distinguishable kind.
                    Result::Err(error::ParseIntError {
                        kind: error::IntErrorKind,
                    })
                }
            }
            /// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
            pub fn from_be_bytes(bytes: [core::primitive::u8; $Bytes]) -> $Self {
                paste! { [<from_be_bytes_ $Name>](bytes) }
            }
            /// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
            pub fn from_le_bytes(bytes: [core::primitive::u8; $Bytes]) -> $Self {
                paste! { [<from_le_bytes_ $Name>](bytes) }
            }
            /// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
            pub fn to_be_bytes(bytes: $Self) -> [core::primitive::u8; $Bytes] {
                paste! { [<to_be_bytes_ $Name>](bytes) }
            }
            /// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
            pub fn to_le_bytes(bytes: $Self) -> [core::primitive::u8; $Bytes] {
                rust_primitives::slice::array_from_fn(|i| {
                    // The identity, and what proves the shift in range.
                    (bytes >> (8u32 * i as core::primitive::u32 % $Bits)) as core::primitive::u8
                })
            }
            /// See [`std::primitive::u8::checked_div`] (and similar for other integer types)
            pub fn checked_div(x: $Self, y: $Self) -> Option<$Self> {
                if y == 0 {
                    Option::None
                } else {
                    Option::Some(x / y)
                }
            }
            /// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
            #[hax_lib::requires(y != 0)]
            pub unsafe fn unchecked_div(x: $Self, y: $Self) -> $Self {
                x / y
            }
            /// See [`std::primitive::u8::checked_rem`] (and similar for other integer types)
            pub fn checked_rem(x: $Self, y: $Self) -> Option<$Self> {
                if y == 0 {
                    Option::None
                } else {
                    Option::Some(x % y)
                }
            }
            /// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
            #[hax_lib::requires(y != 0)]
            pub unsafe fn unchecked_rem(x: $Self, y: $Self) -> $Self {
                x % y
            }
            /// See [`std::primitive::u8::is_power_of_two`] (and similar for other unsigned integer types)
            pub fn is_power_of_two(x: $Self) -> bool {
                x != 0 && (x & (x - 1)) == 0
            }
            /// See [`std::primitive::u8::div_ceil`] (and similar for other unsigned integer types)
            #[hax_lib::requires(y != 0)]
            pub fn div_ceil(x: $Self, y: $Self) -> $Self {
                let d = x / y;
                let r = x % y;
                if r > 0 { d + 1 } else { d }
            }
            /// See [`std::primitive::u8::is_multiple_of`] (and similar for other unsigned integer types)
            pub fn is_multiple_of(x: $Self, y: $Self) -> bool {
                if y == 0 {
                    x == 0 // 0 divides only 0
                } else {
                    x % y == 0
                }
            }
            // The following methods require additions to rust_primitives:
            // /// See [`std::primitive::u8::trailing_zeros`] (and similar for other integer types)
            // #[hax_lib::opaque]
            // fn trailing_zeros(x: $Self) -> core::primitive::u32 {
            //     paste! { [<trailing_zeros_ $Name>](x) }
            // }
            // /// See [`std::primitive::u8::swap_bytes`] (and similar for other integer types)
            // #[hax_lib::opaque]
            // fn swap_bytes(x: $Self) -> $Self {
            //     paste! { [<swap_bytes_ $Name>](x) }
            // }
            /// See [`std::primitive::u8::wrapping_neg`] (and similar for other integer types)
            // Modelled as `0.wrapping_sub(x)` (the definition of `wrapping_neg`)
            // to reuse the existing `wrapping_sub` primitive.
            pub fn wrapping_neg(x: $Self) -> $Self {
                paste! { [<wrapping_sub_ $Name>](0, x) }
            }
        }
    };
}

use hax_lib::int::ToInt;

macro_rules! iint_impl {
    (
        $Self: ty,
        $USelf: ty,
        $Name: ty,
        $Max: expr,
        $Min: expr,
        $Bits: expr,
        $Bytes: expr,
    ) => {
        #[hax_lib::attributes]
        impl $Name {
            /// See [`std::primitive::i8::MIN`] (and similar for other signed integer types)
            pub const MIN: $Self = $Min;
            /// See [`std::primitive::i8::MAX`] (and similar for other signed integer types)
            pub const MAX: $Self = $Max;
            /// See [`std::primitive::i8::BITS`] (and similar for other signed integer types)
            pub const BITS: core::primitive::u32 = $Bits;
            pub fn wrapping_add(x: $Self, y: $Self) -> $Self {
                paste! { [<wrapping_add_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::saturating_add`] (and similar for other integer types)
            pub fn saturating_add(x: $Self, y: $Self) -> $Self {
                paste! { [<saturating_add_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::overflowing_add`] (and similar for other integer types)
            pub fn overflowing_add(x: $Self, y: $Self) -> ($Self, bool) {
                paste! { [<overflowing_add_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::checked_add`] (and similar for other integer types)
            pub fn checked_add(x: $Self, y: $Self) -> Option<$Self> {
                let (result, overflowed) = Self::overflowing_add(x, y);
                if overflowed {
                    Option::None
                } else {
                    Option::Some(result)
                }
            }
            /// See [`std::primitive::u8::unchecked_add`] (and similar for other integer types)
            #[hax_lib::requires(x.to_int() + y.to_int() <= <$Name>::MAX.to_int() && x.to_int() + y.to_int() >= <$Name>::MIN.to_int())]
            pub unsafe fn unchecked_add(x: $Self, y: $Self) -> $Self {
                x + y
            }
            /// See [`std::primitive::u8::wrapping_sub`] (and similar for other integer types)
            pub fn wrapping_sub(x: $Self, y: $Self) -> $Self {
                paste! { [<wrapping_sub_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::saturating_sub`] (and similar for other integer types)
            pub fn saturating_sub(x: $Self, y: $Self) -> $Self {
                paste! { [<saturating_sub_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::overflowing_sub`] (and similar for other integer types)
            pub fn overflowing_sub(x: $Self, y: $Self) -> ($Self, bool) {
                paste! { [<overflowing_sub_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::checked_sub`] (and similar for other integer types)
            pub fn checked_sub(x: $Self, y: $Self) -> Option<$Self> {
                let (result, overflowed) = Self::overflowing_sub(x, y);
                if overflowed {
                    Option::None
                } else {
                    Option::Some(result)
                }
            }
            /// See [`std::primitive::u8::unchecked_sub`] (and similar for other integer types)
            #[hax_lib::requires(x.to_int() - y.to_int() <= <$Name>::MAX.to_int() && x.to_int() - y.to_int() >= <$Name>::MIN.to_int())]
            pub unsafe fn unchecked_sub(x: $Self, y: $Self) -> $Self {
                x - y
            }
            /// See [`std::primitive::i8::checked_add_unsigned`] (and similar for other signed integer types)
            pub fn checked_add_unsigned(x: $Self, y: $USelf) -> Option<$Self> {
                // Signed overflow from wrapping_add(x, y as $Self) represents unsigned overflow
                // iff the signed overflow flag matches whether y exceeds the signed maximum.
                let (result, overflowed) = Self::overflowing_add(x, y as $Self);
                if overflowed == (y > <$Name>::MAX as $USelf) {
                    Option::Some(result)
                } else {
                    Option::None
                }
            }
            /// See [`std::primitive::i8::checked_sub_unsigned`] (and similar for other signed integer types)
            pub fn checked_sub_unsigned(x: $Self, y: $USelf) -> Option<$Self> {
                let (result, overflowed) = Self::overflowing_sub(x, y as $Self);
                if overflowed == (y > <$Name>::MAX as $USelf) {
                    Option::Some(result)
                } else {
                    Option::None
                }
            }
            /// See [`std::primitive::u8::wrapping_mul`] (and similar for other integer types)
            pub fn wrapping_mul(x: $Self, y: $Self) -> $Self {
                paste! { [<wrapping_mul_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::saturating_mul`] (and similar for other integer types)
            pub fn saturating_mul(x: $Self, y: $Self) -> $Self {
                paste! { [<saturating_mul_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::overflowing_mul`] (and similar for other integer types)
            pub fn overflowing_mul(x: $Self, y: $Self) -> ($Self, bool) {
                paste! { [<overflowing_mul_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::checked_mul`] (and similar for other integer types)
            pub fn checked_mul(x: $Self, y: $Self) -> Option<$Self> {
                let (result, overflowed) = Self::overflowing_mul(x, y);
                if overflowed {
                    Option::None
                } else {
                    Option::Some(result)
                }
            }
            /// See [`std::primitive::u8::unchecked_mul`] (and similar for other integer types)
            #[hax_lib::requires(x.to_int() * y.to_int() <= <$Name>::MAX.to_int() && x.to_int() * y.to_int() >= <$Name>::MIN.to_int())]
            pub unsafe fn unchecked_mul(x: $Self, y: $Self) -> $Self {
                x * y
            }
            /// See [`std::primitive::u8::rem_euclid`] (and similar for other integer types)
            // `MIN % -1` overflows, like `MIN / -1`.
            #[hax_lib::requires(y != 0 && !(x == <$Name>::MIN && y == -1))]
            pub fn rem_euclid(x: $Self, y: $Self) -> $Self {
                paste! { [<rem_euclid_ $Name>](x, y) }
            }
            /// See [`std::primitive::u8::pow`] (and similar for other integer types)
            #[hax_lib::requires(<$Name>::overflowing_pow(x, exp).1 == false)]
            pub fn pow(x: $Self, exp: core::primitive::u32) -> $Self {
                paste! { [<pow_ $Name>](x, exp) }
            }
            /// See [`std::primitive::u8::overflowing_pow`] (and similar for other integer types)
            pub fn overflowing_pow(x: $Self, exp: core::primitive::u32) -> ($Self, bool) {
                paste! { [<overflowing_pow_ $Name>](x, exp) }
            }
            /// See [`std::primitive::u8::checked_pow`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::exclude)] //avoid cyclic dependency
            pub fn checked_pow(x: $Self, exp: core::primitive::u32) -> Option<$Self> {
                let (result, overflowed) = Self::overflowing_pow(x, exp);
                if overflowed {
                    Option::None
                } else {
                    Option::Some(result)
                }
            }
            /// See [`std::primitive::u8::count_ones`] (and similar for other integer types)
            pub fn count_ones(x: $Self) -> core::primitive::u32 {
                let mut n = 0u32;
                for i in 0u32..$Bits {
                    // Rules out the `n + 1` overflow; F* alone needs it.
                    #[cfg(hax_backend_fstar)]
                    hax_lib::loop_invariant!(|i: core::primitive::u32| n <= i);
                    if (x >> i) & 1 == 1 {
                        n += 1;
                    }
                }
                n
            }
            /// See [`std::primitive::i8::abs`] (and similar for other signed integer types)
            #[hax_lib::requires(x > <$Name>::MIN)]
            pub fn abs(x: $Self) -> $Self {
                if x < 0 { -x } else { x }
            }
            /// See [`std::primitive::i8::unsigned_abs`] (and similar for other signed integer types)
            pub fn unsigned_abs(x: $Self) -> $USelf {
                if x < 0 { (-(x + 1)) as $USelf + 1 } else { x as $USelf }
            }
            /// See [`std::primitive::u8::rotate_right`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
            pub fn rotate_right(x: $Self, n: core::primitive::u32) -> $Self {
                paste! { [<rotate_right_ $Name>](x, n) }
            }
            /// See [`std::primitive::u8::rotate_left`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
            pub fn rotate_left(x: $Self, n: core::primitive::u32) -> $Self {
                paste! { [<rotate_left_ $Name>](x, n) }
            }
            /// See [`std::primitive::u8::leading_zeros`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
            pub fn leading_zeros(x: $Self) -> core::primitive::u32 {
                paste! { [<leading_zeros_ $Name>](x) }
            }
            /// See [`std::primitive::u8::ilog2`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
            #[hax_lib::requires(x > 0)]
            pub fn ilog2(x: $Self) -> core::primitive::u32 {
                paste! { [<ilog2_ $Name>](x) }
            }
            /// See [`std::primitive::u8::from_str_radix`] (and similar for other integer types)
            #[hax_lib::opaque]
            #[hax_lib::requires(radix >= 2 && radix <= 36)]
            pub fn from_str_radix(
                src: &str,
                radix: core::primitive::u32,
            ) -> Result<$Self, error::ParseIntError> {
                let (parsed, value) = paste! { [<from_str_radix_ $Name>](src, radix) };
                if parsed {
                    Result::Ok(value)
                } else {
                    // The model's `ParseIntError` carries no distinguishable kind.
                    Result::Err(error::ParseIntError {
                        kind: error::IntErrorKind,
                    })
                }
            }
            /// See [`std::primitive::u8::from_be_bytes`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
            pub fn from_be_bytes(bytes: [core::primitive::u8; $Bytes]) -> $Self {
                paste! { [<from_be_bytes_ $Name>](bytes) }
            }
            /// See [`std::primitive::u8::from_le_bytes`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
            pub fn from_le_bytes(bytes: [core::primitive::u8; $Bytes]) -> $Self {
                paste! { [<from_le_bytes_ $Name>](bytes) }
            }
            /// See [`std::primitive::u8::to_be_bytes`] (and similar for other integer types)
            #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
            pub fn to_be_bytes(bytes: $Self) -> [core::primitive::u8; $Bytes] {
                paste! { [<to_be_bytes_ $Name>](bytes) }
            }
            /// See [`std::primitive::u8::to_le_bytes`] (and similar for other integer types)
            pub fn to_le_bytes(bytes: $Self) -> [core::primitive::u8; $Bytes] {
                rust_primitives::slice::array_from_fn(|i| {
                    // The identity, and what proves the shift in range.
                    (bytes >> (8u32 * i as core::primitive::u32 % $Bits))
                        as core::primitive::u8
                })
            }
            /// See [`std::primitive::i8::checked_div`] (and similar for other signed integer types)
            pub fn checked_div(x: $Self, y: $Self) -> Option<$Self> {
                if y == 0 || (x == <$Name>::MIN && y == -1) {
                    Option::None
                } else {
                    Option::Some(x / y)
                }
            }
            /// See [`std::primitive::u8::unchecked_div`] (and similar for other integer types)
            #[hax_lib::requires(y != 0 && (x != <$Name>::MIN || y != -1))]
            pub unsafe fn unchecked_div(x: $Self, y: $Self) -> $Self {
                x / y
            }
            /// See [`std::primitive::i8::checked_rem`] (and similar for other signed integer types)
            pub fn checked_rem(x: $Self, y: $Self) -> Option<$Self> {
                if y == 0 || (x == <$Name>::MIN && y == -1) {
                    Option::None
                } else {
                    Option::Some(x % y)
                }
            }
            /// See [`std::primitive::u8::unchecked_rem`] (and similar for other integer types)
            #[hax_lib::requires(y != 0 && (x != <$Name>::MIN || y != -1))]
            pub unsafe fn unchecked_rem(x: $Self, y: $Self) -> $Self {
                x % y
            }
            /// See [`std::primitive::i8::signum`] (and similar for other signed integer types)
            pub fn signum(x: $Self) -> $Self {
                if x > 0 {
                    1
                } else if x == 0 {
                    0
                } else {
                    -1
                }
            }
            /// See [`std::primitive::i8::div_ceil`] (and similar for other signed integer types)
            // `requires` rules out the div-by-zero and `MIN / -1` panics.
            #[hax_lib::requires(y != 0 && !(x == <$Name>::MIN && y == -1))]
            pub fn div_ceil(x: $Self, y: $Self) -> $Self {
                let d = x / y;
                let r = x % y;
                // round up only when the remainder shares the divisor's sign
                if (r > 0 && y > 0) || (r < 0 && y < 0) {
                    d + 1
                } else {
                    d
                }
            }
            // The following methods require additions to rust_primitives:
            // /// See [`std::primitive::i8::trailing_zeros`] (and similar for other signed integer types)
            // #[hax_lib::opaque]
            // fn trailing_zeros(x: $Self) -> core::primitive::u32 {
            //     paste! { [<trailing_zeros_ $Name>](x) }
            // }
            // /// See [`std::primitive::i8::swap_bytes`] (and similar for other signed integer types)
            // #[hax_lib::opaque]
            // fn swap_bytes(x: $Self) -> $Self {
            //     paste! { [<swap_bytes_ $Name>](x) }
            // }
            /// See [`std::primitive::i8::wrapping_neg`] (and similar for other signed integer types)
            // Modelled as `0.wrapping_sub(x)` (the definition of `wrapping_neg`)
            // to reuse the existing `wrapping_sub` primitive.
            pub fn wrapping_neg(x: $Self) -> $Self {
                paste! { [<wrapping_sub_ $Name>](0, x) }
            }
        }
    };
}

// These types are a trick to define impls on the right names as
// it is forbidden to do it on primitive types
/// See [`std::primitive::u8`]
// F*-only: `charon::exclude` would drop these dummy types while their `impl`
// blocks still reference them (see f32.rs).
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
pub struct u8;
/// See [`std::primitive::u16`]
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
pub struct u16;
/// See [`std::primitive::u32`]
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
pub struct u32;
/// See [`std::primitive::u64`]
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
pub struct u64;
/// See [`std::primitive::u128`]
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
pub struct u128;
/// See [`std::primitive::usize`]
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
pub struct usize;
/// See [`std::primitive::i8`]
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
pub struct i8;
/// See [`std::primitive::i16`]
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
pub struct i16;
/// See [`std::primitive::i32`]
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
pub struct i32;
/// See [`std::primitive::i64`]
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
pub struct i64;
/// See [`std::primitive::i128`]
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
pub struct i128;
/// See [`std::primitive::isize`]
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
pub struct isize;

// Placeholders to get the same impl numbering as in core:
#[hax_lib::attributes]
impl i8 {}
#[hax_lib::attributes]
impl i16 {}
#[hax_lib::attributes]
impl i32 {}
#[hax_lib::attributes]
impl i64 {}
#[hax_lib::attributes]
impl i128 {}
#[hax_lib::attributes]
impl isize {}

uint_impl! {
    core::primitive::u8,
    u8,
    255,
    8,
    1,
}

uint_impl! {
    core::primitive::u16,
    u16,
    65535,
    16,
    2,
}

uint_impl! {
    core::primitive::u32,
    u32,
    4294967295,
    32,
    4,
}

uint_impl! {
    core::primitive::u64,
    u64,
    18446744073709551615,
    64,
    8,
}

uint_impl! {
    core::primitive::u128,
    u128,
    340282366920938463463374607431768211455,
    128,
    16,
}

uint_impl! {
    core::primitive::usize,
    usize,
    USIZE_MAX,
    // A const-ref, not a literal, so `usize::BITS` extracts as `RustM _` while
    // a consumer's `usize::BITS` is pure. `patch_lean.py` drops the extracted
    // definition in favour of the pure `CoreModels.Core.FunsPrologue` one.
    SIZE_BITS,
    SIZE_BYTES,
}

iint_impl! {
    core::primitive::i8,
    core::primitive::u8,
    i8,
    127,
    -128,
    8,
    1,
}

iint_impl! {
    core::primitive::i16,
    core::primitive::u16,
    i16,
    32767,
    -32768,
    16,
    2,
}

iint_impl! {
    core::primitive::i32,
    core::primitive::u32,
    i32,
    2147483647,
    -2147483648,
    32,
    4,
}

iint_impl! {
    core::primitive::i64,
    core::primitive::u64,
    i64,
    9223372036854775807,
    -9223372036854775808,
    64,
    8,
}

iint_impl! {
    core::primitive::i128,
    core::primitive::u128,
    i128,
    170141183460469231731687303715884105727,
    -170141183460469231731687303715884105728,
    128,
    16,
}

iint_impl! {
    core::primitive::isize,
    core::primitive::usize,
    isize,
    ISIZE_MAX,
    ISIZE_MIN,
    // Extracts as `RustM _`; see the `usize` note above.
    SIZE_BITS,
    SIZE_BYTES,
}

macro_rules! impl_default_for_int {
    ($($t:ty),*) => {
        $(
            #[hax_lib::attributes]
            impl crate::default::Default for $t {
                fn default() -> $t {
                    0
                }
            }
        )*
    };
}

impl_default_for_int!(
    core::primitive::u8,
    core::primitive::u16,
    core::primitive::u32,
    core::primitive::u64,
    core::primitive::u128,
    core::primitive::usize,
    core::primitive::i8,
    core::primitive::i16,
    core::primitive::i32,
    core::primitive::i64,
    core::primitive::i128,
    core::primitive::isize
);

#[hax_lib::attributes]
impl crate::default::Default for bool {
    /// See [`std::default::Default`]
    fn default() -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::Inject;
    use pastey::paste;
    use proptest::prelude::*;

    macro_rules! int_test {
        ($($t:ty)*) => {
            paste! {
                $(
                    #[test]
                    fn [<test_ $t _min>]() {
                        assert_eq!(super::$t::MIN, $t::MIN)
                    }
                    #[test]
                    fn [<test_ $t _max>]() {
                        assert_eq!(super::$t::MAX, $t::MAX)
                    }
                    #[test]
                    fn [<test_ $t _bits>]() {
                        assert_eq!(super::$t::BITS, $t::BITS)
                    }
                    proptest! {
                        #[test]
                        fn [<test_ $t _wrapping_add>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::wrapping_add(x.inject(), y.inject()), x.wrapping_add(y));
                        }

                        #[test]
                        fn [<test_ $t _saturating_add>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::saturating_add(x.inject(), y.inject()), x.saturating_add(y));
                        }

                        #[test]
                        fn [<test_ $t _overflowing_add>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::overflowing_add(x.inject(), y.inject()), x.overflowing_add(y));
                        }

                        #[test]
                        fn [<test_ $t _checked_add>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::checked_add(x.inject(), y.inject()), x.checked_add(y).inject());
                        }

                        #[test]
                        fn [<test_ $t _checked_sub>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::checked_sub(x.inject(), y.inject()), x.checked_sub(y).inject());
                        }

                        #[test]
                        fn [<test_ $t _checked_mul>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::checked_mul(x.inject(), y.inject()), x.checked_mul(y).inject());
                        }

                        #[test]
                        fn [<test_ $t _overflowing_pow>](x in any::<$t>(), exp in 0u32..=140) {
                            prop_assert_eq!(super::$t::overflowing_pow(x.inject(), exp), x.overflowing_pow(exp));
                        }

                        #[test]
                        fn [<test_ $t _checked_pow>](x in any::<$t>(), exp in 0u32..=140) {
                            prop_assert_eq!(super::$t::checked_pow(x.inject(), exp), x.checked_pow(exp).inject());
                        }

                        #[test]
                        fn [<test_ $t _wrapping_sub>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::wrapping_sub(x.inject(), y.inject()), x.wrapping_sub(y));
                        }

                        #[test]
                        fn [<test_ $t _wrapping_neg>](x in any::<$t>()) {
                            prop_assert_eq!(super::$t::wrapping_neg(x.inject()), x.wrapping_neg());
                        }

                        #[test]
                        fn [<test_ $t _saturating_sub>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::saturating_sub(x.inject(), y.inject()), x.saturating_sub(y));
                        }

                        #[test]
                        fn [<test_ $t _overflowing_sub>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::overflowing_sub(x.inject(), y.inject()), x.overflowing_sub(y));
                        }

                        #[test]
                        fn [<test_ $t _wrapping_mul>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::wrapping_mul(x.inject(), y.inject()), x.wrapping_mul(y));
                        }

                        #[test]
                        fn [<test_ $t _saturating_mul>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::saturating_mul(x.inject(), y.inject()), x.saturating_mul(y));
                        }

                        #[test]
                        fn [<test_ $t _overflowing_mul>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::overflowing_mul(x.inject(), y.inject()), x.overflowing_mul(y));
                        }

                        // `checked_rem_euclid`, not `y != 0`: signed `MIN % -1`
                        // overflows too, and both sides panic on it.
                        #[test]
                        fn [<test_ $t _rem_euclid>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assume!(x.checked_rem_euclid(y).is_some());
                            prop_assert_eq!(super::$t::rem_euclid(x.inject(), y.inject()), x.rem_euclid(y));
                        }

                        #[test]
                        fn [<test_ $t _count_ones>](x in any::<$t>()) {
                            prop_assert_eq!(super::$t::count_ones(x.inject()), x.count_ones());
                        }

                        // `n` past `BITS` too: Rust rotates by `n % BITS`.
                        #[test]
                        fn [<test_ $t _rotate_right>](
                            x in any::<$t>(),
                            n in prop_oneof![Just(0u32), Just($t::BITS), 0u32..$t::BITS, any::<u32>()],
                        ) {
                            prop_assert_eq!(super::$t::rotate_right(x.inject(), n), x.rotate_right(n));
                        }

                        #[test]
                        fn [<test_ $t _rotate_left>](
                            x in any::<$t>(),
                            n in prop_oneof![Just(0u32), Just($t::BITS), 0u32..$t::BITS, any::<u32>()],
                        ) {
                            prop_assert_eq!(super::$t::rotate_left(x.inject(), n), x.rotate_left(n));
                        }

                        #[test]
                        fn [<test_ $t _leading_zeros>](x in any::<$t>()) {
                            prop_assert_eq!(super::$t::leading_zeros(x.inject()), x.leading_zeros());
                        }

                        #[test]
                        fn [<test_ $t _from_be_bytes>](bytes in any::<[u8; $t::BITS as usize / 8]>()) {
                            prop_assert_eq!(super::$t::from_be_bytes(bytes.inject()), $t::from_be_bytes(bytes));
                        }

                        #[test]
                        fn [<test_ $t _from_le_bytes>](bytes in any::<[u8; $t::BITS as usize / 8]>()) {
                            prop_assert_eq!(super::$t::from_le_bytes(bytes.inject()), $t::from_le_bytes(bytes));
                        }

                        #[test]
                        fn [<test_ $t _to_be_bytes>](x in any::<$t>()) {
                            prop_assert_eq!(super::$t::to_be_bytes(x.inject()), x.to_be_bytes().inject());
                        }

                        #[test]
                        fn [<test_ $t _to_le_bytes>](x in any::<$t>()) {
                            prop_assert_eq!(super::$t::to_le_bytes(x.inject()), x.to_le_bytes().inject());
                        }

                        #[test]
                        fn [<test_ $t _checked_div>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::checked_div(x.inject(), y.inject()), x.checked_div(y).inject());
                        }

                        #[test]
                        fn [<test_ $t _checked_rem>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::checked_rem(x.inject(), y.inject()), x.checked_rem(y).inject());
                        }

                        // `y` is fixed at zero: a full-range `y` almost never hits
                        // the divide-by-zero arm for the wider types.
                        #[test]
                        fn [<test_ $t _checked_div_by_zero>](x in any::<$t>()) {
                            prop_assert_eq!(
                                super::$t::checked_div(x.inject(), (0 as $t).inject()),
                                x.checked_div(0).inject());
                        }

                        #[test]
                        fn [<test_ $t _checked_rem_by_zero>](x in any::<$t>()) {
                            prop_assert_eq!(
                                super::$t::checked_rem(x.inject(), (0 as $t).inject()),
                                x.checked_rem(0).inject());
                        }

                        // Three generators: short digits mostly parse, the wider
                        // alphabet overflows or fails per radix, `.*` is junk.
                        #[test]
                        fn [<test_ $t _from_str_radix>](
                            s in prop_oneof!["[0-9]{1,2}", "[0-9a-zA-Z+-]{0,12}", ".*"],
                            radix in 2u32..=36,
                        ) {
                            prop_assert_eq!(
                                super::$t::from_str_radix(&s, radix),
                                $t::from_str_radix(&s, radix).inject()
                            );
                        }

                        // `div_ceil` panics exactly where `checked_div` is `None`.
                        #[test]
                        fn [<test_ $t _div_ceil>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assume!(x.checked_div(y).is_some());
                            prop_assert_eq!(super::$t::div_ceil(x.inject(), y.inject()), x.div_ceil(y));
                        }
                    }
                )*
            }
        }
    }

    // Tests for unsigned-only operations.
    macro_rules! uint_test {
        ($($t:ty)*) => {
            paste! {
                $(
                    proptest! {
                        #[test]
                        fn [<test_ $t _pow>](
                            x in prop_oneof![(0 as $t)..=(16 as $t), any::<$t>()],
                            exp in 0u32..=4,
                        ) {
                            prop_assume!(x.checked_pow(exp).is_some());
                            prop_assert_eq!(super::$t::pow(x.inject(), exp), x.pow(exp));
                        }

                        #[test]
                        fn [<test_ $t _ilog2>](x in any::<$t>()) {
                            if x > 0 {
                                prop_assert_eq!(super::$t::ilog2(x.inject()), x.ilog2());
                            }
                        }

                        #[test]
                        fn [<test_ $t _is_power_of_two>](x in any::<$t>()) {
                            prop_assert_eq!(super::$t::is_power_of_two(x.inject()), x.is_power_of_two());
                        }

                        #[test]
                        fn [<test_ $t _is_multiple_of>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assert_eq!(super::$t::is_multiple_of(x.inject(), y.inject()), x.is_multiple_of(y));
                        }

                        // Zero divides only zero; `y` is pinned so the arm is
                        // always taken, and `x` covers both answers.
                        #[test]
                        fn [<test_ $t _is_multiple_of_zero>](x in prop_oneof![Just(0 as $t), any::<$t>()]) {
                            prop_assert_eq!(
                                super::$t::is_multiple_of(x.inject(), (0 as $t).inject()),
                                x.is_multiple_of(0));
                        }
                    }
                )*
            }
        }
    }

    // Tests for signed-only operations.
    macro_rules! iint_test {
        ($($t:ty)*) => {
            paste! {
                $(
                    proptest! {
                        #[test]
                        fn [<test_ $t _pow>](
                            x in prop_oneof![(-16 as $t)..=(16 as $t), any::<$t>()],
                            exp in 0u32..=4,
                        ) {
                            prop_assume!(x.checked_pow(exp).is_some());
                            prop_assert_eq!(super::$t::pow(x.inject(), exp), x.pow(exp));
                        }

                        #[test]
                        fn [<test_ $t _abs>](x in any::<$t>()) {
                            if x != $t::MIN {
                                prop_assert_eq!(super::$t::abs(x.inject()), x.abs());
                            }
                        }

                        // `MIN` is biased in: the only input where `-x` overflows.
                        #[test]
                        fn [<test_ $t _unsigned_abs>](x in prop_oneof![Just(<$t>::MIN), any::<$t>()]) {
                            prop_assert_eq!(super::$t::unsigned_abs(x.inject()), x.unsigned_abs());
                        }

                        #[test]
                        fn [<test_ $t _ilog2>](x in any::<$t>()) {
                            if x > 0 {
                                prop_assert_eq!(super::$t::ilog2(x.inject()), x.ilog2());
                            }
                        }

                        // `x` is pinned to `MIN` so that the `y == -1` half of the
                        // `checked_div`/`checked_rem` overflow guard is reached.
                        #[test]
                        fn [<test_ $t _checked_div_at_min>](y in prop_oneof![Just(-1 as $t), any::<$t>()]) {
                            prop_assert_eq!(
                                super::$t::checked_div(<$t>::MIN.inject(), y.inject()),
                                <$t>::MIN.checked_div(y).inject());
                        }

                        #[test]
                        fn [<test_ $t _checked_rem_at_min>](y in prop_oneof![Just(-1 as $t), any::<$t>()]) {
                            prop_assert_eq!(
                                super::$t::checked_rem(<$t>::MIN.inject(), y.inject()),
                                <$t>::MIN.checked_rem(y).inject());
                        }

                        // `0` is biased in: the zero arm is otherwise out of reach
                        // for the wider types.
                        #[test]
                        fn [<test_ $t _signum>](x in prop_oneof![Just(0 as $t), any::<$t>()]) {
                            prop_assert_eq!(super::$t::signum(x.inject()), x.signum());
                        }
                    }
                )*
            }
        }
    }

    // Tests for signed operations that take an unsigned argument.
    macro_rules! iint_mixed_test {
        ($(($signed:ty, $unsigned:ty))*) => {
            paste! {
                $(
                    proptest! {
                        #[test]
                        fn [<test_ $signed _checked_add_unsigned>](x in any::<$signed>(), y in any::<$unsigned>()) {
                            prop_assert_eq!(
                                super::$signed::checked_add_unsigned(x.inject(), y.inject()),
                                x.checked_add_unsigned(y).inject(),
                            );
                        }

                        #[test]
                        fn [<test_ $signed _checked_sub_unsigned>](x in any::<$signed>(), y in any::<$unsigned>()) {
                            prop_assert_eq!(
                                super::$signed::checked_sub_unsigned(x.inject(), y.inject()),
                                x.checked_sub_unsigned(y).inject(),
                            );
                        }
                    }
                )*
            }
        }
    }

    // Their `requires` rules out overflow, so the domain is exactly where
    // `checked_*` answers `Some` — including `i*::MIN / -1`.
    macro_rules! unchecked_test {
        ($($t:ty)*) => {
            paste! {
                $(
                    proptest! {
                        #[test]
                        fn [<test_ $t _unchecked_add>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assume!(x.checked_add(y).is_some());
                            prop_assert_eq!(
                                unsafe { super::$t::unchecked_add(x.inject(), y.inject()) },
                                unsafe { x.unchecked_add(y) });
                        }

                        #[test]
                        fn [<test_ $t _unchecked_sub>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assume!(x.checked_sub(y).is_some());
                            prop_assert_eq!(
                                unsafe { super::$t::unchecked_sub(x.inject(), y.inject()) },
                                unsafe { x.unchecked_sub(y) });
                        }

                        // Full-range pairs almost always overflow: halve `y` until it fits.
                        #[test]
                        fn [<test_ $t _unchecked_mul>](x in any::<$t>(), y in any::<$t>()) {
                            let mut y = y;
                            while x.checked_mul(y).is_none() {
                                y /= 2;
                            }
                            prop_assert_eq!(
                                unsafe { super::$t::unchecked_mul(x.inject(), y.inject()) },
                                unsafe { x.unchecked_mul(y) });
                        }

                        // std has no `unchecked_div`/`unchecked_rem`; `/` and `%` stand in.
                        #[test]
                        fn [<test_ $t _unchecked_div>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assume!(x.checked_div(y).is_some());
                            prop_assert_eq!(
                                unsafe { super::$t::unchecked_div(x.inject(), y.inject()) },
                                x / y);
                        }

                        #[test]
                        fn [<test_ $t _unchecked_rem>](x in any::<$t>(), y in any::<$t>()) {
                            prop_assume!(x.checked_rem(y).is_some());
                            prop_assert_eq!(
                                unsafe { super::$t::unchecked_rem(x.inject(), y.inject()) },
                                x % y);
                        }
                    }
                )*
            }
        }
    }

    macro_rules! rem_euclid_panic_test {
        ($($t:ty)*) => {
            paste! {
                $(
                    #[test]
                    fn [<test_ $t _rem_euclid_by_zero_panics>]() {
                        let (x, y) = (std::hint::black_box(7 as $t), std::hint::black_box(0 as $t));
                        crate::testing::panics_like_core(
                            || super::$t::rem_euclid(x.inject(), y.inject()),
                            || x.rem_euclid(y),
                        );
                    }
                )*
            }
        }
    }
    rem_euclid_panic_test! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

    macro_rules! rem_euclid_overflow_test {
        ($($t:ty)*) => {
            paste! {
                $(
                    #[test]
                    fn [<test_ $t _rem_euclid_min_by_neg_one_panics>]() {
                        let (x, y) = (std::hint::black_box(<$t>::MIN), std::hint::black_box(-1 as $t));
                        crate::testing::panics_like_core(
                            || super::$t::rem_euclid(x.inject(), y.inject()),
                            || x.rem_euclid(y),
                        );
                    }
                )*
            }
        }
    }
    rem_euclid_overflow_test! { i8 i16 i32 i64 i128 isize }

    // The proptests above stay inside the non-overflowing domain. `MAX.pow(2)`
    // is out of range for every integer type.
    macro_rules! pow_overflow_test {
        ($($t:ty)*) => {
            paste! {
                $(
                    #[test]
                    fn [<test_ $t _pow_overflow_panics>]() {
                        let (x, exp) =
                            (std::hint::black_box(<$t>::MAX), std::hint::black_box(2u32));
                        crate::testing::panics_like_core(
                            || super::$t::pow(x.inject(), exp),
                            || x.pow(exp),
                        );
                    }
                )*
            }
        }
    }
    pow_overflow_test! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

    int_test! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    unchecked_test! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    uint_test! { u8 u16 u32 u64 u128 usize }
    iint_test! { i8 i16 i32 i64 i128 isize }
    iint_mixed_test! { (i8, u8) (i16, u16) (i32, u32) (i64, u64) (i128, u128) (isize, usize) }

    macro_rules! default_test {
        ($($t:ty)*) => {
            paste! {
                $(
                    #[test]
                    fn [<test_ $t _default>]() {
                        assert_eq!(<$t as crate::default::Default>::default(), <$t as std::default::Default>::default());
                    }
                )*
            }
        }
    }

    default_test! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize bool }
}
