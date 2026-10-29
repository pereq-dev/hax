//! ProVerif-backend checks that keep each `Result` model sound.
//!
//! The default model ends the trace on every `Err`, so it is only sound for
//! code that never goes on after one: [`reject_recovery`] rejects every place
//! where a value that may hold a `Result` is not propagated.
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
use crate::backends::proverif::analysis::{body_is_hidden, quote_text, scan_declarations};
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

/// Whether the model extracts the function with these `meta` and `body`.
fn is_extracted(meta: &Metadata, body: &Expr) -> bool {
    fn is_quote(e: &Expr) -> bool {
        match &*e.kind {
            ExprKind::Quote { .. } => true,
            ExprKind::Block { body, .. } => is_quote(body),
            ExprKind::GlobalId(g) => *g == crate::names::rust_primitives::hax::dropped_body,
            _ => false,
        }
    }
    !is_late_skip(meta) && !body_is_hidden(meta) && !is_quote(body)
}

/// Applies `f` to the name, parameters and body of every function the model
/// extracts.
fn for_each_extracted_fn(items: &mut [Item], mut f: impl FnMut(GlobalId, &mut [Param], &mut Expr)) {
    for item in items.iter_mut() {
        match &mut item.kind {
            ItemKind::Fn {
                name, params, body, ..
            } => {
                if is_extracted(&item.meta, body) {
                    f(*name, params, body)
                }
            }
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
    let mut out = HashSet::new();
    for item in items {
        match &item.kind {
            ItemKind::Fn { name, body, .. } if is_extracted(&item.meta, body) => {
                out.insert(*name);
            }
            ItemKind::Impl { items, .. } => out.extend(items.iter().filter_map(|ii| {
                matches!(&ii.kind, ImplItemKind::Fn { body, .. } if is_extracted(&ii.meta, body))
                    .then_some(ii.ident)
            })),
            _ => {}
        }
    }
    out
}

/// Which types may hold a `Result`.
#[derive(Default)]
struct ResultTypes {
    /// The crate's types with a field that may hold a `Result`.
    holding: HashSet<GlobalId>,
    /// Each field of the crate's types → the types of the other fields of its
    /// variant.
    other_fields: HashMap<GlobalId, Vec<Ty>>,
}

impl ResultTypes {
    fn new(items: &[Item]) -> Self {
        let mut fields: HashMap<GlobalId, Vec<Ty>> = HashMap::new();
        let mut other_fields = HashMap::new();
        for item in items {
            if let ItemKind::Type { name, variants, .. } = &item.kind {
                for variant in variants {
                    let tys: Vec<Ty> = variant
                        .arguments
                        .iter()
                        .map(|(_, ty, _)| ty.clone())
                        .collect();
                    fields.entry(*name).or_default().extend(tys.iter().cloned());
                    for (i, (field, _, _)) in variant.arguments.iter().enumerate() {
                        let others = tys.iter().enumerate().filter(|(j, _)| *j != i);
                        other_fields.insert(*field, others.map(|(_, ty)| ty.clone()).collect());
                    }
                }
            }
        }
        let mut types = Self {
            holding: HashSet::new(),
            other_fields,
        };
        loop {
            let new: Vec<GlobalId> = fields
                .iter()
                .filter(|(name, tys)| {
                    !types.holding.contains(*name) && tys.iter().any(|ty| types.holds(ty, true))
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
        match self.other_fields.get(&projector) {
            Some(others) => holding_arg(None) || others.iter().any(|t| self.holds(t, true)),
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

/// Whether `pat` binds the local `x`.
fn binds(pat: &Pat, x: &LocalId) -> bool {
    struct Finder<'a>(&'a LocalId, bool);
    impl AstVisitor for Finder<'_> {
        fn enter_pat_kind(&mut self, kind: &PatKind) {
            self.1 |= matches!(kind, PatKind::Binding { var, .. } if var == self.0);
        }
    }
    let mut finder = Finder(x, false);
    finder.visit_pat(pat);
    finder.1
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

/// Whether every path through `e` uses the local `x`.
fn used_on_all_paths(x: &LocalId, e: &Expr) -> bool {
    let used = |e: &Expr| used_on_all_paths(x, e);
    match &*e.kind {
        ExprKind::LocalId(y) => y == x,
        ExprKind::Let { lhs, rhs, body } => used(rhs) || (!binds(lhs, x) && used(body)),
        ExprKind::If {
            condition,
            then,
            else_,
        } => used(condition) || (used(then) && else_.as_ref().is_some_and(used)),
        ExprKind::Match { scrutinee, arms } => {
            used(scrutinee)
                || (!arms.is_empty() && arms.iter().all(|a| !binds(&a.pat, x) && used(&a.body)))
        }
        ExprKind::App { head, args, .. } => used(head) || args.iter().any(used),
        ExprKind::Construct { fields, base, .. } => {
            fields.iter().any(|(_, e)| used(e)) || base.as_ref().is_some_and(used)
        }
        ExprKind::Array(es) => es.iter().any(used),
        ExprKind::Block { body: e, .. }
        | ExprKind::Ascription { e, .. }
        | ExprKind::Borrow { inner: e, .. }
        | ExprKind::AddressOf { inner: e, .. }
        | ExprKind::Return { value: e } => used(e),
        ExprKind::Loop {
            state: Some(state), ..
        } => used(&state.init),
        ExprKind::Quote { contents } => contents
            .0
            .iter()
            .any(|c| matches!(c, QuoteContent::Expr(e) if used(e))),
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

const RECOVERY_CONTEXT: &str = "ProverifOkOnlyResults";

/// What the default Result model cannot express, for the error messages.
const RECOVERY_HINT: &str = "the default Result model ends the trace on every `Err`, so it \
     cannot express code that goes on after one; propagate the error, or pass `--result-errors`";

/// Rejects, under the default Result model, the code that may go on after an
/// `Err`: every value that may hold a `Result` must be propagated, by being
/// returned (possibly inside a tuple or a struct), passed to a function the
/// model extracts (which checks its parameter in turn), or matched by
/// patterns whose arms that may see an `Err` return one or diverge.
struct Recovery {
    types: ResultTypes,
    extracted: HashSet<GlobalId>,
}

impl Recovery {
    /// Checks `e`, whose value is propagated when `carried`.
    fn expr(&self, e: &mut Expr, carried: bool) {
        let holds = self.types.holds(&e.ty, true);
        let mut rejected = vec![];
        match &mut *e.kind {
            ExprKind::Let { lhs, rhs, body } => {
                self.pat(lhs, body, &mut rejected);
                self.expr(rhs, true);
                self.expr(body, carried);
            }
            ExprKind::Match { scrutinee, arms } => {
                self.expr(scrutinee, true);
                for arm in arms {
                    self.pat(&arm.pat, &arm.body, &mut rejected);
                    if let Some(guard) = &mut arm.guard {
                        let GuardKind::IfLet { lhs, rhs } = &mut guard.kind;
                        self.pat(lhs, &arm.body, &mut rejected);
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
                for (_, field) in fields {
                    self.expr(field, carried);
                }
                if let Some(base) = base {
                    self.expr(base, false);
                }
            }
            ExprKind::Array(es) => {
                for e in es {
                    self.expr(e, carried);
                }
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
                    Some(g) if self.extracted.contains(&g) => !generics_hold,
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
            }
            ExprKind::Closure {
                params,
                body,
                captures,
            } => {
                for param in params.iter() {
                    self.pat(param, body, &mut rejected);
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
                        self.pat(pat, body, &mut rejected);
                        self.expr(iterator, false);
                    }
                    LoopKind::ForIndexLoop { start, end, .. } => {
                        self.expr(start, false);
                        self.expr(end, false);
                    }
                    _ => {}
                }
                if let Some(state) = state {
                    self.pat(&state.body_pat, body, &mut rejected);
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

    /// Checks `pat`, whose bindings scope over `cont`, and adds the
    /// rejections to `rejected`.
    fn pat(&self, pat: &Pat, cont: &Expr, rejected: &mut Vec<Diagnostic>) {
        if !self.types.holds(&pat.ty, false) {
            return;
        }
        let mut reject = |what: &str| {
            rejected.push(pat_diagnostic(
                pat,
                RECOVERY_CONTEXT,
                format!("{what}: {RECOVERY_HINT}"),
            ))
        };
        match &*pat.kind {
            PatKind::Wild if !err_certain(cont) => {
                reject("this pattern discards a `Result`, which may be an `Err`")
            }
            PatKind::Binding { var, sub_pat, .. } => {
                if !used_on_all_paths(var, cont) {
                    reject("the `Result` bound here is not used on every path")
                }
                if let Some(sub_pat) = sub_pat {
                    self.pat(sub_pat, cont, rejected);
                }
            }
            PatKind::Construct { constructor, .. } if *constructor == Err::Constructor => {
                if !err_certain(cont) {
                    reject("this arm handles an `Err` without returning one")
                }
            }
            PatKind::Construct { fields, .. } => {
                for (_, field) in fields {
                    self.pat(field, cont, rejected);
                }
            }
            PatKind::Or { sub_pats: pats } | PatKind::Array { args: pats } => {
                for p in pats {
                    self.pat(p, cont, rejected);
                }
            }
            PatKind::Ascription { pat: inner, .. } | PatKind::Deref { sub_pat: inner } => {
                self.pat(inner, cont, rejected)
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
        extracted: extracted_fns(items),
    };
    for_each_extracted_fn(items, |_, params, body| {
        let mut rejected = vec![];
        for param in params.iter() {
            recovery.pat(&param.pat, body, &mut rejected);
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

/// How a value relates to the unknown errors.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Taint {
    /// Faithful.
    Clean,
    /// A value holding `Result`s whose `Err` may be unknown; the rest of it is
    /// faithful.
    Results,
    /// An unknown error, or a value computed from one.
    Error,
}

/// Taint analysis of the unknown errors (see [`Taint`]). A tainted `Result`
/// may only be matched with an irrefutable pattern inside `Err`; an unknown
/// error may not be matched with a refutable pattern, branched on or passed
/// to a function whose body is not extracted.
struct UnknownErrors<'a> {
    types: &'a ResultTypes,
    extracted: &'a HashSet<GlobalId>,
    /// The extracted functions whose result may be tainted.
    tainted_fns: HashSet<GlobalId>,
    /// The parameters, by function and position, that may receive a tainted
    /// value.
    tainted_params: HashSet<(GlobalId, usize)>,
    /// The tainted locals of the current function.
    locals: HashSet<LocalId>,
    /// Whether to reject, or only to compute the taints.
    reject: bool,
    /// Whether a taint was added, during the fixpoint.
    changed: bool,
}

impl UnknownErrors<'_> {
    /// The taint of a value of type `ty` that is tainted or not.
    fn taint(&self, ty: &Ty, tainted: bool) -> Taint {
        match (tainted, self.types.holds(ty, false)) {
            (false, _) => Taint::Clean,
            (true, true) => Taint::Results,
            (true, false) => Taint::Error,
        }
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
                self.locals.insert(var.clone());
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
        if self.expr(c) {
            self.reject_expr(c, "this condition depends on an error value");
        }
    }

    /// Whether the value of `e` is tainted.
    fn expr(&mut self, e: &mut Expr) -> bool {
        let mut reject_here = None;
        let mut rejected = vec![];
        let tainted = match &mut *e.kind {
            ExprKind::LocalId(x) => self.locals.contains(x),
            ExprKind::Let { lhs, rhs, body } => {
                let t = self.expr(rhs);
                self.pat(lhs, self.taint(&rhs.ty, t), &mut rejected);
                self.expr(body)
            }
            ExprKind::Match { scrutinee, arms } => {
                let t = self.expr(scrutinee);
                let t = self.taint(&scrutinee.ty, t);
                let mut out = false;
                for arm in arms {
                    self.pat(&arm.pat, t, &mut rejected);
                    if let Some(guard) = &mut arm.guard {
                        let GuardKind::IfLet { lhs, rhs } = &mut guard.kind;
                        let t = self.expr(rhs);
                        self.pat(lhs, self.taint(&rhs.ty, t), &mut rejected);
                    }
                    out |= self.expr(&mut arm.body);
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
                else_.as_mut().is_some_and(|e| self.expr(e)) || t
            }
            ExprKind::App { head, args, .. } => {
                self.expr(head);
                let taints: Vec<Taint> = args
                    .iter_mut()
                    .map(|a| {
                        let t = self.expr(a);
                        self.taint(&a.ty, t)
                    })
                    .collect();
                let any = taints.iter().any(|t| *t != Taint::Clean);
                let any_error = taints.contains(&Taint::Error);
                let returns_result = self.types.holds(&e.ty, false);
                match &*head.kind {
                    ExprKind::GlobalId(g) if self.extracted.contains(g) => {
                        for (i, t) in taints.iter().enumerate() {
                            if *t != Taint::Clean && self.tainted_params.insert((*g, i)) {
                                self.changed = true;
                            }
                        }
                        self.tainted_fns.contains(g)
                    }
                    ExprKind::GlobalId(g) if *g == never_to_any || g.is_projector() => any,
                    ExprKind::GlobalId(g) => {
                        let rendered = render_id(g);
                        let r = rendered.as_str();
                        if IDENTITY.contains(&r) {
                            any
                        } else if [
                            "core__result__Impl__err",
                            "core__result__Impl__unwrap_err",
                            "core__result__Impl__expect_err",
                        ]
                        .contains(&r)
                        {
                            taints.first().is_some_and(|t| *t != Taint::Clean)
                        } else if any_error {
                            reject_here = Some(
                                "an error value is passed to a function whose body is not extracted",
                            );
                            true
                        } else if returns_result {
                            // The error a model returns is unknown, except
                            // for these, which keep the error of an argument.
                            let keeps_err = (KEEP_ERR.contains(&r)
                                && r != "core__result__Impl__map_err")
                                || r == "core__option__Impl__ok_or";
                            !keeps_err || any
                        } else {
                            false
                        }
                    }
                    _ => any,
                }
            }
            ExprKind::Construct { fields, base, .. } => {
                let mut out = base.as_mut().is_some_and(|b| self.expr(b));
                for (_, f) in fields {
                    out |= self.expr(f);
                }
                out
            }
            ExprKind::Array(es) => es.iter_mut().fold(false, |acc, e| self.expr(e) || acc),
            ExprKind::Block { body: inner, .. }
            | ExprKind::Ascription { e: inner, .. }
            | ExprKind::Borrow { inner, .. }
            | ExprKind::AddressOf { inner, .. }
            | ExprKind::Return { value: inner } => self.expr(inner),
            ExprKind::Closure { body, captures, .. } => {
                let t = captures
                    .iter_mut()
                    .fold(false, |acc, c| self.expr(c) || acc);
                self.expr(body) || t
            }
            ExprKind::Loop {
                body, kind, state, ..
            } => {
                match &mut **kind {
                    LoopKind::WhileLoop { condition } => self.condition(condition),
                    LoopKind::ForLoop { pat, iterator } => {
                        let t = self.expr(iterator);
                        self.pat(pat, self.taint(&iterator.ty, t), &mut rejected);
                    }
                    LoopKind::ForIndexLoop { start, end, .. } => {
                        self.expr(start);
                        self.expr(end);
                    }
                    _ => {}
                }
                match state {
                    Some(state) => {
                        let init = self.expr(&mut state.init);
                        // A later iteration starts from what this one returns.
                        let reject = std::mem::replace(&mut self.reject, false);
                        let taint = self.taint(&state.init.ty, init);
                        self.pat(&state.body_pat, taint, &mut vec![]);
                        let later = self.expr(&mut body.clone());
                        self.reject = reject;
                        let taint = self.taint(&state.init.ty, init || later);
                        self.pat(&state.body_pat, taint, &mut rejected);
                        self.expr(body) || init
                    }
                    None => self.expr(body),
                }
            }
            ExprKind::Break { value, state, .. } => {
                let t = state.as_mut().is_some_and(|s| self.expr(s));
                self.expr(value) || t
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
                self.types.holds(&e.ty, false)
            }
            _ => false,
        };
        if !rejected.is_empty() {
            reject_scope(e, rejected);
        } else if let Some(what) = reject_here {
            self.reject_expr(e, what);
        }
        tainted
    }

    /// Analyses the function `name`; returns whether its result is tainted.
    fn function(&mut self, name: GlobalId, params: &mut [Param], body: &mut Expr) -> bool {
        self.locals.clear();
        let mut rejected = vec![];
        for (i, param) in params.iter().enumerate() {
            let t = self.tainted_params.contains(&(name, i));
            let t = self.taint(&param.ty, t);
            self.pat(&param.pat, t, &mut rejected);
        }
        let out = self.expr(body);
        reject_scope(body, rejected);
        out
    }
}

/// The names that the verbatim ProVerif text of `items` declares as a `fun`
/// or a `reduc`, whose application is a bare term.
fn bare_term_names(items: &[Item]) -> HashSet<String> {
    items
        .iter()
        .filter_map(|item| match &item.kind {
            ItemKind::Quote { quote, .. } => Some(scan_declarations(&quote_text(quote))),
            _ => None,
        })
        .flatten()
        .filter(|(kw, _)| *kw != "letfun" && *kw != "const")
        .map(|(_, name)| name)
        .collect()
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
        if let ExprKind::App { head, .. } = &*expr.kind
            && let ExprKind::GlobalId(g) = &*head.kind
            && self.types.holds(&expr.ty, false)
            && self.bare.contains(&render_id(g))
        {
            self.error(
                expr.clone(),
                DiagnosticInfoKind::Unimplemented {
                    issue_id: None,
                    details: Some(format!(
                        "`{}` returns a `Result`, but its model is a `fun` or a `reduc`, whose \
                         value is neither `Ok(..)` nor `Err(..)` under `--result-errors`; \
                         model it with a `letfun` returning `hax_result_ok(..)` or \
                         `hax_result_err(..)`",
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
        // The opaque and `pv_constructor` functions render as a `fun`.
        let mut bare = bare_term_names(items);
        for item in items.iter() {
            let fns: Vec<(GlobalId, &Metadata)> = match &item.kind {
                ItemKind::Fn { name, .. } => vec![(*name, &item.meta)],
                ItemKind::Impl { items, .. } => items
                    .iter()
                    .filter(|ii| matches!(ii.kind, ImplItemKind::Fn { .. }))
                    .map(|ii| (ii.ident, &ii.meta))
                    .collect(),
                _ => vec![],
            };
            for (name, meta) in fns {
                if meta
                    .hax_attributes()
                    .any(|a| matches!(a, AttrPayload::Erased | AttrPayload::PVConstructor))
                {
                    bare.insert(render_id(&name));
                }
            }
        }

        let mut analysis = UnknownErrors {
            types: &types,
            extracted: &extracted,
            tainted_fns: HashSet::new(),
            tainted_params: HashSet::new(),
            locals: HashSet::new(),
            reject: false,
            changed: true,
        };
        // The taints only grow, so this terminates.
        let mut scratch = items.clone();
        while analysis.changed {
            analysis.changed = false;
            for_each_extracted_fn(&mut scratch, |name, params, body| {
                if analysis.function(name, params, body) && analysis.tainted_fns.insert(name) {
                    analysis.changed = true;
                }
            });
        }
        analysis.reject = true;
        for_each_extracted_fn(items, |name, params, body| {
            analysis.function(name, params, body);
        });

        BareResults {
            types,
            bare,
            ..Default::default()
        }
        .visit(items);
    }
}
