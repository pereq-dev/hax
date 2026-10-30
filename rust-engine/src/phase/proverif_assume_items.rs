//! ProVerif-backend phase: apply `--assume-items`.
//!
//! The functions and methods the clauses select are modelled like
//! `#[hax_lib::pv_handwritten]` ones, with their body dropped.

use hax_lib_macros_types::AttrPayload;
use hax_types::cli_options::{InclusionClause, InclusionKind};

use crate::ast::identifiers::GlobalId;
use crate::ast::*;
use crate::phase::Phase;

/// Phase marking the items `--assume-items` selects as hand-written.
pub struct ProverifAssumeItems<'a> {
    /// The `--assume-items` clauses.
    pub clauses: &'a [InclusionClause],
}

impl ProverifAssumeItems<'_> {
    /// Whether the clauses select `id`: the last clause matching it decides.
    fn is_assumed(&self, id: GlobalId) -> bool {
        self.clauses
            .iter()
            .rev()
            .find(|clause| id.matches_namespace(&clause.namespace))
            .is_some_and(|clause| !matches!(clause.kind, InclusionKind::Excluded))
    }

    fn mark(&self, id: GlobalId, meta: &mut Metadata) {
        if self.is_assumed(id) {
            meta.attributes.push(Attribute {
                kind: AttributeKind::Hax(AttrPayload::PVHandwritten),
                span: meta.span,
            })
        }
    }
}

impl Phase for ProverifAssumeItems<'_> {
    fn apply(&self, items: &mut Vec<Item>) {
        for item in items.iter_mut() {
            match &mut item.kind {
                ItemKind::Fn { name, .. } => self.mark(*name, &mut item.meta),
                ItemKind::Impl { items, .. } => {
                    for impl_item in items {
                        if matches!(impl_item.kind, ImplItemKind::Fn { .. }) {
                            self.mark(impl_item.ident, &mut impl_item.meta);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
