//! ProVerif-backend phase: reject constructs the model would get silently
//! wrong, and that have no simple correct encoding.

use std::collections::HashSet;

use crate::ast::*;
use crate::ast::{diagnostics::*, visitors::*};
use crate::backends::is_late_skip;
use crate::backends::proverif::analysis::hides_body;
use crate::backends::proverif::{primitives_pvl_names, render_id, render_local, warning_attribute};
use crate::names::core::ops::range::Range;
use crate::phase::Phase;
use hax_lib_macros_types::AttrPayload;

/// Phase rejecting constructs the ProVerif model cannot express faithfully.
#[derive(Default, Debug)]
pub struct ProverifRejectUnsupported;

#[setup_error_handling_struct]
#[derive(Default)]
struct RejectUnsupported {
    /// The const generic parameters in scope, which the model erases.
    const_generics: HashSet<LocalId>,
    /// The field accessors the model declares.
    accessors: HashSet<String>,
    /// The structs of the crate, by type and constructor, whose field order
    /// is known.
    structs: HashSet<GlobalId>,
}

impl VisitorWithContext for RejectUnsupported {
    fn context(&self) -> Context {
        Context::Phase(stringify!(ProverifRejectUnsupported).to_string())
    }
}

/// Whether `pat` contains an or-pattern.
fn has_or_pattern(pat: &Pat) -> bool {
    struct Finder(bool);
    impl AstVisitor for Finder {
        fn enter_pat_kind(&mut self, kind: &PatKind) {
            self.0 |= matches!(kind, PatKind::Or { .. });
        }
    }
    let mut finder = Finder(false);
    finder.visit_pat(pat);
    finder.0
}

impl RejectUnsupported {
    fn reject(&mut self, expr: &Expr, details: &str) {
        self.error(
            expr.clone(),
            DiagnosticInfoKind::Unimplemented {
                issue_id: None,
                details: Some(details.into()),
            },
        )
    }
}

impl AstVisitorMut for RejectUnsupported {
    setup_error_handling_impl!();

    // Types and generic arguments are erased, so a const generic in them is
    // harmless.
    fn visit_ty(&mut self, _: &mut Ty) {}
    fn visit_generic_value(&mut self, _: &mut GenericValue) {}

    fn visit_expr(&mut self, expr: &mut Expr) {
        self.visit_inner(expr);
        match &*expr.kind {
            // A field read on a struct whose accessors the model does not
            // declare would be an unrelated one-way function.
            ExprKind::App { head, args, .. }
                if args.len() == 1
                    && matches!(&*head.kind, ExprKind::GlobalId(g)
                        if g.is_projector()
                            && g.expect_tuple().is_none()
                            && !self.accessors.contains(&render_id(g))) =>
            {
                self.reject(
                    expr,
                    "read of a field of a struct the model does not declare (e.g. an opaque or external one)",
                )
            }
            // The field order of an external record is unknown, so its literal
            // may not match the positional constructor.
            ExprKind::Construct {
                constructor,
                fields,
                base,
                is_record: true,
                ..
            } if (fields.len() > 1 || base.is_some())
                && *constructor != Range::Constructor
                && !self.structs.contains(constructor) =>
            {
                self.reject(expr, "record literal of a struct whose field order is unknown")
            }
            // Their value depends on the erased type argument only.
            ExprKind::App { head, .. }
                if matches!(&*head.kind, ExprKind::GlobalId(g)
                    if ["core__mem__size_of", "core__mem__align_of"].contains(&render_id(g).as_str())) =>
            {
                let span = expr.meta.span;
                expr.meta.attributes.push(warning_attribute(
                    "`size_of` or `align_of` modelled without its type argument, so that it is the same for every type",
                    span,
                ))
            }
            ExprKind::LocalId(id) if self.const_generics.contains(id) => {
                self.reject(expr, "use of a const generic parameter as a value")
            }
            // A failing guard must retry the next alternative, which the
            // guard compilation of `ProverifBindFailable` does not do.
            ExprKind::Match { arms, .. }
                if arms
                    .iter()
                    .any(|arm| arm.guard.is_some() && has_or_pattern(&arm.pat)) =>
            {
                self.reject(expr, "match arm with both an or-pattern and a guard")
            }
            _ => {}
        }
    }
}

