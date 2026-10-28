//! This modules allows to import the THIR AST produced by the frontend, and convert it to the engine's internal AST

use crate::ast;
use crate::ast::HasKind as _;
use crate::ast::identifiers::global_id::ReservedSuffix;
use crate::ast::identifiers::global_id::TupleId;
use crate::symbol::Symbol;
use hax_frontend_exporter as frontend;

fn unsupported(msg: &str, issue_id: u32, span: ast::span::Span) -> ast::ErrorNode {
    let fragment = ast::fragment::Fragment::Unknown(msg.to_owned());
    let diagnostic = ast::diagnostics::Diagnostic::new(
        fragment.clone(),
        ast::diagnostics::DiagnosticInfo {
            context: ast::diagnostics::Context::Import,
            span,
            kind: hax_types::diagnostics::Kind::Unimplemented {
                issue_id: Some(issue_id),
                details: Some(msg.to_owned()),
            },
        },
    );
    ast::ErrorNode {
        fragment: Box::new(fragment),
        diagnostics: vec![diagnostic],
    }
}
fn assertion_failure(msg: &str, span: ast::span::Span) -> ast::ErrorNode {
    let fragment = ast::fragment::Fragment::Unknown(msg.to_owned());
    let diagnostic = ast::diagnostics::Diagnostic::new(
        fragment.clone(),
        ast::diagnostics::DiagnosticInfo {
            context: ast::diagnostics::Context::Import,
            span,
            kind: hax_types::diagnostics::Kind::AssertionFailure {
                details: msg.to_owned(),
            },
        },
    );
    ast::ErrorNode {
        fragment: Box::new(fragment),
        diagnostics: vec![diagnostic],
    }
}

struct Context {
    owner_hint: Option<hax_frontend_exporter::DefId>,
    impl_expr_offset: u64,
}

fn is_self_type_constraint(
    gc: &ast::GenericConstraint,
    container: &frontend::AssocItemContainer,
) -> bool {
    let frontend::AssocItemContainer::TraitContainer { trait_ref } = container else {
        return false;
    };
    let trait_ = trait_ref.contents().def_id.import_as_value();

    match gc {
        ast::GenericConstraint::TypeClass(ast::ImplIdent { goal, .. }) =>
            goal
            .args
            .first()
            .and_then(ast::GenericValue::expect_ty)
            .map(|ty| matches!(ty.0.as_ref(), ast::TyKind::Param(local) if local.0 == Symbol::new("Self")))
            .unwrap_or(false)
            && goal.trait_ == trait_,
        _ => false,
    }
}

fn resugar_index_mut(expr: &ast::Expr) -> Option<(&ast::Expr, &ast::Expr)> {
    if let ast::ExprKind::App { head, args, .. } = expr.kind()
        && let ast::ExprKind::GlobalId(method) = head.kind()
        && let [lhs, index] = args.as_slice()
    {
        use crate::names::core::ops::index::*;
        match (*method, lhs.kind()) {
            (IndexMut::index_mut, ast::ExprKind::Borrow { inner: lhs, .. }) => Some((lhs, index)),
            (Index::index, _) => Some((lhs, index)),
            _ => None,
        }
    } else {
        None
    }
}

fn lhs_from_expr(expr: &ast::Expr) -> ast::Lhs {
    let ty = expr.ty.clone();
    if let ast::ExprKind::LocalId(var) = expr.kind() {
        return ast::Lhs::LocalVar {
            var: var.clone(),
            ty: ty.clone(),
        };
    }
    let expr = expr.unbox_underef();
    if let Some((e, index)) = resugar_index_mut(expr) {
        ast::Lhs::ArrayAccessor {
            e: Box::new(lhs_from_expr(e)),
            ty: ty.clone(),
            index: index.clone(),
        }
    } else if let ast::ExprKind::App { head, args, .. } = expr.kind()
        && let [arg] = args.as_slice()
        && let ast::ExprKind::GlobalId(field) = head.kind()
        && field.is_projector()
    {
        ast::Lhs::FieldAccessor {
            e: Box::new(lhs_from_expr(arg)),
            ty,
            field: *field,
        }
    } else {
        ast::Lhs::ArbitraryExpr(Box::new(expr.clone()))
    }
}

trait SpannedImport<Out> {
    fn spanned_import(&self, context: &Context, span: ast::span::Span) -> Out;
}

trait Import<Out> {
    fn import(&self, context: &Context) -> Out;
}

impl<T: Import<Out>, Out> Import<Vec<Out>> for Vec<T> {
    fn import(&self, context: &Context) -> Vec<Out> {
        self.iter()
            .map(|value| Import::import(value, context))
            .collect()
    }
}

impl<T: SpannedImport<Out>, Out> SpannedImport<Vec<Out>> for Vec<T> {
    fn spanned_import(&self, context: &Context, span: ast::span::Span) -> Vec<Out> {
        self.iter()
            .map(|value| value.spanned_import(context, span))
            .collect()
    }
}

trait DefIdImportHelpers {
    fn import_as_value(&self) -> ast::GlobalId;
    fn import_as_nonvalue(&self) -> ast::GlobalId;
}

impl DefIdImportHelpers for frontend::DefId {
    fn import_as_value(&self) -> ast::GlobalId {
        ast::GlobalId::from_frontend(self.clone(), true)
    }

    fn import_as_nonvalue(&self) -> ast::GlobalId {
        ast::GlobalId::from_frontend(self.clone(), false)
    }
}

impl Import<ast::span::Span> for frontend::Span {
    fn import(&self, context: &Context) -> ast::span::Span {
        ast::span::Span::from_exporter(self.clone(), context.owner_hint.as_ref())
    }
}

fn import_attributes(context: &Context, attrs: &[frontend::Attribute]) -> ast::Attributes {
    attrs.iter().flat_map(|attr| attr.import(context)).collect()
}

fn has_automatically_derived(attrs: &ast::Attributes) -> bool {
    attrs.iter().any(|attr| {
        matches!(
            attr.kind,
            ast::AttributeKind::Tool { ref path, .. } if path == "automatically_derived"
        )
    })
}

/// The markers rustc leaves behind when it expands `#[cfg(..)]` and
/// `#[cfg_attr(..)]` attributes. They exist only so that rustc can report better
/// spans in its own diagnostics: they carry no information for us. Note their
/// paths are purposefully invalid Rust identifiers, so that users cannot write
/// them: keeping them around makes the Rust printer produce code that cannot be
/// parsed back.
const CFG_TRACE_ATTRIBUTES: &[&str] = &["<cfg_trace>", "<cfg_attr_trace>"];

impl Import<Option<ast::Attribute>> for frontend::Attribute {
    fn import(&self, context: &Context) -> Option<ast::Attribute> {
        match self {
            frontend::Attribute::Unparsed(frontend::AttrItem { path, .. })
                if CFG_TRACE_ATTRIBUTES.contains(&path.as_str()) =>
            {
                None
            }
            frontend::Attribute::Parsed(frontend::AttributeKind::DocComment {
                kind,
                span,
                comment,
                ..
            }) => {
                let kind = match kind {
                    frontend::CommentKind::Block => ast::DocCommentKind::Block,
                    frontend::CommentKind::Line => ast::DocCommentKind::Line,
                };
                Some(ast::Attribute {
                    kind: ast::AttributeKind::DocComment {
                        kind,
                        body: comment.clone(),
                    },
                    span: span.import(context),
                })
            }
            frontend::Attribute::Parsed(frontend::AttributeKind::AutomaticallyDerived(span)) => {
                let kind = ast::AttributeKind::Tool {
                    path: "automatically_derived".to_owned(),
                    tokens: String::new(),
                };
                Some(ast::Attribute {
                    kind,
                    span: span.import(context),
                })
            }
            frontend::Attribute::Unparsed(frontend::AttrItem {
                path,
                args:
                    frontend::AttrArgs::Eq {
                        expr: frontend::MetaItemLit { symbol, .. },
                        ..
                    },
                span,
            }) if path == "doc" => {
                let kind = ast::AttributeKind::DocComment {
                    kind: ast::DocCommentKind::Line,
                    body: symbol.clone(),
                };
                Some(ast::Attribute {
                    kind,
                    span: span.import(context),
                })
            }
            frontend::Attribute::Unparsed(frontend::AttrItem { path, args, span }) => {
                let tokens =
                    if let frontend::AttrArgs::Delimited(frontend::DelimArgs { tokens, .. }) = args
                    {
                        tokens.clone()
                    } else {
                        String::new()
                    };
                Some(ast::Attribute {
                    kind: ast::AttributeKind::Tool {
                        path: path.clone(),
                        tokens,
                    },
                    span: span.import(context),
                })
            }
            _ => None,
        }
    }
}

impl Import<ast::Attributes> for Vec<frontend::Attribute> {
    fn import(&self, context: &Context) -> ast::Attributes {
        self.iter()
            .filter_map(|value| value.import(context))
            .collect()
    }
}

impl Import<ast::GenericParam> for frontend::GenericParamDef {
    fn import(&self, context: &Context) -> ast::GenericParam {
        let span = self.span.import(context);
        let frontend::GenericParamDef { name, kind, .. } = self;
        let kind = match kind {
            frontend::GenericParamDefKind::Lifetime => ast::GenericParamKind::Lifetime,
            frontend::GenericParamDefKind::Type { .. } => ast::GenericParamKind::Type,
            frontend::GenericParamDefKind::Const { ty, .. } => ast::GenericParamKind::Const {
                ty: ty.spanned_import(context, span),
            },
        };
        ast::GenericParam {
            ident: ast::LocalId(Symbol::new(name.clone())),
            meta: ast::Metadata {
                span,
                attributes: self.attributes.import(context),
            },
            kind,
        }
    }
}

impl SpannedImport<ast::GenericValue> for frontend::GenericArg {
    fn spanned_import(&self, context: &Context, span: ast::span::Span) -> ast::GenericValue {
        match self {
            frontend::GenericArg::Lifetime(_) => ast::GenericValue::Lifetime,
            frontend::GenericArg::Type(ty) => {
                ast::GenericValue::Ty(ty.spanned_import(context, span))
            }
            frontend::GenericArg::Const(decorated) => {
                ast::GenericValue::Expr(frontend::Expr::from(decorated.clone()).import(context))
            }
        }
    }
}

impl Import<Vec<ast::GenericParam>> for frontend::TyGenerics {
    fn import(&self, context: &Context) -> Vec<ast::GenericParam> {
        self.params
            .iter()
            .map(|value| value.import(context))
            .collect()
    }
}

impl SpannedImport<Vec<(ast::ImplExpr, ast::ImplIdent)>> for Vec<frontend::ImplExpr> {
    fn spanned_import(
        &self,
        context: &Context,
        span: ast::span::Span,
    ) -> Vec<(ast::ImplExpr, ast::ImplIdent)> {
        let impl_exprs: Vec<ast::ImplExpr> = self.spanned_import(context, span);
        impl_exprs
            .into_iter()
            .enumerate()
            .map(|(i, ie)| {
                let impl_ident = ast::ImplIdent {
                    goal: ie.goal.clone(),
                    name: impl_expr_name(i as u64),
                };
                (ie, impl_ident)
            })
            .collect()
    }
}

