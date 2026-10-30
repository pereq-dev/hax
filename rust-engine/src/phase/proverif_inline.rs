//! ProVerif-backend phase: inline functions ProVerif cannot declare.
//!
//! ProVerif has no functions as values, so a function taking a function-typed
//! parameter is replaced at every call site by an α-renamed copy of its body,
//! in which the calls of closure arguments are β-reduced. So is a function
//! tagged `#[hax_lib::pv_inline]`. The inlined items are marked late-skip, so
//! they are not emitted.

use std::collections::{HashMap, HashSet};

use hax_lib_macros_types::{AttrPayload, ItemStatus};

use crate::ast::identifiers::GlobalId;
use crate::ast::span::Span;
use crate::ast::utils::mappers::{Renaming, ScopedRenamer};
use crate::ast::visitors::*;
use crate::ast::*;
use crate::backends::proverif::analysis::hides_body;
use crate::backends::proverif::{generated_local, render_local};
use crate::phase::Phase;

/// Phase inlining `#[hax_lib::pv_inline]` and higher-order functions.
#[derive(Default, Debug)]
pub struct ProverifInline;

/// Bound on the substitution fixpoint, to keep the phase total.
const MAX_INLINE_DEPTH: usize = 100;

/// The tool attribute emitted by `#[hax_lib::pv_inline]`. It is not an
/// `AttrPayload` variant, so that the OCaml engine passes it through unparsed.
const PV_INLINE_PATH: &str = "_hax::pv_inline";

/// Whether the function is tagged `#[hax_lib::pv_inline]` or is higher-order,
/// and its body is emitted.
fn is_inlinable(meta: &Metadata, params: &[Param]) -> bool {
    let tagged = meta
        .attributes
        .iter()
        .any(|a| matches!(&a.kind, AttributeKind::Tool { path, .. } if path == PV_INLINE_PATH));
    let higher_order = params
        .iter()
        .any(|p| matches!(p.ty.kind(), TyKind::Arrow { .. }));
    (tagged || higher_order) && !hides_body(meta, params)
}

/// The functions of `inlinable` that reach themselves through the bodies of
/// `inlinable`, whose inlining would not terminate.
fn recursive(inlinable: &HashMap<GlobalId, (Vec<Pat>, Expr)>) -> HashSet<GlobalId> {
    let callees: HashMap<GlobalId, Vec<GlobalId>> = inlinable
        .iter()
        .map(|(f, (_, body))| {
            let callees = inlinable.keys().filter(|g| body.mentions(**g));
            (*f, callees.copied().collect())
        })
        .collect();
    let reaches_itself = |f: GlobalId| {
        let mut seen = HashSet::new();
        let mut stack = callees[&f].clone();
        while let Some(g) = stack.pop() {
            if g == f {
                return true;
            }
            if seen.insert(g) {
                stack.extend(&callees[&g]);
            }
        }
        false
    };
    callees
        .keys()
        .copied()
        .filter(|f| reaches_itself(*f))
        .collect()
}

/// Renames every binder to a fresh `inl<N>__<name>`.
struct Fresh<'a>(&'a mut usize);

impl Renaming for Fresh<'_> {
    type Key = LocalId;
    fn key(&self, id: &LocalId) -> LocalId {
        id.clone()
    }
    fn fresh(&mut self, id: &LocalId) -> LocalId {
        *self.0 += 1;
        generated_local(&format!("inl{}__{}", self.0, render_local(id)))
    }
}

/// `params` and `body` with every binder given a fresh name, so that the
/// arguments substituted in, which live in the caller's scope, cannot be
/// captured.
fn freshen(params: &mut [Pat], body: &mut Expr, next_fresh: &mut usize) {
    let mut renamer = ScopedRenamer::new(Fresh(next_fresh));
    for param in params.iter_mut() {
        renamer.bind(param);
    }
    renamer.visit_expr(body);
}

/// `let p1 = e1 in … let pn = en in body`.
fn wrap_in_lets(lets: Vec<(Pat, Expr)>, body: Expr, span: Span) -> Expr {
    lets.into_iter().rev().fold(body, |body, (lhs, rhs)| {
        let ty = body.ty.clone();
        ExprKind::Let { lhs, rhs, body }.promote(ty, span)
    })
}

struct InlineVisitor<'a> {
    /// `name → (params, body)` of the inlinable functions.
    inlinable: &'a HashMap<GlobalId, (Vec<Pat>, Expr)>,
    /// Counter numbering the binders of inlined bodies.
    next_fresh: usize,
}

