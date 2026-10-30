//! ProVerif-backend phase: make the local binders of each function unique, then
//! inline the single-use ones.
//!
//! `&mut` lowering threads state through chains of `let self = … in …`, which
//! ProVerif warns about (`identifier self rebound`), so every rebinding of a
//! name is renamed: `self_1`, `self_2`, …. With unique names, the single-use
//! aliases ANF introduces (`let hoist21 = out1 in …`) can be inlined without
//! capture.

use std::collections::{HashMap, HashSet};

use crate::ast::utils::mappers::{Renaming, ScopedRenamer};
use crate::ast::visitors::*;
use crate::ast::*;
use crate::backends::proverif::{expr_is_atomic, is_generated_binder, render_local};
use crate::names::rust_primitives::hax::{logical_op_and, logical_op_or};
use crate::phase::Phase;
use crate::symbol::Symbol;

/// Phase renaming rebound locals and inlining single-use ones.
#[derive(Default, Debug)]
pub struct ProverifSimplifyBindings;

/// Numbers the successive bindings of a name. Binders are resolved by their
/// `LocalId`, but numbered by their rendered name, as ProVerif sees them, so
/// that two locals rendering alike get distinct names.
struct Versions {
    /// Rendered name → number of bindings seen so far.
    counter: HashMap<String, usize>,
    /// Every rendered name of the function, so that a new `{name}_{v}` never
    /// captures a variable the user named so.
    taken: HashSet<String>,
}

impl Renaming for Versions {
    type Key = LocalId;

    fn key(&self, id: &LocalId) -> LocalId {
        id.clone()
    }

    /// The first binding of a name keeps it and each later one becomes
    /// `{name}_{v}`. A name of the form of a binder the printer generates is
    /// always renamed, to `{name}_v{v}`.
    fn fresh(&mut self, original: &LocalId) -> LocalId {
        let name = render_local(original);
        let generated = is_generated_binder(&name);
        let version = self.counter.entry(name.clone()).or_insert(0);
        if *version == 0 && !generated {
            *version = 1;
            return original.clone();
        }
        let separator = if generated { "_v" } else { "_" };
        loop {
            let candidate = format!("{name}{separator}{version}");
            *version += 1;
            if !is_generated_binder(&candidate) && self.taken.insert(candidate.clone()) {
                return LocalId(Symbol::new(candidate));
            }
        }
    }
}

/// Rename the rebindings of `params` and `body`. A parameter counts as the
/// first binding of its name.
fn freshen(params: &mut [Param], body: &mut Expr) {
    struct Names(HashSet<String>);
    impl AstVisitor for Names {
        fn visit_local_id(&mut self, id: &LocalId) {
            self.0.insert(render_local(id));
        }
    }
    let mut names = Names(HashSet::new());
    for param in params.iter() {
        names.visit_pat(&param.pat);
    }
    names.visit_expr(body);
    let mut renamer = ScopedRenamer::new(Versions {
        counter: HashMap::new(),
        taken: names.0,
    });
    for param in params.iter_mut() {
        match param.pat.as_binder() {
            Some(id) if !is_generated_binder(&render_local(id)) => {
                renamer.renaming.counter.insert(render_local(id), 1);
                renamer.declare(id.clone());
            }
            _ => renamer.bind(&mut param.pat),
        }
    }
    renamer.visit_expr(body);
}

/// The number of uses of `x` in `e`, and whether `e` contains a quote. The
/// substitution does not rewrite quotes, so a quote in scope makes `x`
/// ineligible for inlining.
fn count_uses(e: &Expr, x: &LocalId) -> (usize, bool) {
    struct Count<'a>(&'a LocalId, usize, bool);
    impl AstVisitor for Count<'_> {
        fn visit_expr(&mut self, e: &Expr) {
            match &*e.kind {
                ExprKind::LocalId(id) => self.1 += usize::from(id == self.0),
                ExprKind::Quote { .. } => self.2 = true,
                _ => self.visit_inner(e),
            }
        }
    }
    let mut count = Count(x, 0, false);
    count.visit_expr(e);
    (count.1, count.2)
}

