//! ProVerif-backend phase: bind the failable terms the printer tests, and drop
//! match guards.
//!
//! ProVerif takes the `else` branch of `let <pat> = <value> in … else …` also
//! when evaluating `value` fails, whereas a failure in Rust ends the execution.
//! So each term the printer destructures with an `else` (an `if` condition, the
//! left operand of `&&`/`||`, a match scrutinee) is first bound with a `let`
//! when it can fail: a binding without `else` fails the whole term. The printer
//! also re-tests a scrutinee in every arm, where a call whose model draws a
//! fresh value would draw one per arm.
//!
//! A guarded arm `p if let q = e => body` followed by the arms `rest` becomes
//!
//! ```text
//! let arm = match s { p => match e { q => Some(body), _ => None }, _ => None } in
//! match arm { Some(v) => v, _ => match s { rest } }
//! ```
//!
//! so that `rest` is printed once; `_ if let q = e => body` simply becomes
//! `match e { q => body, _ => match s { rest } }`.

use crate::ast::span::Span;
use crate::ast::visitors::*;
use crate::ast::*;
use crate::backends::proverif::analysis::{Analysis, has_error};
use crate::names::rust_primitives::hax::{logical_op_and, logical_op_or};
use crate::phase::Phase;

/// Phase binding failable terms and dropping match guards.
#[derive(Default, Debug)]
pub struct ProverifBindFailable;

struct BindFailable<'a> {
    analysis: &'a Analysis,
    /// Counter numbering the binders of the current function.
    next: usize,
}

impl BindFailable<'_> {
    /// The binder and the use of a fresh local `hax__{kind}{n}` of type `ty`.
    fn fresh(&mut self, kind: &str, ty: &Ty, span: Span) -> (Pat, Expr) {
        let var: LocalId = format!("hax__{kind}{}", self.next).as_str().into();
        self.next += 1;
        (
            PatKind::var_pat(var.clone()).promote(ty.clone(), span),
            ExprKind::LocalId(var).promote(ty.clone(), span),
        )
    }

    /// `body(e)`, where `e` is first bound to a fresh `hax__{kind}{n}` if it
    /// can fail.
    fn bind(&mut self, kind: &str, e: Expr, body: impl FnOnce(&mut Self, Expr) -> Expr) -> Expr {
        if self.analysis.cannot_fail(&e) {
            return body(self, e);
        }
        let span = e.meta.span;
        let (pat, local) = self.fresh(kind, &e.ty, span);
        let body = body(self, local);
        let ty = body.ty.clone();
        ExprKind::Let {
            lhs: pat,
            rhs: e,
            body,
        }
        .promote(ty, span)
    }

    /// `match scrutinee { arms }` of type `ty` without guards; `scrutinee`
    /// cannot fail, so it can be tested again.
    fn drop_guards(&mut self, scrutinee: Expr, mut arms: Vec<Arm>, ty: &Ty, span: Span) -> Expr {
        let Some(i) = arms.iter().position(|arm| arm.guard.is_some()) else {
            return ExprKind::Match { scrutinee, arms }.promote(ty.clone(), span);
        };
        let rest = arms.split_off(i + 1);
        let guarded = arms.pop().expect("the guarded arm");
        let Some(Guard {
            kind: GuardKind::IfLet { lhs, rhs },
            ..
        }) = guarded.guard
        else {
            unreachable!("the guarded arm has a guard")
        };
        let wild = |ty: &Ty| PatKind::Wild.promote(ty.clone(), span);
        let matching = |scrutinee: Expr, pat: Pat, body: Expr, otherwise: Expr| {
            let ty = body.ty.clone();
            let wild = wild(&scrutinee.ty);
            ExprKind::Match {
                scrutinee,
                arms: vec![
                    Arm::non_guarded(pat, body, span),
                    Arm::non_guarded(wild, otherwise, span),
                ],
            }
            .promote(ty, span)
        };
        let guarded = if matches!(&*guarded.pat.kind, PatKind::Wild) {
            self.bind("cond", rhs, |this, rhs| {
                let rest = this.drop_guards(scrutinee.clone(), rest, ty, span);
                matching(rhs, lhs, guarded.body, rest)
            })
        } else {
            let option_ty = TyKind::App {
                head: crate::names::core::option::Option,
                args: vec![GenericValue::Ty(ty.clone())],
            }
            .promote();
            let none = Expr::none(option_ty.clone(), span);
            let some = Expr::some(guarded.body, option_ty.clone(), span);
            let test = self.bind("cond", rhs, |_, rhs| matching(rhs, lhs, some, none.clone()));
            let value = matching(scrutinee.clone(), guarded.pat, test, none);
            let (arm_pat, arm) = self.fresh("arm", &option_ty, span);
            let (val_pat, val) = self.fresh("val", ty, span);
            let rest = self.drop_guards(scrutinee.clone(), rest, ty, span);
            let unwrap = matching(arm, Pat::some(val_pat, option_ty, span), val, rest);
            ExprKind::Let {
                lhs: arm_pat,
                rhs: value,
                body: unwrap,
            }
            .promote(ty.clone(), span)
        };
        if arms.is_empty() {
            return guarded;
        }
        arms.push(Arm::non_guarded(wild(&scrutinee.ty), guarded, span));
        ExprKind::Match { scrutinee, arms }.promote(ty.clone(), span)
    }
}

