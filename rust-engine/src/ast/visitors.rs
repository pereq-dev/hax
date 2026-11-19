//! Syntax tree traversals to walk a shared or mutable borrow of the syntax tree
//! of Hax. The visitors are generated using the [`derive_generic_visitor`]
//! library.
//!
//! This module provides visitors of different flavors of visitors, and visitor
//! wrappers that can enhance the default behavior of a visitor.
//!
//! We provide four main visitors.
//!  - [`AstVisitor`] and [`AstVisitorMut`]: visitor that never early exit.
//!  - [`AstEarlyExitVisitor`] and [`AstEarlyExitVisitorMut`]: visitor that can early exit.
//!
//! Each trait provides methods `visit_expr`, `visit_ty`, etc. enabling easy AST
//! traversal.
//!
//! Importantly, we also provide visitor wrappers that enhance visitors with
//! common useful behavior. See the module [`wrappers`] for more information.

use super::*;
use derive_generic_visitor::*;
use hax_lib_macros_types::AttrPayload;

pub mod wrappers {
    //! This module provides a visitor wrappers, or transformer of visitors.
    //! Such wrappers transform the behavior of a visitor.
    //!
    //! For example, [`SpanWrapper`] takes care of keeping track of [`Span`]s
    //! while traversing an AST.

    use std::ops::Deref;

    use super::{infallible::AstVisitable as AstVisitableInfallible, *};
    use diagnostics::*;

