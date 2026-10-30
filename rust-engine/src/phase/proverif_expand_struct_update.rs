//! ProVerif-backend phase: normalize record construction to full positional
//! constructors in declaration order, and expand struct-update constructors.
//!
//! ProVerif constructors are positional and there is no record-update syntax,
//! while the printer emits fields in AST order and projectors in declaration
//! order. Since everything is `bitstring`, a mismatch still type-checks but is
//! unsound. This phase rewrites:
//!
//!  - `S { f: x, ..base }`, and the `self = Construct { constructor = <type id>,
//!    fields = [(f, x)], base = Some(self) }` that `TrivializeAssignLhs` makes
//!    of `self.f = x`, into a `base: None` constructor whose missing fields are
//!    projected from the base;
//!  - record literals and patterns into declaration order, filling the fields
//!    a pattern omits with wildcards.

use std::collections::HashMap;

use crate::ast::identifiers::GlobalId;
use crate::ast::identifiers::global_id::TupleId;
use crate::ast::visitors::*;
use crate::ast::*;
use crate::backends::proverif::generated_local;
use crate::names::core::ops::range::Range;
use crate::phase::Phase;

/// Phase that rewrites record constructors to full positional ones.
#[derive(Default, Debug)]
pub struct ProverifExpandStructUpdate;

/// The ordered field list and constructor id of a struct, indexed (in the
/// visitor) by both the type id and the variant id.
#[derive(Clone)]
struct StructInfo {
    /// The variant constructor id (e.g. `ClientState__ClientState`).
    constructor: GlobalId,
    /// Whether the fields are named.
    is_record: bool,
    /// The fields in declaration order: `(field id, field type)`.
    fields: Vec<(GlobalId, Ty)>,
}

impl StructInfo {
    /// Every field in declaration order: the one given in `explicit`, or else
    /// `missing(field, type)`. `None` if `missing` gives none.
    fn reorder<T>(
        &self,
        mut explicit: Vec<(GlobalId, T)>,
        mut missing: impl FnMut(GlobalId, &Ty) -> Option<T>,
    ) -> Option<Vec<(GlobalId, T)>> {
        self.fields
            .iter()
            .map(|(field, ty)| {
                let value = match explicit.iter().position(|(g, _)| g == field) {
                    Some(i) => explicit.swap_remove(i).1,
                    None => missing(*field, ty)?,
                };
                Some((*field, value))
            })
            .collect()
    }
}