impl SpannedImport<Option<ast::GenericConstraint>> for frontend::Clause {
    fn spanned_import(
        &self,
        context: &Context,
        span: ast::span::Span,
    ) -> Option<ast::GenericConstraint> {
        match &self.kind.value {
            frontend::ClauseKind::Trait(trait_predicate) => {
                let args = trait_predicate
                    .trait_ref
                    .generic_args
                    .spanned_import(context, span);
                let trait_ = trait_predicate.trait_ref.def_id.import_as_nonvalue();
                let goal = ast::TraitGoal { trait_, args };

                Some(ast::GenericConstraint::TypeClass(ast::ImplIdent {
                    goal,
                    name: impl_expr_name(self.id.0),
                }))
            }
            frontend::ClauseKind::Projection(frontend::ProjectionPredicate {
                impl_expr,
                assoc_item,
                ty,
            }) => {
                let impl_ = impl_expr.spanned_import(context, span);
                let assoc_item = assoc_item.def_id.import_as_nonvalue();
                let ty = ty.spanned_import(context, span);
                Some(ast::GenericConstraint::Equality(ast::ProjectionPredicate {
                    impl_,
                    assoc_item,
                    ty,
                }))
            }
            _ => None,
        }
    }
}

impl Import<Vec<ast::GenericConstraint>> for frontend::GenericPredicates {
    fn import(&self, context: &Context) -> Vec<ast::GenericConstraint> {
        let mut type_idx: u64 = context.impl_expr_offset;
        self.predicates
            .iter()
            .filter_map(|(clause, span)| {
                let span = span.import(context);
                let mut gc = clause.spanned_import(context, span)?;
                if let ast::GenericConstraint::TypeClass(impl_ident) = &mut gc {
                    impl_ident.name = impl_expr_name(type_idx);
                    type_idx += 1;
                }
                Some(gc)
            })
            .collect()
    }
}

impl Import<ast::Generics> for frontend::ParamEnv {
    fn import(&self, context: &Context) -> ast::Generics {
        ast::Generics {
            params: self.generics.import(context),
            constraints: self.predicates.import(context),
        }
    }
}

impl Import<ast::SafetyKind> for frontend::Safety {
    fn import(&self, _context: &Context) -> ast::SafetyKind {
        match self {
            frontend::Safety::Unsafe => ast::SafetyKind::Unsafe,
            frontend::Safety::Safe => ast::SafetyKind::Safe,
        }
    }
}

fn import_fn_sig(
    context: &Context,
    fn_sig: &frontend::TyFnSig,
    span: ast::span::Span,
) -> ast::TyKind {
    let inputs = if fn_sig.inputs.is_empty() {
        vec![ast::TyKind::unit().promote()]
    } else {
        fn_sig
            .inputs
            .iter()
            .map(|t| t.spanned_import(context, span))
            .collect()
    };
    ast::TyKind::Arrow {
        inputs,
        output: fn_sig.output.spanned_import(context, span),
    }
}

impl SpannedImport<ast::Ty> for frontend::Ty {
    fn spanned_import(&self, context: &Context, span: ast::span::Span) -> ast::Ty {
        let kind = match self.kind() {
            frontend::TyKind::Bool => ast::TyKind::Primitive(ast::PrimitiveTy::Bool),
            frontend::TyKind::Char => ast::TyKind::Primitive(ast::PrimitiveTy::Char),
            frontend::TyKind::Int(int_ty) => {
                ast::TyKind::Primitive(ast::PrimitiveTy::Int(int_ty.into()))
            }
            frontend::TyKind::Uint(uint_ty) => {
                ast::TyKind::Primitive(ast::PrimitiveTy::Int(uint_ty.into()))
            }
            frontend::TyKind::Float(float_ty) => {
                ast::TyKind::Primitive(ast::PrimitiveTy::Float(float_ty.into()))
            }
            frontend::TyKind::FnDef { fn_sig, .. } | frontend::TyKind::Arrow(fn_sig) => {
                import_fn_sig(context, &fn_sig.as_ref().value, span)
            }
            frontend::TyKind::Closure(frontend::ClosureArgs { fn_sig, .. }) => {
                import_fn_sig(context, &fn_sig.value, span)
            }
            frontend::TyKind::Adt(item_ref) => {
                let head = item_ref.def_id.import_as_nonvalue();
                let args = item_ref.generic_args.spanned_import(context, span);
                ast::TyKind::App { head, args }
            }
            frontend::TyKind::Foreign(..) => {
                ast::TyKind::Error(unsupported("Foreign type", 928, span))
            }
            frontend::TyKind::Str => ast::TyKind::Primitive(ast::PrimitiveTy::Str),
            frontend::TyKind::Array(item_ref) => {
                if let [
                    frontend::GenericArg::Type(ty),
                    frontend::GenericArg::Const(length),
                ] = &item_ref.generic_args[..]
                {
                    ast::TyKind::Array {
                        ty: ty.spanned_import(context, span),
                        length: Box::new(length.import(context)),
                    }
                } else {
                    ast::TyKind::Error(assertion_failure(
                        "Wrong generics for array: expected a type and a constant. See synthetic_items in hax frontend.",
                        span,
                    ))
                }
            }
            frontend::TyKind::Slice(ty) => {
                if let [frontend::GenericArg::Type(ty)] = &ty.generic_args[..] {
                    ast::TyKind::Slice(ty.spanned_import(context, span))
                } else {
                    ast::TyKind::Error(assertion_failure(
                        "Wrong generics for slice: expected a type. See synthetic_items in hax frontend.",
                        span,
                    ))
                }
            }
            frontend::TyKind::RawPtr(..) => ast::TyKind::RawPointer,
            frontend::TyKind::Ref(_region, ty, mutable) => ast::TyKind::Ref {
                inner: ty.as_ref().spanned_import(context, span),
                mutable: *mutable,
                region: ast::Region,
            },
            frontend::TyKind::Dynamic(_, generic_predicates, _region) => {
                let goals = generic_predicates
                    .predicates
                    .iter()
                    .map(|(clause, _span)| match &clause.kind.value {
                        frontend::ClauseKind::Trait(frontend::TraitPredicate {
                            trait_ref, ..
                        }) => Ok(ast::DynTraitGoal {
                            trait_: trait_ref.def_id.import_as_nonvalue(),
                            non_self_args: trait_ref.generic_args.spanned_import(context, span)
                                [1..]
                                .to_vec(),
                        }),
                        _ => Err(assertion_failure("type Dyn with non trait predicate", span)),
                    })
                    .collect::<Result<Vec<_>, _>>();
                match goals {
                    Ok(goals) => ast::TyKind::Dyn(goals),
                    Err(e) => ast::TyKind::Error(e),
                }
            }
            frontend::TyKind::Coroutine(_) => {
                ast::TyKind::Error(unsupported("Coroutine type", 924, span))
            }
            frontend::TyKind::Never => ast::TyKind::App {
                head: crate::names::rust_primitives::hax::Never,
                args: Vec::new(),
            },
            frontend::TyKind::Tuple(items) => {
                let args = items.generic_args.spanned_import(context, span);
                ast::TyKind::tuple(args)
            }
            frontend::TyKind::Alias(frontend::Alias {
                kind: frontend::AliasKind::Projection { impl_expr, .. },
                def_id,
                ..
            }) => ast::TyKind::AssociatedType {
                impl_: impl_expr.spanned_import(context, span),
                item: def_id.import_as_nonvalue(),
            },
            frontend::TyKind::Alias(frontend::Alias {
                kind: frontend::AliasKind::Opaque { .. },
                def_id,
                ..
            }) => ast::TyKind::Opaque(def_id.import_as_nonvalue()),
            frontend::TyKind::Alias(frontend::Alias {
                kind: frontend::AliasKind::Inherent,
                ..
            }) => ast::TyKind::Error(assertion_failure(
                "Ty::Alias with AliasTyKind::Inherent",
                span,
            )),
            frontend::TyKind::Alias(frontend::Alias {
                kind: frontend::AliasKind::Free,
                ..
            }) => ast::TyKind::Error(assertion_failure("Ty::Alias with AliasTyKind::Free", span)),
            frontend::TyKind::Param(frontend::ParamTy { name, .. }) => {
                ast::TyKind::Param(ast::LocalId(Symbol::new(name.clone())))
            }
            frontend::TyKind::Bound(..) => ast::TyKind::Error(assertion_failure(
                "type Bound: should be gone after typechecking",
                span,
            )),
            frontend::TyKind::Placeholder(..) => ast::TyKind::Error(assertion_failure(
                "type Placeholder: should be gone after typechecking",
                span,
            )),
            frontend::TyKind::Infer(..) => ast::TyKind::Error(assertion_failure(
                "type Infer: should be gone after typechecking",
                span,
            )),
            frontend::TyKind::Error => ast::TyKind::Error(assertion_failure(
                "got type `Error`: Rust compilation probably failed.",
                span,
            )),
            frontend::TyKind::Todo(_) => ast::TyKind::Error(assertion_failure("type Todo", span)),
        };
        kind.promote()
    }
}

impl SpannedImport<ast::literals::Literal> for frontend::ConstantLiteral {
    fn spanned_import(&self, _context: &Context, _span: ast::span::Span) -> ast::literals::Literal {
        match self {
            frontend::ConstantLiteral::Bool(b) => ast::literals::Literal::Bool(*b),
            frontend::ConstantLiteral::Char(c) => ast::literals::Literal::Char(*c),
            frontend::ConstantLiteral::Float(f, float_ty) => match f.strip_prefix("-") {
                Some(f) => ast::literals::Literal::Float {
                    value: Symbol::new(f),
                    negative: true,
                    kind: float_ty.into(),
                },
                None => ast::literals::Literal::Float {
                    value: Symbol::new(f),
                    negative: false,
                    kind: float_ty.into(),
                },
            },
            frontend::ConstantLiteral::Int(frontend::ConstantInt::Int(v, ty)) => {
                ast::literals::Literal::Int {
                    value: Symbol::new(v.abs().to_string()),
                    negative: *v < 0,
                    kind: ty.into(),
                }
            }
            frontend::ConstantLiteral::Int(frontend::ConstantInt::Uint(v, ty)) => {
                ast::literals::Literal::Int {
                    value: Symbol::new(v.to_string()),
                    negative: false,
                    kind: ty.into(),
                }
            }
            frontend::ConstantLiteral::PtrNoProvenance(_) => {
                panic!("constant literal: PtrNoProvenance")
            }
            frontend::ConstantLiteral::Str(s) => ast::literals::Literal::String(Symbol::new(s)),
            frontend::ConstantLiteral::ByteStr(items) => {
                // Represent a byte string as an array of u8 literals, like the OCaml importer.
                let s = String::from_utf8_lossy(items).to_string();
                ast::literals::Literal::String(Symbol::new(&s))
            }
        }
    }
}

