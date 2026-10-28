#![cfg_attr(feature = "rustc", feature(rustc_private))]
#![doc = include_str!("../README.md")]

pub(crate) mod prelude;

/// The CLI options for `cargo-hax`. The types defines in this module
/// are also used by the driver and the engine.
pub mod cli_options;

/// Type to represent errors, mainly in `hax-engine`. The engine
/// doesn't do any reporting itself: it only sends JSON to its stdout,
/// and `cargo-hax` takes care of reporting everything in a rustc
/// style.
pub mod diagnostics;

/// The types used to communicate between `cargo-hax` and the custom
/// driver.
pub mod driver_api;

/// The types used to communicate between `cargo-hax` and
/// `hax-engine`.
pub mod engine_api;

/// Lean naming conventions: the library name of a crate and the aeneas
/// `[[require]]` block of a lakefile.
pub mod lean;

/// Compile-time version of hax
pub const HAX_VERSION: &str = env!("HAX_VERSION");
