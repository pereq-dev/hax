//! Source positions.

use crate::interning::{Internable, Interned, InterningTable};
use hax_rust_engine_macros::*;
use std::sync::{LazyLock, Mutex};

/// Creates a fresh identifier for a span.
fn fresh_id() -> u32 {
    use std::sync::atomic::{AtomicU32 as AtomicInt, Ordering};
    static CURRENT_ID: AtomicInt = AtomicInt::new(0);
    CURRENT_ID.fetch_add(1, Ordering::Relaxed)
}

/// Position of a Rust source
#[derive_group_for_ast]
struct SpanData {
    /// A vector of spans as defined by the frontend.
    /// This is useful for supporting in a trivial way union of spans.
    data: Vec<hax_frontend_exporter::Span>,
    /// A reference to the item in which this span lives. This information is
    /// used to attribute diagnostics to items, and for debugging and profiling
    /// purposes, e.g. for `cargo hax into --stats backend`.
    owner_hint: Option<Interned<hax_frontend_exporter::DefId>>,
}

impl SpanData {
    /// Creates a dummy span.
    fn dummy() -> Self {
        let lo: hax_frontend_exporter::Loc = hax_frontend_exporter::Loc { line: 0, col: 0 };
        let hi = lo.clone();
        SpanData {
            data: vec![hax_frontend_exporter::Span {
                lo,
                hi,
                filename: hax_frontend_exporter::FileName::Custom("dumny".into()),
                rust_span_data: None,
            }],
            owner_hint: None,
        }
    }

    /// Creates a [`Span`] given information from the hax exporter.
    fn from_exporter(
        span: hax_frontend_exporter::Span,
        owner_hint: Option<&hax_frontend_exporter::DefId>,
    ) -> Self {
        Self {
            data: vec![span],
            owner_hint: owner_hint.map(Interned::intern),
        }
    }
}

/// Position of a Rust source
#[derive_group_for_ast]
#[derive(Copy)]
pub struct Span {
    #[serde(flatten)]
    data: Interned<SpanData>,
    /// A unique identifier. Since we store spans almost for every node of the
    /// AST, having a unique identifier for spans gives us a fine-grained way of
    /// referring to sub-nodes in debugging context. This id is indeed mostly
    /// used by the web debugger.
    id: u32,
}

impl Internable for SpanData {
    fn interning_table() -> &'static Mutex<InterningTable<Self>> {
        static TABLE: LazyLock<Mutex<InterningTable<SpanData>>> =
            LazyLock::new(|| Mutex::new(InterningTable::default()));
        &TABLE
    }
}

impl Span {
    /// Creates a dummy span.
    pub fn dummy() -> Self {
        static DUMMY_SPAN: LazyLock<Span> = LazyLock::new(|| {
            let data = Interned::intern(&SpanData::dummy());
            Span {
                data,
                id: fresh_id(),
            }
        });
        *DUMMY_SPAN
    }

    /// Creates a [`Span`] given information from the hax exporter.
    pub fn from_exporter(
        span: hax_frontend_exporter::Span,
        owner_hint: Option<&hax_frontend_exporter::DefId>,
    ) -> Self {
        let data = Interned::intern(&SpanData::from_exporter(span, owner_hint));
        Self {
            data,
            id: fresh_id(),
        }
    }

    /// Get a vector of frontend spans given a [`Span`].
    pub fn as_frontend_spans(self) -> &'static [hax_frontend_exporter::Span] {
        &self.data.get().data
    }

    /// The item in which this span lives, if known.
    pub fn owner_hint(self) -> Option<&'static hax_frontend_exporter::DefId> {
        self.data.get().owner_hint.map(|owner| owner.get())
    }
}

impl Internable for hax_frontend_exporter::DefId {
    fn interning_table() -> &'static Mutex<InterningTable<Self>> {
        static TABLE: LazyLock<Mutex<InterningTable<hax_frontend_exporter::DefId>>> =
            LazyLock::new(|| Mutex::new(InterningTable::default()));
        &TABLE
    }
}