/// Replace the uses of `x` in `e` by `replacement`, outside of quotes.
fn substitute(e: &mut Expr, x: &LocalId, replacement: &Expr) {
    struct Subst<'a>(&'a LocalId, &'a Expr);
    impl AstVisitorMut for Subst<'_> {
        fn visit_expr(&mut self, e: &mut Expr) {
            match &*e.kind {
                ExprKind::LocalId(id) if id == self.0 => *e = self.1.clone(),
                ExprKind::Quote { .. } => {}
                _ => self.visit_inner(e),
            }
        }
    }
    Subst(x, replacement).visit_expr(e)
}

/// Whether the single occurrence of `x` in `e` is evaluated unconditionally:
/// reached without passing through a branch, a closure or a loop body.
fn uncond_use(e: &Expr, x: &LocalId) -> bool {
    match &*e.kind {
        ExprKind::LocalId(id) => id == x,
        ExprKind::Let { rhs, body, .. } => uncond_use(rhs, x) || uncond_use(body, x),
        // The right operand of `&&` / `||` is evaluated conditionally.
        ExprKind::App { head, args, .. }
            if matches!(&*head.kind, ExprKind::GlobalId(g)
                if *g == logical_op_and || *g == logical_op_or) =>
        {
            args.first().is_some_and(|a| uncond_use(a, x))
        }
        ExprKind::App { head, args, .. } => {
            uncond_use(head, x) || args.iter().any(|a| uncond_use(a, x))
        }
        ExprKind::Construct { fields, base, .. } => {
            fields.iter().any(|(_, v)| uncond_use(v, x))
                || base.as_ref().is_some_and(|b| uncond_use(b, x))
        }
        ExprKind::Array(es) => es.iter().any(|e| uncond_use(e, x)),
        ExprKind::Ascription { e, .. } => uncond_use(e, x),
        ExprKind::If { condition, .. } => uncond_use(condition, x),
        ExprKind::Match { scrutinee, .. } => uncond_use(scrutinee, x),
        ExprKind::Borrow { inner, .. } | ExprKind::AddressOf { inner, .. } => uncond_use(inner, x),
        ExprKind::Block { body, .. } => uncond_use(body, x),
        ExprKind::Return { value } => uncond_use(value, x),
        _ => false,
    }
}

/// Inlines, bottom-up, each `let x = e in body` whose `x` is used once. ProVerif
/// evaluates a `let` eagerly, so a leaf `e` is inlined anywhere, a failable
/// atomic one only into an unconditionally evaluated position, and any other
/// never. The `let` of an unused leaf is dropped.
struct InlineSingleUse;

impl AstVisitorMut for InlineSingleUse {
    fn visit_expr(&mut self, expr: &mut Expr) {
        // A quote is verbatim ProVerif, which names its own variables.
        if matches!(&*expr.kind, ExprKind::Quote { .. }) {
            return;
        }
        self.visit_inner(expr);
        let ExprKind::Let { lhs, rhs, body } = &*expr.kind else {
            return;
        };
        let Some(x) = lhs.as_binder() else {
            return;
        };
        let inlined = match count_uses(body, x) {
            (_, true) => None,
            (0, _) if rhs.is_leaf() => Some(body.clone()),
            (1, _) if rhs.is_leaf() || (expr_is_atomic(rhs) && uncond_use(body, x)) => {
                let mut body = body.clone();
                substitute(&mut body, x, rhs);
                Some(body)
            }
            _ => None,
        };
        if let Some(inlined) = inlined {
            *expr = inlined;
        }
    }
}

impl Phase for ProverifSimplifyBindings {
    fn apply(&self, items: &mut Vec<Item>) {
        let simplify = |params: &mut [Param], body: &mut Expr| {
            freshen(params, body);
            InlineSingleUse.visit_expr(body);
        };
        for item in items.iter_mut() {
            match &mut item.kind {
                ItemKind::Fn { body, params, .. } => simplify(params, body),
                ItemKind::Impl { items, .. } => {
                    for impl_item in items {
                        if let ImplItemKind::Fn { body, params } = &mut impl_item.kind {
                            simplify(params, body)
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
