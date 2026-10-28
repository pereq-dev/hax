//! Copies of the relevant `THIR` types. THIR represents a HIR (function) body augmented with type
//! information and lightly desugared.
use crate::prelude::*;

#[cfg(feature = "rustc")]
use rustc_middle::thir;

/// Reflects [`thir::LogicalOp`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'a, S>, from: thir::LogicalOp, state: S as _s)]
pub enum LogicalOp {
    And,
    Or,
}

/// Reflects [`thir::LintLevel`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'slt, S: UnderOwnerState<'slt> + HasThir<'slt>>, from: thir::LintLevel, state: S as gstate)]
pub enum LintLevel {
    Inherited,
    Explicit(HirId),
}

#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: ExprState<'tcx>>, from: thir::FruInfo<'tcx>, state: S as gstate)]
/// Field Record Update (FRU) informations, this reflects [`thir::FruInfo`]
pub struct FruInfo {
    /// The base, e.g. `Foo {x: 1, .. base}`
    pub base: Expr,
    pub field_types: Vec<Ty>,
}

#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: ExprState<'tcx>>, from: thir::AdtExprBase<'tcx>, state: S as gstate)]
pub enum AdtExprBase {
    None,
    Base(FruInfo),
    DefaultFields(Vec<Ty>),
}

/// A field expression: a field name along with a value
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct FieldExpr {
    pub field: DefId,
    pub value: Expr,
}

/// Reflects [`thir::AdtExpr`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct AdtExpr {
    pub info: VariantInformations,
    pub user_ty: Option<CanonicalUserType>,
    pub fields: Vec<FieldExpr>,
    pub base: AdtExprBase,
    /// The representation annotation of the ADT type being constructed.
    /// Populated via [`rustc_abi::AdtDef::repr`], which works for both
    /// local and external (e.g. `core::option::Option`) ADT types.
    pub repr: ReprOptions,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: ExprState<'tcx>> SInto<S, AdtExpr> for thir::AdtExpr<'tcx> {
    fn sinto(&self, s: &S) -> AdtExpr {
        let variants = self.adt_def.variants();
        let variant: &rustc_middle::ty::VariantDef = &variants[self.variant_index];
        AdtExpr {
            info: get_variant_information(&self.adt_def, self.variant_index, s),
            fields: self
                .fields
                .iter()
                .map(|f| FieldExpr {
                    field: variant.fields[f.name].did.sinto(s),
                    value: f.expr.sinto(s),
                })
                .collect(),
            base: self.base.sinto(s),
            user_ty: self.user_ty.sinto(s),
            repr: self.adt_def.repr().sinto(s),
        }
    }
}

/// Reflects [`thir::LocalVarId`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct LocalIdent {
    pub name: String,
    pub id: HirId,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, LocalIdent> for thir::LocalVarId {
    fn sinto(&self, s: &S) -> LocalIdent {
        LocalIdent {
            name: s
                .base()
                .local_ctx
                .borrow()
                .vars
                .get(self)
                .s_unwrap(s)
                .to_string(),
            id: self.0.sinto(s),
        }
    }
}

#[cfg(feature = "rustc")]
impl<S> SInto<S, u64> for rustc_middle::mir::interpret::AllocId {
    fn sinto(&self, _: &S) -> u64 {
        self.0.get()
    }
}

/// Reflects [`thir::BlockSafety`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S>, from: thir::BlockSafety, state: S as _s)]
pub enum BlockSafety {
    Safe,
    BuiltinUnsafe,
    #[custom_arm(FROM_TYPE::ExplicitUnsafe{..} => BlockSafety::ExplicitUnsafe,)]
    ExplicitUnsafe,
}

/// Reflects [`rustc_middle::middle::region::ScopeData`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx> + HasThir<'tcx>>, from: rustc_middle::middle::region::ScopeData, state: S as gstate)]
pub enum ScopeData {
    Node,
    CallSite,
    Arguments,
    Destruction,
    IfThen,
    IfThenRescope,
    MatchGuard,
    Remainder(FirstStatementIndex),
}

sinto_as_usize!(rustc_middle::middle::region, FirstStatementIndex);

