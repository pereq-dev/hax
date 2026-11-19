//! Pretty-printing support for the hax AST.
//!
//! This module defines the trait [`PrettyAst`], which is the **primary trait a printer should
//! implement**.
//!
//! # Quickstart
//! In most printers you:
//! 1. Implement [`Printer`] for your printer type,
//! 2. Implement [`PrettyAst`] for that printer type,
//! 3. Call `ast_value.to_document(&print)` on AST values.
//!
//! See [`crate::backends`] for backend and printer examples.

use std::{borrow::Cow, fmt::Display};

use super::*;
use crate::ast::*;
use pretty::BoxAllocator;

use crate::symbol::Symbol;
use literals::*;
use resugared::*;

mod debug_json;
mod to_document;
pub use debug_json::*;
pub use to_document::*;

#[macro_export]
/// Similar to [`std::todo`], but returns a document instead of panicking with a message.
/// In addition, `todo_document!` accepts a prefix to point to a specific issue number.
///
/// ## Examples:
/// - `todo_document!(allocator)`
/// - `todo_document!(allocator, "This is a todo")`
/// - `todo_document!(allocator, issue 42)`
/// - `todo_document!(allocator, issue 42, "This is a todo")`
macro_rules! todo_document {
    ($allocator:ident, issue $issue:literal) => {
        {return $allocator.todo_document(&format!("TODO_LINE_{}", std::line!()), Some($issue));}
    };
    ($allocator:ident, issue $issue:literal, $($tt:tt)*) => {
        {
            let message = format!($($tt)*);
            return $allocator.todo_document(&message, Some($issue));
        }
    };
    ($allocator:ident,) => {
        {return $allocator.todo_document(&format!("TODO_LINE_{}", std::line!()), None);}
    };
    ($allocator:ident, $($tt:tt)*) => {
        {
            let message = format!($($tt)*);
            return $allocator.todo_document(&message, None);
        }
    };
}
pub use todo_document;

/// Expand a list of values into documents and concatenate them in order.
///
/// This helper mirrors [`pretty::docs!`] but automatically calls
/// [`ToDocumentOwned::to_document_owned`] on each argument before appending it
/// to the accumulator that starts as [`PrettyAstExt::nil`].
#[macro_export]
macro_rules! pretty_ast_docs {
    ($printer: expr, $docs:expr) => {{
        use $crate::printer::pretty_ast::{ToDocumentOwned};
        $docs.to_document_owned($printer)
    }};
    ($printer: expr, $($docs:expr),*$(,)?) => {{
        use $crate::printer::pretty_ast::{ToDocumentOwned};
        nil!()
        $(.append($docs.to_document_owned($printer)))*
    }};
}
pub use pretty_ast_docs;

/// Convert a collection of values into documents separated by another
/// document.
///
/// It forwards to [`PrettyAstExt::intersperse`] after materialising the
/// separator. The macro exists so call sites can stay concise while still
/// benefiting from the allocator captured by [`install_pretty_helpers!`].
#[macro_export]
macro_rules! pretty_ast_intersperse {
    ($printer: expr, $docs:expr, $sep: expr$(,)?) => {{
        let docs = $docs;
        let sep = $sep;
        $crate::printer::pretty_ast::PrettyAstExt::intersperse($printer, docs, sep)
    }};
}
pub use pretty_ast_intersperse;