    /// A visitor wrapper that tracks span while visiting the AST. Whenever an
    /// AST node that carries a span is visited, using this wrapper, the ambient
    /// span is mutated and accessible via the `HasSpan` trait.
    pub struct SpanWrapper<'a, V>(pub &'a mut V);

    impl<'a, V: HasSpan> SpanWrapper<'a, V> {
        /// Performs a spanned action: calls the function `action` on
        /// `ast_fragment`, with the contextual span information in `self` being
        /// the span found in `ast_fragment`.
        fn spanned_action<T: Deref, U>(
            &mut self,
            ast_fragment: T,
            action: impl Fn(&mut Self, T) -> U,
        ) -> U
        where
            T::Target: HasSpan,
        {
            let span_before = self.0.span();
            *self.0.span_mut() = ast_fragment.span();
            // Perform the provided action on `ast_fragment` with `ast_fragment`'s span as contextual span.
            let result = action(self, ast_fragment);
            *self.0.span_mut() = span_before;
            result
        }
    }

    impl<'a, V: AstVisitorMut + HasSpan> AstVisitorMut for SpanWrapper<'a, V> {
        fn visit_inner<T>(&mut self, x: &mut T)
        where
            T: AstVisitableInfallible,
            T: for<'s> DriveMut<'s, AstVisitableInfallibleWrapper<Self>>,
        {
            x.drive_map(self.0)
        }
        fn visit_item(&mut self, x: &mut Item) {
            self.spanned_action(x, Self::visit_inner)
        }
        fn visit_expr(&mut self, x: &mut Expr) {
            self.spanned_action(x, Self::visit_inner)
        }
        fn visit_pat(&mut self, x: &mut Pat) {
            self.spanned_action(x, Self::visit_inner)
        }
        fn visit_guard(&mut self, x: &mut Guard) {
            self.spanned_action(x, Self::visit_inner)
        }
        fn visit_arm(&mut self, x: &mut Arm) {
            self.spanned_action(x, Self::visit_inner)
        }
        fn visit_impl_item(&mut self, x: &mut ImplItem) {
            self.spanned_action(x, Self::visit_inner)
        }
        fn visit_trait_item(&mut self, x: &mut TraitItem) {
            self.spanned_action(x, Self::visit_inner)
        }
        fn visit_generic_param(&mut self, x: &mut GenericParam) {
            self.spanned_action(x, Self::visit_inner)
        }
        fn visit_attribute(&mut self, x: &mut Attribute) {
            self.spanned_action(x, Self::visit_inner)
        }
        fn visit_spanned_ty(&mut self, x: &mut SpannedTy) {
            self.spanned_action(x, Self::visit_inner)
        }
    }

    /// A visitor wrapper that automatically collects errors in `ErrorNode`s.
    /// Coupled with the trait `VisitorWithErrors`, this provides an `error`
    /// method on a visitor that can be used to throw errors, which will be
    /// automatically inlined in the AST on the closest error-capable node.
    pub struct ErrorWrapper<'a, V>(pub &'a mut V);

    /// An opaque error vault. This is the state manipulated by the visitor wrapper [`ErrorWrapper`].
    /// It is purposefully not-inspectable.
    #[derive(Default)]
    pub struct ErrorVault(Vec<Diagnostic>);
    impl ErrorVault {
        fn add(&mut self, diagnostic: Diagnostic) {
            self.0.push(diagnostic);
        }
    }

    /// Helper struct that contains error-handling related state information.
    /// This is used internally by [`setup_error_handling_struct`].
    pub struct ErrorHandlingState(pub Span, pub ErrorVault);
    impl Default for ErrorHandlingState {
        fn default() -> Self {
            Self(Span::dummy(), Default::default())
        }
    }

    #[macro_export]
    /// Use this macro in an implementation of `AstVisitorMut` to get automatic spans and error handling.
    macro_rules! setup_error_handling_impl {
        () => {
            fn visit<T: $crate::ast::visitors::AstVisitableInfallible>(&mut self, x: &mut T) {
                $crate::ast::visitors::wrappers::SpanWrapper(
                    &mut $crate::ast::visitors::wrappers::ErrorWrapper(self),
                )
                .visit(x)
            }
        };
    }
    pub use setup_error_handling_impl;

    /// Mark a visitor with a specific diagnostic context.
    pub trait VisitorWithContext {
        /// Returns the diagnostic context for this visitor.
        fn context(&self) -> Context;
    }

    impl<T: HasSpan> HasSpan for ErrorWrapper<'_, T> {
        fn span(&self) -> Span {
            self.0.span()
        }

        fn span_mut(&mut self) -> &mut Span {
            self.0.span_mut()
        }
    }

    /// A visitor that can throw errors. It should be used in combination with
    /// `ErrorWrapper`, which will take care of bubbling error up to the nearest
    /// parent capable of representing errors. For instance, if you error out in
    /// a literal, the error will be represented in the parent expression or the
    /// parent type, as nodes [`ExprKind::Error`] or [`TyKind ::Error`].
    pub trait VisitorWithErrors: HasSpan + VisitorWithContext {
        /// Projects the error vault.
        fn error_vault(&mut self) -> &mut ErrorVault;
        /// Send an error.
        fn error(&mut self, node: impl Into<Fragment>, kind: DiagnosticInfoKind) {
            let context = self.context();
            let span = self.span();
            self.error_vault().add(Diagnostic::new(
                node,
                DiagnosticInfo {
                    context,
                    span,
                    kind,
                },
            ));
        }
    }

    impl<'a, V: VisitorWithErrors> ErrorWrapper<'a, V> {
        fn error_handled_action<
            T: FallibleAstNode + Clone + std::fmt::Debug + Into<Fragment>,
            U,
        >(
            &mut self,
            x: &mut T,
            action: impl Fn(&mut Self, &mut T) -> U,
        ) -> U {
            let diagnostics_snapshot = self.0.error_vault().0.clone();
            self.0.error_vault().0.clear();
            let result = action(self, x);
            let diagnostics: Vec<_> = self.0.error_vault().0.drain(..).collect();
            if !diagnostics.is_empty() {
                x.set_error(ErrorNode {
                    fragment: Box::new(x.clone().into()),
                    diagnostics,
                });
            }
            self.0.error_vault().0 = diagnostics_snapshot;
            result
        }
    }

    impl<'a, V: AstVisitorMut + VisitorWithErrors> AstVisitorMut for ErrorWrapper<'a, V> {
        fn visit_inner<T>(&mut self, x: &mut T)
        where
            T: AstVisitableInfallible,
            T: for<'s> DriveMut<'s, AstVisitableInfallibleWrapper<Self>>,
        {
            x.drive_map(self.0)
        }
        fn visit_item(&mut self, x: &mut Item) {
            self.error_handled_action(x, Self::visit_inner)
        }
        fn visit_pat(&mut self, x: &mut Pat) {
            self.error_handled_action(x, Self::visit_inner)
        }
        fn visit_expr(&mut self, x: &mut Expr) {
            self.error_handled_action(x, Self::visit_inner)
        }
        fn visit_ty(&mut self, x: &mut Ty) {
            self.error_handled_action(x, Self::visit_inner)
        }
    }
}

#[hax_rust_engine_macros::replace(AstNodes => include(VisitableAstNodes))]
mod replaced {
    use super::*;
    pub mod infallible {
        use super::*;

