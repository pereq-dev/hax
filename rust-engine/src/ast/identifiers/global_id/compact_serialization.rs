//! Helper module that provides serialization and deserialization of DefId to
//! compact representations. This is solely for conciseness purposes of the
//! generated code.
//!
//! Concretely, this module defines `Repr` a (JSON-compact) representation of `DefId`s without parents.
//! It provides a bijection from the fields `krate`, `path`, and `kind` of `DefId` and `Repr`.
//! The choice of `Repr` itself is irrelevant. Anything that produces compact JSON is good.

use crate::interning::Internable;
use hax_frontend_exporter::{DefKind, DefPathItem, DisambiguatedDefPathItem};

use super::{DefIdInner, ExplicitDefId};
/// The compact representation: a tuple (krate name, path, defkind, is_constructor)
/// The path is a vector of tuples (DefPathItem, disambiguator).
type Repr = (String, Vec<(DefPathItem, u32)>, DefKind, bool);
/// `BorrowedRepr` is the borrowed variant of `Repr`. Useful for serialization.
type BorrowedRepr<'a> = (
    &'a String,
    Vec<(&'a DefPathItem, &'a u32)>,
    &'a DefKind,
    bool,
);

/// Serialize an explicit def id into a compact represented string
pub fn serialize(edid: &ExplicitDefId) -> String {
    let did = &edid.def_id;
    let path = did
        .path
        .iter()
        .map(
            |DisambiguatedDefPathItem {
                 data,
                 disambiguator,
             }| (data, disambiguator),
        )
        .collect::<Vec<_>>();
    let data: BorrowedRepr<'_> = (&did.krate, path, &did.kind, edid.is_constructor);
    serde_json::to_string(&data).unwrap()
}

/// Deserialize from a (string) compact representation and a parent
pub fn deserialize(s: &str, parent: Option<ExplicitDefId>) -> ExplicitDefId {
    let (krate, path, kind, is_constructor): Repr = serde_json::from_str(s).unwrap();
    ExplicitDefId {
        def_id: DefIdInner {
            parent: parent.map(|parent| parent.def_id),
            krate,
            path: path
                .into_iter()
                .map(|(data, disambiguator)| DisambiguatedDefPathItem {
                    data,
                    disambiguator,
                })
                .collect(),
            kind,
        }
        .intern(),
        is_constructor,
    }
}