/// Reflects [`rustc_middle::middle::region::Scope`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx> + HasThir<'tcx>>, from: rustc_middle::middle::region::Scope, state: S as gstate)]
pub struct Scope {
    pub local_id: ItemLocalId,
    pub data: ScopeData,
}

sinto_as_usize!(rustc_hir::hir_id, ItemLocalId);

/// Reflects [`thir::Block`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: ExprState<'tcx>>, from: thir::Block, state: S as gstate)]
pub struct Block {
    pub targeted_by_break: bool,
    pub region_scope: Scope,
    pub span: Span,
    pub stmts: Vec<Stmt>,
    pub expr: Option<Expr>,
    pub safety_mode: BlockSafety,
}

/// Reflects [`thir::Stmt`]
#[derive(AdtInto)]
#[args(<'tcx, S: ExprState<'tcx>>, from: thir::Stmt<'tcx>, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct Stmt {
    pub kind: StmtKind,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: ExprState<'tcx>> SInto<S, Block> for thir::BlockId {
    fn sinto(&self, s: &S) -> Block {
        s.thir().blocks[*self].sinto(s)
    }
}

#[cfg(feature = "rustc")]
impl<'tcx, S: ExprState<'tcx>> SInto<S, Stmt> for thir::StmtId {
    fn sinto(&self, s: &S) -> Stmt {
        s.thir().stmts[*self].sinto(s)
    }
}

#[cfg(feature = "rustc")]
impl<'tcx, S: ExprState<'tcx>> SInto<S, Expr> for thir::Expr<'tcx> {
    fn sinto(&self, s: &S) -> Expr {
        let s = &s.with_ty(self.ty);
        let (hir_id, attributes) = self.hir_id_and_attributes(s);
        let hir_id = hir_id.map(|hir_id| hir_id.index());
        let unrolled = self.unroll_scope(s);
        let thir::Expr { span, kind, ty, .. } = unrolled;
        let contents = match kind {
            // Introduce intermediate `Cast` from `T` to `U` when casting from a `#[repr(T)]` enum to `U`
            thir::ExprKind::Cast { source } => {
                if let rustc_middle::ty::TyKind::Adt(adt, _) = s.thir().exprs[source].ty.kind() {
                    let tcx = s.base().tcx;
                    let contents = kind.sinto(s);
                    let repr_type = if adt.is_enum() {
                        use crate::rustc_middle::ty::util::IntTypeExt;
                        adt.repr().discr_type().to_ty(tcx)
                    } else {
                        ty
                    };
                    if repr_type == ty {
                        contents
                    } else {
                        ExprKind::Cast {
                            source: Decorated {
                                ty: repr_type.sinto(s),
                                span: span.sinto(s),
                                contents: Box::new(contents),
                                hir_id,
                                attributes: vec![],
                            },
                        }
                    }
                } else {
                    kind.sinto(s)
                }
            }
            thir::ExprKind::NonHirLiteral { lit, .. } => {
                let cexpr: ConstantExpr =
                    (ConstantExprKind::Literal(scalar_int_to_constant_literal(s, lit, ty)))
                        .decorate(ty.sinto(s), span.sinto(s));
                return cexpr.into();
            }
            thir::ExprKind::ZstLiteral { .. } => {
                if ty.is_phantom_data() {
                    let rustc_middle::ty::Adt(def, _) = ty.kind() else {
                        supposely_unreachable_fatal!(s[span], "PhantomDataNotAdt"; {kind, ty})
                    };
                    let adt_def = AdtExpr {
                        info: get_variant_information(def, rustc_abi::FIRST_VARIANT, s),
                        user_ty: None,
                        base: AdtExprBase::None,
                        fields: vec![],
                        repr: def.repr().sinto(s),
                    };
                    return Expr {
                        contents: Box::new(ExprKind::Adt(adt_def)),
                        span: self.span.sinto(s),
                        ty: ty.sinto(s),
                        hir_id,
                        attributes,
                    };
                }
                let (def_id, generics) = match ty.kind() {
                    rustc_middle::ty::Adt(adt_def, generics) => {
                        // Here, we should only get `struct Name;` structs.
                        s_assert!(s, adt_def.variants().len() == 1);
                        s_assert!(s, generics.is_empty());
                        (adt_def.did(), generics)
                    }
                    rustc_middle::ty::TyKind::FnDef(def_id, generics) => (*def_id, generics),
                    ty_kind => {
                        let ty_kind = ty_kind.sinto(s);
                        supposely_unreachable_fatal!(
                            s[span],
                            "ZstLiteral ty≠FnDef(...) or PhantomData or naked Struct";
                            {kind, span, ty, ty_kind}
                        );
                    }
                };
                let item = translate_item_ref(s, def_id, generics);
                let tcx = s.base().tcx;
                let constructor = if tcx.is_constructor(def_id) {
                    let adt_def = tcx.adt_def(rustc_utils::get_closest_parent_type(&tcx, def_id));
                    let variant_index = adt_def.variant_index_with_id(tcx.parent(def_id));
                    Some(rustc_utils::get_variant_information(
                        &adt_def,
                        variant_index,
                        s,
                    ))
                } else {
                    None
                };
                return Expr {
                    contents: Box::new(ExprKind::GlobalName { item, constructor }),
                    span: self.span.sinto(s),
                    ty: ty.sinto(s),
                    hir_id,
                    attributes,
                };
            }
            thir::ExprKind::Field {
                lhs,
                variant_index,
                name,
            } => {
                let lhs_ty = s.thir().exprs[lhs].ty.kind();
                let idx = variant_index.index();
                if idx != 0 {
                    let _ = supposely_unreachable!(
                        s[span],
                        "ExprKindFieldIdxNonZero"; {
                            kind,
                            span,
                            ty,
                            ty.kind()
                        }
                    );
                };
                match lhs_ty {
                    rustc_middle::ty::TyKind::Adt(adt_def, _generics) => {
                        let variant = adt_def.variant(variant_index);
                        ExprKind::Field {
                            field: variant.fields[name].did.sinto(s),
                            lhs: lhs.sinto(s),
                        }
                    }
                    rustc_middle::ty::TyKind::Tuple(..) => ExprKind::TupleField {
                        field: name.index(),
                        lhs: lhs.sinto(s),
                    },
                    _ => supposely_unreachable_fatal!(
                        s[span],
                        "ExprKindFieldBadTy"; {
                            kind,
                            span,
                            ty.kind(),
                            lhs_ty
                        }
                    ),
                }
            }
            _ => kind.sinto(s),
        };
        Decorated {
            ty: ty.sinto(s),
            span: span.sinto(s),
            contents: Box::new(contents),
            hir_id,
            attributes,
        }
    }
}

