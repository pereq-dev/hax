//! ProVerif-backend phase: resolve trait-method calls to concrete impl methods.
//!
//! ProVerif has no traits, so a trait-method call must name the impl method
//! it dispatches to. A call is rewritten when exactly one impl's trait goal
//! matches the call's resolved goal (`ImplExpr::goal`), the impl's own generic
//! parameters matching anything; references are peeled only when no impl
//! matches exactly, for std's blanket impls on references.
//!
//! `x.into()` resolves as `From::from(x)`. A call dispatching to a default
//! method is rejected, except `ne`, which is
//! `!eq`, and the methods of a derived impl, which `primitives.pvl` models. So are an ambiguous call and an unresolved call to a trait of the
//! crate (e.g. on a generic `Self`). Calls to other traits (e.g. std traits
//! modelled in `primitives.pvl`) are left untouched.

use std::collections::{HashMap, HashSet};

use crate::ast::identifiers::GlobalId;
use crate::ast::identifiers::global_id::view::PathSegmentPayload;
use crate::ast::*;
use crate::ast::{diagnostics::*, visitors::*};
use crate::backends::proverif::analysis::visit_emitted;
use crate::backends::proverif::{kept_call_attribute, unextracted_attribute, warning_attribute};
use crate::names::core::cmp::PartialEq;
use crate::names::core::convert;
use crate::names::core::ops::bit::Not;
use crate::phase::Phase;
use hax_types::cli_options::Namespace;

/// Phase resolving user trait-method calls to their concrete impl methods.
pub struct ProverifResolveTraitCalls<'a> {
    /// The `--keep-trait-calls` paths: calls of the trait items they match
    /// stay calls of the trait item.
    pub kept: &'a [String],
}

/// A trait impl, with what a call must match to dispatch to it.
struct Candidate {
    /// The impl's generic parameters, which match anything.
    vars: HashSet<LocalId>,
    /// The impl's trait goal: `Self` followed by the trait's arguments.
    goal_args: Vec<GenericValue>,
    /// The methods the impl defines, by name.
    methods: HashMap<String, GlobalId>,
    /// Whether the impl is derived, whose methods are left to `primitives.pvl`.
    derived: bool,
}

#[setup_error_handling_struct]
#[derive(Default)]
struct ResolveTraitCallsVisitor {
    /// The impls of each trait.
    impls: HashMap<GlobalId, Vec<Candidate>>,
    /// The traits the crate defines.
    local_traits: HashSet<GlobalId>,
    /// The types the crate defines.
    local_types: HashSet<GlobalId>,
    /// The `--keep-trait-calls` paths.
    kept: Vec<Namespace>,
}

/// Whether `ty` is one of `types`, possibly behind references: by the orphan
/// rule, an impl for such a type (or with such a trait argument) is in the
/// crate.
fn is_local_type(ty: &Ty, types: &HashSet<GlobalId>) -> bool {
    matches!(peel_refs(ty).kind(), TyKind::App { head, .. } if types.contains(head))
}

/// What a trait-method call dispatches to.
enum Resolution {
    Method(GlobalId),
    /// `ne` by default: `!eq`.
    NotEq(GlobalId),
    Unknown,
    /// A method of an impl that is not extracted: an opaque constructor.
    Unextracted(String),
    /// Left as it is, with a warning.
    Warned(String),
    Rejected(String),
}

impl VisitorWithContext for ResolveTraitCallsVisitor {
    fn context(&self) -> Context {
        Context::Phase(stringify!(ProverifResolveTraitCalls).to_string())
    }
}

/// Matches the generic arguments of a call's trait goal against an impl's,
/// binding the impl's generic parameters consistently. With `peel`,
/// references are peeled on both sides.
struct Matcher<'a> {
    vars: &'a HashSet<LocalId>,
    bound: HashMap<LocalId, GenericValue>,
    peel: bool,
}