impl Phase for ProverifRejectUnsupported {
    fn apply(&self, items: &mut Vec<Item>) {
        let const_generics = |generics: &[&Generics]| -> HashSet<LocalId> {
            generics
                .iter()
                .flat_map(|g| &g.params)
                .filter(|p| matches!(p.kind, GenericParamKind::Const { .. }))
                .map(|p| p.ident.clone())
                .collect()
        };
        let mut accessors: HashSet<String> = primitives_pvl_names().into_iter().collect();
        let mut structs = HashSet::new();
        for item in items.iter() {
            if let ItemKind::Type { name, variants, .. } = &item.kind {
                structs.insert(*name);
                structs.extend(variants.iter().map(|v| v.name));
            }
            if let ItemKind::Type { variants, .. } = &item.kind
                && !item
                    .meta
                    .hax_attributes()
                    .any(|a| matches!(a, AttrPayload::Erased))
            {
                for variant in variants {
                    accessors.extend(
                        variant
                            .arguments
                            .iter()
                            .map(|(field, _, _)| render_id(field)),
                    );
                }
            }
        }
        let mut visitor = RejectUnsupported {
            accessors,
            structs,
            ..Default::default()
        };
        // The bodies the printer omits need no model.
        for item in items.iter_mut().filter(|item| !is_late_skip(&item.meta)) {
            match &mut item.kind {
                ItemKind::Fn { params, .. } if hides_body(&item.meta, params) => {}
                ItemKind::Impl {
                    generics, items, ..
                } => {
                    for ii in items.iter_mut() {
                        match &ii.kind {
                            _ if is_late_skip(&ii.meta) => {}
                            ImplItemKind::Fn { params, .. } if hides_body(&ii.meta, params) => {}
                            _ => {
                                visitor.const_generics = const_generics(&[generics, &ii.generics]);
                                visitor.visit(ii)
                            }
                        }
                    }
                }
                ItemKind::Fn { generics, .. } => {
                    visitor.const_generics = const_generics(&[generics]);
                    visitor.visit(item)
                }
                _ => {
                    visitor.const_generics = HashSet::new();
                    visitor.visit(item)
                }
            }
        }
    }
}

/// Phase rejecting the std fallible conversions under `--result-errors`,
/// whose `result_errors.pvl` model always succeeds.
#[derive(Default, Debug)]
pub struct ProverifRejectTryConversions;

#[setup_error_handling_struct]
#[derive(Default)]
struct RejectTryConversions;

impl VisitorWithContext for RejectTryConversions {
    fn context(&self) -> Context {
        Context::Phase(stringify!(ProverifRejectTryConversions).to_string())
    }
}

impl AstVisitorMut for RejectTryConversions {
    setup_error_handling_impl!();

    fn visit_expr(&mut self, expr: &mut Expr) {
        self.visit_inner(expr);
        if let ExprKind::App { head, .. } = &*expr.kind
            && let ExprKind::GlobalId(g) = &*head.kind
            && [
                "core__convert__TryInto__try_into",
                "core__convert__TryFrom__try_from",
            ]
            .contains(&render_id(g).as_str())
        {
            self.error(
                expr.clone(),
                DiagnosticInfoKind::Unimplemented {
                    issue_id: None,
                    details: Some(
                        "std fallible conversion under `--result-errors`, whose error is not modelled"
                            .into(),
                    ),
                },
            )
        }
    }
}

impl Phase for ProverifRejectTryConversions {
    fn apply(&self, items: &mut Vec<Item>) {
        RejectTryConversions::default().visit(items)
    }
}

/// Phase warning, before `&mut` parameters are functionalized, about the
/// `replace_body` functions with one: the replacement gives the result, and
/// the model keeps the `&mut` parameters unchanged. That is right for an
/// `rng`, whose randomness the replacement draws with `new`, but not for a
/// value the Rust code updates.
#[derive(Default, Debug)]
pub struct ProverifMarkMutReplaceBody;

/// Whether `body` is only a verbatim quote, as `replace_body` makes it
/// (a `hax_lib::inline` call until `TransformHaxLibInline`).
fn is_replaced(body: &Expr) -> bool {
    match &*body.kind {
        ExprKind::App { head, .. } => matches!(&*head.kind, ExprKind::GlobalId(g)
            if *g == crate::names::hax_lib::inline || *g == crate::names::hax_lib::inline_unsafe),
        ExprKind::Block { body, .. } => is_replaced(body),
        _ => false,
    }
}

impl Phase for ProverifMarkMutReplaceBody {
    fn apply(&self, items: &mut Vec<Item>) {
        let mark = |meta: &mut Metadata, params: &[Param], body: &Expr| {
            let kept: Vec<String> = params
                .iter()
                .filter(|p| matches!(p.ty.kind(), TyKind::Ref { mutable: true, .. }))
                .map(|p| match p.pat.as_binder() {
                    Some(id) => format!("`{}`", render_local(id)),
                    None => "a pattern".into(),
                })
                .collect();
            if !kept.is_empty() && is_replaced(body) {
                let message = format!(
                    "`replace_body` keeps the `&mut` parameters {} unchanged",
                    kept.join(", ")
                );
                meta.attributes.push(warning_attribute(&message, meta.span));
            }
        };
        for item in items.iter_mut() {
            match &mut item.kind {
                ItemKind::Fn { params, body, .. } => mark(&mut item.meta, params, body),
                ItemKind::Impl { items, .. } => {
                    for ii in items {
                        if let ImplItemKind::Fn { params, body } = &ii.kind {
                            mark(&mut ii.meta, params, body)
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
