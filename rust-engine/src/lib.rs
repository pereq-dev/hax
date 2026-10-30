//! The Rust engine of hax.

#![feature(fn_traits, unboxed_closures)]
#![warn(
    rustdoc::broken_intra_doc_links,
    missing_docs,
    unused_qualifications,
    unused_crate_dependencies
)]

// `hax-lib` is a dev-dependency only so that docs can link to it.
#[cfg(test)]
use hax_lib as _;

pub mod ast;
pub mod attributes;
pub mod backends;
pub mod debugger;
pub mod hax_io;
pub mod import_thir;
pub mod interning;
pub mod names;
pub mod ocaml_engine;
pub mod phase;
pub mod printer;
pub mod resugarings;
pub mod symbol;
