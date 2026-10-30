//! Rust↔Lean equivalence tests.
//!
//! Each `#[hax_test::equiv] pub fn ... -> bool { ... }` in the submodules
//! below pins down a single observation about how a `core::*` / `std::*`
//! item behaves on a concrete input. The framework checks the same
//! observation twice:
//!
//! - **Rust side**: `cargo test` runs the function and asserts it returns
//!   `true`. (The `#[hax_test::equiv]` attribute, from the `hax-test-macros`
//!   crate, generates the `#[test]` wrapper.)
//! - **Lean side**: `gen_lean_tests.py` scans every file under `src/`
//!   for these attributes and emits a `#guard <qualified-name> == .ok true`
//!   per test into `lean/RustLeanTests/LeanTests.lean`, so the Lean build
//!   fails if Aeneas's translation of the function does not also evaluate
//!   to `RustM.ok true`.
//!
//! Agreement between the two sides is the actual property under test:
//! the Lean translation of our `core_models` library must match Rust
//! std's behaviour on every input we exercise here.
//!
//! ## Layout
//!
//! Mirrors the structure of the `core-models` crate so a contributor
//! adding a new item to `core-models/src/core/foo.rs` knows exactly
//! where to add the matching equivalence tests
//! (`source/src/core/foo.rs`).

// Some tests deliberately exercise edge comparisons like `u8::MAX < 0u8`
// to pin the trait-dispatch behaviour at the extremes; rustc warns
// those are tautologically false, but that *is* the observation we want
// to verify.
#![allow(unused_comparisons)]

pub mod helpers;

pub mod alloc;
pub mod core;
