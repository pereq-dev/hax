//! `core-models`: A Rust Model for the `core` Library
//!
//! `core-models` is a simplified, self-contained model of Rust’s `core` library. It aims to provide
//! a purely Rust-based specification of `core`'s fundamental operations, making them easier to
//! understand, analyze, and formally verify. Unlike `core`, which may rely on platform-specific
//! intrinsics and compiler magic, `core-models` expresses everything in plain Rust, prioritizing
//! clarity and explicitness over efficiency.
//!
//! ## Key Features
//!
//! - **Partial Modeling**: `core-models` includes only a subset of `core`, focusing on modeling
//!   fundamental operations rather than providing a complete replacement.
//! - **Exact Signatures**: Any item that exists in both `core-models` and `core` has the same type signature,
//!   ensuring compatibility with formal verification efforts.
//! - **Purely Functional Approach**: Where possible, `core-models` favors functional programming principles,
//!   avoiding unnecessary mutation and side effects to facilitate formal reasoning.
//! - **Explicit Implementations**: Even low-level operations, such as SIMD, are modeled explicitly using
//!   Rust constructs like bit arrays and partial maps.
//! - **Extra Abstractions**: `core-models` includes additional helper types and functions to support
//!   modeling. These extra items are marked appropriately to distinguish them from `core` definitions.
//!
//! ## Intended Use
//!
//! `core-models` is designed as a reference model for formal verification and reasoning about Rust programs.
//! By providing a readable, well-specified version of `core`'s behavior, it serves as a foundation for
//! proof assistants and other verification tools.

#![allow(dead_code, unused)]
// `coverage(off)` is unstable; `cfg(coverage_nightly)` is set only by
// `cargo llvm-cov`, so normal builds and extraction never see this.
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
// int_roundings: lets the proptests call std's still-unstable signed `div_ceil`.
// `fmt_helpers_for_derive`: the proptests compare against std's
// `Formatter::debug_struct_fields_finish`, which is unstable.
// `slice_range`: the proptests compare against std's `slice::try_range`.
#![cfg_attr(
    test,
    feature(step_trait, int_roundings, fmt_helpers_for_derive, slice_range)
)]

// Lean-only modules. They carry no operations -- the types exist so that a
// client mentioning them resolves, which is all Aeneas's Lean library gives
// them too. The F* extraction drops them through the `-i` flags on the
// `fstar-core-models` Makefile target.
#[path = "core/alloc.rs"]
pub mod alloc;
#[path = "core/panic.rs"]
pub mod panic;
#[path = "core/pin.rs"]
pub mod pin;
#[path = "core/ptr.rs"]
pub mod ptr;
#[path = "core/sync.rs"]
pub mod sync;

#[path = "core/array.rs"]
pub mod array;
#[path = "core/bool.rs"]
pub mod bool;
#[path = "core/borrow.rs"]
pub mod borrow;
#[path = "core/clone.rs"]
pub mod clone;
#[path = "core/cmp.rs"]
pub mod cmp;
#[path = "core/convert.rs"]
pub mod convert;
#[path = "core/default.rs"]
pub mod default;
#[path = "core/error.rs"]
pub mod error;
#[path = "core/f32.rs"]
pub mod f32;
#[path = "core/fmt.rs"]
pub mod fmt;
#[path = "core/hash.rs"]
pub mod hash;
#[path = "core/hint.rs"]
pub mod hint;
#[path = "core/intrinsics.rs"]
pub mod intrinsics;
#[path = "core/iter.rs"]
pub mod iter;
#[path = "core/marker.rs"]
pub mod marker;
#[path = "core/mem.rs"]
pub mod mem;
#[path = "core/num/mod.rs"]
pub mod num;
#[path = "core/ops.rs"]
pub mod ops;
#[path = "core/option.rs"]
pub mod option;
#[path = "core/panicking.rs"]
pub mod panicking;
#[path = "core/result.rs"]
pub mod result;
#[path = "core/slice.rs"]
pub mod slice;
#[path = "core/str.rs"]
pub mod str;

#[cfg(test)]
pub mod testing;
