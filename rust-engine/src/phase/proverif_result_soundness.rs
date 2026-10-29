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

/// What a library function does with a `Result` it receives as its first
/// argument.
#[derive(Clone, Copy, PartialEq, Eq)]
enum LibCall {
    /// Keeps the `Err` in its result.
    KeepErr,
    /// Maps the error through an opaque closure.
    MapErr,
    /// Panics on an `Err`, and returns the `Ok` payload.
    PanicOnErr,
    /// Reads whether it is an `Err`, and the `Ok` payload.
    ReadOk,
    /// Returns the error.
    ReadErr,
    /// `Option::ok_or`: the payload goes into `Ok`, the fallback into `Err`.
    OkOr,
}

/// The library function `g` that handles `Result`s, if it is one. These
/// names, but `map_err`, are not in `crate::names`.
fn lib_call(g: &GlobalId) -> Option<LibCall> {
    if *g == crate::names::core::result::Impl::map_err {
        return Some(LibCall::MapErr);
    }
    Some(match render_id(g).as_str() {
        "core__clone__Clone__clone"
        | "core__convert__From__from"
        | "core__convert__Into__into"
        | "core__ops__try_trait__FromResidual__from_residual"
        | "core__ops__try_trait__Try__branch"
        | "core__option__Impl_4__transpose"
        | "core__result__Impl__map" => LibCall::KeepErr,
        "core__result__Impl__expect" | "core__result__Impl__unwrap" => LibCall::PanicOnErr,
        "core__result__Impl__is_err"
        | "core__result__Impl__is_ok"
        | "core__result__Impl__ok"
        | "core__result__Impl__unwrap_or" => LibCall::ReadOk,
        "core__result__Impl__err"
        | "core__result__Impl__expect_err"
        | "core__result__Impl__unwrap_err" => LibCall::ReadErr,
        "core__option__Impl__ok_or" => LibCall::OkOr,
        _ => return None,
    })
}

/// The prefixes of the names that hax's own libraries model, without events.
const LIBRARY_PREFIXES: &[&str] = &[
    "alloc__",
    "core__",
    "hax_lib__",
    "rust_primitives__",
    "std__",
];

/// `cryptolib.pvl`, which hand-written models apply.
const CRYPTOLIB_PVL: &str = include_str!("../../../hax-lib/proof-libs/proverif/cryptolib.pvl");

/// The libraries hax ships, which emit no event and insert in no table.
const LIBRARIES: &[&str] = &[
    crate::backends::proverif::PRIMITIVES_PVL,
    crate::backends::proverif::RESULT_PVL,
    crate::backends::proverif::RESULT_ERRORS_PVL,
    CRYPTOLIB_PVL,
];

/// The libraries an extraction with `--result-errors` is checked against.
const RESULT_ERRORS_LIBRARIES: &[&str] = &[
    crate::backends::proverif::PRIMITIVES_PVL,
    crate::backends::proverif::RESULT_ERRORS_PVL,
    CRYPTOLIB_PVL,
];

/// The identifiers of verbatim ProVerif text.
fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|w| !w.is_empty())
}