#[macro_export]
/// Install pretty-printing helpers partially applied with a given local
/// allocator.
///
/// This macro declares a set of small, local macros that proxy to the
/// underlying [`pretty::DocAllocator`] methods and macro while capturing your
/// allocator value. It keeps printing code concise and avoids passing the
/// allocator around explicitly.
///
/// # Syntax
/// ```rust,ignore
/// install_pretty_helpers!(alloc_ident: AllocatorType)
/// ```
///
/// - `alloc_ident`: the in-scope variable that implements both
///   [`pretty::DocAllocator`] and [`Printer`].
/// - `AllocatorType`: the concrete type of that variable.
///
/// # What gets installed
/// - macro shorthands for common allocator methods:
///   [`PrettyAstExt::nil`], [`PrettyAstExt::fail`],
///   [`PrettyAstExt::hardline`], [`PrettyAstExt::space`],
///   [`PrettyAstExt::line`], [`PrettyAstExt::line_`],
///   [`PrettyAstExt::softline`], [`PrettyAstExt::softline_`],
///   [`PrettyAstExt::as_string`], [`PrettyAstExt::text`],
///   [`PrettyAstExt::concat`], [`PrettyAstExt::intersperse`],
///   [`PrettyAstExt::column`], [`PrettyAstExt::nesting`],
///   [`PrettyAstExt::reflow`].
/// - a partially applied version of [`pretty::docs!`].
/// - [`todo_document!`]: produce a placeholder document (that does not panic).
macro_rules! install_pretty_helpers {
    ($allocator:ident : $allocator_type:ty) => {
        $crate::printer::pretty_ast::install_pretty_helpers!(
            @$allocator,
            #[doc = ::std::concat!("Proxy macro for [`", stringify!($crate), "::printer::pretty_ast::todo_document`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            #[doc = ::std::concat!(r#"Example: `disambiguated_todo!("Error message")` or `disambiguated_todo!(issue #123, "Error message with issue attached")`."#)]
            disambiguated_todo{$crate::printer::pretty_ast::todo_document!},
            #[doc = ::std::concat!("Proxy macro for [`pretty::docs`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            docs{$crate::printer::pretty_ast::pretty_ast_docs!},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::nil`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            nil{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::nil},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::fail`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            fail{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::fail},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::hardline`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            hardline{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::hardline},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::space`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            space{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::space},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::line`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            disambiguated_line{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::line},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::line_`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            line_{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::line_},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::softline`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            softline{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::softline},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::softline_`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            softline_{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::softline_},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::as_string`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            as_string{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::as_string},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::text`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            text{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::text},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::concat`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            disambiguated_concat{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::concat},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::intersperse`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            intersperse{$crate::printer::pretty_ast::pretty_ast_intersperse!},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::column`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            column{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::column},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::nesting`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            nesting{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::nesting},
            #[doc = ::std::concat!("Proxy macro for [`PrettyAstExt::reflow`] that automatically uses `", stringify!($allocator),"` as allocator.")]
            reflow{<$allocator_type as $crate::printer::pretty_ast::PrettyAstExt<_>>::reflow}
        );
    };
    (@$allocator:ident, $($(#[$($attrs:tt)*])*$name:ident{$($callable:tt)*}),*) => {
        $(
            #[hax_rust_engine_macros::partial_apply($($callable)*, $allocator,)]
            #[allow(unused)]
            $(#[$($attrs)*])*
            macro_rules! $name {}
        )*
    };
}
pub use install_pretty_helpers;

/// `PrettyAstExt` exposes `DocAllocator`-style constructors for printers.
///
/// Every method simply forwards to the global [`pretty::BoxAllocator`] so that printers
/// implementing [`PrettyAst`] can build documents without juggling allocator plumbing.
pub trait PrettyAstExt<A: 'static>: Sized {
    /// Returns an empty document.
    /// Mirrors [`pretty::DocAllocator::nil`].
    fn nil(&self) -> DocBuilder<A> {
        pretty::DocAllocator::nil(&BoxAllocator)
    }

    /// Produces a document that fails rendering immediately.
    /// Mirrors [`pretty::DocAllocator::fail`].
    ///
    /// This is typically used to abort rendering inside the left side of a [`pretty::Doc::Union`].
    fn fail(&self) -> DocBuilder<A> {
        pretty::DocAllocator::fail(&BoxAllocator)
    }

    /// Inserts a mandatory line break.
    /// Mirrors [`pretty::DocAllocator::hardline`].
    fn hardline(&self) -> DocBuilder<A> {
        pretty::DocAllocator::hardline(&BoxAllocator)
    }

    /// Inserts a single space that disappears when groups flatten.
    /// Mirrors [`pretty::DocAllocator::space`].
    fn space(&self) -> DocBuilder<A> {
        pretty::DocAllocator::space(&BoxAllocator)
    }

    /// Acts like a `\n` but behaves like `space` once grouped onto a single line.
    /// Mirrors [`pretty::DocAllocator::line`].
    fn line(&self) -> DocBuilder<A> {
        pretty::DocAllocator::line(&BoxAllocator)
    }

    /// Acts like `line` but collapses to `nil` if grouped on a single line.
    /// Mirrors [`pretty::DocAllocator::line_`].
    fn line_(&self) -> DocBuilder<A> {
        pretty::DocAllocator::line_(&BoxAllocator)
    }

    /// Acts like `space` when the document fits the page, otherwise behaves like `line`.
    /// Mirrors [`pretty::DocAllocator::softline`].
    fn softline(&self) -> DocBuilder<A> {
        pretty::DocAllocator::softline(&BoxAllocator)
    }

    /// Acts like `nil` when the document fits the page, otherwise behaves like `line_`.
    /// Mirrors [`pretty::DocAllocator::softline_`].
    fn softline_(&self) -> DocBuilder<A> {
        pretty::DocAllocator::softline_(&BoxAllocator)
    }

    /// Renders `data` via its [`Display`] implementation.
    /// Mirrors [`pretty::DocAllocator::as_string`].
    ///
    /// The resulting document must not contain explicit line breaks.
    fn as_string<U: Display>(&self, data: U) -> DocBuilder<A> {
        pretty::DocAllocator::as_string(&BoxAllocator, data)
    }

    /// Renders the provided text verbatim.
    /// Mirrors [`pretty::DocAllocator::text`].
    ///
    /// The supplied string must not contain line breaks.
    fn text<'a>(&self, data: impl Into<Cow<'a, str>>) -> DocBuilder<A> {
        self.as_string(data.into())
    }

    /// Concatenates the given values after turning each into a document.
    /// Mirrors [`pretty::DocAllocator::concat`].
    fn concat<I>(&self, docs: I) -> DocBuilder<A>
    where
        I::Item: ToDocumentOwned<Self, A>,
        I: IntoIterator,
    {
        pretty::DocAllocator::concat(
            &BoxAllocator,
            docs.into_iter().map(|doc| doc.to_document_owned(self)),
        )
    }

    /// Concatenates documents while interspersing `separator` between every pair.
    /// Mirrors [`pretty::DocAllocator::intersperse`].
    ///
    /// `separator` may need to be cloned; consider cheap pointer documents like `RefDoc` or `RcDoc`.
    fn intersperse<I, S>(&self, docs: I, separator: S) -> DocBuilder<A>
    where
        I::Item: ToDocumentOwned<Self, A>,
        I: IntoIterator,
        S: ToDocumentOwned<Self, A> + Clone,
        A: Clone,
    {
        let separator = separator.to_document_owned(self);
        pretty::DocAllocator::intersperse(
            &BoxAllocator,
            docs.into_iter().map(|doc| doc.to_document_owned(self)),
            separator,
        )
    }

    /// Reflows `text`, inserting `softline` wherever whitespace appears.
    /// Mirrors [`pretty::DocAllocator::reflow`].
    fn reflow(&self, text: &'static str) -> DocBuilder<A>
    where
        A: Clone,
    {
        pretty::DocAllocator::reflow(&BoxAllocator, text)
    }
}

impl<A: 'static + Clone, P: PrettyAst<A>> PrettyAstExt<A> for P {}

/// Generate a dispatcher macro that forwards a token to specialised macros.
macro_rules! make_cases_macro {
    (
        $macro_name:ident,
        $(
            $($idents:ident)|* => $target:ident,
        )*
        _ => $fallback:ident $(,)?
    ) => {
        macro_rules! $macro_name {
            $(
                $(
                    ($idents $tt:tt) => { $target!($tt); };
                )*
            )*
            ($anything:ident $tt:tt) => { $fallback!($tt); };
        }
    };
}

/// Helper macro used to ignore a matched arm in `make_cases_macro!`.
macro_rules! skip {
    ($tt:tt) => {};
}
/// Helper macro used to keep the body for specific matches in
/// `make_cases_macro!`.
macro_rules! keep {
    ({$($tt:tt)*}) => { $($tt)* };
}

make_cases_macro!(method_deny_list,
    ExprKind | PatKind | TyKind | GuardKind | ImplExprKind | ImplItemKind | TraitItemKind | AttributeKind | DocCommentKind => skip,
    Signedness  | IntSize => skip,
    ItemQuoteOrigin | ItemQuoteOriginKind | ItemQuoteOriginPosition => skip,
    ControlFlowKind | LoopState | LoopKind => skip,
    _ => keep
);

make_cases_macro!(span_handling,
    Item | Expr | Pat | Guard | Arm | ImplItem | TraitItem | GenericParam | Attribute | Attribute => keep,
    _ => skip
);

/// A trait that provides an optional contextual span for printers: during a
/// pretty printing job, spans will be inserted so that errors are always tagged
/// with precise location information.
///
/// This should not be implemented by hand, instead, use
/// [`hax_rust_engine_macros::setup_printer_struct`].
pub trait HasContextualSpan: Clone {
    /// Clone the printer, adding a span hint. Useful for errors.
    fn with_span(&self, _span: Span) -> Self;

    /// Returns the span currently associated with the printer, if any.
    fn span(&self) -> Option<Span>;
}

/// Declare the `PrettyAst` trait and wiring for deriving `ToDocument` for AST
/// nodes.
macro_rules! mk {
    ($($ty:ident),*) => {
        pastey::paste! {
            /// A trait that defines a print method per type in the AST.
            ///
            /// This is the main trait a printer should implement.
            ///
            /// You then implement the actual formatting logic in the generated
            /// per-type methods. These methods are intentionally marked
            /// `#[deprecated]` to discourage calling them directly; instead,
            /// call `node.to_document(self)` from the [`ToDocument`] trait to
            /// ensure annotations and spans are applied correctly.
            ///
            /// Note that using `install_pretty_helpers!` will produce macro
            /// that implicitly use `self` as allocator. Take a look at a
            /// printer in the [`backends`] module for an example.
            pub trait PrettyAst<A: 'static + Clone>: Sized + HasContextualSpan {
                /// A name for this instance of `PrettyAst`.
                /// Useful for diagnostics and debugging.
                const NAME: &'static str;

                /// Emit a diagnostic with proper context and span.
                fn emit_diagnostic(&self, kind: hax_types::diagnostics::Kind) {
                    let span = self.span().unwrap_or_else(|| Span::dummy());
                    use crate::ast::diagnostics::{DiagnosticInfo, Context};
                    (DiagnosticInfo {
                        context: Context::Printer(Self::NAME.to_string()),
                        span,
                        kind
                    }).emit()
                }

                /// Produce a non-panicking placeholder document. In general, prefer the use of the helper macro [`todo_document!`].
                fn todo_document(&self, message: &str, issue_id: Option<u32>) -> DocBuilder<A> {
                    self.emit_diagnostic(hax_types::diagnostics::Kind::Unimplemented {
                        issue_id,
                        details: Some(message.into()),
                    });
                    self.as_string(message)
                }

                /// Produce a structured error document for an unimplemented
                /// method.
                ///
                /// Printers may override this for nicer diagnostics (e.g.,
                /// colored "unimplemented" banners or links back to source
                /// locations). The default produces a small, debuggable piece
                /// of text that includes the method name and a JSON handle for
                /// the AST fragment (via [`DebugJSON`]).
                fn unimplemented_method(&self, method: &str, ast: ast::fragment::FragmentRef<'_>) -> DocBuilder<A> {
                    let debug_json = DebugJSON(ast).to_string();
                    self.emit_diagnostic(hax_types::diagnostics::Kind::Unimplemented {
                        issue_id: None,
                        details: Some(format!("The method `{method}` is not implemented in the backend {}. To show the AST fragment that could not be printed, run {debug_json}.", Self::NAME)),
                    });
                    self.text(format!("`{method}` unimpl, {debug_json}", )).parens()
                }

                $(
                    method_deny_list!($ty{
                        #[doc = "Define how the printer formats a value of this AST type."]
                        #[doc = "Do not call this method directly. Use [`ToDocument::to_document`] instead, so annotations/spans are preserved correctly."]
                        #[deprecated = "Do not call this method directly. Use [`ToDocument::to_document`] instead, so annotations/spans are preserved correctly."]
                        fn [<$ty:snake>](&self, [<$ty:snake>]: &$ty) -> DocBuilder<A> {
                            mk!(@method_body $ty [<$ty:snake>] self [<$ty:snake>])
                        }
                    });
                )*
            }

            $(
                method_deny_list!($ty{
                    impl<A: 'static + Clone, P: PrettyAst<A>> ToDocument<P, A> for $ty {
                        fn to_document(&self, printer: &P) -> DocBuilder<A> {
                            span_handling!($ty{
                                let printer = &(printer.with_span(self.span()));
                            });
                            // Note about deprecation:
                            //   Here is the only place where calling the deprecated methods from the trait `PrettyAst` is fine.
                            //   Here is the place we (will) take care of spans, etc.
                            #[allow(deprecated)]
                            let print = <P as PrettyAst<A>>::[<$ty:snake>];
                            print(printer, self)
                        }
                    }
                });
            )*
        }
    };

    // Special default implementation for specific types
    (@method_body Symbol $meth:ident $self:ident $value:ident) => {
        $self.as_string($value.to_string())
    };
    (@method_body LocalId $meth:ident $self:ident $value:ident) => {
        $value.0.to_document($self)
    };
    (@method_body SpannedTy $meth:ident $self:ident $value:ident) => {
        $value.ty.to_document($self)
    };
    (@method_body $ty:ident $meth:ident $self:ident $value:ident) => {
        $self.unimplemented_method(stringify!($meth), ast::fragment::FragmentRef::from($meth))
    };
}

#[hax_rust_engine_macros::replace(AstNodes => include(VisitableAstNodes))]
mk!(GlobalId, AstNodes);
