//! ProVerif-backend checks that keep each `Result` model sound.
//!
//! The default model ends the trace on every `Err`, so it is only sound for
//! code that neither goes on after one nor does anything observable between
//! producing it and returning it: [`reject_recovery`] rejects every place
//! where a value that may hold a `Result` is not propagated, or where an
//! observable call may run while one is pending.
//!
//! Under `--result-errors`, an `Err` produced by a function whose body is not
//! extracted carries a term of the hand-written model, which need not be the
//! Rust error value. [`ProverifRejectUnknownErrors`] rejects the code that
//! inspects such a value, and the functions whose model is a bare term where
//! an `Ok` or an `Err` is expected.

use std::collections::{HashMap, HashSet};

use crate::ast::identifiers::global_id::TupleId;
use crate::ast::*;
use crate::ast::{diagnostics::*, visitors::*};
use crate::backends::is_late_skip;
use crate::backends::proverif::analysis::{
    body_is_hidden, fn_decls, quote_text, scan_declarations,
};
use crate::backends::proverif::render_id;
use crate::names::core::result::Result::Err;
use crate::names::rust_primitives::hax::never_to_any;
use crate::phase::Phase;
use hax_lib_macros_types::AttrPayload;

/// Library functions that keep the `Err` of their first argument in their
/// result.
const KEEP_ERR: &[&str] = &[
    "core__clone__Clone__clone",
    "core__ops__try_trait__FromResidual__from_residual",
    "core__ops__try_trait__Try__branch",
    "core__option__Impl_4__transpose",
    "core__result__Impl__map",
    "core__result__Impl__map_err",
];

/// Library functions that panic on an `Err` of their first argument.
const PANIC_ON_ERR: &[&str] = &["core__result__Impl__unwrap", "core__result__Impl__expect"];

/// Library functions whose result is their argument.
const IDENTITY: &[&str] = &["core__convert__From__from", "core__convert__Into__into"];

/// Library functions that only read whether their first argument is an `Err`,
/// and its `Ok` payload.
const READ_OK: &[&str] = &[
    "core__result__Impl__expect",
    "core__result__Impl__is_err",
    "core__result__Impl__is_ok",
    "core__result__Impl__ok",
    "core__result__Impl__unwrap",
    "core__result__Impl__unwrap_or",
];

/// Library functions returning the error of their first argument.
const READ_ERR: &[&str] = &[
    "core__result__Impl__err",
    "core__result__Impl__expect_err",
    "core__result__Impl__unwrap_err",
];

/// The prefixes of the names that hax's own libraries model, without events.
const LIBRARY_PREFIXES: &[&str] = &[
    "alloc__",
    "core__",
    "hax_lib__",
    "rust_primitives__",
    "std__",
];

/// The libraries whose `fun`, `reduc` and `const` declarations a hand-written
/// model may apply.
const MODEL_LIBRARIES: &[&str] = &[
    include_str!("../../../hax-lib/proof-libs/proverif/primitives.pvl"),
    include_str!("../../../hax-lib/proof-libs/proverif/cryptolib.pvl"),
];

/// Whether the model extracts the function with these `meta` and `body`.
fn is_extracted(meta: &Metadata, body: &Expr) -> bool {
    let dropped = matches!(&*body.kind, ExprKind::GlobalId(g)
        if *g == crate::names::rust_primitives::hax::dropped_body);
    !is_late_skip(meta) && !body_is_hidden(meta) && quoted_body(body).is_none() && !dropped
}

/// The verbatim body of a function, as `replace_body` makes it.
fn quoted_body(body: &Expr) -> Option<&Quote> {
    match &*body.kind {
        ExprKind::Quote { contents } => Some(contents),
        ExprKind::Block { body, .. } => quoted_body(body),
        _ => None,
    }
}

/// Applies `f` to the name, parameters and body of every function the model
/// extracts.
fn for_each_extracted_fn(items: &mut [Item], mut f: impl FnMut(GlobalId, &mut [Param], &mut Expr)) {
    for item in items.iter_mut() {
        match &mut item.kind {
            ItemKind::Fn {
                name, params, body, ..
            } if is_extracted(&item.meta, body) => f(*name, params, body),
            ItemKind::Impl { items, .. } => {
                for ii in items.iter_mut() {
                    if let ImplItemKind::Fn { params, body } = &mut ii.kind
                        && is_extracted(&ii.meta, body)
                    {
                        f(ii.ident, params, body)
                    }
                }
            }
            _ => {}
        }
    }
}

/// The functions the model extracts, by name.
fn extracted_fns(items: &[Item]) -> HashSet<GlobalId> {
    fn_decls(items)
        .filter(|d| is_extracted(d.meta, d.body))
        .map(|d| d.name)
        .collect()
}

/// Which types may hold a `Result`.
#[derive(Default)]
struct ResultTypes {
    /// The crate's types with a field that may hold a `Result`.
    holding: HashSet<GlobalId>,
    /// Each variant of the crate's types → its fields.
    variants: HashMap<GlobalId, Vec<(GlobalId, Ty)>>,
    /// Each field of the crate's types → the variant it belongs to.
    field_variant: HashMap<GlobalId, GlobalId>,
}

