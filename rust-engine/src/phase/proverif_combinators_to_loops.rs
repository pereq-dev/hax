//! ProVerif-backend phase: rewrite iterator-combinator calls into for-loops.
//!
//! The ProVerif printer cannot print closures. This phase rewrites common
//! combinators applied to a closure (`fold`, `find`, `map(..).collect()`, …)
//! into `ForLoop` nodes, which the printer unrolls to a bounded depth, or into
//! scalar `match`/`if` expressions (`Option::map`, `bool::then`,
//! `Result::map_err`). The closure parameters are reused directly as the
//! binders, so no substitution is needed.

use crate::ast::identifiers::GlobalId;
use crate::ast::literals::Literal;
use crate::ast::span::Span;
use crate::ast::*;
use crate::ast::{diagnostics::*, visitors::*};
use crate::backends::proverif::generated_local;
use crate::names::alloc::vec;
use crate::names::core::iter::traits::iterator::Iterator;
use crate::names::rust_primitives::hax::{array_cons, array_nil, logical_op_or};
use crate::phase::Phase;

/// Phase that rewrites combinators applied to a closure into closure-free code.
#[derive(Default, Debug)]
pub struct ProverifCombinatorsToLoops;

#[setup_error_handling_struct]
#[derive(Default)]
struct ProverifCombinatorsToLoopsVisitor;

impl VisitorWithContext for ProverifCombinatorsToLoopsVisitor {
    fn context(&self) -> Context {
        Context::Phase(stringify!(ProverifCombinatorsToLoops).to_string())
    }
}

/// The parameters and body of a closure.
fn closure(e: &Expr) -> Option<(&[Pat], &Expr)> {
    match &*e.kind {
        ExprKind::Closure { params, body, .. } => Some((params, body)),
        _ => None,
    }
}

/// The parameter and body of a one-parameter closure.
fn closure1(e: &Expr) -> Option<(&Pat, &Expr)> {
    match closure(e)? {
        ([param], body) => Some((param, body)),
        _ => None,
    }
}

/// The binder and the use of a fresh local `name`.
fn local(name: &str, ty: &Ty, span: Span) -> (Pat, Expr) {
    (
        PatKind::var_pat(generated_local(name)).promote(ty.clone(), span),
        ExprKind::LocalId(generated_local(name)).promote(ty.clone(), span),
    )
}

/// `for pat in iterator { body }`, threading `state` as `(init, acc)`.
fn for_loop(pat: &Pat, iterator: &Expr, body: Expr, state: Option<(Expr, Pat)>) -> ExprKind {
    ExprKind::Loop {
        body,
        kind: Box::new(LoopKind::ForLoop {
            pat: pat.clone(),
            iterator: iterator.clone(),
        }),
        state: state.map(|(init, body_pat)| LoopState { init, body_pat }),
        control_flow: None,
        label: None,
    }
}

/// `match scrutinee { Some(some_pat) => on_some, None => on_none }`.
fn match_option(
    scrutinee: Expr,
    some_pat: Pat,
    on_some: Expr,
    on_none: Expr,
    span: Span,
) -> ExprKind {
    let ty = scrutinee.ty.clone();
    ExprKind::Match {
        scrutinee,
        arms: vec![
            Arm::non_guarded(Pat::some(some_pat, ty.clone(), span), on_some, span),
            Arm::non_guarded(Pat::none(ty, span), on_none, span),
        ],
    }
}

/// The `i`-th type argument of `ty`, or `ty` itself if it has another shape
/// (sound, since ProVerif models every value as `bitstring`).
fn type_arg(ty: &Ty, i: usize) -> Ty {
    match ty.kind() {
        TyKind::App { args, .. } => match args.get(i) {
            Some(GenericValue::Ty(arg)) => arg.clone(),
            _ => ty.clone(),
        },
        _ => ty.clone(),
    }
}

/// Whether `ty` is modelled as a Seq, so that collecting into it is pushing
/// every element. `Result`, `Option` or a map collect differently.
fn is_seq(ty: &Ty) -> bool {
    match ty.kind() {
        TyKind::App { head, .. } => *head == vec::Vec,
        TyKind::Slice(_) | TyKind::Array { .. } => true,
        _ => false,
    }
}

/// `f(arg)`, β-reduced when `f` is a closure.
fn apply(f: &Expr, arg: Expr, ty: Ty, span: Span) -> Expr {
    match closure1(f) {
        Some((param, body)) => ExprKind::Let {
            lhs: param.clone(),
            rhs: arg,
            body: body.clone(),
        }
        .promote(ty, span),
        None => ExprKind::App {
            head: f.clone(),
            args: vec![arg],
            generic_args: vec![],
            bounds_impls: vec![],
            trait_: None,
        }
        .promote(ty, span),
    }
}