impl Import<ast::Expr> for frontend::ConstantExpr {
    fn import(&self, context: &Context) -> ast::Expr {
        let Self {
            ty,
            span,
            contents,
            attributes,
            ..
        } = self;
        let span = span.import(context);
        let kind = match contents.as_ref() {
            frontend::ConstantExprKind::Literal(constant_literal) => match constant_literal {
                frontend::ConstantLiteral::ByteStr(items) => {
                    let elems: Vec<ast::Expr> = items
                        .iter()
                        .map(|b| {
                            let ty = ast::TyKind::Primitive(ast::PrimitiveTy::Int(
                                (&frontend::UintTy::U8).into(),
                            ));
                            ast::ExprKind::Literal(ast::literals::Literal::Int {
                                value: Symbol::new(b.to_string()),
                                negative: false,
                                kind: (&frontend::UintTy::U8).into(),
                            })
                            .promote(ty.promote(), span)
                        })
                        .collect();
                    ast::ExprKind::Array(elems)
                }
                _ => ast::ExprKind::Literal(constant_literal.spanned_import(context, span)),
            },
            frontend::ConstantExprKind::Adt { info, fields, .. } => {
                let (is_struct, is_record) = match info.kind {
                    frontend::VariantKind::Struct { named } => (true, named),
                    frontend::VariantKind::Enum { named, .. } => (false, named),
                    frontend::VariantKind::Union => (false, false),
                };
                let constructor = info.variant.import_as_value();
                let fields = fields
                    .iter()
                    .map(|f| (f.field.import_as_value(), f.value.import(context)))
                    .collect();
                ast::ExprKind::Construct {
                    constructor,
                    is_record,
                    is_struct,
                    fields,
                    base: None,
                }
            }
            frontend::ConstantExprKind::Array { fields } => {
                ast::ExprKind::Array(fields.import(context))
            }
            frontend::ConstantExprKind::Tuple { fields } => {
                let length = fields.len();
                let constructor: ast::GlobalId = TupleId::Constructor { length }.into();
                let fields = fields
                    .iter()
                    .enumerate()
                    .map(|(idx, value)| {
                        let field: ast::GlobalId = TupleId::Field { length, field: idx }.into();
                        (field, value.import(context))
                    })
                    .collect();
                ast::ExprKind::Construct {
                    constructor,
                    is_record: false,
                    is_struct: true,
                    fields,
                    base: None,
                }
            }
            frontend::ConstantExprKind::GlobalName(item_ref) => {
                ast::ExprKind::GlobalId(item_ref.contents().def_id.import_as_value())
            }
            frontend::ConstantExprKind::Borrow(inner) => ast::ExprKind::Borrow {
                mutable: false,
                inner: inner.import(context),
            },
            frontend::ConstantExprKind::ConstRef { id } => {
                ast::ExprKind::LocalId(ast::LocalId(Symbol::new(id.name.clone())))
            }
            frontend::ConstantExprKind::TraitConst { .. }
            | frontend::ConstantExprKind::RawBorrow { .. }
            | frontend::ConstantExprKind::Cast { .. }
            | frontend::ConstantExprKind::FnPtr(_)
            | frontend::ConstantExprKind::Memory(_)
            | frontend::ConstantExprKind::Todo(_) => ast::ExprKind::Error(assertion_failure(
                "constant_lit_to_lit: TraitConst | FnPtr | RawBorrow | Cast | Memory",
                span,
            )),
        };
        ast::Expr {
            kind: Box::new(kind),
            ty: ty.spanned_import(context, span),
            meta: ast::Metadata {
                span,
                attributes: import_attributes(context, attributes),
            },
        }
    }
}

fn import_block_expr(
    context: &Context,
    block: &frontend::Block,
    ty: &frontend::Ty,
    full_span: ast::span::Span,
    attributes: Vec<ast::Attribute>,
) -> ast::Expr {
    let typ = ty.spanned_import(context, full_span);
    let safety_mode = match block.safety_mode {
        frontend::BlockSafety::Safe => ast::SafetyKind::Safe,
        frontend::BlockSafety::BuiltinUnsafe | frontend::BlockSafety::ExplicitUnsafe => {
            ast::SafetyKind::Unsafe
        }
    };
    let mut stmts = block.stmts.clone();
    let mut tail_expr: Option<frontend::Expr> = block.expr.clone();

    if tail_expr.is_none()
        && matches!(ty.kind(), frontend::TyKind::Never)
        && let Some(frontend::Stmt {
            kind: frontend::StmtKind::Expr { expr, .. },
        }) = stmts.pop()
    {
        tail_expr = Some(expr);
    }

    let mut acc = if let Some(expr) = tail_expr {
        let body = expr.import(context);
        ast::ExprKind::Block { body, safety_mode }.promote(typ.clone(), full_span)
    } else {
        ast::Expr::unit(full_span)
    };

    for stmt in stmts.into_iter().rev() {
        match stmt.kind {
            frontend::StmtKind::Expr { expr, .. } => {
                let rhs = expr.import(context);
                let lhs = ast::PatKind::Wild.promote(rhs.ty.clone(), rhs.meta.span);
                acc = ast::ExprKind::Let {
                    lhs,
                    rhs,
                    body: acc,
                }
                .promote(typ.clone(), full_span);
            }
            frontend::StmtKind::Let {
                pattern,
                initializer,
                else_block,
                ..
            } => {
                let Some(init) = initializer else {
                    return ast::Expr {
                        kind: Box::new(ast::ExprKind::Error(unsupported(
                            "Sorry, Hax does not support declare-first let bindings (see https://doc.rust-lang.org/rust-by-example/variable_bindings/declare.html) for now.",
                            156,
                            full_span,
                        ))),
                        ty: typ,
                        meta: ast::Metadata {
                            span: full_span,
                            attributes,
                        },
                    };
                };
                let lhs = pattern.import(context);
                let rhs = init.import(context);
                let body = acc;
                if let Some(else_block) = else_block {
                    let else_span = else_block.span.import(context);
                    let mut else_expr =
                        import_block_expr(context, &else_block, ty, else_span, Vec::new());
                    else_expr.ty = body.ty.clone();
                    let arm_then = ast::Arm {
                        pat: lhs,
                        body,
                        guard: None,
                        meta: ast::Metadata {
                            span: full_span,
                            attributes: Vec::new(),
                        },
                    };
                    let arm_else = ast::Arm {
                        pat: ast::PatKind::Wild.promote(arm_then.pat.ty.clone(), else_span),
                        body: else_expr,
                        guard: None,
                        meta: ast::Metadata {
                            span: full_span,
                            attributes: Vec::new(),
                        },
                    };
                    acc = ast::ExprKind::Match {
                        scrutinee: rhs,
                        arms: vec![arm_then, arm_else],
                    }
                    .promote(typ.clone(), full_span)
                } else {
                    acc = ast::ExprKind::Let { lhs, rhs, body }.promote(typ.clone(), full_span)
                }
            }
        }
    }

    ast::Expr {
        ty: typ,
        meta: ast::Metadata {
            span: full_span,
            attributes,
        },
        ..acc
    }
}