        #[visitable_group(
            visitor(drive_map(
                /// An mutable visitor that visits the AST for hax.
                ///
                /// ```rust,ignore
                /// use crate::ast::{diagnostics::*, visitors::*};
                /// #[setup_error_handling_struct]
                /// #[derive(Default)]
                /// struct MyVisitor;
                ///
                /// impl VisitorWithContext for MyVisitor {
                ///     fn context(&self) -> Context {
                ///         Context::Import
                ///     }
                /// }
                ///
                /// impl AstVisitorMut for MyVisitor {
                ///     setup_error_handling_impl!();
                /// }
                ///
                /// // MyVisitor::visit(my_ast_node)
                /// ```
                &mut AstVisitorMut
            ), infallible),
            visitor(drive(
                /// An immutable visitor that visits the AST for hax.
                &AstVisitor
            ), infallible),
            skip(
                String, bool, char, hax_frontend_exporter::Span,
                for<T> crate::interning::Interned<T>,
            ),
            drive(
                for<T: AstVisitable> Box<T>, for<T: AstVisitable> Option<T>, for<T: AstVisitable> Vec<T>,
                for<A: AstVisitable, B: AstVisitable> (A, B),
                for<A: AstVisitable, B: AstVisitable, C: AstVisitable> (A, B, C),
                usize
            ),
            override(AstNodes),
            override_skip(
                Span, Fragment, GlobalId, Diagnostic, AttrPayload,
            ),
        )]
        /// Helper trait to drive visitor.
        pub trait AstVisitable {}
    }

    #[allow(missing_docs)]
    pub mod fallible {
        use super::*;

        #[visitable_group(
            visitor(drive(
                /// An immutable visitor that can exit early.
                &AstEarlyExitVisitor
            )),
            visitor(drive_mut(
                /// An immutable visitor that can exit early and mutate the AST fragments.
                &mut AstEarlyExitVisitorMut
            )),
            skip(
                String, bool, char, hax_frontend_exporter::Span,
                for<T> crate::interning::Interned<T>,
            ),
            drive(
                for<T: AstVisitable> Box<T>, for<T: AstVisitable> Option<T>, for<T: AstVisitable> Vec<T>,
                for<A: AstVisitable, B: AstVisitable> (A, B),
                for<A: AstVisitable, B: AstVisitable, C: AstVisitable> (A, B, C),
                usize
            ),
            override(AstNodes),
            override_skip(
                Span, Fragment, GlobalId, Diagnostic, AttrPayload,
            ),
        )]
        /// Helper trait to drive visitor.
        pub trait AstVisitable {}
    }

    /// This modules provides `dyn` compatible trait for visitors.
    pub mod dyn_compatible {
        use super::*;

        macro_rules! derive_erased_ast_visitors {
            ({$($attrs:tt)*}, $name: ident, $helper: ident, $($ty:ty),*) => {
                $($attrs)*
                pub trait $name<'a>: $($helper<'a, $ty> + )* {}
            };
        }

        macro_rules! render_path {
            ($head:ident) => {stringify!($head)};
            ($head:ident $(::$tail:ident)*) => {
                concat!(stringify!($head), "::", render_path!($($tail)::*))
            };
        }

        macro_rules! make_dyn_compatible {
            ($($visitable_trait:ident)::*, $($visitor_trait:ident)::*, $helper_name: ident, $name: ident, mut:{$($mut:tt)?}, super:{$($super:ident)::*}, $ret:ty) => {
                #[doc = concat!("A [dyn-compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility) trait similar to [`", render_path!($($visitor_trait)::*),"`].")]
                #[doc = concat!("This trait provides one `visit` method to visit a given type `T` with a given visitor.")]
                pub trait $helper_name<'a, T: ?Sized>: $($super)::* {
                    /// Visit a value with the visitor.
                    fn visit(&mut self, _: &'a $($mut)? T) -> $ret;
                }

                impl<'a, T: $($visitable_trait)::*, V: $($visitor_trait)::*> $helper_name<'a, T> for V {
                    fn visit(&mut self, e: &'a $($mut)? T) -> $ret {
                        <Self as $($visitor_trait)::*>::visit(self, e)
                    }
                }
                derive_erased_ast_visitors!({
                    #[doc = concat!("A [dyn-compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility) trait similar to [`", render_path!($($visitor_trait)::*),"`].")]
                    #[doc = concat!("This trait is empty, but it implies a super bound for every type in the AST, so that you can use [`", stringify!($helper_name), "::visit", "`] with the entire AST.")]
                }, $name, $helper_name, AstNodes);

                impl<'a, V: $($visitor_trait)::*> $name<'a> for V {}
            };
        }

        make_dyn_compatible!(
            infallible::AstVisitable,
            infallible::AstVisitorMut,
            AstVisitableMut,
            AstVisitorMut,
            mut:{mut},
            super:{},
            ()
        );
        make_dyn_compatible!(
            infallible::AstVisitable,
            infallible::AstVisitor,
            AstVisitable,
            AstVisitor,
            mut:{},
            super:{},
            ()
        );

        make_dyn_compatible!(
            fallible::AstVisitable,
            fallible::AstEarlyExitVisitorMut,
            AstEarlyExitVisitableMut,
            AstEarlyExitVisitorMut,
            mut:{mut},
            super:{Visitor},
            ControlFlow<<Self as Visitor>::Break>
        );
        make_dyn_compatible!(
            fallible::AstVisitable,
            fallible::AstEarlyExitVisitor,
            AstEarlyExitVisitable,
            AstEarlyExitVisitor,
            mut:{},
            super:{Visitor},
            ControlFlow<<Self as Visitor>::Break>
        );
    }
}