impl Matcher<'_> {
    fn bind(&mut self, var: &LocalId, value: GenericValue) -> bool {
        match self.bound.get(var) {
            Some(previous) => *previous == value,
            None => {
                self.bound.insert(var.clone(), value);
                true
            }
        }
    }

    fn generic(&mut self, pattern: &GenericValue, value: &GenericValue) -> bool {
        match (pattern, value) {
            (GenericValue::Ty(pattern), GenericValue::Ty(value)) => self.ty(pattern, value),
            (GenericValue::Expr(pattern), GenericValue::Expr(value)) => self.expr(pattern, value),
            (GenericValue::Lifetime, GenericValue::Lifetime) => true,
            _ => false,
        }
    }

    fn generics(&mut self, patterns: &[GenericValue], values: &[GenericValue]) -> bool {
        patterns.len() == values.len()
            && patterns
                .iter()
                .zip(values)
                .all(|(pattern, value)| self.generic(pattern, value))
    }

    fn ty(&mut self, pattern: &Ty, value: &Ty) -> bool {
        let (pattern, value) = if self.peel {
            (peel_refs(pattern), peel_refs(value))
        } else {
            (pattern, value)
        };
        match (pattern.kind(), value.kind()) {
            (TyKind::Param(var), _) if self.vars.contains(var) => {
                self.bind(var, GenericValue::Ty(value.clone()))
            }
            (
                TyKind::App { head, args },
                TyKind::App {
                    head: value_head,
                    args: value_args,
                },
            ) => head == value_head && self.generics(args, value_args),
            (TyKind::Slice(pattern), TyKind::Slice(value)) => self.ty(pattern, value),
            (
                TyKind::Array { ty, length },
                TyKind::Array {
                    ty: value_ty,
                    length: value_length,
                },
            ) => self.ty(ty, value_ty) && self.expr(length, value_length),
            (
                TyKind::Arrow { inputs, output },
                TyKind::Arrow {
                    inputs: value_inputs,
                    output: value_output,
                },
            ) => {
                inputs.len() == value_inputs.len()
                    && inputs
                        .iter()
                        .zip(value_inputs)
                        .all(|(pattern, value)| self.ty(pattern, value))
                    && self.ty(output, value_output)
            }
            _ => pattern == value,
        }
    }

    fn expr(&mut self, pattern: &Expr, value: &Expr) -> bool {
        match &*pattern.kind {
            ExprKind::LocalId(var) if self.vars.contains(var) => {
                self.bind(var, GenericValue::Expr(value.clone()))
            }
            kind => *kind == *value.kind,
        }
    }
}

fn peel_refs(ty: &Ty) -> &Ty {
    match ty.kind() {
        TyKind::Ref { inner, .. } => peel_refs(inner),
        _ => ty,
    }
}

/// The final path segment's name (the method name), if it is a named segment.
fn method_name(g: GlobalId) -> Option<String> {
    match g.view().last().payload() {
        PathSegmentPayload::Named(s) => Some(s.to_string()),
        PathSegmentPayload::Unnamed(_) => None,
    }
}

impl ResolveTraitCallsVisitor {
    /// What a call of `method` resolves to, given that its impl is a
    /// `concrete` one rather than a bound.
    fn resolve(&self, goal: &TraitGoal, method: &str, concrete: bool) -> Resolution {
        let impls = self.impls.get(&goal.trait_).map_or(&[][..], Vec::as_slice);
        let matching = |peel| -> Vec<&Candidate> {
            impls
                .iter()
                .filter(|candidate| {
                    Matcher {
                        vars: &candidate.vars,
                        bound: HashMap::new(),
                        peel,
                    }
                    .generics(&candidate.goal_args, &goal.args)
                })
                .collect()
        };
        let exact = matching(false);
        let candidates = if exact.is_empty() {
            matching(true)
        } else {
            exact
        };
        match candidates[..] {
            [candidate] => match candidate.methods.get(method) {
                Some(m) => Resolution::Method(*m),
                None if candidate.derived => Resolution::Unknown,
                None if goal.trait_ == PartialEq && method == "ne" => {
                    match candidate.methods.get("eq") {
                        Some(eq) => Resolution::NotEq(*eq),
                        None => Resolution::Rejected(
                            "`ne` of a `PartialEq` impl whose `eq` is a default".into(),
                        ),
                    }
                }
                // The model has no default bodies of other crates' traits: the
                // call stays one of the trait's item, declared on its own.
                None if !self.local_traits.contains(&goal.trait_) => {
                    Resolution::Warned(format!(
                        "use of the default trait item `{method}`, modelled as an undefined symbol of the trait"
                    ))
                }
                None => Resolution::Rejected(format!(
                    "use of the default trait item `{method}`, which the impl does not override"
                )),
            },
            // An impl whose goal mentions a type of the crate is in the crate.
            [] if concrete
                && goal.args.iter().any(|arg| {
                    matches!(arg, GenericValue::Ty(ty) if is_local_type(ty, &self.local_types))
                }) =>
            {
                Resolution::Unextracted(format!(
                    "use of the trait item `{method}` of an impl that is not extracted (e.g. excluded), modelled as an opaque symbol"
                ))
            }
            [] if !self.local_traits.contains(&goal.trait_) => Resolution::Unknown,
            [] => Resolution::Rejected(format!(
                "use of the trait item `{method}` that resolves to no impl of the crate (e.g. on a generic type)"
            )),
            // References are dropped, so impls for `T` and `&T` look alike.
            _ if !self.local_traits.contains(&goal.trait_) => Resolution::Warned(format!(
                "ambiguous use of the trait item `{method}`: several impls match, so it is modelled as an undefined symbol of the trait"
            )),
            _ => Resolution::Rejected(format!(
                "ambiguous use of the trait item `{method}`: several impls match"
            )),
        }
    }
}

