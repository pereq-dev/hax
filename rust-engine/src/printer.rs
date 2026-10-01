//! Printer infrastructure: allocators, traits, and the printing pipeline.
//!
//! This module contains the common plumbing that backends and printers rely on
//! to turn AST values into formatted text:
//! - [`Allocator`]: a thin wrapper around the `pretty` crate's allocator,
//!   parameterized by the backend, used to produce [`pretty::Doc`] nodes.
//! - [`PrettyAst`]: the trait that printers implement to provide per-type
//!   formatting of Hax AST nodes (re-exported from [`pretty_ast`]).
//! - The resugaring pipeline: a sequence of local AST rewrites that make
//!   emitted code idiomatic for the target language before pretty-printing.

use std::ops::Deref;

use crate::{
    ast::{self, span::Span},
    attributes::LinkedItemGraph,
    printer::pretty_ast::ToDocument,
};
use ast::visitors::dyn_compatible;

pub mod pretty_ast;
pub use pretty_ast::PrettyAst;

pub mod render_view;

pub mod source_map;
pub use source_map::SourceMap;

/// A resugaring is an erased mapper visitor with a name.
/// A resugaring is a *local* transformation on the AST that produces exclusively `ast::resugared` nodes.
/// Any involved or non-local transformation should be a phase, not a resugaring.
///
/// Backends may provide **multiple resugaring phases** to incrementally refine
/// the tree into something idiomatic for the target language (e.g., desugaring
/// pattern sugar into a more uniform core, then resugaring back into target
/// idioms). Each phase mutates the AST in place and should be small, focused,
/// and easy to test.
///
/// If you add a new phase, make sure it appears in the backend’s
/// `resugaring_phases()` list in the correct order.
pub trait Resugaring: for<'a> dyn_compatible::AstVisitorMut<'a> {
    /// Get the name of the resugar.
    fn name(&self) -> String;
}

/// A printer defines a list of resugaring phases.
pub trait Printer: Sized + PrettyAst<Span> + Default + HasLinkedItemGraph {
    /// The name of the printer
    const NAME: &'static str = <Self as PrettyAst<Span>>::NAME;

    /// Target page width handed to the `pretty` layout algorithm.
    const RENDER_WIDTH: usize = 80;
}

/// Getter and setter for `LinkedItemGraph`, useful for printers.
pub trait HasLinkedItemGraph {
    /// Get a reference of the `LinkedItemGraph`.
    fn linked_item_graph(&self) -> &LinkedItemGraph;
    /// Set a `LinkedItemGraph`.
    fn with_linked_item_graph(self, graph: std::rc::Rc<LinkedItemGraph>) -> Self;
}

/// Helper trait to print AST fragments.
pub trait Print<T>
where
    for<'a> dyn Resugaring: dyn_compatible::AstVisitableMut<'a, T>,
{
    /// Print a single AST fragment using this backend.
    fn print_returning_fragment(&mut self, fragment: T) -> (String, SourceMap, T)
    where
        T: ToDocument<Self, Span>;

    /// Print a single AST fragment using this backend.
    fn print(&mut self, fragment: T) -> (String, SourceMap)
    where
        T: ToDocument<Self, Span>;
}

impl<P: Printer, T> Print<T> for P
where
    for<'a> dyn Resugaring: dyn_compatible::AstVisitableMut<'a, T>,
{
    fn print_returning_fragment(&mut self, fragment: T) -> (String, SourceMap, T)
    where
        T: ToDocument<Self, Span>,
    {
        let doc = fragment.to_document(self).into_doc();
        let mut renderer = source_map::SpanPositionRenderer::default();
        // Printers never emit `Doc::fail`, so rendering cannot fail.
        let _ = doc
            .deref()
            .render_raw(<Self as Printer>::RENDER_WIDTH, &mut renderer);
        (renderer.out, renderer.positions, fragment)
    }

    fn print(&mut self, fragment: T) -> (String, SourceMap)
    where
        T: ToDocument<Self, Span>,
    {
        let (rendered, sourcemap, _) = <Self as Print<_>>::print_returning_fragment(self, fragment);
        (rendered, sourcemap)
    }
}
