//! ProVerif-backend phase: model a `Result` by its `Ok` payload.
//!
//! Without `--result-errors`, a `Result` is its `Ok` payload and producing an
//! `Err` ends the trace, so no value is ever an `Err`. This phase replaces
//! `Ok(x)` by `x` in values and patterns, and an `Err` value by the error sink.
//! A match arm, an or-pattern alternative or a `let` that needs an `Err` never
//! matches, so it is dropped; any other `Err` pattern is rejected.

use crate::ast::span::Span;
use crate::ast::*;
use crate::ast::{diagnostics::*, visitors::*};
use crate::names::core::result::Result::{Err, Ok};
use crate::names::rust_primitives::hax::never_to_any;
use crate::phase::Phase;

/// Phase modelling a `Result` by its `Ok` payload.
#[derive(Default, Debug)]
pub struct ProverifOkOnlyResults;

#[setup_error_handling_struct]
#[derive(Default)]
struct OkOnlyResults;

impl VisitorWithContext for OkOnlyResults {
    fn context(&self) -> Context {
        Context::Phase(stringify!(ProverifOkOnlyResults).to_string())
    }
}

/// Whether `pat` only matches values that contain an `Err`.
fn pat_needs_err(pat: &Pat) -> bool {
    match &*pat.kind {
        PatKind::Construct {
            constructor,
            fields,
            ..
        } => *constructor == Err::Constructor || fields.iter().any(|(_, p)| pat_needs_err(p)),
        PatKind::Ascription { pat, .. } => pat_needs_err(pat),
        PatKind::Binding {
            sub_pat: Some(sub_pat),
            ..
        }
        | PatKind::Deref { sub_pat } => pat_needs_err(sub_pat),
        PatKind::Or { sub_pats } => sub_pats.iter().all(pat_needs_err),
        PatKind::Array { args } => args.iter().any(pat_needs_err),
        _ => false,
    }
}

/// Whether the arm needs an `Err`, in its pattern or its guard.
fn arm_needs_err(arm: &Arm) -> bool {
    pat_needs_err(&arm.pat)
        || arm.guard.as_ref().is_some_and(|guard| {
            let GuardKind::IfLet { lhs, .. } = &guard.kind;
            pat_needs_err(lhs)
        })
}

/// The error sink, which the printer renders as `bitstring_err()`.
fn sink(ty: Ty, span: Span) -> Expr {
    Expr::standalone_fn_app(never_to_any, vec![], vec![], ty, span)
}

impl AstVisitorMut for OkOnlyResults {
    setup_error_handling_impl!();

    fn visit_expr(&mut self, expr: &mut Expr) {
        // What never matches is dropped before its patterns are visited.
        match &mut *expr.kind {
            ExprKind::Let { lhs, .. } if pat_needs_err(lhs) => {
                *expr = sink(expr.ty.clone(), expr.meta.span);
                return;
            }
            ExprKind::Match { arms, .. } => arms.retain(|arm| !arm_needs_err(arm)),
            _ => {}
        }
        self.visit_inner(expr);
        match &mut *expr.kind {
            ExprKind::Construct {
                constructor,
                fields,
                ..
            } if *constructor == Ok::Constructor && fields.len() == 1 => {
                let (_, inner) = fields.remove(0);
                *expr = inner;
            }
            ExprKind::Construct { constructor, .. } if *constructor == Err::Constructor => {
                *expr = sink(expr.ty.clone(), expr.meta.span);
            }
            _ => {}
        }
    }

    fn visit_pat(&mut self, pat: &mut Pat) {
        if let PatKind::Or { sub_pats } = &mut *pat.kind
            && sub_pats.iter().any(|p| !pat_needs_err(p))
        {
            sub_pats.retain(|p| !pat_needs_err(p));
            if sub_pats.len() == 1 {
                *pat = sub_pats.remove(0);
            }
        }
        self.visit_inner(pat);
        match &mut *pat.kind {
            PatKind::Construct {
                constructor,
                fields,
                ..
            } if *constructor == Ok::Constructor && fields.len() == 1 => {
                let (_, inner) = fields.remove(0);
                *pat = inner;
            }
            PatKind::Construct { constructor, .. } if *constructor == Err::Constructor => self
                .error(
                    pat.clone(),
                    DiagnosticInfoKind::Unimplemented {
                        issue_id: None,
                        details: Some(
                            "an `Err` pattern cannot match under the default Result model; \
                             pass `--result-errors` to model `Err` values"
                                .into(),
                        ),
                    },
                ),
            _ => {}
        }
    }
}

impl Phase for ProverifOkOnlyResults {
    fn apply(&self, items: &mut Vec<Item>) {
        OkOnlyResults::default().visit(items)
    }
}