impl InlineVisitor<'_> {
    /// The inlined copy of a call of an inlinable function.
    ///
    /// A closure or leaf argument is substituted for its parameter, so that a
    /// call of a closure parameter is β-reduced. Any other argument is bound
    /// with a `let`, so it is evaluated exactly once, as in Rust.
    fn inline_call(&mut self, expr: &Expr) -> Option<Expr> {
        let ExprKind::App { head, args, .. } = &*expr.kind else {
            return None;
        };
        let ExprKind::GlobalId(g) = &*head.kind else {
            return None;
        };
        let (mut params, mut body) = self.inlinable.get(g).cloned()?;
        if params.len() != args.len() {
            return None;
        }
        freshen(&mut params, &mut body, &mut self.next_fresh);
        let mut env = HashMap::new();
        let mut lets = Vec::new();
        for (param, arg) in params.into_iter().zip(args) {
            let is_substitutable = arg.is_leaf() || matches!(&*arg.kind, ExprKind::Closure { .. });
            match param.as_binder() {
                Some(id) if is_substitutable => {
                    env.insert(id.clone(), arg.clone());
                }
                _ => lets.push((param, arg.clone())),
            }
        }
        SubstArgs {
            env: &env,
            next_fresh: &mut self.next_fresh,
        }
        .visit_expr(&mut body);
        Some(wrap_in_lets(lets, body, expr.meta.span))
    }
}

impl AstVisitorMut for InlineVisitor<'_> {
    fn visit_expr(&mut self, expr: &mut Expr) {
        // An inlined body may itself call another inlinable.
        for _ in 0..MAX_INLINE_DEPTH {
            match self.inline_call(expr) {
                Some(new) => *expr = new,
                None => break,
            }
        }
        self.visit_inner(expr)
    }
}

/// Substitutes the closure and leaf arguments of an inlined call for their
/// parameters, β-reducing every call of a substituted closure. The body has
/// been α-renamed, so no binder in it shadows a parameter.
struct SubstArgs<'a> {
    env: &'a HashMap<LocalId, Expr>,
    next_fresh: &'a mut usize,
}

impl AstVisitorMut for SubstArgs<'_> {
    fn visit_expr(&mut self, expr: &mut Expr) {
        // A call of a parameter: substitute in the arguments only, so that the
        // head is still the parameter when it is β-reduced below.
        if let ExprKind::App { head, args, .. } = &mut *expr.kind
            && let ExprKind::LocalId(id) = &*head.kind
            && let Some(replacement) = self.env.get(id)
        {
            for arg in args.iter_mut() {
                self.visit_expr(arg);
            }
            match &*replacement.kind {
                ExprKind::Closure { params, body, .. } if params.len() == args.len() => {
                    let (mut params, mut body) = (params.clone(), body.clone());
                    freshen(&mut params, &mut body, self.next_fresh);
                    let lets = params.into_iter().zip(args.iter().cloned()).collect();
                    *expr = wrap_in_lets(lets, body, expr.meta.span);
                }
                _ => *head = replacement.clone(),
            }
            return;
        }
        self.visit_inner(expr);
        if let ExprKind::LocalId(id) = &*expr.kind
            && let Some(replacement) = self.env.get(id)
        {
            *expr = replacement.clone();
        }
    }
}

fn mark_late_skip(meta: &mut Metadata) {
    meta.attributes.push(Attribute {
        kind: AttributeKind::Hax(AttrPayload::ItemStatus(ItemStatus::Included {
            late_skip: true,
        })),
        span: meta.span,
    });
}

impl Phase for ProverifInline {
    fn apply(&self, items: &mut Vec<Item>) {
        let mut inlinable = HashMap::new();
        let mut record = |meta: &Metadata, name: GlobalId, params: &[Param], body: &Expr| {
            if is_inlinable(meta, params) {
                let params = params.iter().map(|p| p.pat.clone()).collect();
                inlinable.insert(name, (params, body.clone()));
            }
        };
        for item in items.iter() {
            match &item.kind {
                ItemKind::Fn {
                    name, body, params, ..
                } => record(&item.meta, *name, params, body),
                ItemKind::Impl { items, .. } => {
                    for impl_item in items {
                        if let ImplItemKind::Fn { body, params } = &impl_item.kind {
                            record(&impl_item.meta, impl_item.ident, params, body);
                        }
                    }
                }
                _ => {}
            }
        }

        for f in recursive(&inlinable) {
            inlinable.remove(&f);
        }

        let mut visitor = InlineVisitor {
            inlinable: &inlinable,
            next_fresh: 0,
        };
        for item in items.iter_mut() {
            if inlinable.contains_key(&item.ident) {
                mark_late_skip(&mut item.meta);
            }
            if let ItemKind::Impl { items, .. } = &mut item.kind {
                for impl_item in items {
                    if inlinable.contains_key(&impl_item.ident) {
                        mark_late_skip(&mut impl_item.meta);
                    }
                }
            }
            visitor.visit_item(item);
        }
    }
}