impl BindFailable<'_> {
    fn rewrite(&mut self, expr: Expr) -> Expr {
        let Expr { kind, ty, meta } = expr;
        let span = meta.span;
        match *kind {
            // The printer drops an `if` whose condition is an error.
            ExprKind::If {
                condition,
                then,
                else_,
            } if !has_error(&condition) => self.bind("cond", condition, |_, condition| {
                ExprKind::If {
                    condition,
                    then,
                    else_,
                }
                .promote(ty, span)
            }),
            ExprKind::App {
                head,
                mut args,
                generic_args,
                bounds_impls,
                trait_,
            } if args.len() == 2
                && matches!(&*head.kind, ExprKind::GlobalId(g)
                    if *g == logical_op_and || *g == logical_op_or) =>
            {
                let lhs = args.remove(0);
                self.bind("cond", lhs, |_, lhs| {
                    args.insert(0, lhs);
                    ExprKind::App {
                        head,
                        args,
                        generic_args,
                        bounds_impls,
                        trait_,
                    }
                    .promote(ty, span)
                })
            }
            // The printer renders a match without arms as the error sink.
            ExprKind::Match { scrutinee, arms } if !arms.is_empty() => {
                self.bind("scrut", scrutinee, |this, scrutinee| {
                    this.drop_guards(scrutinee, arms, &ty, span)
                })
            }
            kind => Expr {
                kind: Box::new(kind),
                ty,
                meta,
            },
        }
    }
}

impl AstVisitorMut for BindFailable<'_> {
    fn visit_expr(&mut self, expr: &mut Expr) {
        let placeholder = Expr::unit(expr.meta.span);
        *expr = self.rewrite(std::mem::replace(expr, placeholder));
        // The terms bound here are now locals, which are not bound again.
        self.visit_inner(expr)
    }
}

impl Phase for ProverifBindFailable {
    fn apply(&self, items: &mut Vec<Item>) {
        let analysis = Analysis::new(items);
        let mut visitor = BindFailable {
            analysis: &analysis,
            next: 0,
        };
        for item in items.iter_mut() {
            match &mut item.kind {
                ItemKind::Fn { body, .. } => {
                    visitor.next = 0;
                    visitor.visit_expr(body);
                }
                ItemKind::Impl { items, .. } => {
                    for impl_item in items {
                        if let ImplItemKind::Fn { body, .. } = &mut impl_item.kind {
                            visitor.next = 0;
                            visitor.visit_expr(body);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