#[cfg(feature = "rustc")]
impl<'tcx, S: ExprState<'tcx>> SInto<S, Expr> for thir::ExprId {
    fn sinto(&self, s: &S) -> Expr {
        s.thir().exprs[*self].sinto(s)
    }
}

#[cfg(feature = "rustc")]
impl<'tcx, S: ExprState<'tcx>> SInto<S, Pat> for thir::Pat<'tcx> {
    fn sinto(&self, s: &S) -> Pat {
        let thir::Pat { span, kind, ty } = self;
        let contents = match kind {
            thir::PatKind::Leaf { subpatterns } => match ty.kind() {
                rustc_middle::ty::TyKind::Adt(adt_def, args) => (thir::PatKind::Variant {
                    adt_def: *adt_def,
                    args,
                    variant_index: rustc_abi::VariantIdx::from_usize(0),
                    subpatterns: subpatterns.clone(),
                })
                .sinto(s),
                rustc_middle::ty::TyKind::Tuple(tys) => {
                    // Build a full-arity vector, filling unmatched positions with wildcards
                    // so that tuple patterns with `..` (ellipsis) are correctly expanded.
                    let mut full_subpatterns: Vec<Pat> = tys
                        .iter()
                        .map(|elem_ty| Decorated {
                            ty: elem_ty.sinto(s),
                            span: span.sinto(s),
                            contents: Box::new(PatKind::Wild),
                            hir_id: None,
                            attributes: vec![],
                        })
                        .collect();
                    for field_pat in subpatterns.iter() {
                        full_subpatterns[field_pat.field.index()] = field_pat.pattern.sinto(s);
                    }
                    PatKind::Tuple {
                        subpatterns: full_subpatterns,
                    }
                }
                _ => supposely_unreachable_fatal!(
                    s[span],
                    "PatLeafNonAdtTy";
                    {ty.kind(), kind}
                ),
            },
            _ => kind.sinto(s),
        };
        Decorated {
            ty: ty.sinto(s),
            span: span.sinto(s),
            contents: Box::new(contents),
            hir_id: None,
            attributes: vec![],
        }
    }
}