impl ResultTypes {
    fn new(items: &[Item]) -> Self {
        let mut types = Self::default();
        let mut type_fields: Vec<(GlobalId, Vec<Ty>)> = vec![];
        for item in items {
            if let ItemKind::Type { name, variants, .. } = &item.kind {
                let mut tys = vec![];
                for variant in variants {
                    let fields: Vec<(GlobalId, Ty)> = variant
                        .arguments
                        .iter()
                        .map(|(field, ty, _)| (*field, ty.clone()))
                        .collect();
                    for (field, ty) in &fields {
                        types.field_variant.insert(*field, variant.name);
                        tys.push(ty.clone());
                    }
                    types.variants.insert(variant.name, fields);
                }
                type_fields.push((*name, tys));
            }
        }
        loop {
            let new: Vec<GlobalId> = type_fields
                .iter()
                .filter(|(name, tys)| {
                    !types.holding.contains(name) && tys.iter().any(|ty| types.holds(ty, true))
                })
                .map(|(name, _)| *name)
                .collect();
            if new.is_empty() {
                return types;
            }
            types.holding.extend(new);
        }
    }

    /// Whether a value of type `ty` may hold a `Result`; with `arrows`, also
    /// a closure that may return one.
    fn holds(&self, ty: &Ty, arrows: bool) -> bool {
        match ty.kind() {
            TyKind::App { head, args } => {
                *head == crate::names::core::result::Result
                    || self.holding.contains(head)
                    || args
                        .iter()
                        .any(|a| matches!(a, GenericValue::Ty(t) if self.holds(t, arrows)))
            }
            TyKind::Ref { inner, .. } | TyKind::Slice(inner) => self.holds(inner, arrows),
            TyKind::Array { ty, .. } => self.holds(ty, arrows),
            TyKind::Arrow { output, .. } => arrows && self.holds(output, arrows),
            _ => false,
        }
    }

    /// Whether a value of type `ty` (the type of the matched value) has a
    /// field that `fields` leaves out, as `..` does, and that may hold a
    /// `Result`.
    fn omits_result(&self, constructor: GlobalId, fields: &[(GlobalId, Pat)], ty: &Ty) -> bool {
        let Some(all) = self.variants.get(&constructor) else {
            return false;
        };
        let generic_holds = matches!(ty.kind(), TyKind::App { args, .. }
            if args.iter().any(|a| matches!(a, GenericValue::Ty(t) if self.holds(t, false))));
        all.iter().any(|(field, field_ty)| {
            !fields.iter().any(|(f, _)| f == field)
                && (self.holds(field_ty, false) || generic_holds)
        })
    }

    /// Whether projecting `projector` out of a value of type `ty` drops a
    /// part that may hold a `Result`.
    fn projection_drops_result(&self, projector: GlobalId, ty: &Ty) -> bool {
        let TyKind::App { args, .. } = ty.kind() else {
            return true;
        };
        let holding_arg = |skip: Option<usize>| {
            args.iter().enumerate().any(|(i, a)| {
                Some(i) != skip && matches!(a, GenericValue::Ty(t) if self.holds(t, true))
            })
        };
        if let Some(TupleId::Field { field, .. }) = projector.expect_tuple() {
            return holding_arg(Some(field));
        }
        // A field whose type is a type parameter holds what the arguments do.
        let others = self
            .field_variant
            .get(&projector)
            .and_then(|v| self.variants.get(v));
        match others {
            Some(fields) => {
                holding_arg(None)
                    || fields
                        .iter()
                        .any(|(f, t)| *f != projector && self.holds(t, true))
            }
            None => true,
        }
    }
}

/// Replaces `e` by an error node.
fn reject_expr(e: &mut Expr, context: &str, details: String) {
    let kind = DiagnosticInfoKind::Unimplemented {
        issue_id: None,
        details: Some(details),
    };
    let info = DiagnosticInfo {
        context: Context::Phase(context.into()),
        span: e.meta.span,
        kind,
    };
    *e.kind = ExprKind::Error(ErrorNode {
        fragment: Box::new(e.clone().into()),
        diagnostics: vec![Diagnostic::new(e.clone(), info)],
    });
}

/// Reports a rejection of the pattern `p`.
fn pat_diagnostic(p: &Pat, context: &str, details: String) -> Diagnostic {
    let kind = DiagnosticInfoKind::Unimplemented {
        issue_id: None,
        details: Some(details),
    };
    let info = DiagnosticInfo {
        context: Context::Phase(context.into()),
        span: p.meta.span,
        kind,
    };
    Diagnostic::new(p.clone(), info)
}

/// Replaces `scope`, which holds rejected patterns, by an error node carrying
/// their `diagnostics`.
fn reject_scope(scope: &mut Expr, diagnostics: Vec<Diagnostic>) {
    if !diagnostics.is_empty() {
        *scope.kind = ExprKind::Error(ErrorNode {
            fragment: Box::new(scope.clone().into()),
            diagnostics,
        });
    }
}

/// The locals that `pat` binds.
fn binders(pat: &Pat) -> Vec<LocalId> {
    struct Finder(Vec<LocalId>);
    impl AstVisitor for Finder {
        fn enter_pat_kind(&mut self, kind: &PatKind) {
            if let PatKind::Binding { var, .. } = kind {
                self.0.push(var.clone());
            }
        }
    }
    let mut finder = Finder(vec![]);
    finder.visit_pat(pat);
    finder.0
}

/// Whether `pat` matches every value.
fn irrefutable(pat: &Pat) -> bool {
    match &*pat.kind {
        PatKind::Wild => true,
        PatKind::Binding { sub_pat, .. } => sub_pat.as_ref().is_none_or(irrefutable),
        PatKind::Ascription { pat, .. } | PatKind::Deref { sub_pat: pat } => irrefutable(pat),
        PatKind::Or { sub_pats } => sub_pats.iter().any(irrefutable),
        _ => false,
    }
}

