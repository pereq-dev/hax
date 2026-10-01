//! The "resugaring" phases used by printers.

//! This module defines resugarings instances (see
//! [`hax_rust_engine::ast::Resugaring`] for the definition of a
//! resugaring). Each backend defines its own set of resugaring phases.

use crate::ast::identifiers::GlobalId;
use crate::ast::resugared::*;
use crate::ast::visitors::*;
use crate::ast::*;
use crate::printer::*;

/// Transforms [`ItemKind::Fn`] of arity zero into [`ResugaredItemKind::Constant`].
/// Rust `const` items are encoded by the `ImportThir` phase of the hax engine as function of arity zero.
/// Functions of arity zero themselves are encoded as functions operating on one argument of type `()`.
#[derive(Copy, Clone, Default)]
pub struct FunctionsToConstants;

impl AstVisitorMut for FunctionsToConstants {
    fn enter_item_kind(&mut self, item_kind: &mut ItemKind) {
        let ItemKind::Fn {
            name,
            generics,
            body,
            params,
            safety: SafetyKind::Safe,
        } = item_kind
        else {
            return;
        };
        if !params.is_empty() {
            return;
        }
        *item_kind = ItemKind::Resugared(ResugaredItemKind::Constant {
            name: *name,
            body: body.clone(),
            generics: generics.clone(),
        });
    }
    fn enter_impl_item_kind(&mut self, item_kind: &mut ImplItemKind) {
        if let ImplItemKind::Fn { body, params } = item_kind
            && params.is_empty()
        {
            *item_kind =
                ImplItemKind::Resugared(ResugaredImplItemKind::Constant { body: body.clone() })
        }
    }
}

impl Resugaring for FunctionsToConstants {
    fn name(&self) -> String {
        "functions-to-constants".to_string()
    }
}

/// Tuples resugaring. Resugars tuple constructors to the dedicated expression variant [`ResugaredExprKind::Tuple`],
/// and tuple types to the dedicated type variant [`ResugaredTyKind::Tuple`].
pub struct Tuples;

impl AstVisitorMut for Tuples {
    fn enter_expr_kind(&mut self, x: &mut ExprKind) {
        let (constructor, fields) = match x {
            ExprKind::Construct {
                constructor,
                is_record: false,
                is_struct: true,
                base: None,
                fields,
            } => (constructor, &fields[..]),
            ExprKind::GlobalId(constructor) => (constructor, &[][..]),
            _ => return,
        };
        if constructor.expect_tuple().is_some() {
            let args = fields.iter().map(|(_, e)| e).cloned().collect();
            *x = ExprKind::Resugared(ResugaredExprKind::Tuple(args))
        }
    }
    fn enter_ty_kind(&mut self, x: &mut TyKind) {
        let TyKind::App { head, args } = x else {
            return;
        };
        if head.expect_tuple().is_some() {
            let Some(args) = args
                .iter()
                .map(GenericValue::expect_ty)
                .collect::<Option<Vec<_>>>()
            else {
                return;
            };
            *x = TyKind::Resugared(ResugaredTyKind::Tuple(args.into_iter().cloned().collect()))
        }
    }
}

impl Resugaring for Tuples {
    fn name(&self) -> String {
        "tuples".to_string()
    }
}

/// Let-pure resugaring. Use to identify expressions of the form `let x ← pure ..`, where the arrow
/// can be turned into a normal assignment `:=`
pub struct LetPure;

impl AstVisitorMut for LetPure {
    fn enter_expr_kind(&mut self, expr: &mut ExprKind) {
        const PURE: GlobalId = crate::names::rust_primitives::hax::explicit_monadic::pure;
        if let ExprKind::Let { lhs, rhs, body } = expr
            && let ExprKind::App {
                head,
                args,
                generic_args,
                bounds_impls,
                trait_: None,
            } = rhs.kind()
            && *head.kind() == ExprKind::GlobalId(PURE)
            && let ([pure_rhs], [], []) = (&args[..], &generic_args[..], &bounds_impls[..])
        {
            *expr = ExprKind::Resugared(ResugaredExprKind::LetPure {
                lhs: lhs.clone(),
                rhs: pure_rhs.clone(),
                body: body.clone(),
            })
        }
    }
}

impl Resugaring for LetPure {
    fn name(&self) -> String {
        "let_pure".to_string()
    }
}

/// Recursive function detection. Identifies functions whose body contains a
/// reference to their own name and resugars them to [`ResugaredItemKind::RecursiveFn`].
#[derive(Copy, Clone, Default)]
pub struct RecursiveFunctions;

impl AstVisitorMut for RecursiveFunctions {
    fn visit_item_kind(&mut self, item_kind: &mut ItemKind) {
        if let ItemKind::Fn {
            name,
            generics,
            body,
            params,
            safety,
        } = &*item_kind
            && body.mentions(*name)
        {
            *item_kind = ItemKind::Resugared(ResugaredItemKind::RecursiveFn {
                name: *name,
                generics: generics.clone(),
                body: body.clone(),
                params: params.clone(),
                safety: safety.clone(),
            });
        }
    }
}

impl Resugaring for RecursiveFunctions {
    fn name(&self) -> String {
        "recursive-functions".to_string()
    }
}

/// Record ellipsis resugaring. Identifies record-like `Construct` patterns where
/// some fields are wildcards and resugars them into `ConstructWithEllipsis`,
/// dropping the wildcard fields so the printer can emit `..`.
pub struct RecordEllipsis;

impl AstVisitorMut for RecordEllipsis {
    fn enter_pat_kind(&mut self, x: &mut PatKind) {
        let PatKind::Construct {
            constructor,
            is_record: true,
            is_struct,
            fields,
        } = x
        else {
            return;
        };
        let non_wild: Vec<_> = fields
            .iter()
            .filter(|(_, pat)| !matches!(&*pat.kind, PatKind::Wild))
            .cloned()
            .collect();
        if non_wild.len() < fields.len() {
            *x = ResugaredPatKind::ConstructWithEllipsis {
                constructor: *constructor,
                is_struct: *is_struct,
                fields: non_wild,
            }
            .into();
        }
    }
}

impl Resugaring for RecordEllipsis {
    fn name(&self) -> String {
        "record-ellipsis".to_string()
    }
}