impl Import<ast::Expr> for frontend::Expr {
    fn import(&self, context: &Context) -> ast::Expr {
        let Self {
            ty,
            span,
            contents,
            attributes,
            ..
        } = self;
        let span = span.import(context);
        let raw_attributes: Vec<Option<ast::Attribute>> = attributes.import(context);
        let attributes: Vec<ast::Attribute> = raw_attributes.into_iter().flatten().collect();
        let binop_id = |op| {
            use crate::names::core::cmp::*;
            use crate::names::core::ops::{arith::*, bit::*};
            use crate::names::rust_primitives::hax::machine_int as hax_machine_int;
            use frontend::BinOp as Op;
            match op {
                Op::Add | Op::AddUnchecked => Add::add,
                Op::Sub | Op::SubUnchecked => Sub::sub,
                Op::Mul | Op::MulUnchecked => Mul::mul,
                Op::Div => Div::div,
                Op::Rem => Rem::rem,
                Op::BitXor => BitXor::bitxor,
                Op::BitAnd => BitAnd::bitand,
                Op::BitOr => BitOr::bitor,
                Op::Shl | Op::ShlUnchecked => Shl::shl,
                Op::Shr | Op::ShrUnchecked => Shr::shr,
                Op::Lt => PartialOrd::lt,
                Op::Le => PartialOrd::le,
                Op::Ne => PartialEq::ne,
                Op::Ge => PartialOrd::ge,
                Op::Gt => PartialOrd::gt,
                Op::Eq => PartialEq::eq,
                Op::Offset => crate::names::core::ptr::const_ptr::Impl::offset,
                Op::Cmp => hax_machine_int::cmp,
                Op::AddWithOverflow => hax_machine_int::add_with_overflow,
                Op::SubWithOverflow => hax_machine_int::sub_with_overflow,
                Op::MulWithOverflow => hax_machine_int::mul_with_overflow,
            }
        };
        let binop_call = |op, x, y, out_type| -> _ {
            use frontend::BinOp as Op;
            let needs_borrow = matches!(op, Op::Lt | Op::Le | Op::Ne | Op::Ge | Op::Gt | Op::Eq);
            let borrow_if_needed = if needs_borrow {
                |e: ast::Expr| {
                    use crate::ast::traits::HasKind;
                    if matches!(e.ty.kind(), ast::TyKind::Ref { .. }) {
                        e
                    } else {
                        let meta = e.meta.clone();
                        let ty = ast::TyKind::Ref {
                            inner: e.ty.clone(),
                            mutable: false,
                            region: ast::Region,
                        };
                        let kind = ast::ExprKind::Borrow {
                            mutable: false,
                            inner: e,
                        };
                        ast::Expr {
                            kind: Box::new(kind),
                            ty: ty.promote(),
                            meta,
                        }
                    }
                }
            } else {
                |e: ast::Expr| e
            };
            let (bounds_impls, trait_, generic_args) = {
                // TODO: we pretend the call is a standalone funtion call.
                // This is not true, here we're calling methods.
                // This should be fixed.
                (vec![], None, vec![])
            };
            ast::ExprKind::fn_app(
                binop_id(op),
                generic_args,
                vec![borrow_if_needed(x), borrow_if_needed(y)],
                out_type,
                bounds_impls,
                trait_,
                span,
            )
        };
        let assign_binop = |op: frontend::AssignOp| match op {
            frontend::AssignOp::AddAssign => frontend::BinOp::Add,
            frontend::AssignOp::SubAssign => frontend::BinOp::Sub,
            frontend::AssignOp::MulAssign => frontend::BinOp::Mul,
            frontend::AssignOp::DivAssign => frontend::BinOp::Div,
            frontend::AssignOp::RemAssign => frontend::BinOp::Rem,
            frontend::AssignOp::BitXorAssign => frontend::BinOp::BitXor,
            frontend::AssignOp::BitAndAssign => frontend::BinOp::BitAnd,
            frontend::AssignOp::BitOrAssign => frontend::BinOp::BitOr,
            frontend::AssignOp::ShlAssign => frontend::BinOp::Shl,
            frontend::AssignOp::ShrAssign => frontend::BinOp::Shr,
        };
        let kind = match contents.as_ref() {
            frontend::ExprKind::Box { value } => {
                let value = value.import(context);
                let ty = ty.spanned_import(context, span);
                let id = crate::names::rust_primitives::hax::box_new;
                ast::ExprKind::standalone_fn_app(id, vec![], vec![value], ty, span)
            }
            frontend::ExprKind::If {
                if_then_scope: _,
                cond,
                then,
                else_opt,
            } => {
                if let frontend::ExprKind::Let { expr, pat } = cond.contents.as_ref() {
                    let scrutinee = expr.import(context);
                    let pat = pat.import(context);
                    let then_expr = then.import(context);
                    let else_expr = else_opt
                        .as_ref()
                        .map(|value| value.import(context))
                        .unwrap_or_else(|| ast::Expr::unit(span));
                    let arm_then = ast::Arm {
                        pat,
                        body: then_expr,
                        guard: None,
                        meta: ast::Metadata {
                            span,
                            attributes: Vec::new(),
                        },
                    };
                    let wildcard_pat = ast::Pat {
                        kind: Box::new(ast::PatKind::Wild),
                        ..arm_then.pat.clone()
                    };
                    let arm_else = ast::Arm {
                        pat: wildcard_pat,
                        body: else_expr,
                        guard: None,
                        meta: ast::Metadata {
                            span,
                            attributes: Vec::new(),
                        },
                    };
                    ast::ExprKind::Match {
                        scrutinee,
                        arms: vec![arm_then, arm_else],
                    }
                } else {
                    ast::ExprKind::If {
                        condition: cond.import(context),
                        then: then.import(context),
                        else_: else_opt.as_ref().map(|value| value.import(context)),
                    }
                }
            }
            frontend::ExprKind::Call {
                ty: _,
                fun,
                args,
                from_hir_call: _,
                fn_span: _,
            } => {
                let mut args = args.import(context);
                if args.is_empty() {
                    args.push(ast::Expr::unit(span));
                }
                if let frontend::ExprKind::GlobalName { item, .. } = fun.contents.as_ref() {
                    let mut head = fun.import(context);
                    *head.kind = ast::ExprKind::GlobalId(item.contents().def_id.import_as_value());
                    let generic_args = item.contents().generic_args.spanned_import(context, span);
                    let bounds_impls = item
                        .contents()
                        .impl_exprs
                        .iter()
                        .map(|ie| ie.spanned_import(context, span))
                        .collect();
                    let trait_ = item.contents().in_trait.as_ref().map(|ie| {
                        let impl_expr = ie.spanned_import(context, span);
                        let args = impl_expr.goal.args.clone();
                        (impl_expr, args)
                    });
                    ast::ExprKind::App {
                        head,
                        args,
                        generic_args,
                        bounds_impls,
                        trait_,
                    }
                } else {
                    let head = fun.import(context);
                    ast::ExprKind::App {
                        head,
                        args,
                        generic_args: Vec::new(),
                        bounds_impls: Vec::new(),
                        trait_: None,
                    }
                }
            }
            frontend::ExprKind::Deref { arg } => {
                let result_ty = ty.spanned_import(context, span);
                ast::ExprKind::standalone_fn_app(
                    crate::names::rust_primitives::hax::deref_op,
                    vec![],
                    vec![arg.import(context)],
                    result_ty,
                    span,
                )
            }
            frontend::ExprKind::Binary { op, lhs, rhs } => {
                let result_ty = ty.spanned_import(context, span);
                binop_call(*op, lhs.import(context), rhs.import(context), result_ty)
            }
            frontend::ExprKind::LogicalOp { op, lhs, rhs } => {
                let result_ty = ty.spanned_import(context, span);
                let id = match op {
                    frontend::LogicalOp::And => crate::names::rust_primitives::hax::logical_op_and,
                    frontend::LogicalOp::Or => crate::names::rust_primitives::hax::logical_op_or,
                };
                ast::ExprKind::standalone_fn_app(
                    id,
                    vec![],
                    vec![lhs.import(context), rhs.import(context)],
                    result_ty,
                    span,
                )
            }
            frontend::ExprKind::Unary { op, arg } => {
                let result_ty = ty.spanned_import(context, span);
                let id = match op {
                    frontend::UnOp::Not => crate::names::core::ops::bit::Not::not,
                    frontend::UnOp::Neg => crate::names::core::ops::arith::Neg::neg,
                    frontend::UnOp::PtrMetadata => crate::names::rust_primitives::hax::cast_op,
                };
                ast::ExprKind::standalone_fn_app(
                    id,
                    vec![],
                    vec![arg.import(context)],
                    result_ty,
                    span,
                )
            }
            frontend::ExprKind::Cast { source } => {
                let source_ty = source.ty.spanned_import(context, span);
                let result_ty = ty.spanned_import(context, span);
                let cast_id = if let ast::TyKind::App { head, .. } = source_ty.0.as_ref() {
                    if head.expect_tuple().is_none() {
                        Some(head.with_suffix(ReservedSuffix::Cast))
                    } else {
                        None
                    }
                } else {
                    None
                };
                let id = cast_id.unwrap_or(crate::names::rust_primitives::hax::cast_op);
                ast::ExprKind::standalone_fn_app(
                    id,
                    vec![],
                    vec![source.import(context)],
                    result_ty,
                    span,
                )
            }
            frontend::ExprKind::Use { source } => return source.import(context),
            frontend::ExprKind::NeverToAny { source } => ast::ExprKind::standalone_fn_app(
                crate::names::rust_primitives::hax::never_to_any,
                vec![],
                vec![source.import(context)],
                ty.spanned_import(context, span),
                span,
            ),
            frontend::ExprKind::PointerCoercion { cast, source } => {
                let result_ty = ty.spanned_import(context, span);
                match cast {
                    frontend::PointerCoercion::ClosureFnPointer(frontend::Safety::Safe)
                    | frontend::PointerCoercion::ReifyFnPointer => return source.import(context),
                    frontend::PointerCoercion::Unsize(_) => ast::ExprKind::standalone_fn_app(
                        crate::names::rust_primitives::unsize,
                        vec![],
                        vec![source.import(context)],
                        result_ty,
                        span,
                    ),
                    _ => ast::ExprKind::Error(assertion_failure(
                        &format!("Pointer, with [cast] being {:?}", cast),
                        span,
                    )),
                }
            }
            frontend::ExprKind::Loop { body } => ast::ExprKind::Loop {
                body: body.import(context),
                kind: Box::new(ast::LoopKind::UnconditionalLoop),
                state: None,
                control_flow: None,
                label: None,
            },
            frontend::ExprKind::Match { scrutinee, arms } => ast::ExprKind::Match {
                scrutinee: scrutinee.import(context),
                arms: arms
                    .iter()
                    .map(|arm| ast::Arm {
                        pat: arm.pattern.import(context),
                        body: arm.body.import(context),
                        guard: arm.guard.as_ref().map(|g| ast::Guard {
                            kind: match g.contents.as_ref() {
                                frontend::ExprKind::Let { expr, pat } => ast::GuardKind::IfLet {
                                    lhs: pat.import(context),
                                    rhs: expr.import(context),
                                },
                                _ => ast::GuardKind::IfLet {
                                    lhs: ast::Pat {
                                        kind: Box::new(ast::PatKind::Constant {
                                            lit: ast::literals::Literal::Bool(true),
                                        }),
                                        ty: ast::TyKind::Primitive(ast::PrimitiveTy::Bool)
                                            .promote(),
                                        meta: ast::Metadata {
                                            span,
                                            attributes: Vec::new(),
                                        },
                                    },
                                    rhs: g.import(context),
                                },
                            },
                            meta: ast::Metadata {
                                span: g.span.import(context),
                                attributes: g.attributes.import(context),
                            },
                        }),
                        meta: ast::Metadata {
                            span: arm.span.import(context),
                            attributes: Vec::new(),
                        },
                    })
                    .collect(),
            },
            frontend::ExprKind::Let { expr: _, pat: _ } => ast::ExprKind::Error(unsupported(
                "Let-chains (e.g. `if let .. && let ..`) are not supported.",
                2018,
                span,
            )),
            frontend::ExprKind::Block { block } => {
                return import_block_expr(context, block, ty, span, attributes.clone());
            }
            frontend::ExprKind::Assign { lhs, rhs } => {
                let lhs = lhs.import(context);
                let rhs = rhs.import(context);
                ast::ExprKind::Assign {
                    lhs: lhs_from_expr(&lhs),
                    value: rhs,
                }
            }
            frontend::ExprKind::AssignOp { op, lhs, rhs } => {
                let bin_op = assign_binop(*op);
                let lhs = lhs.import(context);
                let rhs = rhs.import(context);
                let result_ty = lhs.ty.clone();
                let op_expr = binop_call(bin_op, lhs.clone(), rhs, result_ty.clone())
                    .promote(result_ty, span);
                ast::ExprKind::Assign {
                    lhs: lhs_from_expr(&lhs),
                    value: op_expr,
                }
            }
            frontend::ExprKind::Field { field, lhs } => ast::ExprKind::standalone_fn_app(
                field.import_as_value(),
                vec![],
                vec![lhs.import(context)],
                ty.spanned_import(context, span),
                span,
            ),
            frontend::ExprKind::TupleField { field, lhs } => {
                let length = match lhs.ty.kind() {
                    frontend::TyKind::Tuple(item_ref) => item_ref.generic_args.len(),
                    _ => panic!("TupleField on non-tuple type"),
                };
                let projector: ast::GlobalId = TupleId::Field {
                    length,
                    field: *field,
                }
                .into();

                ast::ExprKind::standalone_fn_app(
                    projector,
                    vec![],
                    vec![lhs.import(context)],
                    ty.spanned_import(context, span),
                    span,
                )
            }
            frontend::ExprKind::Index { lhs, index } => {
                let result_ty = ty.spanned_import(context, span);
                let id = crate::names::core::ops::index::Index::index;
                ast::ExprKind::standalone_fn_app(
                    id,
                    vec![],
                    vec![lhs.import(context), index.import(context)],
                    result_ty,
                    span,
                )
            }
            frontend::ExprKind::VarRef { id } => ast::ExprKind::LocalId(ast::LocalId::from(id)),
            frontend::ExprKind::ConstRef { id } => {
                ast::ExprKind::LocalId(ast::LocalId(Symbol::new(id.name.clone())))
            }
            frontend::ExprKind::GlobalName {
                item,
                constructor: _,
            } => {
                let ident = item.contents().def_id.import_as_value();
                ast::ExprKind::GlobalId(ident)
            }
            frontend::ExprKind::UpvarRef {
                closure_def_id: _,
                var_hir_id,
            } => ast::ExprKind::LocalId(ast::LocalId::from(var_hir_id)),
            frontend::ExprKind::Borrow { borrow_kind, arg } => {
                let inner = arg.import(context);
                let mutable = matches!(borrow_kind, frontend::BorrowKind::Mut { .. });
                ast::ExprKind::Borrow { mutable, inner }
            }
            frontend::ExprKind::RawBorrow { mutability, arg } => ast::ExprKind::AddressOf {
                mutable: *mutability,
                inner: arg.import(context),
            },
            frontend::ExprKind::Break { label: _, value } => {
                let value = value
                    .as_ref()
                    .map(|value| value.import(context))
                    .unwrap_or_else(|| ast::Expr::unit(span));
                ast::ExprKind::Break {
                    value,
                    label: None, // TODO: honour the label (issue #1800)
                    state: None,
                }
            }
            frontend::ExprKind::Continue { label: _ } => ast::ExprKind::Continue {
                label: None, // TODO: honour the label (issue #1800)
                state: None,
            },
            frontend::ExprKind::Return { value } => {
                let value = value
                    .as_ref()
                    .map(|value| value.import(context))
                    .unwrap_or_else(|| ast::Expr::unit(span));
                ast::ExprKind::Return { value }
            }
            frontend::ExprKind::ConstBlock(_item_ref) => {
                ast::ExprKind::Error(unsupported("ConstBlock", 923, span))
            }
            frontend::ExprKind::Repeat { value, count } => {
                let value_expr: ast::Expr = value.import(context);
                let count_expr = count.import(context);
                let repeated = ast::Expr::standalone_fn_app(
                    crate::names::rust_primitives::hax::repeat,
                    vec![],
                    vec![value_expr, count_expr],
                    ty.spanned_import(context, span),
                    span,
                );
                ast::ExprKind::standalone_fn_app(
                    crate::names::alloc::boxed::Impl::new,
                    vec![],
                    vec![repeated],
                    ty.spanned_import(context, span),
                    span,
                )
            }
            frontend::ExprKind::Array { fields } => ast::ExprKind::Array(fields.import(context)),
            frontend::ExprKind::Tuple { fields } => {
                let length = fields.len();
                let constructor: ast::GlobalId = TupleId::Constructor { length }.into();
                let fields = fields
                    .iter()
                    .enumerate()
                    .map(|(idx, value)| {
                        let field: ast::GlobalId = TupleId::Field { length, field: idx }.into();
                        (field, value.import(context))
                    })
                    .collect();
                ast::ExprKind::Construct {
                    constructor,
                    is_record: false,
                    is_struct: true,
                    fields,
                    base: None,
                }
            }
            frontend::ExprKind::Adt(adt_expr) => {
                let (is_struct, is_record) = match adt_expr.info.kind {
                    frontend::VariantKind::Struct { named } => (true, named),
                    frontend::VariantKind::Enum { named, .. } => (false, named),
                    frontend::VariantKind::Union => (false, false),
                };
                let constructor = adt_expr.info.variant.import_as_value();
                let base = match &adt_expr.base {
                    frontend::AdtExprBase::None => None,
                    frontend::AdtExprBase::Base(info) => Some(info.base.import(context)),
                    frontend::AdtExprBase::DefaultFields(_) => {
                        return ast::ExprKind::Error(unsupported(
                            "Default field values: not supported",
                            1386,
                            span,
                        ))
                        .promote(ty.spanned_import(context, span), span);
                    }
                };
                let fields = adt_expr
                    .fields
                    .iter()
                    .map(|f| (f.field.import_as_value(), f.value.import(context)))
                    .collect();
                ast::ExprKind::Construct {
                    constructor,
                    is_record,
                    is_struct,
                    fields,
                    base,
                }
            }
            frontend::ExprKind::PlaceTypeAscription { source: _, .. } => {
                ast::ExprKind::Error(assertion_failure(
                    "Got a unexpected node `PlaceTypeAscription`. Please report, we were not able to figure out an expression yielding that node: a bug report would be very valuable here!",
                    span,
                ))
            }
            frontend::ExprKind::ValueTypeAscription { source, .. } => {
                return source.import(context);
            }
            frontend::ExprKind::Closure {
                params,
                body,
                upvars,
                ..
            } => {
                let mut params: Vec<ast::Pat> = params
                    .iter()
                    .filter_map(|param| param.pat.as_ref().map(|pat| pat.import(context)))
                    .collect();
                if params.is_empty() {
                    let ty = ast::TyKind::unit().promote();
                    params.push(ast::PatKind::Wild.promote(ty, span));
                }
                ast::ExprKind::Closure {
                    params,
                    body: body.import(context),
                    captures: upvars.import(context),
                }
            }
            frontend::ExprKind::Literal { lit, neg } => {
                let mut literal = match &lit.node {
                    frontend::LitKind::Bool(b) => ast::literals::Literal::Bool(*b),
                    frontend::LitKind::Char(c) => ast::literals::Literal::Char(*c),
                    frontend::LitKind::Byte(b) => ast::literals::Literal::Int {
                        value: Symbol::new(b.to_string()),
                        negative: false,
                        kind: (&frontend::UintTy::U8).into(),
                    },
                    frontend::LitKind::Str(s, _) => ast::literals::Literal::String(Symbol::new(s)),
                    frontend::LitKind::Int(value, kind) => {
                        use frontend::LitIntType::*;
                        let kind = match (kind, ty.kind()) {
                            (Signed(int_ty), _) => ast::literals::IntKind::from(int_ty),
                            (Unsigned(uint_ty), _) => ast::literals::IntKind::from(uint_ty),
                            (Unsuffixed, frontend::TyKind::Int(int_ty)) => {
                                ast::literals::IntKind::from(int_ty)
                            }
                            (Unsuffixed, frontend::TyKind::Uint(uint_ty)) => {
                                ast::literals::IntKind::from(uint_ty)
                            }
                            _ => panic!("Unsuffixed int literal without int/uint type"),
                        };
                        ast::literals::Literal::Int {
                            value: Symbol::new(value.to_string()),
                            negative: false,
                            kind,
                        }
                    }
                    frontend::LitKind::Float(value, float_ty) => ast::literals::Literal::Float {
                        value: Symbol::new(value),
                        negative: false,
                        kind: match (float_ty, ty.kind()) {
                            (frontend::LitFloatType::Suffixed(k), _) => {
                                ast::literals::FloatKind::from(k)
                            }
                            (frontend::LitFloatType::Unsuffixed, frontend::TyKind::Float(k)) => {
                                ast::literals::FloatKind::from(k)
                            }
                            _ => panic!("Unsuffixed float literal without float type"),
                        },
                    },
                    frontend::LitKind::CStr(bytes, _) | frontend::LitKind::ByteStr(bytes, _) => {
                        let elems: Vec<ast::Expr> = bytes
                            .iter()
                            .map(|b| {
                                ast::ExprKind::Literal(ast::literals::Literal::Int {
                                    value: Symbol::new(b.to_string()),
                                    negative: false,
                                    kind: (&frontend::UintTy::U8).into(),
                                })
                                .promote(
                                    ast::TyKind::Primitive(ast::PrimitiveTy::Int(
                                        (&frontend::UintTy::U8).into(),
                                    ))
                                    .promote(),
                                    span,
                                )
                            })
                            .collect();
                        return ast::ExprKind::Array(elems)
                            .promote(ty.spanned_import(context, span), span);
                    }
                    frontend::LitKind::Err(_) => {
                        return ast::ExprKind::Error(assertion_failure(
                                "[import_thir:literal] got an error literal: this means the Rust compiler or Hax's frontend probably reported errors above.",
                                span,
                            )).promote(ty.spanned_import(context, span), span);
                    }
                };
                if *neg {
                    match &mut literal {
                        ast::literals::Literal::Int { negative, .. }
                        | ast::literals::Literal::Float { negative, .. } => {
                            *negative = true;
                        }
                        _ => {
                            return ast::ExprKind::Error(assertion_failure(
                                "Unexpected negation on non-numeric literal",
                                span,
                            ))
                            .promote(ty.spanned_import(context, span), span);
                        }
                    }
                }
                ast::ExprKind::Literal(literal)
            }
            frontend::ExprKind::ZstLiteral { .. } => ast::ExprKind::Error(assertion_failure(
                "`ZstLiteral` are expected to be handled before-hand",
                span,
            )),
            frontend::ExprKind::NamedConst { item, user_ty: _ } => {
                let generic_args: Vec<ast::GenericValue> =
                    item.contents().generic_args.spanned_import(context, span);
                let const_args: Vec<ast::Expr> = generic_args
                    .iter()
                    .filter_map(|gv| match gv {
                        ast::GenericValue::Expr(e) => Some(e.clone()),
                        _ => None,
                    })
                    .collect();
                let def_id = item.contents().def_id.import_as_value();
                if const_args.is_empty() && item.contents().in_trait.is_none() {
                    ast::ExprKind::GlobalId(def_id)
                } else {
                    ast::ExprKind::fn_app(
                        def_id,
                        vec![],
                        const_args,
                        ty.spanned_import(context, span),
                        vec![],
                        item.contents().in_trait.as_ref().map(|impl_expr| {
                            (
                                impl_expr.spanned_import(context, span),
                                generic_args.clone(),
                            )
                        }),
                        span,
                    )
                }
            }
            frontend::ExprKind::ConstParam { param, def_id: _ } => {
                ast::ExprKind::LocalId(ast::LocalId(Symbol::new(param.name.clone())))
            }
            frontend::ExprKind::StaticRef { def_id, .. } => {
                ast::ExprKind::GlobalId(def_id.import_as_value())
            }
            frontend::ExprKind::Yield { value: _ } => ast::ExprKind::Error(unsupported(
                "Got expression `Yield`: coroutines are not supported by hax",
                924,
                span,
            )),
            frontend::ExprKind::Todo(payload) => ast::ExprKind::Error(assertion_failure(
                &format!("expression Todo\n{}", payload),
                span,
            )),
        };
        ast::Expr {
            kind: Box::new(kind),
            ty: ty.spanned_import(context, span),
            meta: ast::Metadata { span, attributes },
        }
    }
}

