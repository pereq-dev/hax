//! ProVerif-backend phase: eliminate binding sub-patterns (`x @ p`) and
//! destructuring parameters.
//!
//! A ProVerif pattern binds a variable or destructures a constructor; it has no
//! way to do both at once. `x @ p` is rewritten into the binder `x` alone,
//! followed by a `let p = x in …` over the scope the sub-pattern's variables
//! belong to. Likewise, a `letfun` parameter is a plain binder, so a
//! destructuring parameter `p` becomes a binder `x` and a `let p = x in …`.
//!
//! A nested destructure that fails goes to the error sink, whereas a match arm
//! whose pattern fails falls through to the next arm. The two agree as long as
//! the sub-pattern always matches, so a refutable one under a match arm is
//! rejected instead.

use crate::ast::visitors::*;
use crate::ast::{diagnostics::*, *};

use crate::backends::proverif::generated_local;
use crate::phase::Phase;

/// Phase that flattens `x @ p` patterns for the ProVerif printer.
#[derive(Default, Debug)]
pub struct ProverifFlattenBindingSubpatterns;

#[setup_error_handling_struct]
#[derive(Default)]
struct FlattenVisitor;

impl VisitorWithContext for FlattenVisitor {
    fn context(&self) -> Context {
        Context::Phase(stringify!(ProverifFlattenBindingSubpatterns).to_string())
    }
}

/// Strip every `sub_pat` out of `pat`, returning each one paired with the
/// variable that now binds the value it matches. A sub-pattern comes before
/// the ones nested in it, since it binds the variables they destructure.
fn strip(pat: &mut Pat, out: &mut Vec<(LocalId, Ty, Pat)>) {
    match &mut *pat.kind {
        PatKind::Binding { var, sub_pat, .. } => {
            if let Some(mut inner) = sub_pat.take() {
                let position = out.len();
                strip(&mut inner, out);
                out.insert(position, (var.clone(), pat.ty.clone(), inner));
            }
        }
        PatKind::Construct { fields, .. } => {
            for (_, p) in fields.iter_mut() {
                strip(p, out)
            }
        }
        PatKind::Array { args } => {
            for p in args.iter_mut() {
                strip(p, out)
            }
        }
        PatKind::Or { sub_pats } => {
            for p in sub_pats.iter_mut() {
                strip(p, out)
            }
        }
        PatKind::Ascription { pat, .. } | PatKind::Deref { sub_pat: pat } => strip(pat, out),
        PatKind::Constant { .. } | PatKind::Wild | PatKind::Resugared(_) | PatKind::Error(_) => {}
    }
}

/// Wrap `body` in one `let <sub-pattern> = <var> in …` per stripped binding,
/// the first one outermost.
fn nest(body: &mut Expr, stripped: &[(LocalId, Ty, Pat)]) {
    for (var, ty, pat) in stripped.iter().rev() {
        let span = pat.meta.span;
        let rhs = ExprKind::LocalId(var.clone()).promote(ty.clone(), span);
        let ty = body.ty.clone();
        *body = ExprKind::Let {
            lhs: pat.clone(),
            rhs,
            body: body.clone(),
        }
        .promote(ty, span);
    }
}

/// A pattern that names parts of a value without testing it, so destructuring
/// against it can never fail. `is_struct` marks the constructors of
/// single-variant types, tuples included.
fn is_irrefutable(pat: &Pat) -> bool {
    match &*pat.kind {
        PatKind::Wild => true,
        PatKind::Binding { sub_pat, .. } => sub_pat.as_ref().is_none_or(is_irrefutable),
        PatKind::Ascription { pat, .. } | PatKind::Deref { sub_pat: pat } => is_irrefutable(pat),
        PatKind::Construct {
            is_struct, fields, ..
        } => *is_struct && fields.iter().all(|(_, p)| is_irrefutable(p)),
        _ => false,
    }
}

/// Strip `pat` and push the sub-patterns onto `body`.
fn flatten(pat: &mut Pat, body: &mut Expr) -> Vec<(LocalId, Ty, Pat)> {
    let mut stripped = Vec::new();
    strip(pat, &mut stripped);
    nest(body, &stripped);
    stripped
}

/// Flatten the parameters of a function, binding each destructuring one to a
/// fresh `hax__param{i}` that `body` then destructures.
fn flatten_params(params: &mut [Param], body: &mut Expr) {
    for (i, param) in params.iter_mut().enumerate() {
        flatten(&mut param.pat, body);
        let is_binder = param.pat.as_binder().is_some()
            || match &*param.pat.kind {
                PatKind::Wild => true,
                PatKind::Construct {
                    is_struct, fields, ..
                } => *is_struct && fields.is_empty(),
                _ => false,
            };
        if !is_binder {
            let var = generated_local(&format!("hax__param{i}"));
            let binder = PatKind::var_pat(var.clone()).promote(param.ty.clone(), param.pat.span());
            let pat = std::mem::replace(&mut param.pat, binder);
            nest(body, &[(var, param.ty.clone(), pat)]);
        }
    }
}

impl AstVisitorMut for FlattenVisitor {
    setup_error_handling_impl!();

    fn visit_expr(&mut self, expr: &mut Expr) {
        self.visit_inner(expr);
        match &mut *expr.kind {
            ExprKind::Let { lhs, body, .. } => {
                flatten(lhs, body);
            }
            ExprKind::Closure { params, body, .. } => {
                for pat in params.iter_mut() {
                    flatten(pat, body);
                }
            }
            ExprKind::Loop { kind, body, .. } => {
                if let LoopKind::ForLoop { pat, .. } = &mut **kind {
                    flatten(pat, body);
                }
            }
            _ => {}
        }
    }

    fn visit_arm(&mut self, arm: &mut Arm) {
        self.visit_inner(arm);
        let stripped = flatten(&mut arm.pat, &mut arm.body);
        for (_, _, pat) in &stripped {
            if !is_irrefutable(pat) {
                self.error(
                    pat.clone(),
                    DiagnosticInfoKind::ExplicitRejection {
                        reason: "ProVerif has no binding sub-pattern (`x @ p`). One whose `p` \
                                 can fail to match is not modelable here: the arm would have \
                                 to fall through to the next one, and the nested destructure \
                                 it becomes goes to the error sink instead."
                            .to_string(),
                        issue_id: None,
                    },
                );
            }
        }
        // A guard is evaluated in the arm's scope, so it sees the sub-pattern's
        // bindings too.
        if let Some(guard) = &mut arm.guard {
            let GuardKind::IfLet { rhs, .. } = &mut guard.kind;
            nest(rhs, &stripped);
        }
    }

    fn visit_item(&mut self, item: &mut Item) {
        self.visit_inner(item);
        if let ItemKind::Fn { params, body, .. } = &mut item.kind {
            flatten_params(params, body)
        }
    }

    fn visit_impl_item(&mut self, impl_item: &mut ImplItem) {
        self.visit_inner(impl_item);
        if let ImplItemKind::Fn { params, body, .. } = &mut impl_item.kind {
            flatten_params(params, body)
        }
    }
}

impl Phase for ProverifFlattenBindingSubpatterns {
    fn apply(&self, items: &mut Vec<Item>) {
        FlattenVisitor::default().visit(items)
    }
}