impl AstVisitorMut for ResolveTraitCallsVisitor {
    setup_error_handling_impl!();

    fn visit_expr(&mut self, expr: &mut Expr) {
        self.visit_inner(expr);

        let ExprKind::App {
            head,
            trait_: Some((impl_expr, _)),
            ..
        } = &*expr.kind
        else {
            return;
        };
        let ExprKind::GlobalId(g) = &*head.kind else {
            return;
        };
        let Some(mname) = method_name(*g) else {
            return;
        };
        if self.kept.iter().any(|kept| g.matches_namespace(kept)) {
            let span = expr.meta.span;
            expr.meta.attributes.push(kept_call_attribute(span));
            return;
        }
        let with_head = |expr: &mut Expr, method: GlobalId| {
            if let ExprKind::App { head, .. } = &mut *expr.kind {
                *head.kind = ExprKind::GlobalId(method);
            }
        };
        let concrete = matches!(
            &*impl_expr.kind,
            ImplExprKind::Concrete(_) | ImplExprKind::ImplApp { .. } | ImplExprKind::Self_
        );
        // `x.into()` is `From::from(x)`, for the blanket `Into` impl.
        let resolution = match &impl_expr.goal.args[..] {
            [src, dst] if *g == convert::Into::into => self.resolve(
                &TraitGoal {
                    trait_: convert::From,
                    args: vec![dst.clone(), src.clone()],
                },
                "from",
                concrete,
            ),
            _ => self.resolve(&impl_expr.goal, &mname, concrete),
        };
        match resolution {
            Resolution::Method(method) => with_head(expr, method),
            Resolution::NotEq(eq) => {
                let mut eq_call = expr.clone();
                with_head(&mut eq_call, eq);
                *expr = Expr::standalone_fn_app(
                    Not::not,
                    vec![],
                    vec![eq_call],
                    expr.ty.clone(),
                    expr.meta.span,
                );
            }
            Resolution::Unknown => {}
            Resolution::Unextracted(message) => {
                let span = expr.meta.span;
                expr.meta.attributes.push(unextracted_attribute(span));
                expr.meta.attributes.push(warning_attribute(&message, span));
            }
            Resolution::Warned(message) => {
                let span = expr.meta.span;
                expr.meta.attributes.push(warning_attribute(&message, span));
            }
            Resolution::Rejected(details) => self.error(
                expr.clone(),
                DiagnosticInfoKind::Unimplemented {
                    issue_id: None,
                    details: Some(details),
                },
            ),
        }
    }
}

impl Phase for ProverifResolveTraitCalls<'_> {
    fn apply(&self, items: &mut Vec<Item>) {
        let mut impls: HashMap<GlobalId, Vec<Candidate>> = HashMap::new();
        let mut local_traits = HashSet::new();
        let mut local_types = HashSet::new();
        for item in items.iter() {
            let (generics, of_trait, impl_items) = match &item.kind {
                ItemKind::Type { name, .. } => {
                    local_types.insert(*name);
                    continue;
                }
                ItemKind::Trait { .. } => {
                    local_traits.insert(item.ident);
                    continue;
                }
                ItemKind::Impl {
                    generics,
                    of_trait,
                    items,
                    ..
                } => (generics, of_trait, items),
                _ => continue,
            };
            let methods = impl_items
                .iter()
                .filter(|ii| matches!(ii.kind, ImplItemKind::Fn { .. }))
                .filter_map(|ii| Some((method_name(ii.ident)?, ii.ident)))
                .collect();
            impls.entry(of_trait.trait_).or_default().push(Candidate {
                vars: generics.params.iter().map(|p| p.ident.clone()).collect(),
                goal_args: of_trait.args.clone(),
                methods,
                derived: item.meta.attributes.iter().any(|a| {
                    matches!(&a.kind, AttributeKind::Tool { path, .. } if path == "automatically_derived")
                }),
            });
        }

        let mut visitor = ResolveTraitCallsVisitor {
            impls,
            local_traits,
            local_types,
            kept: self.kept.iter().cloned().map(Namespace::from).collect(),
            ..Default::default()
        };
        // Only the emitted code needs its calls resolved.
        let visitor = std::cell::RefCell::new(&mut visitor);
        visit_emitted(
            items,
            |item| visitor.borrow_mut().visit(item),
            |ii| visitor.borrow_mut().visit(ii),
        )
    }
}