impl Import<(ast::GlobalId, ast::Ty, Vec<ast::Attribute>)> for frontend::FieldDef {
    fn import(&self, context: &Context) -> (ast::GlobalId, ast::Ty, Vec<ast::Attribute>) {
        (
            self.did.import_as_value(),
            self.ty.spanned_import(context, self.span.import(context)),
            self.attributes.import(context),
        )
    }
}

impl Import<ast::Expr> for frontend::ThirBody {
    fn import(&self, context: &Context) -> ast::Expr {
        self.expr.import(context)
    }
}

impl Import<ast::PatKind> for frontend::PatKind {
    fn import(&self, context: &Context) -> ast::PatKind {
        match self {
            frontend::PatKind::Wild | frontend::PatKind::Missing => ast::PatKind::Wild,
            frontend::PatKind::AscribeUserType { subpattern, .. } => ast::PatKind::Ascription {
                pat: subpattern.import(context),
                ty: ast::SpannedTy {
                    span: ast::span::Span::dummy(),
                    ty: subpattern
                        .ty
                        .spanned_import(context, ast::span::Span::dummy()),
                },
            },
            frontend::PatKind::Binding {
                mode,
                var,
                subpattern,
                ..
            } => {
                let mutable = mode.mutability;
                let mode = match mode.by_ref {
                    frontend::ByRef::Yes(_, mutability) => ast::BindingMode::ByRef(if mutability {
                        ast::BorrowKind::Mut
                    } else {
                        ast::BorrowKind::Shared
                    }),
                    frontend::ByRef::No => ast::BindingMode::ByValue,
                };
                ast::PatKind::Binding {
                    mutable,
                    var: ast::LocalId::from(var),
                    mode,
                    sub_pat: subpattern.as_ref().map(|value| value.import(context)),
                }
            }
            frontend::PatKind::Variant {
                info, subpatterns, ..
            } => {
                let (is_struct, is_record) = match info.kind {
                    frontend::VariantKind::Struct { named } => (true, named),
                    frontend::VariantKind::Enum { named, .. } => (false, named),
                    frontend::VariantKind::Union => (false, false),
                };
                let constructor = info.variant.import_as_value();
                let fields = subpatterns
                    .iter()
                    .map(|f| (f.field.import_as_value(), f.pattern.import(context)))
                    .collect();
                ast::PatKind::Construct {
                    constructor,
                    is_record,
                    is_struct,
                    fields,
                }
            }
            frontend::PatKind::Tuple { subpatterns } => {
                let length = subpatterns.len();
                let constructor: ast::GlobalId = TupleId::Constructor { length }.into();
                let fields = subpatterns
                    .iter()
                    .enumerate()
                    .map(|(idx, pat)| {
                        let field: ast::GlobalId = TupleId::Field { length, field: idx }.into();
                        (field, pat.import(context))
                    })
                    .collect();
                ast::PatKind::Construct {
                    constructor,
                    is_record: false,
                    is_struct: true,
                    fields,
                }
            }
            frontend::PatKind::Deref { subpattern } => ast::PatKind::Deref {
                sub_pat: subpattern.import(context),
            },
            frontend::PatKind::DerefPattern { .. } => ast::PatKind::Error(unsupported(
                "pat DerefPattern",
                926,
                ast::span::Span::dummy(),
            )),
            frontend::PatKind::Constant { value } => {
                use ast::*;
                fn expr_to_pat(expr: Expr) -> Pat {
                    let Expr { kind, ty, meta } = expr;
                    let kind = match *kind {
                        ExprKind::Literal(lit) => PatKind::Constant { lit },
                        ExprKind::Array(args) => PatKind::Array {
                            args: args.into_iter().map(expr_to_pat).collect(),
                        },
                        ExprKind::Borrow { mutable: _, inner } => PatKind::Deref {
                            sub_pat: expr_to_pat(inner),
                        },
                        kind => PatKind::Error(assertion_failure(
                            &format!(
                                "expr_to_pat: the given expression could not be interpreted as a pattern. kind={kind:#?}"
                            ),
                            meta.span,
                        )),
                    };
                    let kind = Box::new(kind);
                    Pat { kind, ty, meta }
                }
                *expr_to_pat(value.import(context)).kind
            }
            frontend::PatKind::ExpandedConstant { subpattern, .. } => {
                *subpattern.import(context).kind
            }
            frontend::PatKind::Range(_) => {
                ast::PatKind::Error(unsupported("pat Range", 925, ast::span::Span::dummy()))
            }
            frontend::PatKind::Slice { .. } | frontend::PatKind::Array { .. } => {
                ast::PatKind::Error(unsupported(
                    "Pat:Array or Pat:Slice",
                    804,
                    ast::span::Span::dummy(),
                ))
            }
            frontend::PatKind::Or { pats } => ast::PatKind::Or {
                sub_pats: pats.import(context),
            },
            frontend::PatKind::Never => {
                ast::PatKind::Error(unsupported("pat Never", 927, ast::span::Span::dummy()))
            }
            frontend::PatKind::Error(_) => ast::PatKind::Error(assertion_failure(
                "`Error` node: Rust compilation failed. If Rust compilation was fine, please file an issue.",
                ast::span::Span::dummy(),
            )),
        }
    }
}
impl Import<ast::Pat> for frontend::Pat {
    fn import(&self, context: &Context) -> ast::Pat {
        let Self {
            ty,
            span,
            contents,
            hir_id: _,
            attributes,
        } = self;
        let span = span.import(context);
        let kind = match contents.as_ref() {
            frontend::PatKind::AscribeUserType {
                ascription: _,
                subpattern,
            } => ast::PatKind::Ascription {
                pat: subpattern.import(context),
                ty: ast::SpannedTy {
                    span,
                    ty: ty.spanned_import(context, span),
                },
            },
            other => other.import(context),
        };
        ast::Pat {
            kind: Box::new(kind),
            ty: ty.spanned_import(context, span),
            meta: ast::Metadata {
                span,
                attributes: attributes.import(context),
            },
        }
    }
}