/// Whether verbatim ProVerif text acts observably by itself: an event, or an
/// insertion in a table.
fn acts_observably(text: &str) -> bool {
    words(text).any(|w| w == "event" || w == "insert")
}

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

    /// Whether a value of type `ty` may hold a `Result`, counting the types
    /// that a generic may instantiate with one.
    fn may_hold(&self, ty: &Ty) -> bool {
        match ty.kind() {
            TyKind::Param(_)
            | TyKind::AssociatedType { .. }
            | TyKind::Opaque(_)
            | TyKind::Dyn(_) => true,
            TyKind::App { args, .. } => {
                self.holds(ty, false)
                    || args
                        .iter()
                        .any(|a| matches!(a, GenericValue::Ty(t) if self.may_hold(t)))
            }
            TyKind::Ref { inner, .. } | TyKind::Slice(inner) => self.may_hold(inner),
            TyKind::Array { ty, .. } => self.may_hold(ty),
            _ => self.holds(ty, false),
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

/// A rejection of a pattern, with its context and reason.
struct Rejection(Pat, &'static str, String);

/// A rejection of the pattern `p`.
fn pat_diagnostic(p: &Pat, context: &'static str, details: String) -> Rejection {
    Rejection(p.clone(), context, details)
}

/// Replaces `scope`, which holds rejected patterns, by an error node
/// reporting each distinct rejection once.
fn reject_scope(scope: &mut Expr, rejections: Vec<Rejection>) {
    let mut seen = HashSet::new();
    let diagnostics: Vec<Diagnostic> = rejections
        .into_iter()
        .filter(|Rejection(p, _, details)| seen.insert((p.meta.span, details.clone())))
        .map(|Rejection(p, context, details)| {
            let info = DiagnosticInfo {
                context: Context::Phase(context.into()),
                span: p.meta.span,
                kind: DiagnosticInfoKind::Unimplemented {
                    issue_id: None,
                    details: Some(details),
                },
            };
            Diagnostic::new(p, info)
        })
        .collect();
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

/// The right-hand side of the `if let` guard of `arm`.
fn guard_rhs(arm: &Arm) -> Option<&Expr> {
    arm.guard.as_ref().map(|g| {
        let GuardKind::IfLet { rhs, .. } = &g.kind;
        rhs
    })
}

/// Whether `pat` matches every `Err`, matched against a `Result`.
fn covers_err(pat: &Pat) -> bool {
    match &*pat.kind {
        PatKind::Construct {
            constructor,
            fields,
            ..
        } => *constructor == Err::Constructor && fields.iter().all(|(_, p)| irrefutable(p)),
        PatKind::Binding {
            sub_pat: Some(p), ..
        }
        | PatKind::Ascription { pat: p, .. }
        | PatKind::Deref { sub_pat: p } => covers_err(p),
        PatKind::Or { sub_pats } => sub_pats.iter().any(covers_err),
        _ => irrefutable(pat),
    }
}

/// Whether `e` mentions the local `x`.
fn mentions(x: &LocalId, e: &Expr) -> bool {
    struct Finder<'a>(&'a LocalId, bool);
    impl AstVisitor for Finder<'_> {
        fn enter_expr_kind(&mut self, kind: &ExprKind) {
            self.1 |= matches!(kind, ExprKind::LocalId(y) if y == self.0);
        }
    }
    let mut finder = Finder(x, false);
    finder.visit_expr(e);
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

/// Which calls the model may observe. ProVerif terms may emit events and
/// insert in tables, so a hand-written model may be observable, and so is any
/// function or model calling one.
#[derive(Default)]
pub(crate) struct Effects {
    /// The functions the model extracts.
    extracted: HashSet<GlobalId>,
    /// The crate's functions and the names its verbatim ProVerif declares, by
    /// rendered name, and whether calling one may be observable.
    observable: HashMap<String, bool>,
    /// The names the libraries declare.
    library: HashSet<String>,
}

impl Effects {
    /// The observable calls of `items`.
    pub(crate) fn of(items: &[Item]) -> Self {
        let mut effects = Self {
            extracted: extracted_fns(items),
            library: LIBRARIES
                .iter()
                .flat_map(|lib| scan_declarations(lib))
                .map(|(_, name)| name)
                .collect(),
            ..Default::default()
        };
        // Each function or declaration acts observably by itself, or calls
        // other ones, by name.
        let mut graph: HashMap<String, (bool, Vec<String>)> = HashMap::new();
        for item in items {
            if let ItemKind::Quote { quote, .. } = &item.kind {
                let text = quote_text(quote);
                let calls: Vec<String> = words(&text).map(String::from).collect();
                for (_, name) in scan_declarations(&text) {
                    let node = graph.entry(name).or_default();
                    node.0 |= acts_observably(&text);
                    node.1.extend(calls.iter().cloned());
                }
            }
        }
        for d in fn_decls(items) {
            graph.entry(render_id(&d.name)).or_default();
        }
        let known: HashSet<String> = graph.keys().cloned().collect();
        for d in fn_decls(items) {
            let (direct, calls) = if effects.extracted.contains(&d.name) {
                let mut calls = vec![];
                (effects.walk(d.body, &known, &mut calls), calls)
            } else if let Some(quote) = quoted_body(d.body) {
                let text = quote_text(quote);
                (
                    acts_observably(&text),
                    words(&text).map(String::from).collect(),
                )
            } else {
                // A handwritten or unimported body is unknown; an opaque or
                // `pv_constructor` function renders as a `fun`.
                let fun = d
                    .meta
                    .hax_attributes()
                    .any(|a| matches!(a, AttrPayload::Erased | AttrPayload::PVConstructor));
                (!fun, vec![])
            };
            let node = graph.get_mut(&render_id(&d.name)).unwrap();
            node.0 |= direct;
            node.1.extend(calls);
        }
        effects.observable = graph.iter().map(|(n, (d, _))| (n.clone(), *d)).collect();
        loop {
            let new: Vec<String> = graph
                .iter()
                .filter(|(n, (_, calls))| {
                    !effects.observable[*n]
                        && calls
                            .iter()
                            .any(|c| effects.observable.get(c) == Some(&true))
                })
                .map(|(n, _)| n.clone())
                .collect();
            if new.is_empty() {
                return effects;
            }
            for n in new {
                effects.observable.insert(n, true);
            }
        }
    }

    /// Whether calling `head` may be observable. With `known`, the calls of
    /// the crate's functions and declarations are added to `calls` instead.
    fn call(&self, head: &Expr, known: Option<(&HashSet<String>, &mut Vec<String>)>) -> bool {
        let ExprKind::GlobalId(g) = &*head.kind else {
            return true;
        };
        if *g == never_to_any || g.is_projector() || g.is_constructor() {
            return false;
        }
        let rendered = render_id(g);
        if let Some((known, calls)) = known
            && known.contains(&rendered)
        {
            calls.push(rendered);
            return false;
        }
        if let Some(observable) = self.observable.get(&rendered) {
            return *observable;
        }
        !(self.library.contains(&rendered)
            || LIBRARY_PREFIXES.iter().any(|p| rendered.starts_with(p)))
    }

    /// Whether calling `head` may be observable.
    fn call_is_observable(&self, head: &Expr) -> bool {
        self.call(head, None)
    }

    /// Whether a quote in a function body may be observable.
    fn quote_is_observable(&self, quote: &Quote) -> bool {
        let text = quote_text(quote);
        acts_observably(&text) || words(&text).any(|w| self.observable.get(w) == Some(&true))
    }

    /// Whether evaluating `e` may be observable by itself; the calls of the
    /// `known` functions and declarations are added to `calls`.
    fn walk(&self, e: &Expr, known: &HashSet<String>, calls: &mut Vec<String>) -> bool {
        struct Finder<'a>(&'a Effects, &'a HashSet<String>, &'a mut Vec<String>, bool);
        impl AstVisitor for Finder<'_> {
            fn enter_expr_kind(&mut self, kind: &ExprKind) {
                self.3 |= match kind {
                    ExprKind::App { head, .. } => self.0.call(head, Some((self.1, self.2))),
                    ExprKind::Quote { contents } => {
                        let text = quote_text(contents);
                        self.2.extend(words(&text).map(String::from));
                        acts_observably(&text)
                    }
                    _ => false,
                };
            }
        }
        let mut finder = Finder(self, known, calls, false);
        finder.visit_expr(e);
        finder.3
    }

    /// Whether evaluating `e` may be observable.
    pub(crate) fn has_effect(&self, e: &Expr) -> bool {
        struct Finder<'a>(&'a Effects, bool);
        impl AstVisitor for Finder<'_> {
            fn enter_expr_kind(&mut self, kind: &ExprKind) {
                self.1 |= match kind {
                    ExprKind::App { head, .. } => self.0.call_is_observable(head),
                    ExprKind::Quote { contents } => self.0.quote_is_observable(contents),
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
struct Recovery<'a> {
    types: ResultTypes,
    effects: &'a Effects,
    /// The parameter types of each function.
    params: HashMap<GlobalId, Vec<Ty>>,
}

impl Recovery<'_> {
    /// How the branch `e` uses `x`. A branch that returns an `Err`, with
    /// nothing observable, needs not use it: every `Err` ends the trace alike.
    fn branch_use(&self, x: &LocalId, e: &Expr) -> Use {
        match self.first_use(x, e) {
            Use::First => Use::First,
            _ if err_certain(e) && !self.effects.has_effect(e) => Use::First,
            u => u,
        }
    }

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
                .then(|| self.branch_use(x, body))
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
                    self.branch_use(x, then),
                    else_
                        .as_ref()
                        .map_or(Use::Neither, |e| self.branch_use(x, e)),
                ])
            }),
            ExprKind::Match { scrutinee, arms } => self.first_use(x, scrutinee).then(|| {
                Use::join(
                    arms.iter()
                        .map(|arm| scoped(&arm.pat, guard_rhs(arm), &arm.body)),
                )
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
            ExprKind::Loop { kind, state, .. } => {
                let mut es: Vec<&Expr> = state.iter().map(|s| &s.init).collect();
                match &**kind {
                    LoopKind::WhileLoop { condition } => es.push(condition),
                    LoopKind::ForLoop { iterator, .. } => es.push(iterator),
                    LoopKind::ForIndexLoop { start, end, .. } => es.extend([start, end]),
                    _ => {}
                }
                let body = effect(mentions(x, e) || self.effects.has_effect(e));
                seq(&es, body)
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
        // Rejected here, `e` is not rejected again where it flows.
        let not_carried = !carried && self.types.holds(&e.ty, true);
        let carried = carried || not_carried;
        let mut rejected = vec![];
        match &mut *e.kind {
            ExprKind::Let { lhs, rhs, body } => {
                self.pat(lhs, None, body, &mut rejected);
                self.expr(rhs, true);
                self.expr(body, carried);
            }
            ExprKind::Match { scrutinee, arms } => {
                self.expr(scrutinee, true);
                // Whether an earlier arm already takes every `Err` of a
                // `Result` scrutinee.
                let mut err_taken = false;
                for arm in arms {
                    self.arm_pat(
                        &arm.pat,
                        err_taken,
                        guard_rhs(arm),
                        &arm.body,
                        &mut rejected,
                    );
                    if let Some(guard) = &mut arm.guard {
                        let GuardKind::IfLet { lhs, rhs } = &mut guard.kind;
                        // On an `Err` the guard does not take, Rust falls
                        // through to the next arms; the model does not.
                        if self.types.holds(&rhs.ty, false) && !covers_err(lhs) {
                            rejected.push(pat_diagnostic(
                                lhs,
                                RECOVERY_CONTEXT,
                                format!(
                                    "on an `Err`, this guard falls through to the next arms: \
                                     {RECOVERY_HINT}"
                                ),
                            ));
                        }
                        self.pat(lhs, None, &arm.body, &mut rejected);
                        self.expr(rhs, true);
                    }
                    err_taken |= arm.guard.is_none() && covers_err(&arm.pat);
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
                let lib = callee.as_ref().and_then(lib_call);
                let arg_carried = |i: usize, arg: &Expr| match callee {
                    Some(g) if g == never_to_any => true,
                    Some(g) if g.is_projector() => {
                        carried && !self.types.projection_drops_result(g, &arg.ty)
                    }
                    // The callee checks a parameter whose type holds a
                    // `Result`, but not a generic one.
                    Some(g) if self.effects.extracted.contains(&g) => {
                        !generics_hold
                            && self
                                .params
                                .get(&g)
                                .and_then(|ps| ps.get(i))
                                .is_some_and(|t| self.types.holds(t, true))
                    }
                    _ if i == 0 && lib == Some(LibCall::PanicOnErr) => true,
                    _ if i == 0 && matches!(lib, Some(LibCall::KeepErr | LibCall::MapErr)) => {
                        carried
                    }
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
                        self.expr(e, false);
                    }
                }
            }
            ExprKind::Error(_) => return,
            _ => {}
        }
        if !rejected.is_empty() {
            reject_scope(e, rejected);
        } else if not_carried {
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
        rejected: &mut Vec<Rejection>,
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

    /// Checks the pattern of a match arm; with `err_taken`, an earlier arm
    /// already takes every `Err` of the scrutinee, which then is an `Ok` here.
    fn arm_pat(
        &self,
        pat: &Pat,
        err_taken: bool,
        guard: Option<&Expr>,
        cont: &Expr,
        rejected: &mut Vec<Rejection>,
    ) {
        let is_result = matches!(pat.ty.kind(), TyKind::App { head, .. }
            if *head == crate::names::core::result::Result);
        let catch_all = matches!(
            &*pat.kind,
            PatKind::Wild | PatKind::Binding { sub_pat: None, .. }
        );
        if err_taken && is_result && catch_all {
            return;
        }
        self.pat(pat, guard, cont, rejected)
    }

    /// Checks `pat`, whose bindings scope over `guard` then `cont`, and adds
    /// the rejections to `rejected`.
    fn pat(&self, pat: &Pat, guard: Option<&Expr>, cont: &Expr, rejected: &mut Vec<Rejection>) {
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
                    .then(|| self.branch_use(var, cont));
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
pub(crate) fn reject_recovery(items: &mut [Item], effects: &Effects) {
    let recovery = Recovery {
        types: ResultTypes::new(items),
        effects,
        params: fn_decls(items)
            .map(|d| (d.name, d.params.iter().map(|p| p.ty.clone()).collect()))
            .collect(),
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
/// matched with a refutable pattern, branched on, spliced into verbatim
/// ProVerif or passed to a function whose body is not extracted.
struct UnknownErrors<'a> {
    types: &'a ResultTypes,
    extracted: &'a HashSet<GlobalId>,
    /// The taint of the result of each extracted function.
    fns: HashMap<GlobalId, Taint>,
    /// The taint each parameter, by function and position, may receive.
    params: HashMap<(GlobalId, usize), Taint>,
    /// The taint each loop state binder, by function, may have on a later
    /// iteration.
    loop_states: HashMap<(GlobalId, LocalId), Taint>,
    /// The taint of each local in scope.
    locals: HashMap<LocalId, Taint>,
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

    fn reject_expr(&self, e: &mut Expr, what: &str) {
        if self.reject {
            reject_expr(e, UNKNOWN_CONTEXT, format!("{what}: {UNKNOWN_HINT}"))
        }
    }

    /// Binds the locals of `pat`, matched against a value of taint `taint`,
    /// and adds the rejections to `rejected`.
    fn pat(&mut self, pat: &Pat, taint: Taint, rejected: &mut Vec<Rejection>) {
        // The parts of a tainted `Result` that hold no `Result` are faithful.
        if taint == Taint::Clean || (taint == Taint::Results && !self.types.may_hold(&pat.ty)) {
            for x in binders(pat) {
                self.locals.insert(x, Taint::Clean);
            }
            return;
        }
        let refutable_err = matches!(&*pat.kind, PatKind::Construct { constructor, fields, .. }
            if *constructor == Err::Constructor && !fields.iter().all(|(_, p)| irrefutable(p)));
        match &*pat.kind {
            PatKind::Wild => {}
            PatKind::Binding { var, sub_pat, .. } => {
                self.locals.insert(var.clone(), taint);
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
                for x in binders(pat) {
                    self.locals.insert(x, Taint::Error);
                }
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

    /// The taint of a call of `head` (or of a reference to the constant
    /// `head`) on arguments of taints `args`, of type `ty`; `Err` with a
    /// reason when the call must be rejected.
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
        if *g == never_to_any || g.is_constructor() {
            return Ok(if g.is_constructor() {
                all
            } else {
                Taint::Clean
            });
        }
        let first = args.first().copied().unwrap_or_default();
        // The `Ok` payload of a tainted `Result` may hold a `Result` in turn.
        let payload = |t: Taint| match t {
            Taint::Results if !self.types.may_hold(ty) => Taint::Clean,
            t => t,
        };
        if g.is_projector() {
            return Ok(payload(first));
        }
        Ok(match lib_call(g) {
            Some(LibCall::KeepErr) => all,
            // The error is mapped through an opaque closure.
            Some(LibCall::MapErr) => all.max(Taint::Results),
            Some(LibCall::OkOr) => first.max(args.get(1).copied().unwrap_or_default().in_err()),
            Some(LibCall::ReadErr) if first != Taint::Clean => Taint::Error,
            Some(LibCall::ReadErr) => Taint::Clean,
            Some(LibCall::PanicOnErr | LibCall::ReadOk) => payload(all),
            None if all != Taint::Clean => {
                return Err(
                    "a `Result` or an error coming from a hand-written model is passed to a \
                     function whose body is not extracted",
                );
            }
            // The error a model returns is unknown.
            None if self.types.may_hold(ty) => Taint::Results,
            None => Taint::Clean,
        })
    }

    /// The taint of the value of `e`.
    fn expr(&mut self, e: &mut Expr) -> Taint {
        let mut reject_here = None;
        let mut rejected = vec![];
        // The bindings of a scope do not outlive it.
        let opens_scope = matches!(
            &*e.kind,
            ExprKind::Let { .. }
                | ExprKind::Match { .. }
                | ExprKind::Closure { .. }
                | ExprKind::Loop { .. }
        );
        let scope = opens_scope.then(|| self.locals.clone());
        let taint = match &mut *e.kind {
            ExprKind::LocalId(x) => self.locals.get(x).copied().unwrap_or_default(),
            // A constant is a call without arguments.
            ExprKind::GlobalId(_) => {
                let head = e.clone();
                self.call(&head, &[], &e.ty).unwrap_or_default()
            }
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
                    self.locals.clone_from(scope.as_ref().unwrap());
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
                // A closure value carries the taint of what it returns.
                let head_taint = if matches!(&*head.kind, ExprKind::GlobalId(_)) {
                    Taint::Clean
                } else {
                    self.expr(head)
                };
                let taints: Vec<Taint> = args.iter_mut().map(|a| self.expr(a)).collect();
                match self.call(head, &taints, &e.ty) {
                    Ok(t) => t.max(head_taint),
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
            ExprKind::Closure {
                params,
                body,
                captures,
            } => {
                let t = captures
                    .iter_mut()
                    .fold(Taint::Clean, |acc, c| acc.max(self.expr(c)));
                for param in params.iter() {
                    self.pat(param, Taint::Clean, &mut rejected);
                }
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
                        // A later iteration starts from what this one returns,
                        // which earlier passes of the fixpoint recorded.
                        let init = self.expr(&mut state.init);
                        let current = self.current.unwrap();
                        let state_binders = binders(&state.body_pat);
                        let later = state_binders
                            .iter()
                            .filter_map(|x| self.loop_states.get(&(current, x.clone())))
                            .copied()
                            .max()
                            .unwrap_or_default();
                        self.pat(&state.body_pat, init.max(later), &mut rejected);
                        let out = self.expr(body);
                        for x in state_binders {
                            Self::raise(
                                &mut self.loop_states,
                                (current, x),
                                out,
                                &mut self.changed,
                            );
                        }
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
                let mut spliced = Taint::Clean;
                for c in &mut contents.0 {
                    if let QuoteContent::Expr(e) = c {
                        spliced = spliced.max(self.expr(e));
                    }
                }
                if spliced != Taint::Clean {
                    reject_here = Some(
                        "a `Result` or an error coming from a hand-written model is spliced into \
                         verbatim ProVerif",
                    );
                }
                if self.types.may_hold(&e.ty) {
                    Taint::Results
                } else {
                    Taint::Clean
                }
            }
            _ => Taint::Clean,
        };
        if let Some(scope) = scope {
            self.locals = scope;
        }
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
        self.locals.clear();
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

/// The names declared as a `fun`, a `reduc` or a `const`, whose application
/// is a bare term: in the crate's verbatim ProVerif, the libraries, and the
/// opaque and `pv_constructor` functions, which render as a `fun`.
fn bare_term_names(items: &[Item]) -> HashSet<String> {
    let quotes = items.iter().filter_map(|item| match &item.kind {
        ItemKind::Quote { quote, .. } => Some(quote_text(quote)),
        _ => None,
    });
    let mut names: HashSet<String> = quotes
        .chain(RESULT_ERRORS_LIBRARIES.iter().map(|lib| lib.to_string()))
        .flat_map(|text| scan_declarations(&text))
        .filter(|(kw, _)| *kw != "letfun")
        .map(|(_, name)| name)
        .collect();
    names.extend(
        fn_decls(items)
            .filter(|d| {
                d.meta
                    .hax_attributes()
                    .any(|a| matches!(a, AttrPayload::Erased | AttrPayload::PVConstructor))
            })
            .map(|d| render_id(&d.name)),
    );
    names
}

/// Wraps each verbatim body (as `replace_body`, `pv_extern` and
/// `pv_inverse_of` make it) of a function returning a `Result`, so that its
/// value is one: an `Ok` or an `Err` is kept, any other value becomes an `Ok`,
/// and a failure an `Err`, with an unknown error.
fn wrap_bare_results(items: &mut [Item]) {
    let wrap = |body: &mut Expr| {
        let returns_result = matches!(body.ty.kind(), TyKind::App { head, .. }
            if *head == crate::names::core::result::Result);
        let body = match &mut *body.kind {
            ExprKind::Block { body, .. } => body,
            _ => body,
        };
        if let ExprKind::Quote { contents } = &mut *body.kind
            && returns_result
        {
            contents
                .0
                .insert(0, QuoteContent::Verbatim("let hax__model = (".into()));
            contents.0.push(QuoteContent::Verbatim(
                ") in (let core__result__Result__Ok(hax__v) = hax__model in hax__model \
                 else let core__result__Result__Err(hax__v) = hax__model in hax__model \
                 else hax_result_ok(hax__model)) \
                 else hax_result_err(rust_primitives__hax__Tuple0__Tuple0)"
                    .into(),
            ));
        }
    };
    for item in items.iter_mut() {
        match &mut item.kind {
            ItemKind::Fn { body, .. } => wrap(body),
            ItemKind::Impl { items, .. } => {
                for ii in items.iter_mut() {
                    if let ImplItemKind::Fn { body, .. } = &mut ii.kind {
                        wrap(body)
                    }
                }
            }
            _ => {}
        }
    }
}

/// Rejects the uses of a function or a constant returning a `Result` whose
/// model is a bare term: neither `Ok(..)` nor `Err(..)` matches it, so both
/// paths would be lost.
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
                         `--result-errors`; give it a `replace_body` model",
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
        let bare = bare_term_names(items);
        wrap_bare_results(items);
        let extracted = extracted_fns(items);

        let mut analysis = UnknownErrors {
            types: &types,
            extracted: &extracted,
            fns: HashMap::new(),
            params: HashMap::new(),
            loop_states: HashMap::new(),
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