/// Whether every path through `e` yields a value holding an `Err`, or
/// diverges.
fn err_certain(e: &Expr) -> bool {
    match &*e.kind {
        ExprKind::Construct {
            constructor,
            fields,
            base,
            ..
        } => {
            *constructor == Err::Constructor
                || fields.iter().any(|(_, e)| err_certain(e))
                || base.as_ref().is_some_and(err_certain)
        }
        ExprKind::App { head, .. } => {
            matches!(&*head.kind, ExprKind::GlobalId(g) if *g == never_to_any)
        }
        ExprKind::Let { body, .. } => err_certain(body),
        ExprKind::If {
            then,
            else_: Some(else_),
            ..
        } => err_certain(then) && err_certain(else_),
        ExprKind::Match { arms, .. } => {
            !arms.is_empty() && arms.iter().all(|a| err_certain(&a.body))
        }
        ExprKind::Array(es) => es.iter().any(err_certain),
        ExprKind::Block { body: e, .. } | ExprKind::Ascription { e, .. } => err_certain(e),
        _ => false,
    }
}

/// Which calls the model may observe. ProVerif terms may emit events, so a
/// hand-written model may be observable, and so is any function calling one.
struct Effects {
    extracted: HashSet<GlobalId>,
    observable: HashSet<GlobalId>,
}

impl Effects {
    fn new(items: &[Item], extracted: HashSet<GlobalId>) -> Self {
        let mut effects = Self {
            extracted,
            observable: HashSet::new(),
        };
        loop {
            let new: Vec<GlobalId> = fn_decls(items)
                .filter(|d| {
                    effects.extracted.contains(&d.name)
                        && !effects.observable.contains(&d.name)
                        && effects.has_effect(d.body)
                })
                .map(|d| d.name)
                .collect();
            if new.is_empty() {
                return effects;
            }
            effects.observable.extend(new);
        }
    }

    /// Whether calling `head` may be observable.
    fn call_is_observable(&self, head: &Expr) -> bool {
        match &*head.kind {
            ExprKind::GlobalId(g)
                if *g == never_to_any || g.is_projector() || g.is_constructor() =>
            {
                false
            }
            ExprKind::GlobalId(g) if self.extracted.contains(g) => self.observable.contains(g),
            ExprKind::GlobalId(g) => {
                let rendered = render_id(g);
                !LIBRARY_PREFIXES.iter().any(|p| rendered.starts_with(p))
            }
            _ => true,
        }
    }

    /// Whether evaluating `e` may be observable.
    fn has_effect(&self, e: &Expr) -> bool {
        struct Finder<'a>(&'a Effects, bool);
        impl AstVisitor for Finder<'_> {
            fn enter_expr_kind(&mut self, kind: &ExprKind) {
                self.1 |= match kind {
                    ExprKind::App { head, .. } => self.0.call_is_observable(head),
                    ExprKind::Quote { .. } => true,
                    _ => false,
                };
            }
        }
        let mut finder = Finder(self, false);
        finder.visit_expr(e);
        finder.1
    }
}

/// How the evaluation of an expression relates to a local, in evaluation
/// order.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Use {
    /// Every path uses it before anything observable.
    First,
    /// No path uses it or does anything observable.
    Neither,
    /// Some path does something observable before using it, or never uses it.
    Late,
}

impl Use {
    /// `self`, then `next`.
    fn then(self, next: impl FnOnce() -> Use) -> Use {
        match self {
            Use::Neither => next(),
            _ => self,
        }
    }

    /// One of several paths.
    fn join(paths: impl IntoIterator<Item = Use>) -> Use {
        let mut out = None;
        for u in paths {
            out = match (out, u) {
                (None, u) => Some(u),
                (Some(a), b) if a == b => Some(a),
                _ => Some(Use::Late),
            };
        }
        out.unwrap_or(Use::Neither)
    }
}

const RECOVERY_CONTEXT: &str = "ProverifOkOnlyResults";

/// What the default Result model cannot express, for the error messages.
const RECOVERY_HINT: &str = "the default Result model ends the trace on every `Err`, so it \
     cannot express code that goes on after one, or does anything observable before returning \
     it; propagate the error at once, or pass `--result-errors`";

/// Rejects, under the default Result model, the code that may go on after an
/// `Err`, or do something observable while one is pending: every value that
/// may hold a `Result` must be propagated, by being returned (possibly inside
/// a tuple or a struct), passed to a function the model extracts (which
/// checks its parameter in turn), or matched by patterns whose arms that may
/// see an `Err` return one or diverge, with nothing observable on the way.
struct Recovery {
    types: ResultTypes,
    effects: Effects,
}