pub use replaced::dyn_compatible;
use replaced::{fallible, infallible};

pub use fallible::{
    AstEarlyExitVisitor, AstEarlyExitVisitorMut, AstVisitable as AstVisitableFallible,
    AstVisitableWrapper,
};
pub use hax_rust_engine_macros::setup_error_handling_struct;
pub use infallible::{
    AstVisitable as AstVisitableInfallible, AstVisitableInfallibleWrapper, AstVisitor,
    AstVisitorMut,
};
pub use wrappers::{VisitorWithContext, VisitorWithErrors, setup_error_handling_impl};

#[test]
fn double_literals_in_ast() {
    use crate::ast::diagnostics::*;

    #[setup_error_handling_struct]
    #[derive(Default)]
    struct DoubleU8Literals;

    impl VisitorWithContext for DoubleU8Literals {
        fn context(&self) -> Context {
            Context::Import
        }
    }

    impl AstVisitorMut for DoubleU8Literals {
        setup_error_handling_impl!();

        fn visit_literal(&mut self, x: &mut Literal) {
            let Literal::Int { value, .. } = x else {
                return;
            };
            let Ok(n): Result<u8, _> = str::parse(value) else {
                return self.error(
                    x.clone(),
                    DiagnosticInfoKind::AssertionFailure {
                        details: "Bad literal".into(),
                    },
                );
            };
            let n = (n as u16) * 2;
            if n >= u8::MAX as u16 {
                return self.error(
                    x.clone(),
                    DiagnosticInfoKind::AssertionFailure {
                        details: "Literal too big".into(),
                    },
                );
            }
            *value = Symbol::new(&format!("{}", n));
        }
    }

    // Syntax helpers
    let int_kind = IntKind {
        size: IntSize::S8,
        signedness: Signedness::Signed,
    };
    let mk_lit = |n: isize| Literal::Int {
        value: Symbol::new(&format!("{}", n)),
        negative: false,
        kind: int_kind.clone(),
    };
    let meta = Metadata {
        span: Span::dummy(),
        attributes: vec![],
    };
    let mk_lit_expr = |n| Expr {
        kind: Box::new(ExprKind::Literal(mk_lit(n))),
        ty: Ty(Box::new(TyKind::Primitive(PrimitiveTy::Int(
            int_kind.clone(),
        )))),
        meta: meta.clone(),
    };
    let mk_array = |exprs| Expr {
        kind: Box::new(ExprKind::Array(exprs)),
        ty: Ty(Box::new(TyKind::RawPointer)), // wrong type, but this is not important for this test.
        meta: meta.clone(),
    };
    let mut lit_expr_200 = mk_lit_expr(200);

    // Creates the expression `[50u8, 100u8, 200u8]`: the last one cannot be doubled, and will cause an error.
    let mut e = mk_array(vec![
        mk_lit_expr(50),
        mk_lit_expr(100),
        lit_expr_200.clone(),
    ]);

    // Visit the expression.
    DoubleU8Literals::default().visit(&mut e);

    // Transform `lit_expr_200` into the error `DoubleU8Literal` should produce
    lit_expr_200.set_error(ErrorNode {
        fragment: Box::new(lit_expr_200.clone().into()),
        diagnostics: vec![Diagnostic::new(
            mk_lit(200),
            DiagnosticInfo {
                span: lit_expr_200.span(),
                context: Context::Import,
                kind: DiagnosticInfoKind::AssertionFailure {
                    details: "Literal too big".into(),
                },
            },
        )],
    });

    // Check that the visitor works as expected
    assert_eq!(
        e,
        mk_array(vec![mk_lit_expr(100), mk_lit_expr(200), lit_expr_200])
    );
}