#[cfg(feature = "rustc")]
impl<'tcx, S: ExprState<'tcx>> SInto<S, Arm> for thir::ArmId {
    fn sinto(&self, s: &S) -> Arm {
        s.thir().arms[*self].sinto(s)
    }
}

/// Reflects [`thir::StmtKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: ExprState<'tcx>>, from: thir::StmtKind<'tcx>, state: S as gstate)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum StmtKind {
    Expr {
        scope: Scope,
        expr: Expr,
    },
    Let {
        remainder_scope: Scope,
        init_scope: Scope,
        pattern: Pat,
        initializer: Option<Expr>,
        else_block: Option<Block>,
        lint_level: LintLevel,
        #[value(attribute_from_scope(gstate, init_scope).1)]
        /// The attribute on this `let` binding
        attributes: Vec<Attribute>,
    },
}

/// Reflects [`thir::Ascription`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx> + HasThir<'tcx>>, from: thir::Ascription<'tcx>, state: S as gstate)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct Ascription {
    pub annotation: CanonicalUserTypeAnnotation,
    pub variance: Variance,
}

/// Reflects [`thir::PatRange`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct PatRange {
    pub lo: PatRangeBoundary,
    pub hi: PatRangeBoundary,
    pub end: RangeEnd,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, PatRange> for thir::PatRange<'tcx> {
    fn sinto(&self, s: &S) -> PatRange {
        let sinto_bdy = |bdy| match bdy {
            thir::PatRangeBoundary::Finite(valtree) => PatRangeBoundary::Finite(
                valtree_to_constant_expr(s, valtree, self.ty, rustc_span::DUMMY_SP),
            ),
            thir::PatRangeBoundary::NegInfinity => PatRangeBoundary::NegInfinity,
            thir::PatRangeBoundary::PosInfinity => PatRangeBoundary::PosInfinity,
        };
        PatRange {
            lo: sinto_bdy(self.lo),
            hi: sinto_bdy(self.hi),
            end: self.end.sinto(s),
        }
    }
}

/// Reflects [`thir::PatRangeBoundary`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum PatRangeBoundary {
    Finite(ConstantExpr),
    NegInfinity,
    PosInfinity,
}

/// A field pattern: a field name along with a pattern
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct FieldPat {
    pub field: DefId,
    pub pattern: Pat,
}

pub type Pat = Decorated<PatKind>;

