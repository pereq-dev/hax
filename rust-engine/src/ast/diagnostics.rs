//! Diagnostic types used to represent and propagate errors (or warnings, notes,
//! etc.) within the AST.
//!
//! This module is used to attach semantic or translation errors to AST nodes.

use crate::ast::*;
use hax_rust_engine_macros::*;

pub use hax_types::diagnostics::Kind as DiagnosticInfoKind;

/// Error diagnostic
#[derive_group_for_ast]
pub struct Diagnostic {
    node: Box<Fragment>,
    info: DiagnosticInfo,
}

/// Error description and location
#[derive_group_for_ast]
#[must_use]
pub struct DiagnosticInfo {
    /// Diagnostic context
    pub context: Context,
    /// Location in the source code
    pub span: Span,
    /// Error type
    pub kind: DiagnosticInfoKind,
}

impl DiagnosticInfo {
    /// Emits the diagnostic information.
    pub fn emit(&self) {
        crate::hax_io::report_diagnostic(hax_types::diagnostics::Diagnostics {
            kind: self.kind.clone(),
            span: self.span.as_frontend_spans().to_vec(),
            context: format!("{}", self.context),
            owner_id: self.span.owner_hint().cloned(),
        })
    }
}

impl Diagnostic {
    /// Get diagnostic information
    pub fn info(&self) -> &DiagnosticInfo {
        &self.info
    }
    /// Get diagnostic node of origin
    pub fn node(&self) -> &Fragment {
        &self.node
    }
    /// Report an error
    pub fn new(node: impl Into<Fragment>, info: DiagnosticInfo) -> Self {
        let node = node.into();
        info.emit();
        Self {
            node: Box::new(node),
            info,
        }
    }
}

/// Context of an error
#[derive_group_for_ast]
pub enum Context {
    /// Error during import from THIR
    Import,
    /// Error during the projection from identifiers to views
    NameView,
    /// Error in a printer
    Printer(String),
    /// Error in an engine phase
    Phase(String),
    /// Debugger
    Debugger,
    /// Unknown
    Unknown,
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Context::Import => write!(f, "Importer"),
            Context::NameView => write!(f, "Name rendering"),
            Context::Printer(p) => write!(f, "{p} Printer"),
            Context::Phase(p) => write!(f, "Engine phase ({p})"),
            Context::Debugger => write!(f, "Debugger"),
            Context::Unknown => write!(f, "Unknown"),
        }
    }
}
