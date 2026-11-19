//! This module provides a list of handy `DefId` for the engine.
//! The list of `DefId`s comes from the crate `/engine/names`: any name mentioned
//! in that crate will be provided here automatically.
//!
//! For example, to be able to resugar `std::ops::Add::add(x, y)` into `x + y`,
//! we need to:
//!  1. match on the expression `std::ops::Add::add(x, y)`, figure out it is the
//!     application of the function denoted by the global identifier
//!     `std::ops::Add::add` with arguments `x` and `y`.
//!  2. check that global identifier `id: GlobalId` `std::ops::Add::add` is
//!     indeed `std::ops::Add::add`.
//!
//! Point (2.) seems a bit tautological, but we need to write a comparison like
//! `some_id == the_function_add`. This module basically provides such
//! `the_function_add` symbols.
//!
//! As an example, the names `std::option::Option::Some` and `None` will be provided by this module as:
//! ```rust,ignore
//! mod std {
//!     mod option {
//!         mod Option {
//!             fn Some() -> DefId { ... }
//!             fn None() -> DefId { ... }
//!         }
//!     }
//! }
//! ```

pub use crate::ast::identifiers::global_id::generated_names::{codegen, root::*};