/// Reflects [`thir::PatKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: ExprState<'tcx>>, from: thir::PatKind<'tcx>, state: S as gstate)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
#[append(thir::PatKind::Leaf {..} => fatal!(gstate, "PatKind::Leaf: should never come up"),)]
pub enum PatKind {
    Wild,
    Missing,
    AscribeUserType {
        ascription: Ascription,
        subpattern: Pat,
    },
    #[custom_arm(
        thir::PatKind::Binding {name, mode, var, ty, subpattern, is_primary, ..} => {
            let local_ctx = gstate.base().local_ctx;
            local_ctx.borrow_mut().vars.insert(*var, name.to_string());
            PatKind::Binding {
                mode: mode.sinto(gstate),
                var: var.sinto(gstate),
                ty: ty.sinto(gstate),
                subpattern: subpattern.sinto(gstate),
                is_primary: is_primary.sinto(gstate),
            }
        }
    )]
    Binding {
        mode: BindingMode,
        var: LocalIdent, // name VS var? TODO
        ty: Ty,
        subpattern: Option<Pat>,
        is_primary: bool,
    },
    #[custom_arm(
        FROM_TYPE::Variant { adt_def, variant_index, args, subpatterns } => {
            let variant_def_id = adt_def.variant(*variant_index).def_id;
            let item = translate_item_ref(gstate, variant_def_id, args);
            let variants = adt_def.variants();
            let variant: &rustc_middle::ty::VariantDef = &variants[*variant_index];
            let tcx = gstate.base().tcx;
            // Build a map from field index to explicit pattern, so we can
            // fill in wildcards for fields omitted by `..` (ellipsis).
            let explicit: std::collections::HashMap<_, _> = subpatterns
                .iter()
                .map(|f| (f.field, &f.pattern))
                .collect();
            TO_TYPE::Variant {
                item,
                info: get_variant_information(adt_def, *variant_index, gstate),
                subpatterns: variant.fields.iter_enumerated()
                    .map(|(field_idx, field_def)| {
                        let pattern = if let Some(pat) = explicit.get(&field_idx) {
                            pat.sinto(gstate)
                        } else {
                            Decorated {
                                ty: field_def.ty(tcx, args).sinto(gstate),
                                span: rustc_span::DUMMY_SP.sinto(gstate),
                                contents: Box::new(PatKind::Wild),
                                hir_id: None,
                                attributes: vec![],
                            }
                        };
                        FieldPat {
                            field: field_def.did.sinto(gstate),
                            pattern,
                        }
                    })
                    .collect(),
            }
        }
    )]
    Variant {
        /// Reference to variant item definition, with appropriate generics.
        item: ItemRef,
        /// Extra info about the variant.
        info: VariantInformations,
        subpatterns: Vec<FieldPat>,
    },
    #[disable_mapping]
    Tuple {
        subpatterns: Vec<Pat>,
    },
    Deref {
        subpattern: Pat,
    },
    DerefPattern {
        subpattern: Pat,
    },
    Constant {
        value: ConstantExpr,
    },
    ExpandedConstant {
        def_id: DefId,
        subpattern: Pat,
    },
    Range(PatRange),
    Slice {
        prefix: Vec<Pat>,
        slice: Option<Pat>,
        suffix: Vec<Pat>,
    },
    Array {
        prefix: Vec<Pat>,
        slice: Option<Pat>,
        suffix: Vec<Pat>,
    },
    Or {
        pats: Vec<Pat>,
    },
    Never,
    Error(ErrorGuaranteed),
}

/// Reflects [`thir::Arm`]
#[derive(AdtInto)]
#[args(<'tcx, S: ExprState<'tcx>>, from: thir::Arm<'tcx>, state: S as gstate)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct Arm {
    pub pattern: Pat,
    pub guard: Option<Expr>,
    pub body: Expr,
    pub lint_level: LintLevel,
    pub scope: Scope,
    pub span: Span,
    #[value(attribute_from_scope(gstate, scope).1)]
    attributes: Vec<Attribute>,
}

/// Reflects [`thir::Param`]
#[derive(AdtInto)]
#[args(<'tcx, S: ExprState<'tcx>>, from: thir::Param<'tcx>, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct Param {
    pub pat: Option<Pat>,
    pub ty: Ty,
    pub ty_span: Option<Span>,
    pub self_kind: Option<ImplicitSelfKind>,
    pub hir_id: Option<HirId>,
    #[value(hir_id.map(|id| {
        s.base().tcx.hir_attrs(id).sinto(s)
    }).unwrap_or(vec![]))]
    /// attributes on this parameter
    pub attributes: Vec<Attribute>,
}

#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct ThirBody {
    pub expr: Expr,
    pub params: Vec<Param>,
}

pub type Expr = Decorated<ExprKind>;