/// `extend_from_slice(acc, array_cons(elem, array_nil()))`: append `elem` to
/// the Seq `acc`, keeping the collected elements in iteration order.
fn push(acc: Expr, elem: Expr, span: Span) -> Expr {
    let ty = acc.ty.clone();
    let nil = Expr::standalone_fn_app(array_nil, vec![], vec![], ty.clone(), span);
    let singleton = Expr::standalone_fn_app(array_cons, vec![], vec![elem, nil], ty.clone(), span);
    Expr::standalone_fn_app(
        vec::Impl__2::extend_from_slice,
        vec![],
        vec![acc, singleton],
        ty,
        span,
    )
}

/// The loop folding `body` over `iter` from an empty Seq `name`, where `body`
/// builds the next Seq from the accumulator.
fn collect_loop(
    name: &str,
    iter: &Expr,
    elem: &Pat,
    ty: &Ty,
    span: Span,
    body: impl FnOnce(Expr) -> Expr,
) -> ExprKind {
    let (acc_pat, acc) = local(name, ty, span);
    let nil = Expr::standalone_fn_app(array_nil, vec![], vec![], ty.clone(), span);
    for_loop(elem, iter, body(acc), Some((nil, acc_pat)))
}

/// The loop over `iter` keeping the first `Some` that `on_none` yields, in an
/// accumulator `name` of type `ty`.
fn first_some_loop(
    name: &str,
    iter: &Expr,
    elem: &Pat,
    ty: &Ty,
    span: Span,
    on_none: Expr,
) -> ExprKind {
    let (acc_pat, acc) = local(name, ty, span);
    let wild = PatKind::Wild.promote(ty.clone(), span);
    let body = match_option(acc.clone(), wild, acc, on_none, span).promote(ty.clone(), span);
    for_loop(
        elem,
        iter,
        body,
        Some((Expr::none(ty.clone(), span), acc_pat)),
    )
}

/// `match r { Ok(x) => Ok(x), Err(e) => Err(f(e)) }` for `map_err(r, f)`.
fn map_err(r: &Expr, f: &Expr, ty: &Ty, span: Span) -> ExprKind {
    use crate::names::core::result::Result::{Err, Ok};
    let (ok_pat, ok) = local("hax__ok", &type_arg(&r.ty, 0), span);
    let (err_pat, err) = local("hax__err", &type_arg(&r.ty, 1), span);
    let arm = |constructor, field, pat, value| {
        Arm::non_guarded(
            Pat::variant(constructor, vec![(field, pat)], r.ty.clone(), span),
            Expr::variant(constructor, vec![(field, value)], ty.clone(), span),
            span,
        )
    };
    ExprKind::Match {
        scrutinee: r.clone(),
        arms: vec![
            arm(Ok::Constructor, Ok::_0, ok_pat, ok),
            arm(
                Err::Constructor,
                Err::_0,
                err_pat,
                apply(f, err, type_arg(ty, 1), span),
            ),
        ],
    }
}

