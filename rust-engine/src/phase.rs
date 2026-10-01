//! A phase rewrites the AST.

use crate::ast::Item;

// Special kind of unreachability that should be prevented by a phase
macro_rules! unreachable_by_invariant {
    ($phase:ident) => {
        unreachable!(
            "The phase {} should make this unreachable",
            stringify!($phase)
        )
    };
}
pub(crate) use unreachable_by_invariant;

/// A Rust phase that operates on the AST.
pub trait Phase {
    /// Apply the phase on items.
    /// A phase may transform an item into zero, one or more items.
    fn apply(&self, items: &mut Vec<Item>);
}

pub mod legacy;

mod explicit_monadic;
mod filter_unprintable_items;
mod hoist_associated_fns;
mod proverif_assume_items;
mod proverif_bind_failable;
mod proverif_combinators_to_loops;
mod proverif_expand_struct_update;
mod proverif_flatten_binding_subpatterns;
mod proverif_inline;
mod proverif_name_wildcards;
mod proverif_ok_only_results;
mod proverif_reject_higher_order;
mod proverif_reject_unsupported;
mod proverif_resolve_trait_calls;
mod proverif_simplify_bindings;
mod reject_not_do_lean_dsl;

macro_rules! declare_phase_kind {
    {$($name:ident = $phase:expr),*$(,)?} => {
        /// Enumeration of the available phases.
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        pub enum PhaseKind {
            $(
                #[doc = concat!("The phase [`", stringify!($phase), "].")]
                $name,
            )*
            /// The phase [`proverif_assume_items::ProverifAssumeItems`], with
            /// the `--assume-items` clauses.
            ProverifAssumeItems(Vec<hax_types::cli_options::InclusionClause>),
            /// The phase
            /// [`proverif_resolve_trait_calls::ProverifResolveTraitCalls`], with
            /// the `--keep-trait-calls` paths.
            ProverifResolveTraitCalls(Vec<String>),
            /// A legacy (OCaml) phase.
            Legacy(crate::phase::legacy::LegacyOCamlPhase),
        }

        impl crate::phase::Phase for PhaseKind {
            fn apply(&self, items: &mut Vec<Item>) {
                match self {
                    $(Self::$name => $phase.apply(items),)*
                    Self::ProverifAssumeItems(clauses) => {
                        proverif_assume_items::ProverifAssumeItems { clauses }.apply(items)
                    }
                    Self::ProverifResolveTraitCalls(kept) => {
                        proverif_resolve_trait_calls::ProverifResolveTraitCalls { kept }.apply(items)
                    }
                    Self::Legacy(phase) => phase.apply(items),
                }
            }
        }
    };
}

declare_phase_kind! {
    ExplicitMonadic = explicit_monadic::ExplicitMonadic,
    RejectNotDoLeanDSL = reject_not_do_lean_dsl::RejectNotDoLeanDSL,
    HoistAssociatedFns = hoist_associated_fns::HoistAssociatedFns,
    ProverifInline = proverif_inline::ProverifInline,
    ProverifCombinatorsToLoops = proverif_combinators_to_loops::ProverifCombinatorsToLoops,
    ProverifExpandStructUpdate = proverif_expand_struct_update::ProverifExpandStructUpdate,
    ProverifFlattenBindingSubpatterns =
        proverif_flatten_binding_subpatterns::ProverifFlattenBindingSubpatterns,
    ProverifOkOnlyResults = proverif_ok_only_results::ProverifOkOnlyResults,
    ProverifSimplifyBindings = proverif_simplify_bindings::ProverifSimplifyBindings,
    ProverifRejectHigherOrder = proverif_reject_higher_order::ProverifRejectHigherOrder,
    ProverifRejectUnsupported = proverif_reject_unsupported::ProverifRejectUnsupported,
    ProverifRejectTryConversions = proverif_reject_unsupported::ProverifRejectTryConversions,
    ProverifMarkMutReplaceBody = proverif_reject_unsupported::ProverifMarkMutReplaceBody,
    ProverifBindFailable = proverif_bind_failable::ProverifBindFailable,
    ProverifNameWildcards = proverif_name_wildcards::ProverifNameWildcards,
    FilterUnprintableItems = filter_unprintable_items::FilterUnprintableItems,
}