impl Recovery {
    /// How `e` uses `x` (see [`Use`]).
    fn first_use(&self, x: &LocalId, e: &Expr) -> Use {
        let seq = |es: &[&Expr], last: Use| {
            es.iter()
                .fold(Use::Neither, |acc, e| acc.then(|| self.first_use(x, e)))
                .then(|| last)
        };
        let scoped = |pat: &Pat, guard: Option<&Expr>, body: &Expr| {
            if binders(pat).contains(x) {
                return Use::Late;
            }
            guard
                .map_or(Use::Neither, |g| self.first_use(x, g))
                .then(|| self.first_use(x, body))
        };
        let effect = |observable: bool| if observable { Use::Late } else { Use::Neither };
        match &*e.kind {
            ExprKind::LocalId(y) if y == x => Use::First,
            ExprKind::Let { lhs, rhs, body } => {
                self.first_use(x, rhs).then(|| scoped(lhs, None, body))
            }
            ExprKind::If {
                condition,
                then,
                else_,
            } => self.first_use(x, condition).then(|| {
                Use::join([
                    self.first_use(x, then),
                    else_
                        .as_ref()
                        .map_or(Use::Neither, |e| self.first_use(x, e)),
                ])
            }),
            ExprKind::Match { scrutinee, arms } => self.first_use(x, scrutinee).then(|| {
                Use::join(arms.iter().map(|arm| {
                    let guard = arm.guard.as_ref().map(|g| {
                        let GuardKind::IfLet { rhs, .. } = &g.kind;
                        rhs
                    });
                    scoped(&arm.pat, guard, &arm.body)
                }))
            }),
            ExprKind::App { head, args, .. } => {
                let mut es: Vec<&Expr> = vec![head];
                es.extend(args);
                seq(&es, effect(self.effects.call_is_observable(head)))
            }
            ExprKind::Construct { fields, base, .. } => {
                let mut es: Vec<&Expr> = fields.iter().map(|(_, e)| e).collect();
                es.extend(base);
                seq(&es, Use::Neither)
            }
            ExprKind::Array(es) => seq(&es.iter().collect::<Vec<_>>(), Use::Neither),
            ExprKind::Block { body: e, .. }
            | ExprKind::Ascription { e, .. }
            | ExprKind::Borrow { inner: e, .. }
            | ExprKind::AddressOf { inner: e, .. }
            | ExprKind::Return { value: e } => self.first_use(x, e),
            ExprKind::Closure { captures, .. } => {
                seq(&captures.iter().collect::<Vec<_>>(), Use::Neither)
            }
            ExprKind::LocalId(_) | ExprKind::GlobalId(_) | ExprKind::Literal(_) => Use::Neither,
            _ => Use::Late,
        }
    }

    /// Rejects the observable parts of `parts`, evaluated in this order, that
    /// run after a part that may be an `Err`.
    fn effects_after_results(&self, parts: Vec<&mut Expr>) {
        let mut pending = false;
        for part in parts {
            if pending && self.effects.has_effect(part) {
                reject_expr(
                    part,
                    RECOVERY_CONTEXT,
                    format!(
                        "this may be observable, and runs after a `Result` that may be an `Err` \
                         is computed: {RECOVERY_HINT}"
                    ),
                );
            }
            pending |= self.types.holds(&part.ty, false);
        }
    }

    /// Checks `e`, whose value is propagated when `carried`.
    fn expr(&self, e: &mut Expr, carried: bool) {
        let holds = self.types.holds(&e.ty, true);
        let mut rejected = vec![];
        match &mut *e.kind {
            ExprKind::Let { lhs, rhs, body } => {
                self.pat(lhs, None, body, &mut rejected);
                self.expr(rhs, true);
                self.expr(body, carried);
            }
            ExprKind::Match { scrutinee, arms } => {
                self.expr(scrutinee, true);
                for arm in arms {
                    let guard = arm.guard.as_ref().map(|g| {
                        let GuardKind::IfLet { rhs, .. } = &g.kind;
                        rhs
                    });
                    self.pat(&arm.pat, guard, &arm.body, &mut rejected);
                    if let Some(guard) = &mut arm.guard {
                        let GuardKind::IfLet { lhs, rhs } = &mut guard.kind;
                        self.pat(lhs, None, &arm.body, &mut rejected);
                        self.expr(rhs, true);
                    }
                    self.expr(&mut arm.body, carried);
                }
            }
            ExprKind::If {
                condition,
                then,
                else_,
            } => {
                self.expr(condition, false);
                self.expr(then, carried);
                if let Some(else_) = else_ {
                    self.expr(else_, carried);
                }
            }
            ExprKind::Block { body: inner, .. }
            | ExprKind::Ascription { e: inner, .. }
            | ExprKind::Borrow { inner, .. }
            | ExprKind::AddressOf { inner, .. } => self.expr(inner, carried),
            ExprKind::Construct { fields, base, .. } => {
                for (_, field) in fields.iter_mut() {
                    self.expr(field, carried);
                }
                if let Some(base) = base {
                    self.expr(base, false);
                }
                self.effects_after_results(fields.iter_mut().map(|(_, f)| f).collect());
            }
            ExprKind::Array(es) => {
                for e in es.iter_mut() {
                    self.expr(e, carried);
                }
                self.effects_after_results(es.iter_mut().collect());
            }
            ExprKind::App {
                head,
                args,
                generic_args,
                ..
            } => {
                let callee = match &*head.kind {
                    ExprKind::GlobalId(g) => Some(*g),
                    _ => None,
                };
                let generics_hold = generic_args
                    .iter()
                    .any(|g| matches!(g, GenericValue::Ty(t) if self.types.holds(t, true)));
                let rendered = callee.map(|g| render_id(&g)).unwrap_or_default();
                let arg_carried = |i: usize, arg: &Expr| match callee {
                    Some(g) if g == never_to_any => true,
                    Some(g) if g.is_projector() => {
                        carried && !self.types.projection_drops_result(g, &arg.ty)
                    }
                    Some(g) if self.effects.extracted.contains(&g) => !generics_hold,
                    _ if i == 0 && PANIC_ON_ERR.contains(&rendered.as_str()) => true,
                    _ if i == 0 && KEEP_ERR.contains(&rendered.as_str()) => carried,
                    _ => false,
                };
                let carried_args: Vec<bool> = args
                    .iter()
                    .enumerate()
                    .map(|(i, a)| arg_carried(i, a))
                    .collect();
                self.expr(head, true);
                for (arg, carried) in args.iter_mut().zip(carried_args) {
                    self.expr(arg, carried);
                }
                self.effects_after_results(args.iter_mut().collect());
            }
            ExprKind::Closure {
                params,
                body,
                captures,
            } => {
                for param in params.iter() {
                    self.pat(param, None, body, &mut rejected);
                }
                self.expr(body, true);
                for capture in captures {
                    self.expr(capture, true);
                }
            }
            ExprKind::Loop {
                body, kind, state, ..
            } => {
                match &mut **kind {
                    LoopKind::WhileLoop { condition } => self.expr(condition, false),
                    LoopKind::ForLoop { pat, iterator } => {
                        self.pat(pat, None, body, &mut rejected);
                        self.expr(iterator, false);
                    }
                    LoopKind::ForIndexLoop { start, end, .. } => {
                        self.expr(start, false);
                        self.expr(end, false);
                    }
                    _ => {}
                }
                if let Some(state) = state {
                    self.pat(&state.body_pat, None, body, &mut rejected);
                    self.expr(&mut state.init, carried);
                }
                self.expr(body, carried);
            }
            ExprKind::Return { value } => self.expr(value, true),
            ExprKind::Break { value, state, .. } => {
                self.expr(value, carried);
                if let Some(state) = state {
                    self.expr(state, carried);
                }
            }
            ExprKind::Continue {
                state: Some(state), ..
            } => self.expr(state, carried),
            ExprKind::Assign { value, .. } => self.expr(value, false),
            ExprKind::Quote { contents } => {
                for c in &mut contents.0 {
                    if let QuoteContent::Expr(e) = c {
                        self.expr(e, true);
                    }
                }
            }
            ExprKind::Error(_) => return,
            _ => {}
        }
        if !rejected.is_empty() {
            reject_scope(e, rejected);
        } else if holds && !carried {
            reject_expr(
                e,
                RECOVERY_CONTEXT,
                format!("a `Result` used here is not propagated: {RECOVERY_HINT}"),
            );
        }
    }