/// Reflects [`thir::ExprKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: ExprState<'tcx> + HasTy<'tcx>>, from: thir::ExprKind<'tcx>, state: S as gstate)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
#[append(
    thir::ExprKind::Scope {..} => {
        fatal!(gstate, "Scope should have been eliminated at this point");
    },
    thir::ExprKind::Field {..} => {
        fatal!(gstate, "Field should have been eliminated at this point");
    },
    thir::ExprKind::NonHirLiteral {..} => {
        fatal!(gstate, "NonHirLiteral should have been eliminated at this point");
    },
)]
pub enum ExprKind {
    Box {
        value: Expr,
    },
    /// Resugared macros calls. This is deprecated: see
    /// <https://github.com/pereq-dev/hax/issues/145>.
    If {
        if_then_scope: Scope,
        cond: Expr,
        then: Expr,
        else_opt: Option<Expr>,
    },
    #[map({
        let e = gstate.thir().exprs[*fun].unroll_scope(gstate);
        let fun = match e.ty.kind() {
            rustc_middle::ty::TyKind::FnDef(def_id, generics) => {
                let (hir_id, attributes) = e.hir_id_and_attributes(gstate);
                let hir_id = hir_id.map(|hir_id| hir_id.index());
                let item = translate_item_ref(gstate, *def_id, generics);
                let contents = Box::new(ExprKind::GlobalName {
                    item,
                    constructor: None
                });
                Expr {
                    contents,
                    span: e.span.sinto(gstate),
                    ty: e.ty.sinto(gstate),
                    hir_id,
                    attributes,
                }
            },
            rustc_middle::ty::TyKind::FnPtr(..) => {
                e.sinto(gstate)
            },
            ty_kind => {
                let ty_norm: Ty = gstate.base().tcx.normalize_erasing_regions(gstate.typing_env(), *ty).sinto(gstate);
                let ty_kind_sinto = ty_kind.sinto(gstate);
                supposely_unreachable_fatal!(
                    gstate[e.span],
                    "CallNotTyFnDef";
                    {e, ty_kind, ty_kind_sinto, ty_norm}
                );
            }
        };
        TO_TYPE::Call {
            ty: ty.sinto(gstate),
            args: args.sinto(gstate),
            from_hir_call: from_hir_call.sinto(gstate),
            fn_span: fn_span.sinto(gstate),
            fun,
        }
    })]
    /// A call to a function or a method.
    ///
    /// Example: `f(0i8)`, where `f` has signature `fn f<T: Clone>(t: T) -> ()`.
    Call {
        /// The type of the function, substitution applied.
        ///
        /// Example: for the call `f(0i8)`, this is `i8 -> ()`.
        ty: Ty,
        /// The function itself. This can be something else than a name, e.g. a closure.
        ///
        /// Example: for the call `f(0i8)`, this is `f`.
        ///
        /// In the case of a call to a function that's not a closure/fn pointer, the expression
        /// will be a `GlobalName` that contains all the information about generics and whether
        /// this is a direct call or a method call.
        fun: Expr, // TODO: can [ty] and [fun.ty] be different?
        /// The arguments given to the function.
        ///
        /// Example: for the call `f(0i8)`, this is `[0i8]`.
        args: Vec<Expr>,
        from_hir_call: bool,
        fn_span: Span,
    },
    Deref {
        arg: Expr,
    },
    Binary {
        op: BinOp,
        lhs: Expr,
        rhs: Expr,
    },
    LogicalOp {
        op: LogicalOp,
        lhs: Expr,
        rhs: Expr,
    },
    Unary {
        op: UnOp,
        arg: Expr,
    },
    Cast {
        source: Expr,
    },
    Use {
        source: Expr,
    }, // Use a lexpr to get a vexpr.
    NeverToAny {
        source: Expr,
    },
    #[custom_arm(
        &FROM_TYPE::PointerCoercion { cast, source, .. } => {
            let source = &gstate.thir().exprs[source];
            let src_ty = source.ty;
            let tgt_ty = gstate.ty();
            TO_TYPE::PointerCoercion {
                cast: PointerCoercion::sfrom(gstate, cast, src_ty, tgt_ty),
                source: source.sinto(gstate),
            }
        },
    )]
    PointerCoercion {
        cast: PointerCoercion,
        source: Expr,
    },
    Loop {
        body: Expr,
    },
    Match {
        scrutinee: Expr,
        arms: Vec<Arm>,
    },
    Let {
        expr: Expr,
        pat: Pat,
    },
    Block {
        #[serde(flatten)]
        block: Block,
    },
    Assign {
        lhs: Expr,
        rhs: Expr,
    },
    AssignOp {
        op: AssignOp,
        lhs: Expr,
        rhs: Expr,
    },
    #[disable_mapping]
    Field {
        field: DefId,
        lhs: Expr,
    },

    #[disable_mapping]
    TupleField {
        field: usize,
        lhs: Expr,
    },
    Index {
        lhs: Expr,
        index: Expr,
    },
    VarRef {
        id: LocalIdent,
    },
    #[disable_mapping]
    ConstRef {
        id: ParamConst,
    },
    #[disable_mapping]
    GlobalName {
        item: ItemRef,
        constructor: Option<VariantInformations>,
    },
    UpvarRef {
        closure_def_id: DefId,
        var_hir_id: LocalIdent,
    },
    Borrow {
        borrow_kind: BorrowKind,
        arg: Expr,
    },
    RawBorrow {
        mutability: Mutability,
        arg: Expr,
    },
    Break {
        label: Scope,
        value: Option<Expr>,
    },
    Continue {
        label: Scope,
    },
    Return {
        value: Option<Expr>,
    },
    #[custom_arm(FROM_TYPE::ConstBlock { did, args } => TO_TYPE::ConstBlock(translate_item_ref(gstate, *did, args)),)]
    ConstBlock(ItemRef),
    Repeat {
        value: Expr,
        count: ConstantExpr,
    },
    Array {
        fields: Vec<Expr>,
    },
    Tuple {
        fields: Vec<Expr>,
    },
    Adt(AdtExpr),
    PlaceTypeAscription {
        source: Expr,
        user_ty: Option<CanonicalUserType>,
    },
    ValueTypeAscription {
        source: Expr,
        user_ty: Option<CanonicalUserType>,
    },
    #[custom_arm(FROM_TYPE::Closure(e) => {
        let (thir, expr_entrypoint) = get_thir(e.closure_id, gstate);
        let s = &gstate.with_thir(thir.clone());
        TO_TYPE::Closure {
            params: thir.params.raw.sinto(s),
            body: expr_entrypoint.sinto(s),
            upvars: e.upvars.sinto(gstate),
            movability: e.movability.sinto(gstate)
        }
    },
    )]
    Closure {
        params: Vec<Param>,
        body: Expr,
        upvars: Vec<Expr>,
        movability: Option<Movability>,
    },
    Literal {
        lit: Spanned<LitKind>,
        neg: bool, // TODO
    },
    //zero space type
    // This is basically used for functions! e.g. `<T>::from`
    ZstLiteral {
        user_ty: Option<CanonicalUserType>,
    },
    #[custom_arm(FROM_TYPE::NamedConst { def_id, args, user_ty } => TO_TYPE::NamedConst {
        item: translate_item_ref(gstate, *def_id, args),
        user_ty: user_ty.sinto(gstate),
    },)]
    NamedConst {
        item: ItemRef,
        user_ty: Option<CanonicalUserType>,
    },
    ConstParam {
        param: ParamConst,
        def_id: GlobalIdent,
    },
    StaticRef {
        alloc_id: u64,
        ty: Ty,
        def_id: GlobalIdent,
    },
    Yield {
        value: Expr,
    },
    #[todo]
    Todo(String),
}