/// The closure-free form of the call `head(args)` of type `ty`, if `head` is a
/// combinator applied to a closure.
fn rewrite(head: GlobalId, args: &[Expr], ty: &Ty, span: Span) -> Option<ExprKind> {
    use crate::names::core::bool::Impl as Bool;
    use crate::names::core::option::Impl as Option_;
    use crate::names::core::result::Impl as Result_;
    Some(if head == Iterator::fold {
        let [iter, init, f] = args else { return None };
        let ([acc, elem], body) = closure(f)? else {
            return None;
        };
        for_loop(elem, iter, body.clone(), Some((init.clone(), acc.clone())))
    } else if head == Iterator::find {
        // Keeps the first element satisfying `pred`.
        let [iter, f] = args else { return None };
        let (elem, pred) = closure1(f)?;
        let x = elem.as_binder()?;
        let x = ExprKind::LocalId(x.clone()).promote(elem.ty.clone(), span);
        let found = ExprKind::If {
            condition: pred.clone(),
            then: Expr::some(x, ty.clone(), span),
            else_: Some(Expr::none(ty.clone(), span)),
        }
        .promote(ty.clone(), span);
        first_some_loop("__find_acc", iter, elem, ty, span, found)
    } else if head == Iterator::find_map {
        let [iter, f] = args else { return None };
        let (elem, body) = closure1(f)?;
        first_some_loop("__find_map_acc", iter, elem, ty, span, body.clone())
    } else if head == Iterator::any {
        let [iter, f] = args else { return None };
        let (elem, pred) = closure1(f)?;
        let (acc_pat, acc) = local("__any_acc", &Ty::bool(), span);
        let body = Expr::standalone_fn_app(
            logical_op_or,
            vec![],
            vec![acc, pred.clone()],
            Ty::bool(),
            span,
        );
        let init = ExprKind::Literal(Literal::Bool(false)).promote(Ty::bool(), span);
        for_loop(elem, iter, body, Some((init, acc_pat)))
    } else if head == Iterator::for_each {
        let [iter, f] = args else { return None };
        let (elem, body) = closure1(f)?;
        for_loop(elem, iter, body.clone(), None)
    } else if head == Iterator::filter_map {
        // Keeps the `Some` payloads.
        let [iter, f] = args else { return None };
        let (elem, body) = closure1(f)?;
        let (y_pat, y) = local("__filter_map_y", &type_arg(&body.ty, 0), span);
        collect_loop("__filter_map_acc", iter, elem, ty, span, |acc| {
            match_option(body.clone(), y_pat, push(acc.clone(), y, span), acc, span)
                .promote(ty.clone(), span)
        })
    } else if head == Iterator::collect {
        // `collect(map(iter, f))`.
        let [inner] = args else { return None };
        if !is_seq(ty) {
            return None;
        }
        let mut inner = inner;
        while let ExprKind::Ascription { e, .. } | ExprKind::Borrow { inner: e, .. } = &*inner.kind
        {
            inner = e;
        }
        let ExprKind::App {
            head: map, args, ..
        } = &*inner.kind
        else {
            return None;
        };
        if !matches!(&*map.kind, ExprKind::GlobalId(g) if *g == Iterator::map) {
            return None;
        }
        let [iter, f] = &args[..] else { return None };
        let (elem, body) = closure1(f)?;
        collect_loop("__map_acc", iter, elem, ty, span, |acc| {
            push(acc, body.clone(), span)
        })
    } else if head == Option_::map {
        let [opt, f] = args else { return None };
        let (elem, body) = closure1(f)?;
        let x = elem.as_binder()?;
        let x = PatKind::var_pat(x.clone()).promote(elem.ty.clone(), span);
        let some = Expr::some(body.clone(), ty.clone(), span);
        match_option(opt.clone(), x, some, Expr::none(ty.clone(), span), span)
    } else if head == Bool::then {
        // The frontend may lower `|| body` to a closure with one unit
        // parameter.
        let [cond, f] = args else { return None };
        let (params, body) = closure(f)?;
        if params.len() > 1 {
            return None;
        }
        ExprKind::If {
            condition: cond.clone(),
            then: Expr::some(body.clone(), ty.clone(), span),
            else_: Some(Expr::none(ty.clone(), span)),
        }
    } else if head == Result_::map_err {
        let [r, f] = args else { return None };
        map_err(r, f, ty, span)
    } else {
        return None;
    })
}

/// `hax_iter_advanced(iter)`: the iterator left over by `find`/`find_map`/
/// `any`. The loop does not track how far it got, so this is opaque.
fn advanced_iterator(iter: Expr) -> Expr {
    let ty = iter.ty.clone();
    let span = iter.meta.span;
    ExprKind::Quote {
        contents: Quote(vec![
            QuoteContent::Verbatim("hax_iter_advanced(".to_string()),
            QuoteContent::Expr(iter),
            QuoteContent::Verbatim(")".to_string()),
        ]),
    }
    .promote(ty, span)
}

impl AstVisitorMut for ProverifCombinatorsToLoopsVisitor {
    setup_error_handling_impl!();

    fn visit_expr(&mut self, expr: &mut Expr) {
        self.visit_inner(expr);

        let ExprKind::App { head, args, .. } = &*expr.kind else {
            return;
        };
        let ExprKind::GlobalId(head) = &*head.kind else {
            return;
        };
        let span = expr.meta.span;
        // `find`/`find_map`/`any` take `&mut self`, so after functionalization
        // they return `(iterator', result)`: the loop computes the result, and
        // an opaque term stands for the advanced iterator.
        let result_ty = if [Iterator::find, Iterator::find_map, Iterator::any].contains(head)
            && let TyKind::App { head: tuple, args } = expr.ty.kind()
            && let Some(global_id::TupleId::Type { length: 2 }) = tuple.expect_tuple()
            && let Some(GenericValue::Ty(result_ty)) = args.get(1)
        {
            Some(result_ty.clone())
        } else {
            None
        };
        let ty = expr.ty.clone();
        let Some(new_kind) = rewrite(*head, args, result_ty.as_ref().unwrap_or(&ty), span) else {
            return;
        };
        *expr = match result_ty {
            Some(result_ty) => Expr::tuple(
                vec![
                    advanced_iterator(args[0].clone()),
                    new_kind.promote(result_ty, span),
                ],
                span,
            ),
            None => new_kind.promote(ty, span),
        };
    }
}

impl Phase for ProverifCombinatorsToLoops {
    fn apply(&self, items: &mut Vec<Item>) {
        ProverifCombinatorsToLoopsVisitor::default().visit(items)
    }
}