    /// Checks a part of a pattern that only matches, or discards, an `Err`:
    /// the code it guards runs in Rust but not in the model.
    fn err_path(
        &self,
        pat: &Pat,
        guard: Option<&Expr>,
        cont: &Expr,
        what: &str,
        rejected: &mut Vec<Diagnostic>,
    ) {
        let observable =
            guard.is_some_and(|g| self.effects.has_effect(g)) || self.effects.has_effect(cont);
        let reason = if !err_certain(cont) {
            format!("{what}, and goes on without returning one")
        } else if observable {
            format!("{what}, and may do something observable before returning one")
        } else {
            return;
        };
        rejected.push(pat_diagnostic(
            pat,
            RECOVERY_CONTEXT,
            format!("{reason}: {RECOVERY_HINT}"),
        ))
    }

    /// Checks `pat`, whose bindings scope over `guard` then `cont`, and adds
    /// the rejections to `rejected`.
    fn pat(&self, pat: &Pat, guard: Option<&Expr>, cont: &Expr, rejected: &mut Vec<Diagnostic>) {
        if !self.types.holds(&pat.ty, false) {
            return;
        }
        match &*pat.kind {
            PatKind::Wild => self.err_path(
                pat,
                guard,
                cont,
                "this pattern discards a `Result`, which may be an `Err`",
                rejected,
            ),
            PatKind::Binding { var, sub_pat, .. } => {
                let first = guard
                    .map_or(Use::Neither, |g| self.first_use(var, g))
                    .then(|| self.first_use(var, cont));
                if first != Use::First {
                    rejected.push(pat_diagnostic(
                        pat,
                        RECOVERY_CONTEXT,
                        format!(
                            "the `Result` bound here is not used on every path before anything \
                             observable: {RECOVERY_HINT}"
                        ),
                    ))
                }
                if let Some(sub_pat) = sub_pat {
                    self.pat(sub_pat, guard, cont, rejected);
                }
            }
            PatKind::Construct { constructor, .. } if *constructor == Err::Constructor => {
                self.err_path(pat, guard, cont, "this arm handles an `Err`", rejected)
            }
            PatKind::Construct {
                constructor,
                fields,
                ..
            } => {
                // `..` may be filled with wildcards, which have no span of
                // their own.
                let discards = fields.iter().any(|(_, p)| {
                    matches!(*p.kind, PatKind::Wild) && self.types.holds(&p.ty, false)
                });
                if discards || self.types.omits_result(*constructor, fields, &pat.ty) {
                    self.err_path(
                        pat,
                        guard,
                        cont,
                        "this pattern discards a `Result` field, which may be an `Err`",
                        rejected,
                    );
                }
                for (_, field) in fields {
                    if !matches!(*field.kind, PatKind::Wild) {
                        self.pat(field, guard, cont, rejected);
                    }
                }
            }
            PatKind::Or { sub_pats: pats } | PatKind::Array { args: pats } => {
                for p in pats {
                    self.pat(p, guard, cont, rejected);
                }
            }
            PatKind::Ascription { pat: inner, .. } | PatKind::Deref { sub_pat: inner } => {
                self.pat(inner, guard, cont, rejected)
            }
            _ => {}
        }
    }
}

/// Rejects, under the default Result model, the code that may go on after an
/// `Err` (see [`Recovery`]).
pub(crate) fn reject_recovery(items: &mut [Item]) {
    let recovery = Recovery {
        types: ResultTypes::new(items),
        effects: Effects::new(items, extracted_fns(items)),
    };
    for_each_extracted_fn(items, |_, params, body| {
        let mut rejected = vec![];
        for param in params.iter() {
            recovery.pat(&param.pat, None, body, &mut rejected);
        }
        recovery.expr(body, true);
        reject_scope(body, rejected);
    });
}