fn import_params(
    context: &Context,
    params: &Vec<frontend::Param>,
    span: ast::span::Span,
) -> Vec<ast::Param> {
    let params: Vec<ast::Param> = params.spanned_import(context, span);
    if params.is_empty() {
        let ty = ast::TyKind::unit().promote();
        vec![ast::Param {
            pat: ast::PatKind::Wild.promote(ty.clone(), span),
            ty,
            ty_span: None,
            attributes: vec![],
        }]
    } else {
        params
    }
}

impl SpannedImport<ast::Param> for frontend::Param {
    fn spanned_import(&self, context: &Context, span: ast::span::Span) -> ast::Param {
        let frontend::Param {
            pat,
            ty,
            ty_span,
            attributes,
            ..
        } = self;
        let ty_span = ty_span.as_ref().map(|value| value.import(context));
        let ty = ty.spanned_import(context, ty_span.unwrap_or(span));
        ast::Param {
            pat: pat
                .as_ref()
                .map(|value| value.import(context))
                .unwrap_or_else(|| ast::PatKind::Wild.promote(ty.clone(), span)),
            ty,
            ty_span,
            attributes: attributes.import(context),
        }
    }
}

impl Import<ast::Variant> for frontend::VariantDef {
    fn import(&self, context: &Context) -> ast::Variant {
        ast::Variant {
            name: self.def_id.import_as_value(),
            arguments: self.fields.import(context),
            is_record: self.fields.raw.first().is_some_and(|fd| fd.name.is_some()),
            attributes: self.attributes.import(context),
        }
    }
}

impl<I, A: Import<B>, B> Import<Vec<B>> for frontend::IndexVec<I, A> {
    fn import(&self, context: &Context) -> Vec<B> {
        self.raw.iter().map(|value| value.import(context)).collect()
    }
}

fn import_trait_item(
    context: &Context,
    item: &frontend::FullDef<frontend::ThirBody>,
) -> ast::TraitItem {
    let span = item.span.import(context);
    let attributes = item.attributes.import(context);
    let meta = ast::Metadata { span, attributes };
    let (frontend::FullDefKind::AssocConst {
        param_env,
        associated_item,
        ..
    }
    | frontend::FullDefKind::AssocFn {
        param_env,
        associated_item,
        ..
    }
    | frontend::FullDefKind::AssocTy {
        param_env,
        associated_item,
        ..
    }) = &item.kind
    else {
        unreachable!("Found associated item of an unknown kind.")
    };
    let assoc_item_container = &associated_item.container;
    let mut generics = param_env.import(context);
    let mut imported_constraints: Vec<ast::GenericConstraint> = Vec::new();
    let kind = match &item.kind {
        frontend::FullDefKind::AssocConst {
            body: Some(default),
            ..
        } => ast::TraitItemKind::Default {
            params: Vec::new(),
            body: default.import(context),
        },
        frontend::FullDefKind::AssocConst { ty, .. } => {
            ast::TraitItemKind::Fn(ty.spanned_import(context, span))
        }
        frontend::FullDefKind::AssocFn {
            body: Some(default),
            sig,
            param_env,
            ..
        } => {
            generics = import_generics(context, &sig.bound_vars, param_env);
            ast::TraitItemKind::Default {
                params: import_params(context, &default.params, span),
                body: default.import(context),
            }
        }
        frontend::FullDefKind::AssocFn { sig, param_env, .. } => {
            generics = import_generics(context, &sig.bound_vars, param_env);
            let inputs: Vec<ast::Ty> = sig
                .value
                .inputs
                .iter()
                .map(|ty| ty.spanned_import(context, span))
                .collect();
            let inputs = if inputs.is_empty() {
                vec![ast::TyKind::unit().promote()]
            } else {
                inputs
            };
            let output = sig.value.output.spanned_import(context, span);
            ast::TraitItemKind::Fn(ast::TyKind::Arrow { inputs, output }.promote())
        }
        frontend::FullDefKind::AssocTy {
            value: Some(..), ..
        } => ast::TraitItemKind::Error(assertion_failure(
            "Associate types defaults are not supported by hax yet (it is a nightly feature)",
            span,
        )),
        frontend::FullDefKind::AssocTy {
            implied_predicates, ..
        } => {
            imported_constraints = implied_predicates.import(context);
            let type_constraints = imported_constraints
                .iter()
                .filter_map(|gc| match gc {
                    ast::GenericConstraint::TypeClass(t) => Some(t.clone()),
                    _ => None,
                })
                .collect();
            ast::TraitItemKind::Type(type_constraints)
        }
        _ => ast::TraitItemKind::Error(assertion_failure(
            "Found associated item of an unknown kind.",
            span,
        )),
    };
    generics
        .constraints
        .retain(|gc| !is_self_type_constraint(gc, assoc_item_container));
    ast::TraitItem {
        meta,
        kind,
        generics,
        ident: item.def_id().import_as_nonvalue(),
    }
}

impl SpannedImport<ast::TraitGoal> for frontend::TraitRef {
    fn spanned_import(&self, context: &Context, span: ast::span::Span) -> ast::TraitGoal {
        let trait_ = self.def_id.import_as_nonvalue();
        let args = self.generic_args.spanned_import(context, span);
        ast::TraitGoal { trait_, args }
    }
}

fn impl_expr_name(index: u64) -> Symbol {
    Symbol::new(format!("i{}", index))
}

fn browse_path(
    context: &Context,
    item_kind: ast::ImplExprKind,
    chunk: &frontend::ImplExprPathChunk,
    span: ast::span::Span,
) -> ast::ImplExprKind {
    match chunk {
        frontend::ImplExprPathChunk::AssocItem {
            item,
            predicate:
                frontend::Binder {
                    value: frontend::TraitPredicate { trait_ref, .. },
                    ..
                },
            index,
            ..
        } => {
            let ident = ast::ImplIdent {
                goal: trait_ref.spanned_import(context, span),
                name: impl_expr_name(*index as u64),
            };
            let item = item.contents().def_id.import_as_nonvalue();
            ast::ImplExprKind::Projection {
                impl_: ast::ImplExpr {
                    kind: Box::new(item_kind),
                    goal: trait_ref.spanned_import(context, span),
                },
                item,
                ident,
            }
        }
        frontend::ImplExprPathChunk::Parent {
            predicate:
                frontend::Binder {
                    value: frontend::TraitPredicate { trait_ref, .. },
                    ..
                },
            index,
            ..
        } => {
            let ident = ast::ImplIdent {
                goal: trait_ref.spanned_import(context, span),
                name: impl_expr_name(*index as u64),
            };
            ast::ImplExprKind::Parent {
                impl_: ast::ImplExpr {
                    kind: Box::new(item_kind),
                    goal: trait_ref.spanned_import(context, span),
                },
                ident,
            }
        }
    }
}