/// The layout of the tuple of type `ty`, if `constructor` names a tuple type
/// or constructor: `x.0 = v` is the record update `Tuple { 0: v, ..x }`.
fn tuple_info(constructor: GlobalId, ty: &Ty) -> Option<StructInfo> {
    let (TupleId::Type { length } | TupleId::Constructor { length }) =
        constructor.expect_tuple()?
    else {
        return None;
    };
    let TyKind::App { args, .. } = ty.kind() else {
        return None;
    };
    let fields = args
        .iter()
        .enumerate()
        .map(|(field, arg)| match arg {
            GenericValue::Ty(ty) => Some((TupleId::Field { length, field }.into(), ty.clone())),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()?;
    (fields.len() == length).then(|| StructInfo {
        constructor: TupleId::Constructor { length }.into(),
        is_record: false,
        fields,
    })
}

struct ExpandStructUpdateVisitor {
    /// Lookup keyed by both the struct's type id and its variant id.
    structs: HashMap<GlobalId, StructInfo>,
}

impl AstVisitorMut for ExpandStructUpdateVisitor {
    fn visit_expr(&mut self, expr: &mut Expr) {
        // Recurse first so a base expression that is itself a struct update is
        // already expanded before we project fields out of it.
        self.visit_inner(expr);

        let ExprKind::Construct {
            constructor,
            fields,
            base,
            is_struct,
            is_record,
        } = &mut *expr.kind
        else {
            return;
        };
        // A positional constructor with no base is already in declaration order.
        if base.is_none() && !*is_record {
            return;
        }
        let tuple = tuple_info(*constructor, &expr.ty);
        let Some(info) = self.structs.get(constructor).or(tuple.as_ref()) else {
            // `Range`'s declaration order, as `primitives.pvl` declares it.
            if *constructor == Range::Constructor && base.is_none() {
                fields.sort_by_key(|(field, _)| *field != Range::start);
            }
            return;
        };
        let span = expr.meta.span;
        let missing = info
            .fields
            .iter()
            .filter(|(field, _)| !fields.iter().any(|(g, _)| g == field))
            .count();
        if base.is_none() && missing > 0 {
            return;
        }
        // A base that is projected more than once is bound first, so that it
        // is evaluated once, as in Rust.
        let mut binding = None;
        if let Some(b) = base
            && missing > 1
            && !matches!(&*b.kind, ExprKind::LocalId(_))
        {
            let var = generated_local("hax__struct_base");
            let local = ExprKind::LocalId(var.clone()).promote(b.ty.clone(), span);
            binding = Some((var, std::mem::replace(b, local)));
        }
        let project = |field, ty: &Ty| {
            let base = base.as_ref()?;
            Some(Expr::standalone_fn_app(
                field,
                vec![],
                vec![base.clone()],
                ty.clone(),
                span,
            ))
        };
        let Some(new_fields) = info.reorder(std::mem::take(fields), project) else {
            unreachable!("the base fills every missing field")
        };
        let construct = ExprKind::Construct {
            constructor: info.constructor,
            is_record: info.is_record,
            is_struct: *is_struct,
            fields: new_fields,
            base: None,
        }
        .promote(expr.ty.clone(), span);
        *expr = match binding {
            Some((var, rhs)) => ExprKind::Let {
                lhs: PatKind::var_pat(var).promote(rhs.ty.clone(), span),
                rhs,
                body: construct,
            }
            .promote(expr.ty.clone(), span),
            None => construct,
        };
    }

    /// The same declaration-order normalization for record patterns.
    fn visit_pat(&mut self, pat: &mut Pat) {
        self.visit_inner(pat);

        let PatKind::Construct {
            constructor,
            fields,
            is_struct,
            is_record: true,
        } = &mut *pat.kind
        else {
            return;
        };
        let Some(info) = self.structs.get(constructor) else {
            return;
        };
        let span = pat.meta.span;
        let wild = |_, ty: &Ty| Some(PatKind::Wild.promote(ty.clone(), span));
        let Some(fields) = info.reorder(std::mem::take(fields), wild) else {
            unreachable!("a wildcard fills every missing field")
        };
        *pat = PatKind::Construct {
            constructor: info.constructor,
            is_record: info.is_record,
            is_struct: *is_struct,
            fields,
        }
        .promote(pat.ty.clone(), span);
    }
}

impl Phase for ProverifExpandStructUpdate {
    fn apply(&self, items: &mut Vec<Item>) {
        // Structs are also indexed by type id: `TrivializeAssignLhs` uses it as
        // the constructor.
        let mut structs: HashMap<GlobalId, StructInfo> = HashMap::new();
        for item in items.iter() {
            let ItemKind::Type {
                name,
                variants,
                is_struct,
                ..
            } = &item.kind
            else {
                continue;
            };
            for variant in variants {
                let info = StructInfo {
                    constructor: variant.name,
                    is_record: variant.is_record,
                    fields: variant
                        .arguments
                        .iter()
                        .map(|(id, ty, _)| (*id, ty.clone()))
                        .collect(),
                };
                structs.insert(variant.name, info.clone());
                if *is_struct {
                    structs.insert(*name, info);
                }
            }
        }

        let mut visitor = ExpandStructUpdateVisitor { structs };
        for item in items.iter_mut() {
            visitor.visit_item(item)
        }
    }
}