#[cfg(feature = "rustc")]
pub trait ExprKindExt<'tcx> {
    fn hir_id_and_attributes<S: ExprState<'tcx>>(
        &self,
        s: &S,
    ) -> (Option<rustc_hir::HirId>, Vec<Attribute>);
    fn unroll_scope<S: BaseState<'tcx> + HasThir<'tcx>>(&self, s: &S) -> thir::Expr<'tcx>;
}

#[cfg(feature = "rustc")]
impl<'tcx> ExprKindExt<'tcx> for thir::Expr<'tcx> {
    fn hir_id_and_attributes<S: ExprState<'tcx>>(
        &self,
        s: &S,
    ) -> (Option<rustc_hir::HirId>, Vec<Attribute>) {
        match &self.kind {
            thir::ExprKind::Scope {
                region_scope: scope,
                ..
            } => attribute_from_scope(s, scope),
            _ => (None, vec![]),
        }
    }
    fn unroll_scope<S: BaseState<'tcx> + HasThir<'tcx>>(&self, s: &S) -> thir::Expr<'tcx> {
        // TODO: when we see a loop, we should lookup its label! label is actually a scope id
        // we remove scopes here, whence the TODO
        match self.kind {
            thir::ExprKind::Scope { value, .. } => s.thir().exprs[value].unroll_scope(s),
            _ => self.clone(),
        }
    }
}

#[cfg(feature = "rustc")]
pub trait HirIdExt {
    fn index(&self) -> (usize, usize);
}

#[cfg(feature = "rustc")]
impl HirIdExt for rustc_hir::HirId {
    fn index(&self) -> (usize, usize) {
        use crate::rustc_index::Idx;
        (self.owner.def_id.index(), self.local_id.index())
    }
}