fn import_impl_expr_atom(
    context: &Context,
    ie: &frontend::ImplExprAtom,
    span: ast::span::Span,
    goal: ast::TraitGoal,
) -> ast::ImplExprKind {
    match ie {
        frontend::ImplExprAtom::Concrete(item_ref) => {
            ast::ImplExprKind::Concrete(item_ref.spanned_import(context, span))
        }
        frontend::ImplExprAtom::LocalBound { index, path, .. } => {
            let mut kind = ast::ImplExprKind::LocalBound {
                id: impl_expr_name(*index as u64),
            };
            for chunk in path {
                kind = browse_path(context, kind, chunk, span)
            }
            kind
        }
        // This is not produced by the rustc anymore. Instead we get LocalBound with index 0.
        // Self bounds are reconstructed by phase RewriteLocalSelf. We could try to pass the
        // Self constraint in the context (see `import_trait_item`).
        frontend::ImplExprAtom::SelfImpl { path, .. } => {
            let mut kind = ast::ImplExprKind::Self_;
            for chunk in path {
                kind = browse_path(context, kind, chunk, span)
            }
            kind
        }
        frontend::ImplExprAtom::Dyn => ast::ImplExprKind::Dyn,
        frontend::ImplExprAtom::Builtin { .. } => ast::ImplExprKind::Builtin(goal),
        frontend::ImplExprAtom::Error(msg) => ast::ImplExprKind::Error(unsupported(msg, 707, span)),
    }
}

impl SpannedImport<ast::ImplExpr> for frontend::ImplExpr {
    fn spanned_import(&self, context: &Context, span: ast::span::Span) -> ast::ImplExpr {
        let goal = self.r#trait.value.spanned_import(context, span);
        let impl_ = ast::ImplExpr {
            kind: Box::new(import_impl_expr_atom(
                context,
                &self.r#impl,
                span,
                goal.clone(),
            )),
            goal: goal.clone(),
        };
        match &self.r#impl {
            frontend::ImplExprAtom::Concrete(item_ref) if !item_ref.impl_exprs.is_empty() => {
                let args = item_ref
                    .impl_exprs
                    .iter()
                    .map(|ie| ie.spanned_import(context, span))
                    .collect();
                ast::ImplExpr {
                    kind: Box::new(ast::ImplExprKind::ImplApp { impl_, args }),
                    goal,
                }
            }
            _ => impl_,
        }
    }
}

fn generic_param_to_value(p: &ast::GenericParam) -> ast::GenericValue {
    match &p.kind {
        ast::GenericParamKind::Lifetime => ast::GenericValue::Lifetime,
        ast::GenericParamKind::Type => {
            ast::GenericValue::Ty(ast::TyKind::Param(p.ident.clone()).promote())
        }
        ast::GenericParamKind::Const { ty } => ast::GenericValue::Expr(
            ast::ExprKind::LocalId(p.ident.clone()).promote(ty.clone(), p.meta.span),
        ),
    }
}

fn import_generics(
    context: &Context,
    bound_var_kinds: &[frontend::BoundVariableKind],
    param_env: &frontend::ParamEnv,
) -> ast::Generics {
    let mut generics: ast::Generics = param_env.import(context);
    bound_var_kinds
        .iter()
        .flat_map(|var| match var {
            frontend::BoundVariableKind::Region(frontend::BoundRegionKind::Named {
                def_id: _,
                name,
                span,
                attributes,
            }) => {
                let name = name.strip_prefix("'").unwrap_or(name);
                Some(ast::GenericParam {
                    ident: ast::identifiers::LocalId(Symbol::new(name)),
                    meta: ast::Metadata {
                        span: span.import(context),
                        attributes: import_attributes(context, attributes),
                    },
                    kind: ast::GenericParamKind::Lifetime,
                })
            }
            _ => None,
        })
        .for_each(|var| generics.params.push(var));
    generics
}

fn cast_of_enum(
    context: &Context,
    type_id: ast::GlobalId,
    generics: &ast::Generics,
    ty: ast::Ty,
    span: ast::span::Span,
    variants: impl Iterator<Item = (ast::Variant, frontend::VariantDef)>,
) -> ast::Item {
    let name = ast::GlobalId::with_suffix(type_id, ReservedSuffix::Cast);
    let arms = {
        let ast::TyKind::Primitive(ast::PrimitiveTy::Int(int_kind)) = &*ty.0 else {
            return ast::ItemKind::Error(assertion_failure(
                &format!("cast_of_enum: expected int type, got {:?}", ty),
                span,
            ))
            .promote(name, span);
        };
        let mut previous_explicit_determinator: Option<ast::Expr> = None;
        variants
            .map(|(variant, variant_def)| {
                // Each variant comes with a `rustc_middle::ty::VariantDiscr`. Some variant have `Explicit` discr (i.e. an expression)
                // while other have `Relative` discr (the distance to the previous last explicit discr).
                let body = match &variant_def.discr_def {
                    frontend::DiscriminantDefinition::Relative(m) => {
                        let relative = ast::ExprKind::Literal(ast::literals::Literal::Int {
                            value: Symbol::new(m.to_string()),
                            negative: false,
                            kind: int_kind.clone(),
                        })
                        .promote(ty.clone(), span);
                        if let Some(base) = &previous_explicit_determinator {
                            ast::ExprKind::fn_app(
                                crate::names::core::ops::arith::Add::add,
                                vec![],
                                vec![base.clone(), relative],
                                ty.clone(),
                                vec![],
                                None,
                                span,
                            )
                            .promote(ty.clone(), span)
                        } else {
                            relative
                        }
                    }
                    frontend::DiscriminantDefinition::Explicit { def_id, span } => {
                        let e = ast::ExprKind::GlobalId(def_id.import_as_value())
                            .promote(ty.clone(), span.import(context));
                        previous_explicit_determinator = Some(e.clone());
                        e
                    }
                };
                let pat = ast::PatKind::Construct {
                    constructor: variant.name,
                    is_record: variant.is_record,
                    is_struct: false,
                    fields: variant
                        .arguments
                        .iter()
                        .map(|(cid, ty, _)| (*cid, ast::PatKind::Wild.promote(ty.clone(), span)))
                        .collect(),
                }
                .promote(ty.clone(), span);
                ast::Arm::non_guarded(pat, body, span)
            })
            .collect()
    };
    let type_ref = ast::TyKind::App {
        head: type_id,
        args: generics.params.iter().map(generic_param_to_value).collect(),
    }
    .promote();
    let scrutinee_var = ast::LocalId(Symbol::new("x"));
    let params = vec![ast::Param {
        pat: ast::PatKind::var_pat(scrutinee_var.clone()).promote(type_ref.clone(), span),
        ty: type_ref.clone(),
        ty_span: None,
        attributes: Vec::new(),
    }];
    let scrutinee = ast::ExprKind::LocalId(scrutinee_var).promote(type_ref.clone(), span);
    ast::ItemKind::Fn {
        name,
        generics: generics.clone(),
        body: ast::ExprKind::Match { scrutinee, arms }.promote(ty, span),
        params,
        safety: ast::SafetyKind::Safe,
    }
    .promote(name, span)
}

fn expect_body<'a, Body>(
    optional: &'a Option<Body>,
    span: ast::span::Span,
    label: &str,
) -> Result<&'a Body, ast::ErrorNode> {
    optional
        .as_ref()
        .ok_or_else(|| assertion_failure(&format!("Expected body at {label}"), span))
}

fn missing_associated_item() -> core::convert::Infallible {
    panic!("All assoc items should be included in the list of items produced by the frontend.")
}

use std::collections::HashMap;