/// Phase rejecting, under `--result-errors`, what would read the error term of
/// a hand-written model as a Rust error value (see the module documentation).
#[derive(Default, Debug)]
pub struct ProverifRejectUnknownErrors;

const UNKNOWN_CONTEXT: &str = "ProverifRejectUnknownErrors";

/// Why an unknown error must not be inspected, for the error messages.
const UNKNOWN_HINT: &str = "under `--result-errors`, the `Err` of a function whose body is not \
     extracted (opaque, `replace_body`, `pv_inverse_of` or external) is a term of its \
     hand-written model, not the Rust error value; only propagate it, or match it with `Err(_)`";

/// How a value relates to the unknown errors, from the least to the most
/// tainted.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
enum Taint {
    /// Faithful.
    #[default]
    Clean,
    /// A value holding `Result`s whose `Err` may be unknown; the rest of it is
    /// faithful.
    Results,
    /// A value that may hold an unknown error anywhere.
    Error,
}

impl Taint {
    /// The taint of `Err(e)`, where `e` has taint `self`: the unknown part is
    /// inside the `Err`.
    fn in_err(self) -> Taint {
        self.min(Taint::Results)
    }
}

/// Taint analysis of the unknown errors (see [`Taint`]), to a fixpoint over
/// the crate. A tainted `Result` may only be matched with an irrefutable
/// pattern inside `Err`; a value that may hold an unknown error may not be
/// matched with a refutable pattern, branched on or passed to a function
/// whose body is not extracted.
struct UnknownErrors<'a> {
    types: &'a ResultTypes,
    extracted: &'a HashSet<GlobalId>,
    /// The taint of the result of each extracted function.
    fns: HashMap<GlobalId, Taint>,
    /// The taint each parameter, by function and position, may receive.
    params: HashMap<(GlobalId, usize), Taint>,
    /// The taint of each local, by function.
    locals: HashMap<(GlobalId, LocalId), Taint>,
    /// The function being analysed.
    current: Option<GlobalId>,
    /// Whether to reject, or only to compute the taints.
    reject: bool,
    /// Whether a taint grew, during the fixpoint.
    changed: bool,
}

