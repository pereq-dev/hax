//! Regression test for https://github.com/pereq-dev/hax/issues/2026.
//!
//! The item quote macros `hax_lib::<backend>::{before,after,replace}` expand to
//! two things: an `AssociatedItem` marker attribute on the decorated item, and a
//! companion item holding the verbatim payload. The companion item is
//! `cfg`-gated on `hax_backend_<backend>`, while the marker attribute is not:
//! when extracting to some *other* backend, the marker attribute points to an
//! item that does not exist.
//!
//! The engine used to resolve every such marker eagerly and abort with
//! `Could not find item with UID ...`. Each function below must therefore be
//! extracted fine by every backend, with the verbatim payload showing up in the
//! corresponding backend only.
//!
//! The quotes are written under `cfg_attr(hax, ...)`, the form users are
//! expected to use so that `hax-lib` is not needed when compiling normally.

#[cfg_attr(hax, hax_lib::fstar::before("let a_verbatim_fstar_definition = 42"))]
fn decorated_with_an_fstar_quote() {}

#[cfg_attr(hax, hax_lib::coq::before("(* a verbatim Coq comment *)"))]
fn decorated_with_a_coq_quote() {}

#[cfg_attr(hax, hax_lib::legacy_lean::before("-- a verbatim Lean comment"))]
fn decorated_with_a_lean_quote() {}

/// Same, but with a `requires` clause: the F* backend looks up the `Requires`
/// role on this item, and used to choke on the unrelated dangling `ItemQuote`
/// marker while doing so.
#[cfg_attr(hax, hax_lib::legacy_lean::before("-- another verbatim Lean comment"))]
#[hax_lib::requires(x < 100)]
fn quoted_and_decorated(x: u8) -> u8 {
    x + 1
}