/// Import a `FullDef` item produced by the frontend, and produce the corresponding item
/// (or items for inherent impls)
pub fn import_item(
    item: &frontend::FullDef<frontend::ThirBody>,
    all_items: &HashMap<frontend::DefId, &frontend::FullDef<frontend::ThirBody>>,
) -> Vec<ast::Item> {
    let frontend::FullDef {
        this,
        span,
        attributes,
        kind,
        ..
    } = item;
    let context = &Context {
        owner_hint: Some(this.contents().def_id.clone()),
        impl_expr_offset: 0,
    };
    let ident = this.contents().def_id.clone().import_as_nonvalue();
    let span = span.import(context);
    let attributes = attributes.import(context);
    let has_auto = has_automatically_derived(&attributes);
    let mut items = Vec::new();
    let kind = match kind {
        frontend::FullDefKind::Adt {
            param_env,
            adt_kind,
            variants: frontend_variants,
            repr,
            ..
        } => {
            let generics = param_env.import(context);
            let frontend_variants = || frontend_variants.clone().into_iter();
            let variants: Vec<ast::Variant> =
                frontend_variants().map(|v| v.import(context)).collect();
            use frontend::{AdtKind, DiscriminantDefinition};
            let adt_item_kind = {
                let make_type = |is_struct| ast::ItemKind::Type {
                    name: ident,
                    generics: generics.clone(),
                    variants: variants.clone(),
                    is_struct,
                };
                match adt_kind {
                    AdtKind::Enum => make_type(false),
                    AdtKind::Struct => make_type(true),
                    AdtKind::Union => ast::ItemKind::Error(unsupported("Union type", 998, span)),
                    AdtKind::Array | AdtKind::Slice | AdtKind::Tuple => {
                        ast::ItemKind::Error(assertion_failure(
                            &format!(
                                "While translating a item, we got an ADT of kind {adt_kind:#?}. This is not supposed to be ever produced."
                            ),
                            span,
                        ))
                    }
                }
            };

            // For enums that are fieldless (see https://doc.rust-lang.org/reference/items/enumerations.html#casting),
            // we produce a cast function.
            if matches!(adt_kind, AdtKind::Enum) && variants.iter().all(ast::Variant::is_fieldless)
            {
                // Each variant might introduce a anonymous constant defining its discriminant integer
                let discriminant_const_items = frontend_variants().filter_map(|v| {
                    let DiscriminantDefinition::Explicit { def_id, span } = &v.discr_def else {
                        return None;
                    };

                    let span = span.import(context);
                    let name = def_id.import_as_value();
                    let value = v.discr_val.val;
                    let (value, kind) = match v.discr_val.ty.kind() {
                        frontend::TyKind::Int(int_ty) => (value.to_string(), int_ty.into()),
                        frontend::TyKind::Uint(int_ty) => {
                            ((value as i128).to_string(), int_ty.into())
                        }
                        _ => {
                            return Some(
                                ast::ItemKind::Error(assertion_failure("", span))
                                    .promote(name, span),
                            );
                        }
                    };
                    Some(
                        ast::ItemKind::Fn {
                            name,
                            generics: ast::Generics::empty(),
                            body: ast::ExprKind::Literal(ast::literals::Literal::Int {
                                value: Symbol::new(value),
                                negative: false,
                                kind,
                            })
                            .promote(v.discr_val.ty.spanned_import(context, span), span),
                            params: Vec::new(),
                            safety: ast::SafetyKind::Safe,
                        }
                        .promote(name, span),
                    )
                });

                let cast_item = cast_of_enum(
                    context,
                    ident,
                    &generics,
                    repr.typ.spanned_import(context, span),
                    span,
                    variants.into_iter().zip(frontend_variants()),
                );
                return std::iter::once(adt_item_kind.promote(ident, span))
                    .chain(discriminant_const_items)
                    .chain(std::iter::once(cast_item))
                    .collect();
            } else {
                adt_item_kind
            }
        }
        frontend::FullDefKind::TyAlias { param_env, ty } => ast::ItemKind::TyAlias {
            name: ident,
            generics: param_env.import(context),
            ty: ty.spanned_import(context, span),
        },
        frontend::FullDefKind::ForeignTy => {
            ast::ItemKind::Error(unsupported("Foreign type", 928, span))
        }
        frontend::FullDefKind::OpaqueTy => ast::ItemKind::Error(assertion_failure(
            "OpaqueTy should be replaced by Alias in the frontend",
            span,
        )),
        frontend::FullDefKind::Trait {
            param_env,
            implied_predicates,
            items,
            safety,
            ..
        } => {
            let mut generics = param_env.import(context);
            generics.constraints = implied_predicates.import(context);
            ast::ItemKind::Trait {
                name: ident,
                generics,
                items: items
                    .iter()
                    .map(|assoc_item| {
                        let item = all_items
                            .get(&assoc_item.def_id)
                            .expect("Could not find definition for associated item");
                        import_trait_item(context, item)
                    })
                    .collect(),
                safety: safety.import(context),
            }
        }

        frontend::FullDefKind::TraitAlias { .. } => {
            ast::ItemKind::Error(assertion_failure("Trait Alias", span))
        }
        frontend::FullDefKind::TraitImpl {
            param_env,
            trait_pred,
            implied_impl_exprs,
            items,
            ..
        } => {
            let generics = param_env.import(context);
            let trait_ref = trait_pred.trait_ref.contents();
            let of_trait = ast::TraitGoal {
                trait_: trait_ref.def_id.import_as_nonvalue(),
                args: trait_ref
                    .generic_args
                    .iter()
                    .map(|ga| ga.spanned_import(context, span))
                    .collect(),
            };

            let parent_bounds: Vec<(ast::ImplExpr, ast::ImplIdent)> =
                implied_impl_exprs.spanned_import(context, span);
            let items: Vec<ast::ImplItem> = if has_auto {
                Vec::new()
            } else {
                let context = &Context {
                    owner_hint: context.owner_hint.clone(),
                    impl_expr_offset: generics.constraints.len() as u64,
                };
                items
                    .iter()
                    .flat_map(|assoc_item| {
                        // The DefId for this very specific impl associated item.
                        // The DefId of the original associated item on the trait is
                        // `assoc_item.decl_def_id`, here we discard it, but it may
                        // be useful in the future (for e.g. for
                        // https://github.com/cryspen/hax-evit/issues/24).
                        let method_def_id_impl = match &assoc_item.value {
                            hax_frontend_exporter::ImplAssocItemValue::Provided {
                                def_id, ..
                            } => def_id,
                            _ => {
                                // TODO: Here, we skip defaulted associated items.
                                return None;
                            }
                        };
                        let ident = method_def_id_impl.import_as_nonvalue();
                        let assoc_item_def = all_items.get(method_def_id_impl).unwrap_or_else(
                            #[allow(unreachable_code)]
                            || match missing_associated_item() {},
                        );
                        let span = assoc_item_def.span.import(context);
                        let attributes = assoc_item_def.attributes.import(context);
                        let (generics, kind) = match assoc_item_def.kind() {
                            frontend::FullDefKind::AssocTy {
                                param_env, value, ..
                            } => (
                                param_env.import(context),
                                match expect_body(value, span, "import_item/TraitImpl/AssocTy") {
                                    Ok(body) => ast::ImplItemKind::Type {
                                        ty: body.spanned_import(context, span),
                                        parent_bounds: assoc_item
                                            .required_impl_exprs
                                            .spanned_import(context, span),
                                    },
                                    Err(error) => ast::ImplItemKind::Error(error),
                                },
                            ),
                            frontend::FullDefKind::AssocFn {
                                param_env,
                                body,
                                sig,
                                ..
                            } => (
                                import_generics(context, &sig.bound_vars, param_env),
                                match expect_body(body, span, "import_item/TraitImpl/AssocFn") {
                                    Ok(body) => ast::ImplItemKind::Fn {
                                        body: body.import(context),
                                        params: import_params(context, &body.params, span),
                                    },
                                    Err(error) => ast::ImplItemKind::Error(error),
                                },
                            ),
                            frontend::FullDefKind::AssocConst {
                                param_env, body, ..
                            } => (
                                param_env.import(context),
                                match expect_body(body, span, "import_item/TraitImpl/AssocConst") {
                                    Ok(body) => ast::ImplItemKind::Fn {
                                        body: body.import(context),
                                        params: Vec::new(),
                                    },
                                    Err(error) => ast::ImplItemKind::Error(error),
                                },
                            ),
                            #[allow(unreachable_code)]
                            _ => match missing_associated_item() {},
                        };
                        Some(ast::ImplItem {
                            meta: ast::Metadata { span, attributes },
                            generics,
                            kind,
                            ident,
                        })
                    })
                    .collect()
            };

            if let [ast::GenericValue::Ty(self_ty), ..] = &of_trait.args[..] {
                ast::ItemKind::Impl {
                    generics,
                    self_ty: self_ty.clone(),
                    of_trait,
                    items,
                    parent_bounds,
                }
            } else {
                ast::ItemKind::Error(assertion_failure(
                    "Self should always be the first generic argument of a trait application.",
                    span,
                ))
            }
        }
        frontend::FullDefKind::InherentImpl {
            param_env, items, ..
        } => {
            if has_auto {
                return Vec::new();
            }
            return items
                .iter()
                .map(|assoc_item| {
                    let ident = assoc_item.def_id.import_as_nonvalue();
                    let assoc_item = all_items.get(&assoc_item.def_id).unwrap_or_else(
                        #[allow(unused)]
                        || match missing_associated_item() {},
                    );
                    let span = assoc_item.span.import(context);
                    let attributes = assoc_item.attributes.import(context);
                    let impl_generics = param_env.import(context);
                    let context = &Context {
                        owner_hint: context.owner_hint.clone(),
                        impl_expr_offset: impl_generics.constraints.len() as u64,
                    };
                    let kind = match assoc_item.kind() {
                        frontend::FullDefKind::AssocTy {
                            param_env, value, ..
                        } => {
                            let generics = impl_generics.clone().concat(param_env.import(context));
                            match expect_body(value, span, "import_item/InherentImpl/AssocTy") {
                                Ok(body) => ast::ItemKind::TyAlias {
                                    name: ident,
                                    generics,
                                    ty: body.spanned_import(context, span),
                                },
                                Err(err) => ast::ItemKind::Error(err),
                            }
                        }
                        frontend::FullDefKind::AssocFn {
                            param_env,
                            sig,
                            body,
                            ..
                        } => {
                            let generics = impl_generics.clone().concat(import_generics(
                                context,
                                &sig.bound_vars,
                                param_env,
                            ));
                            match expect_body(body, span, "import_item/InherentImpl/AssocFn") {
                                Ok(body) => ast::ItemKind::Fn {
                                    name: ident,
                                    generics,
                                    body: body.import(context),
                                    params: import_params(context, &body.params, span),
                                    safety: sig.value.safety.import(context),
                                },
                                Err(err) => ast::ItemKind::Error(err),
                            }
                        }
                        frontend::FullDefKind::AssocConst {
                            param_env, body, ..
                        } => {
                            let generics = impl_generics.clone().concat(param_env.import(context));
                            match expect_body(body, span, "import_item/InherentImpl/AssocConst") {
                                Ok(body) => ast::ItemKind::Fn {
                                    name: ident,
                                    generics,
                                    body: body.import(context),
                                    params: Vec::new(),
                                    safety: ast::SafetyKind::Safe,
                                },
                                Err(err) => ast::ItemKind::Error(err),
                            }
                        }
                        _ =>
                        {
                            #[allow(unused)]
                            match missing_associated_item() {}
                        }
                    };
                    ast::Item {
                        ident,
                        kind,
                        meta: ast::Metadata { span, attributes },
                    }
                })
                .collect();
        }
        frontend::FullDefKind::Fn {
            param_env,
            sig,
            body,
            ..
        } => match expect_body(body, span, "import_item/Fn") {
            Ok(body) => ast::ItemKind::Fn {
                name: ident,
                generics: import_generics(context, &sig.bound_vars, param_env),
                body: body.import(context),
                params: import_params(context, &body.params, span),
                safety: sig.value.safety.import(context),
            },
            Err(err) => ast::ItemKind::Error(err),
        },
        frontend::FullDefKind::Closure { .. } => {
            ast::ItemKind::Error(assertion_failure("Closure item", span))
        }
        frontend::FullDefKind::Const {
            param_env, body, ..
        } => match expect_body(body, span, "import_item/Const") {
            Ok(body) => ast::ItemKind::Fn {
                name: ident,
                generics: param_env.import(context),
                body: body.import(context),
                params: Vec::new(),
                safety: ast::SafetyKind::Safe,
            },
            Err(err) => ast::ItemKind::Error(err),
        },
        frontend::FullDefKind::Static {
            mutability: true, ..
        } => ast::ItemKind::Error(unsupported("Mutable static item", 1343, span)),
        frontend::FullDefKind::Static {
            mutability: false,
            body,
            ..
        } => match expect_body(body, span, "import_item/Static") {
            Ok(body) => ast::ItemKind::Fn {
                name: ident,
                generics: ast::Generics {
                    params: Vec::new(),
                    constraints: Vec::new(),
                },
                body: body.import(context),
                params: Vec::new(),
                safety: ast::SafetyKind::Safe,
            },
            Err(err) => ast::ItemKind::Error(err),
        },
        frontend::FullDefKind::Use(Some((
            frontend::UsePath {
                res,
                segments,
                rename,
                ..
            },
            _,
        ))) => ast::ItemKind::Use {
            path: segments
                .iter()
                .map(|segment| &segment.ident.0)
                .cloned()
                .collect(),
            is_external: res
                .iter()
                .any(|x| matches!(x, None | Some(frontend::Res::Err))),
            rename: rename.clone(),
        },
        frontend::FullDefKind::Mod { .. } => ast::ItemKind::RustModule,
        frontend::FullDefKind::ExternCrate
        | frontend::FullDefKind::Use { .. }
        | frontend::FullDefKind::TyParam
        | frontend::FullDefKind::ConstParam
        | frontend::FullDefKind::LifetimeParam
        | frontend::FullDefKind::Variant
        | frontend::FullDefKind::Ctor { .. }
        | frontend::FullDefKind::Field
        | frontend::FullDefKind::Macro(_)
        | frontend::FullDefKind::ForeignMod { .. }
        | frontend::FullDefKind::SyntheticCoroutineBody => return Vec::new(),
        frontend::FullDefKind::GlobalAsm => {
            ast::ItemKind::Error(unsupported("Inline assembly item", 1344, span))
        }
        frontend::FullDefKind::AssocConst { .. }
        | frontend::FullDefKind::AssocFn { .. }
        | frontend::FullDefKind::AssocTy { .. } => return Vec::new(), // These item kinds are handled by the case of Impl
    };
    items.push(ast::Item {
        ident,
        kind,
        meta: ast::Metadata { span, attributes },
    });
    items
}
