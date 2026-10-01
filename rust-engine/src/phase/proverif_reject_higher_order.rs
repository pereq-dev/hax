//! ProVerif-backend phase: reject references to functions as values.
//!
//! ProVerif is first-order: an unapplied symbol that expects arguments (e.g.
//! the constructor `A` in `x.map(Self::A)`) cannot be passed around. Such a
//! reference becomes an error node, as does a call through the `Fn` traits
//! (of a closure or function the inlining did not remove).

use crate::ast::*;
use crate::ast::{diagnostics::*, visitors::*};
use crate::backends::proverif::analysis::{Analysis, visit_emitted};
use crate::backends::proverif::render_id;
use crate::names::core::ops::function::{Fn, FnMut, FnOnce};
use crate::phase::Phase;

/// Phase rejecting references to functions and constructors as values.
#[derive(Default, Debug)]
pub struct ProverifRejectHigherOrder;

#[setup_error_handling_struct]
struct RejectHigherOrder<'a> {
    analysis: &'a Analysis,
}

impl VisitorWithContext for RejectHigherOrder<'_> {
    fn context(&self) -> Context {
        Context::Phase(stringify!(ProverifRejectHigherOrder).to_string())
    }
}

impl AstVisitorMut for RejectHigherOrder<'_> {
    setup_error_handling_impl!();

    // A quote names symbols in verbatim ProVerif, where they are bare.
    fn visit_quote(&mut self, _: &mut Quote) {}

    fn visit_expr(&mut self, expr: &mut Expr) {
        let higher_order = |details: &str| DiagnosticInfoKind::Unimplemented {
            issue_id: None,
            details: Some(details.into()),
        };
        match &mut *expr.kind {
            ExprKind::App {
                trait_: Some((impl_expr, _)),
                ..
            } if [Fn, FnMut, FnOnce].contains(&impl_expr.goal.trait_) => self.error(
                expr.clone(),
                higher_order("call of a function value through the `Fn` traits"),
            ),
            // An applied head is not a value.
            ExprKind::App { head, args, .. } if matches!(&*head.kind, ExprKind::GlobalId(_)) => {
                for arg in args {
                    self.visit(arg);
                }
            }
            // A nullary value (of arity 0 everywhere) is unaffected.
            ExprKind::GlobalId(g)
                if self.analysis.arity(&render_id(g)) >= 1
                    || matches!(expr.ty.kind(), TyKind::Arrow { .. }) =>
            {
                self.error(
                    expr.clone(),
                    higher_order("higher-order reference to a function or constructor"),
                )
            }
            _ => self.visit_inner(expr),
        }
    }
}

impl Phase for ProverifRejectHigherOrder {
    fn apply(&self, items: &mut Vec<Item>) {
        let analysis = Analysis::new(items);
        let mut visitor = RejectHigherOrder {
            analysis: &analysis,
            error_handling_state: Default::default(),
        };
        // Only the emitted code needs a first-order form.
        let visitor = std::cell::RefCell::new(&mut visitor);
        visit_emitted(
            items,
            |item| visitor.borrow_mut().visit(item),
            |ii| visitor.borrow_mut().visit(ii),
        )
    }
}