impl UnknownErrors<'_> {
    /// Raises the taint of `key` in `map` to `taint`.
    fn raise<K: std::hash::Hash + Eq>(
        map: &mut HashMap<K, Taint>,
        key: K,
        taint: Taint,
        changed: &mut bool,
    ) {
        let old = map.entry(key).or_default();
        if taint > *old {
            *old = taint;
            *changed = true;
        }
    }

    fn local(&self, x: &LocalId) -> Taint {
        let key = (self.current.unwrap(), x.clone());
        self.locals.get(&key).copied().unwrap_or_default()
    }

    fn reject_expr(&self, e: &mut Expr, what: &str) {
        if self.reject {
            reject_expr(e, UNKNOWN_CONTEXT, format!("{what}: {UNKNOWN_HINT}"))
        }
    }

    /// Binds the locals of `pat`, matched against a value of taint `taint`,
    /// and adds the rejections to `rejected`.
    fn pat(&mut self, pat: &Pat, taint: Taint, rejected: &mut Vec<Diagnostic>) {
        // The parts of a tainted `Result` that hold no `Result` are faithful.
        if taint == Taint::Clean || (taint == Taint::Results && !self.types.holds(&pat.ty, false)) {
            return;
        }
        let refutable_err = matches!(&*pat.kind, PatKind::Construct { constructor, fields, .. }
            if *constructor == Err::Constructor && !fields.iter().all(|(_, p)| irrefutable(p)));
        match &*pat.kind {
            PatKind::Wild => {}
            PatKind::Binding { var, sub_pat, .. } => {
                let key = (self.current.unwrap(), var.clone());
                Self::raise(&mut self.locals, key, taint, &mut self.changed);
                if let Some(sub_pat) = sub_pat {
                    self.pat(sub_pat, taint, rejected);
                }
            }
            PatKind::Ascription { pat: inner, .. } | PatKind::Deref { sub_pat: inner } => {
                self.pat(inner, taint, rejected)
            }
            PatKind::Or { sub_pats } => {
                for p in sub_pats {
                    self.pat(p, taint, rejected);
                }
            }
            _ if taint == Taint::Error || refutable_err => {
                if self.reject {
                    rejected.push(pat_diagnostic(
                        pat,
                        UNKNOWN_CONTEXT,
                        format!("this pattern inspects an error value: {UNKNOWN_HINT}"),
                    ))
                }
            }
            PatKind::Construct {
                constructor,
                fields,
                ..
            } => {
                let taint = if *constructor == Err::Constructor {
                    Taint::Error
                } else {
                    Taint::Results
                };
                for (_, field) in fields {
                    self.pat(field, taint, rejected);
                }
            }
            PatKind::Array { args } => {
                for p in args {
                    self.pat(p, taint, rejected);
                }
            }
            _ => {}
        }
    }

    /// Checks a condition, which must not depend on an unknown error.
    fn condition(&mut self, c: &mut Expr) {
        if self.expr(c) != Taint::Clean {
            self.reject_expr(c, "this condition depends on an error value");
        }
    }

    /// The taint of a call of `head` on arguments of taints `args`, of type
    /// `ty`; `Err` with a reason when the call must be rejected.
    fn call(&mut self, head: &Expr, args: &[Taint], ty: &Ty) -> Result<Taint, &'static str> {
        let all = args.iter().copied().max().unwrap_or_default();
        let ExprKind::GlobalId(g) = &*head.kind else {
            return Ok(all);
        };
        if self.extracted.contains(g) {
            for (i, t) in args.iter().enumerate() {
                Self::raise(&mut self.params, (*g, i), *t, &mut self.changed);
            }
            return Ok(self.fns.get(g).copied().unwrap_or_default());
        }
        if *g == never_to_any {
            return Ok(Taint::Clean);
        }
        let first = args.first().copied().unwrap_or_default();
        if g.is_projector() {
            return Ok(match first {
                Taint::Results if !self.types.holds(ty, false) => Taint::Clean,
                t => t,
            });
        }
        let rendered = render_id(g);
        let r = rendered.as_str();
        Ok(
            if IDENTITY.contains(&r)
                || (KEEP_ERR.contains(&r) && r != "core__result__Impl__map_err")
            {
                all
            } else if r == "core__result__Impl__map_err" {
                // The error is mapped through an opaque closure.
                all.max(Taint::Results)
            } else if r == "core__option__Impl__ok_or" {
                all.in_err()
            } else if READ_ERR.contains(&r) {
                if first == Taint::Clean {
                    Taint::Clean
                } else {
                    Taint::Error
                }
            } else if READ_OK.contains(&r) {
                if all == Taint::Error {
                    Taint::Error
                } else {
                    Taint::Clean
                }
            } else if all != Taint::Clean {
                return Err(
                    "a `Result` or an error coming from a hand-written model is passed to a \
                 function whose body is not extracted",
                );
            } else if self.types.holds(ty, false) {
                // The error a model returns is unknown.
                Taint::Results
            } else {
                Taint::Clean
            },
        )
    }

    /// The taint of the value of `e`.
    fn expr(&mut self, e: &mut Expr) -> Taint {
        let mut reject_here = None;
        let mut rejected = vec![];
        let taint = match &mut *e.kind {
            ExprKind::LocalId(x) => self.local(x),
            ExprKind::Let { lhs, rhs, body } => {
                let t = self.expr(rhs);
                self.pat(lhs, t, &mut rejected);
                self.expr(body)
            }
            ExprKind::Match { scrutinee, arms } => {
                let t = self.expr(scrutinee);
                let mut out = Taint::Clean;
                for arm in arms {
                    self.pat(&arm.pat, t, &mut rejected);
                    if let Some(guard) = &mut arm.guard {
                        let GuardKind::IfLet { lhs, rhs } = &mut guard.kind;
                        let t = self.expr(rhs);
                        self.pat(lhs, t, &mut rejected);
                    }
                    out = out.max(self.expr(&mut arm.body));
                }
                out
            }
            ExprKind::If {
                condition,
                then,
                else_,
            } => {
                self.condition(condition);
                let t = self.expr(then);
                else_.as_mut().map_or(Taint::Clean, |e| self.expr(e)).max(t)
            }
            ExprKind::App { head, args, .. } => {
                self.expr(head);
                let taints: Vec<Taint> = args.iter_mut().map(|a| self.expr(a)).collect();
                match self.call(head, &taints, &e.ty) {
                    Ok(t) => t,
                    // The rejected call becomes an error node.
                    Err(what) => {
                        reject_here = Some(what);
                        Taint::Clean
                    }
                }
            }
            ExprKind::Construct {
                constructor,
                fields,
                base,
                ..
            } => {
                let mut out = base.as_mut().map_or(Taint::Clean, |b| self.expr(b));
                for (_, f) in fields {
                    out = out.max(self.expr(f));
                }
                if *constructor == Err::Constructor {
                    out.in_err()
                } else {
                    out
                }
            }
            ExprKind::Array(es) => es
                .iter_mut()
                .fold(Taint::Clean, |acc, e| acc.max(self.expr(e))),
            ExprKind::Block { body: inner, .. }
            | ExprKind::Ascription { e: inner, .. }
            | ExprKind::Borrow { inner, .. }
            | ExprKind::AddressOf { inner, .. }
            | ExprKind::Return { value: inner } => self.expr(inner),
            ExprKind::Closure { body, captures, .. } => {
                let t = captures
                    .iter_mut()
                    .fold(Taint::Clean, |acc, c| acc.max(self.expr(c)));
                self.expr(body).max(t)
            }
            ExprKind::Loop {
                body, kind, state, ..
            } => {
                match &mut **kind {
                    LoopKind::WhileLoop { condition } => self.condition(condition),
                    LoopKind::ForLoop { pat, iterator } => {
                        let t = self.expr(iterator);
                        self.pat(pat, t, &mut rejected);
                    }
                    LoopKind::ForIndexLoop { start, end, .. } => {
                        self.expr(start);
                        self.expr(end);
                    }
                    _ => {}
                }
                match state {
                    Some(state) => {
                        // A later iteration starts from what this one returns:
                        // the state keeps the taint of earlier passes of the
                        // fixpoint.
                        let init = self.expr(&mut state.init);
                        let earlier = binders(&state.body_pat)
                            .iter()
                            .map(|x| self.local(x))
                            .max()
                            .unwrap_or_default();
                        self.pat(&state.body_pat, init.max(earlier), &mut rejected);
                        let out = self.expr(body);
                        let reject = std::mem::replace(&mut self.reject, false);
                        self.pat(&state.body_pat, out, &mut vec![]);
                        self.reject = reject;
                        init.max(out)
                    }
                    None => self.expr(body),
                }
            }
            ExprKind::Break { value, state, .. } => {
                let t = state.as_mut().map_or(Taint::Clean, |s| self.expr(s));
                self.expr(value).max(t)
            }
            ExprKind::Continue {
                state: Some(state), ..
            } => self.expr(state),
            ExprKind::Assign { value, .. } => self.expr(value),
            ExprKind::Quote { contents } => {
                for c in &mut contents.0 {
                    if let QuoteContent::Expr(e) = c {
                        self.expr(e);
                    }
                }
                if self.types.holds(&e.ty, false) {
                    Taint::Results
                } else {
                    Taint::Clean
                }
            }
            _ => Taint::Clean,
        };
        if !rejected.is_empty() {
            reject_scope(e, rejected);
        } else if let Some(what) = reject_here {
            self.reject_expr(e, what);
        }
        taint
    }

    /// Analyses the function `name`, and records the taint of its result.
    fn function(&mut self, name: GlobalId, params: &mut [Param], body: &mut Expr) {
        self.current = Some(name);
        let mut rejected = vec![];
        for (i, param) in params.iter().enumerate() {
            let t = self.params.get(&(name, i)).copied().unwrap_or_default();
            self.pat(&param.pat, t, &mut rejected);
        }
        let out = self.expr(body);
        Self::raise(&mut self.fns, name, out, &mut self.changed);
        reject_scope(body, rejected);
    }
}

/// The names that the verbatim ProVerif text of `items`, or the libraries a
/// hand-written model may use, declare as a `fun`, a `reduc` or a `const`,
/// whose application is a bare term.
fn bare_term_names(items: &[Item]) -> HashSet<String> {
    let quotes = items.iter().filter_map(|item| match &item.kind {
        ItemKind::Quote { quote, .. } => Some(quote_text(quote)),
        _ => None,
    });
    quotes
        .chain(MODEL_LIBRARIES.iter().map(|lib| lib.to_string()))
        .flat_map(|text| scan_declarations(&text))
        .filter(|(kw, _)| *kw != "letfun")
        .map(|(_, name)| name)
        .collect()
}

/// The functions returning a `Result` whose model is a bare term: opaque and
/// `pv_constructor` functions, which render as a `fun`, and verbatim bodies
/// applying a bare term.
fn bare_result_fns(items: &[Item], types: &ResultTypes) -> HashSet<String> {
    let bare_names = bare_term_names(items);
    let head = |quote: &Quote| {
        let text = quote_text(quote);
        let text = text.trim_start();
        let end = text
            .find(|c: char| !(c.is_alphanumeric() || c == '_'))
            .unwrap_or(text.len());
        text[..end].to_string()
    };
    let mut out: HashSet<String> = bare_names.clone();
    for d in fn_decls(items) {
        if !types.holds(&d.body.ty, false) {
            continue;
        }
        let opaque = d
            .meta
            .hax_attributes()
            .any(|a| matches!(a, AttrPayload::Erased | AttrPayload::PVConstructor));
        let applies_bare = quoted_body(d.body).is_some_and(|q| bare_names.contains(&head(q)));
        if opaque || applies_bare {
            out.insert(render_id(&d.name));
        }
    }
    out
}

/// Rejects the calls to a function returning a `Result` that the model
/// renders as a bare term: neither `Ok(..)` nor `Err(..)` matches it, so
/// both paths would be lost.
#[setup_error_handling_struct]
#[derive(Default)]
struct BareResults {
    types: ResultTypes,
    bare: HashSet<String>,
}

impl VisitorWithContext for BareResults {
    fn context(&self) -> Context {
        Context::Phase(UNKNOWN_CONTEXT.to_string())
    }
}

impl AstVisitorMut for BareResults {
    setup_error_handling_impl!();

    fn visit_expr(&mut self, expr: &mut Expr) {
        self.visit_inner(expr);
        let name = match &*expr.kind {
            ExprKind::App { head, .. } => match &*head.kind {
                ExprKind::GlobalId(g) => Some(*g),
                _ => None,
            },
            // A constant may be referred to unapplied.
            ExprKind::GlobalId(g) if *g != crate::names::rust_primitives::hax::dropped_body => {
                Some(*g)
            }
            _ => None,
        };
        if let Some(g) = &name
            && self.types.holds(&expr.ty, false)
            && self.bare.contains(&render_id(g))
        {
            self.error(
                expr.clone(),
                DiagnosticInfoKind::Unimplemented {
                    issue_id: None,
                    details: Some(format!(
                        "`{}` returns a `Result`, but its model is a `fun`, a `reduc` or a \
                         `const`, whose value is neither `Ok(..)` nor `Err(..)` under \
                         `--result-errors`; model it with a `letfun` returning \
                         `hax_result_ok(..)` or `hax_result_err(..)`",
                        render_id(g)
                    )),
                },
            )
        }
    }
}

impl Phase for ProverifRejectUnknownErrors {
    fn apply(&self, items: &mut Vec<Item>) {
        let types = ResultTypes::new(items);
        let extracted = extracted_fns(items);
        let bare = bare_result_fns(items, &types);

        let mut analysis = UnknownErrors {
            types: &types,
            extracted: &extracted,
            fns: HashMap::new(),
            params: HashMap::new(),
            locals: HashMap::new(),
            current: None,
            reject: false,
            changed: true,
        };
        // The taints only grow, so this terminates. Without rejecting, the
        // analysis leaves the items unchanged.
        while analysis.changed {
            analysis.changed = false;
            for_each_extracted_fn(items, |name, params, body| {
                analysis.function(name, params, body)
            });
        }
        analysis.reject = true;
        for_each_extracted_fn(items, |name, params, body| {
            analysis.function(name, params, body)
        });

        BareResults {
            types,
            bare,
            ..Default::default()
        }
        .visit(items);
    }
}
