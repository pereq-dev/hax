//! Copies of the relevant `HIR` types. HIR represents the code of a rust crate post-macro
//! expansion. It is close to the parsed AST, modulo some desugarings (and macro expansion).
//!
//! This module also includes some `rustc_ast` definitions when they show up in HIR.
use crate::prelude::*;
use crate::sinto_todo;

#[cfg(feature = "rustc")]
use rustc_ast::ast;
#[cfg(feature = "rustc")]
use rustc_hir as hir;
#[cfg(feature = "rustc")]
use rustc_middle::ty;

/// Reflects [`hir::hir_id::HirId`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: BaseState<'tcx>>, from: hir::hir_id::HirId, state: S as gstate)]
pub struct HirId {
    owner: DefId,
    local_id: usize,
    // attrs: String
}
// TODO: If not working: See original

#[cfg(feature = "rustc")]
impl<'tcx, S: BaseState<'tcx>> SInto<S, DefId> for hir::hir_id::OwnerId {
    fn sinto(&self, s: &S) -> DefId {
        self.to_def_id().sinto(s)
    }
}

/// Reflects [`ast::LitFloatType`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: BaseState<'tcx>>, from: ast::LitFloatType, state: S as gstate)]
pub enum LitFloatType {
    Suffixed(FloatTy),
    Unsuffixed,
}

/// Reflects [`hir::Movability`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S>, from: hir::Movability, state: S as _s)]
pub enum Movability {
    Static,
    Movable,
}

#[cfg(feature = "rustc")]
impl<S> SInto<S, Mutability> for hir::Mutability {
    fn sinto(&self, _s: &S) -> Mutability {
        match self {
            Self::Mut => true,
            Self::Not => false,
        }
    }
}

#[cfg(feature = "rustc")]
impl<S> SInto<S, Pinnedness> for hir::Pinnedness {
    fn sinto(&self, _s: &S) -> Pinnedness {
        match self {
            Self::Pinned => true,
            Self::Not => false,
        }
    }
}

/// Reflects [`hir::RangeEnd`]
#[derive(AdtInto)]
#[args(<S>, from: hir::RangeEnd, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum RangeEnd {
    Included,
    Excluded,
}

/// Reflects [`hir::ImplicitSelfKind`]
#[derive(AdtInto)]
#[args(<S>, from: hir::ImplicitSelfKind, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum ImplicitSelfKind {
    Imm,
    Mut,
    RefImm,
    RefMut,
    None,
}

/// Reflects [`hir::FnDecl`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::FnDecl<'tcx>, state: S as tcx)]
pub struct FnDecl {
    pub inputs: Vec<Ty>,
    pub output: FnRetTy,
    pub c_variadic: bool,
    pub implicit_self: ImplicitSelfKind,
    pub lifetime_elision_allowed: bool,
}

/// Reflects [`hir::FnSig`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::FnSig<'tcx>, state: S as tcx)]
pub struct FnSig {
    pub header: FnHeader,
    pub decl: FnDecl,
    pub span: Span,
}

#[derive(AdtInto, JsonSchema)]
#[args(<S>, from: hir::HeaderSafety, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeaderSafety {
    SafeTargetFeatures,
    Normal(Safety),
}

/// Reflects [`hir::FnHeader`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: hir::FnHeader, state: S as tcx)]
pub struct FnHeader {
    pub safety: HeaderSafety,
    pub constness: Constness,
    pub asyncness: IsAsync,
    pub abi: ExternAbi,
}

/// Reflects [`rustc_abi::ExternAbi`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: BaseState<'tcx>>, from: rustc_abi::ExternAbi, state: S as s)]
pub enum ExternAbi {
    Rust,
    C {
        unwind: bool,
    },
    #[todo]
    Other(String),
}

/// Function definition
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct FnDef<Body: IsBody> {
    pub header: FnHeader,
    pub params: Vec<Param>,
    pub ret: Ty,
    pub body: Body,
    pub sig_span: Span,
}

#[cfg(feature = "rustc")]
impl<'x: 'tcx, 'tcx, S: UnderOwnerState<'tcx>> SInto<S, Ty> for hir::Ty<'x> {
    fn sinto(self: &hir::Ty<'x>, s: &S) -> Ty {
        // **Important:**
        // We need a local id here, and we get it from the owner id, which must
        // be local. It is safe to do so, because if we have access to a HIR ty,
        // it necessarily means we are exploring a local item (we don't have
        // access to the HIR of external objects, only their MIR).
        rustc_hir_analysis::lower_ty(s.base().tcx, self).sinto(s)
    }
}

/// Reflects [`hir::UseKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: BaseState<'tcx>>, from: hir::UseKind, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum UseKind {
    Single(Ident),
    Glob,
    ListStem,
}

/// Reflects [`hir::IsAuto`]
#[derive(AdtInto)]
#[args(<S>, from: hir::IsAuto, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum IsAuto {
    Yes,
    No,
}

/// Reflects [`hir::Defaultness`]
#[derive(AdtInto)]
#[args(<S>, from: hir::Defaultness, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum Defaultness {
    Default { has_value: bool },
    Final,
}

/// Reflects [`hir::ImplPolarity`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: hir::ImplPolarity, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum ImplPolarity {
    Positive,
    Negative(Span),
}

/// Reflects [`hir::Constness`]
#[derive(AdtInto)]
#[args(<S>, from: hir::Constness, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum Constness {
    Const,
    NotConst,
}

/// Reflects [`hir::Generics`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::Generics<'tcx>, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct Generics<Body: IsBody> {
    pub params: Vec<GenericParam<Body>>,
    #[value(region_bounds_at_current_owner(tcx))]
    pub bounds: GenericBounds,
    pub has_where_clause_predicates: bool,
    pub where_clause_span: Span,
    pub span: Span,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: BaseState<'tcx>, Body: IsBody> SInto<S, ImplItem<Body>> for hir::ImplItemId {
    fn sinto(&self, s: &S) -> ImplItem<Body> {
        let tcx: rustc_middle::ty::TyCtxt = s.base().tcx;
        let impl_item = tcx.hir_impl_item(*self);
        let s = s.with_owner_id(impl_item.owner_id.to_def_id());
        impl_item.sinto(&s)
    }
}

/// Reflects [`hir::ParamName`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum ParamName {
    Plain(LocalIdent),
    Fresh,
    Error,
}

/// Reflects [`hir::LifetimeParamKind`]
#[derive(AdtInto)]
#[args(<S>, from: hir::LifetimeParamKind, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum LifetimeParamKind {
    Explicit,
    Elided(MissingLifetimeKind),
    Error,
}

/// Reflects [`hir::AnonConst`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: BaseState<'tcx>>, from: hir::AnonConst, state: S as s)]
pub struct AnonConst<Body: IsBody> {
    pub hir_id: HirId,
    pub def_id: GlobalIdent,
    #[map({
        body_from_id::<Body, _>(*x, &s.with_owner_id(hir_id.owner.to_def_id()))
    })]
    pub body: Body,
}

/// Reflects [`hir::ConstArg`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: hir::ConstArg<'tcx>, state: S as s)]
pub struct ConstArg<Body: IsBody> {
    pub hir_id: HirId,
    pub kind: ConstArgKind<Body>,
}

/// Reflects [`hir::ConstArgKind`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: hir::ConstArgKind<'tcx>, state: S as s)]
pub enum ConstArgKind<Body: IsBody> {
    Path(QPath),
    Anon(AnonConst<Body>),
    #[todo]
    Infer(String),
}

/// Reflects [`hir::GenericParamKind`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::GenericParamKind<'tcx>, state: S as tcx)]
pub enum GenericParamKind<Body: IsBody> {
    Lifetime {
        kind: LifetimeParamKind,
    },
    Type {
        /// On use site, Rust always give us all the generic
        /// parameters, no matter the defaultness. This information is
        /// thus not so useful. At the same time, as discussed in
        /// https://github.com/pereq-dev/hax/issues/310, extracting this
        /// default type causes failures when querying Rust for trait
        /// resolution. We thus decided to disable this feature. If
        /// this default type information is useful to you, please
        /// open an issue on https://github.com/pereq-dev/hax.
        #[map(x.map(|_ty| ()))]
        default: Option<()>,
        synthetic: bool,
    },
    Const {
        ty: Ty,
        default: Option<ConstArg<Body>>,
    },
}

/// Reflects [`hir::GenericParam`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::GenericParam<'tcx>, state: S as s)]
pub struct GenericParam<Body: IsBody> {
    pub hir_id: HirId,
    pub def_id: GlobalIdent,
    #[map(match x {
        hir::ParamName::Plain(loc_ident) =>
            ParamName::Plain(LocalIdent {
                name: loc_ident.as_str().to_string(),
                id: self.hir_id.sinto(s)
            }),
        hir::ParamName::Fresh =>
            ParamName::Fresh,
        hir::ParamName::Error { .. } =>
            ParamName::Error,
    })]
    pub name: ParamName,
    pub span: Span,
    pub pure_wrt_drop: bool,
    pub kind: GenericParamKind<Body>,
    pub colon_span: Option<Span>,
    #[value(s.base().tcx.hir_attrs(*hir_id).sinto(s))]
    attributes: Vec<Attribute>,
}

/// Reflects [`hir::ImplItem`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::ImplItem<'tcx>, state: S as s)]
pub struct ImplItem<Body: IsBody> {
    pub ident: Ident,
    pub owner_id: DefId,
    pub generics: Generics<Body>,
    pub kind: ImplItemKind<Body>,
    pub span: Span,
    // Removed fields. If these are used, will need to provide `#[value(..)]` implementations.
    // pub defaultness: Defaultness,
    // pub vis_span: Span,
    #[value(ItemAttributes::from_owner_id(s, *owner_id))]
    /// the attributes on this impl item
    pub attributes: ItemAttributes,
}

/// Reflects [`hir::ImplItemKind`], inlining the body of the items.
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::ImplItemKind<'tcx>, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum ImplItemKind<Body: IsBody> {
    Const(Ty, Body),
    #[custom_arm(hir::ImplItemKind::Fn(sig, body) => {
                ImplItemKind::Fn(make_fn_def::<Body, _>(sig, body, s))
        },)]
    Fn(FnDef<Body>),
    #[custom_arm(hir::ImplItemKind::Type(t) => {
        let parent_bounds = {
            let (tcx, owner_id) = (s.base().tcx, s.owner_id());
            let assoc_item = tcx.opt_associated_item(owner_id).unwrap();
            let impl_did = assoc_item.impl_container(tcx).unwrap();
            // An inherent associated type has no trait declaration to inherit bounds from.
            assoc_item.trait_item_def_id().map_or(vec![], |decl_did| {
                owner_bounds(tcx, decl_did)
                    .iter()
                    .copied()
                    .filter_map(|(clause, span)| super_clause_to_clause_and_impl_expr(s, impl_did, owner_id, clause, span))
                    .collect()
            })
        };
        ImplItemKind::Type {
            ty: t.sinto(s),
            parent_bounds
        }
        },)]
    /// An associated type with its parent bounds inlined.
    Type {
        ty: Ty,
        parent_bounds: Vec<(Clause, ImplExpr, Span)>,
    },
}

/// Reflects [`hir::Impl`].
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::Impl<'tcx>, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct Impl<Body: IsBody> {
    #[value(of_trait.map(|trait_impl_header| trait_impl_header.safety).unwrap_or(rustc_hir::Safety::Safe).sinto(s))]
    pub safety: Safety,
    // Removed fields. If these are used, will need to provide `#[value(..)]` implementations.
    // pub polarity: ImplPolarity,
    // pub defaultness: Defaultness,
    pub generics: Generics<Body>,
    #[map({
        x.map(|_|
            s.base().tcx
                .impl_trait_ref(s.owner_id())
                .instantiate_identity()
                .sinto(s)
        )
    })]
    pub of_trait: Option<TraitRef>,
    pub self_ty: Ty,
    pub items: Vec<ImplItem<Body>>,
    #[value({
        let (tcx, owner_id) = (s.base().tcx, s.owner_id());
        if self.of_trait.is_some() {
            let trait_did = tcx.impl_trait_id(owner_id);
            // Backends match these with the trait's bounds by index.
            owner_bounds(tcx, trait_did)
                .iter()
                .copied()
                .filter_map(|(clause, span)| super_clause_to_clause_and_impl_expr(s, owner_id, owner_id, clause, span))
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    })]
    /// The clauses and impl expressions corresponding to the bounds of the
    /// impl's trait (if not inherent): its super bounds and the bounds on its
    /// generic parameters.
    pub parent_bounds: Vec<(Clause, ImplExpr, Span)>,
}

/// Reflects [`hir::IsAsync`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: hir::IsAsync, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum IsAsync {
    Async(Span),
    NotAsync,
}

/// Reflects [`hir::FnRetTy`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::FnRetTy<'tcx>, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum FnRetTy {
    DefaultReturn(Span),
    Return(Ty),
}

/// Reflects [`hir::VariantData`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::VariantData<'tcx>, state: S as tcx)]
pub enum VariantData {
    Struct {
        fields: Vec<HirFieldDef>,
        recovered: bool,
    },
    Tuple(Vec<HirFieldDef>, HirId, GlobalIdent),
    Unit(HirId, GlobalIdent),
}

#[cfg(feature = "rustc")]
impl<S> SInto<S, bool> for ast::Recovered {
    fn sinto(&self, _s: &S) -> bool {
        match self {
            Self::Yes(_) => true,
            Self::No => false,
        }
    }
}

/// Reflects [`hir::FieldDef`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::FieldDef<'tcx>, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct HirFieldDef {
    pub span: Span,
    pub vis_span: Span,
    pub ident: Ident,
    pub hir_id: HirId,
    pub def_id: GlobalIdent,
    pub ty: Ty,
    #[value(s.base().tcx.hir_attrs(*hir_id).sinto(s))]
    attributes: Vec<Attribute>,
}

/// Reflects [`hir::Variant`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::Variant<'tcx>, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct Variant<Body: IsBody> {
    pub ident: Ident,
    pub hir_id: HirId,
    pub def_id: GlobalIdent,
    #[map(x.sinto(&s.with_owner_id(self.def_id.to_def_id())))]
    pub data: VariantData,
    pub disr_expr: Option<AnonConst<Body>>,
    #[value({
        let tcx = s.base().tcx;
        let variant = tcx
            .adt_def(s.owner_id())
            .variants()
            .into_iter()
            .find(|v| v.def_id == self.def_id.into()).unwrap();
        variant.discr.sinto(s)
    })]
    pub discr: DiscriminantDefinition,
    pub span: Span,
    #[value(s.base().tcx.hir_attrs(*hir_id).sinto(s))]
    pub attributes: Vec<Attribute>,
}

/// Reflects [`hir::UsePath`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: hir::UsePath<'tcx>, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct UsePath {
    pub span: Span,
    #[map(x.iter().map(|res| res.sinto(s)).collect())]
    pub res: Vec<Option<Res>>,
    pub segments: Vec<PathSegment>,
    #[value(self.segments.iter().last().and_then(|segment| {
            match s.base().tcx.hir_node_by_def_id(segment.hir_id.owner.def_id) {
                hir::Node::Item(hir::Item {
                    kind: hir::ItemKind::Use(_, hir::UseKind::Single(ident)),
                    ..
                }) => Some(ident.name.to_ident_string()),
                _ => None,
            }
        }))]
    pub rename: Option<String>,
}

/// Reflects [`hir::def::Res`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: hir::def::Res, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum Res {
    Def(DefKind, DefId),
    PrimTy(PrimTy),
    SelfTyParam {
        trait_: DefId,
    },
    SelfTyAlias {
        alias_to: DefId,
        forbid_generic: bool,
        is_trait_impl: bool,
    },
    SelfCtor(DefId),
    Local(HirId),
    ToolMod,
    NonMacroAttr(NonMacroAttrKind),
    Err,
}

/// Reflects [`hir::PrimTy`]
#[derive(AdtInto)]
#[args(<S>, from: hir::PrimTy, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum PrimTy {
    Int(IntTy),
    Uint(UintTy),
    Float(FloatTy),
    Str,
    Bool,
    Char,
}

/// Reflects [`hir::def::NonMacroAttrKind`]
#[derive(AdtInto)]
#[args(<S>, from: hir::def::NonMacroAttrKind, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum NonMacroAttrKind {
    Builtin(Symbol),
    Tool,
    DeriveHelper,
    DeriveHelperCompat,
}

/// Reflects [`hir::PathSegment`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: hir::PathSegment<'tcx>, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct PathSegment {
    pub ident: Ident,
    pub hir_id: HirId,
    pub res: Res,
    #[map(args.map(|args| args.sinto(s)))]
    pub args: Option<HirGenericArgs>,
    pub infer_args: bool,
}

/// Reflects [`hir::ItemKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::ItemKind<'tcx>, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum ItemKind<Body: IsBody> {
    ExternCrate(Option<Symbol>, Ident),
    Use(UsePath, UseKind),
    Static(Mutability, Ident, Ty, Body),
    Const(Ident, Generics<Body>, Ty, Body),
    #[custom_arm(
        hir::ItemKind::Fn{ ident, sig, generics, body, .. } => {
            ItemKind::Fn {
                ident: ident.sinto(s),
                generics: generics.sinto(s),
                def: make_fn_def::<Body, _>(sig, body, s),
            }
        }
    )]
    Fn {
        ident: Ident,
        generics: Generics<Body>,
        def: FnDef<Body>,
    },

    Macro(Ident, MacroDef, MacroKinds),
    Mod(Ident, Vec<Item<Body>>),
    ForeignMod {
        abi: ExternAbi,
        items: Vec<ForeignItem<Body>>,
    },
    GlobalAsm {
        asm: InlineAsm,
    },
    TyAlias(
        Ident,
        Generics<Body>,
        #[map({
            // Rust doesn't enforce bounds on generic parameters in type aliases. Thus, when
            // translating type aliases, we need to disable trait resolution errors. For more
            // details, please see https://github.com/pereq-dev/hax/issues/707.
            let s = &s.with_base(Base { silence_resolution_errors: true, ..s.base() });
            x.sinto(s)
        })]
        Ty,
    ),
    Enum(
        Ident,
        Generics<Body>,
        EnumDef<Body>,
        #[value({
            let tcx = s.base().tcx;
            tcx.repr_options_of_def(s.owner_id().expect_local()).sinto(s)
        })]
        ReprOptions,
    ),
    Struct(
        Ident,
        Generics<Body>,
        VariantData,
        #[value({
            let tcx = s.base().tcx;
            tcx.repr_options_of_def(s.owner_id().expect_local()).sinto(s)
        })]
        ReprOptions,
    ),
    Union(
        Ident,
        Generics<Body>,
        VariantData,
        #[value({
            let tcx = s.base().tcx;
            tcx.repr_options_of_def(s.owner_id().expect_local()).sinto(s)
        })]
        ReprOptions,
    ),
    Trait(
        Constness,
        IsAuto,
        Safety,
        Ident,
        Generics<Body>,
        GenericBounds,
        Vec<TraitItem<Body>>,
    ),
    TraitAlias(Constness, Ident, Generics<Body>, GenericBounds),
    Impl(Impl<Body>),
}

pub type EnumDef<Body> = Vec<Variant<Body>>;

/// Reflects [`hir::TraitItemKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::TraitItemKind<'tcx>, state: S as tcx)]
#[derive(Clone, Debug, JsonSchema)]
#[derive_group(Serializers)]
pub enum TraitItemKind<Body: IsBody> {
    Const(Ty, Option<Body>),
    #[custom_arm(
        hir::TraitItemKind::Fn(sig, hir::TraitFn::Required(id)) => {
            TraitItemKind::RequiredFn(sig.sinto(tcx), id.sinto(tcx))
        }
    )]
    /// Reflects a required [`hir::TraitItemKind::Fn`]
    RequiredFn(FnSig, Vec<Option<Ident>>),
    #[custom_arm(
        hir::TraitItemKind::Fn(sig, hir::TraitFn::Provided(body)) => {
            TraitItemKind::ProvidedFn(sig.sinto(tcx), make_fn_def::<Body, _>(sig, body, tcx))
        }
    )]
    /// Reflects a provided [`hir::TraitItemKind::Fn`]
    ProvidedFn(FnSig, FnDef<Body>),
    #[custom_arm(
        hir::TraitItemKind::Type(b, ty) => {
            TraitItemKind::Type(b.sinto(tcx), ty.map(|t| t.sinto(tcx)))
        }
    )]
    Type(GenericBounds, Option<Ty>),
}

/// Reflects [`hir::TraitItem`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::TraitItem<'tcx>, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct TraitItem<Body: IsBody> {
    pub ident: Ident,
    pub owner_id: DefId,
    pub generics: Generics<Body>,
    pub kind: TraitItemKind<Body>,
    pub span: Span,
    pub defaultness: Defaultness,
    #[value(ItemAttributes::from_owner_id(s, *owner_id))]
    /// The attributes on this trait item
    pub attributes: ItemAttributes,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>, Body: IsBody> SInto<S, EnumDef<Body>> for hir::EnumDef<'tcx> {
    fn sinto(&self, s: &S) -> EnumDef<Body> {
        self.variants.iter().map(|v| v.sinto(s)).collect()
    }
}

#[cfg(feature = "rustc")]
impl<'a, S: BaseState<'a>, Body: IsBody> SInto<S, TraitItem<Body>> for hir::TraitItemId {
    fn sinto(&self, s: &S) -> TraitItem<Body> {
        let s = s.with_owner_id(self.owner_id.to_def_id());
        let tcx: rustc_middle::ty::TyCtxt = s.base().tcx;
        tcx.hir_trait_item(*self).sinto(&s)
    }
}

#[cfg(feature = "rustc")]
impl<'a, 'tcx, S: UnderOwnerState<'tcx>, Body: IsBody> SInto<S, Vec<Item<Body>>> for hir::Mod<'a> {
    fn sinto(&self, s: &S) -> Vec<Item<Body>> {
        let tcx = s.base().tcx;
        self.item_ids
            .iter()
            .map(|id| tcx.hir_item(*id).sinto(s))
            .collect()
    }
}

/// Reflects [`hir::ForeignItemKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::ForeignItemKind<'tcx>, state: S as tcx)]
#[derive(Clone, Debug, JsonSchema)]
#[derive_group(Serializers)]
pub enum ForeignItemKind<Body: IsBody> {
    Fn(FnSig, Vec<Option<Ident>>, Generics<Body>),
    Static(Ty, Mutability, Safety),
    Type,
}

/// Reflects [`hir::ForeignItem`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx> >, from: hir::ForeignItem<'tcx>, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct ForeignItem<Body: IsBody> {
    pub ident: Ident,
    pub kind: ForeignItemKind<Body>,
    pub owner_id: DefId,
    pub span: Span,
    pub vis_span: Span,
}

#[cfg(feature = "rustc")]
impl<'a, S: UnderOwnerState<'a>, Body: IsBody> SInto<S, ForeignItem<Body>> for hir::ForeignItemId {
    fn sinto(&self, s: &S) -> ForeignItem<Body> {
        let tcx: rustc_middle::ty::TyCtxt = s.base().tcx;
        tcx.hir_foreign_item(*self).sinto(s)
    }
}

/// Reflects [`hir::GenericBounds`]
type GenericBounds = Vec<Clause>;

/// The bounds of `def_id`, with their spans, without sized-related bounds.
#[cfg(feature = "rustc")]
fn owner_bounds<'tcx>(
    tcx: ty::TyCtxt<'tcx>,
    def_id: rustc_span::def_id::DefId,
) -> Predicates<'tcx> {
    use hir::def::DefKind;
    // According to what kind of node we are looking at, we should
    // either call `predicates_defined_on` or `item_bounds`
    let use_item_bounds = match tcx.def_kind(def_id) {
        DefKind::AssocTy => matches!(tcx.def_kind(tcx.parent(def_id)), DefKind::Trait),
        DefKind::OpaqueTy => true,
        _ => false,
    };

    let mut clauses = if use_item_bounds {
        std::borrow::Cow::Borrowed(tcx.explicit_item_bounds(def_id).skip_binder())
    } else {
        predicates_defined_on(tcx, def_id)
    };
    prune_sized_predicates(tcx, &mut clauses);
    clauses
}

/// Compute the bounds for the owner registed in the state `s`
#[cfg(feature = "rustc")]
fn region_bounds_at_current_owner<'tcx, S: UnderOwnerState<'tcx>>(s: &S) -> GenericBounds {
    owner_bounds(s.base().tcx, s.owner_id())
        .iter()
        .map(|(clause, _)| *clause)
        .collect::<Vec<_>>()
        .sinto(s)
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, GenericBounds> for hir::GenericBounds<'tcx> {
    fn sinto(&self, s: &S) -> GenericBounds {
        region_bounds_at_current_owner(s)
    }
}

/// Reflects [`rustc_ast::tokenstream::TokenStream`] as a plain
/// string. If you need to reshape that into Rust tokens or construct,
/// please use, e.g., `syn`.
pub type TokenStream = String;

#[cfg(feature = "rustc")]
impl<'t, S> SInto<S, TokenStream> for rustc_ast::tokenstream::TokenStream {
    fn sinto(&self, _: &S) -> String {
        rustc_ast_pretty::pprust::tts_to_string(self)
    }
}

/// Reflects [`rustc_ast::token::Delimiter`]
#[derive(AdtInto)]
#[args(<S>, from: rustc_ast::token::Delimiter, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
    Invisible(InvisibleOrigin),
}

sinto_todo!(rustc_ast::token, InvisibleOrigin);

/// Reflects [`rustc_ast::ast::DelimArgs`]
#[derive(AdtInto)]
#[args(<S>, from: rustc_ast::ast::DelimArgs, state: S as gstate)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DelimArgs {
    pub dspan: DelimSpan,
    pub delim: Delimiter,
    pub tokens: TokenStream,
}

sinto_todo!(rustc_ast::tokenstream, DelimSpan);

/// Reflects [`ast::MacroDef`]
#[derive(AdtInto)]
#[args(<'tcx, S: BaseState<'tcx>>, from: ast::MacroDef, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct MacroDef {
    pub body: DelimArgs,
    pub macro_rules: bool,
}

/// Reflects [`hir::Item`] (and [`hir::ItemId`])
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct Item<Body: IsBody> {
    pub def_id: Option<GlobalIdent>,
    pub owner_id: DefId,
    pub span: Span,
    pub vis_span: Span,
    pub kind: ItemKind<Body>,
    pub attributes: ItemAttributes,
    pub visibility: Visibility<DefId>,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: BaseState<'tcx>, Body: IsBody> SInto<S, Item<Body>> for hir::Item<'tcx> {
    fn sinto(&self, s: &S) -> Item<Body> {
        use hir::ItemKind::*;
        // TODO: Not all items have an identifier; return `Option` here, or even better: use the
        // ident in the `ItemKind`.
        let name = match self.kind {
            ExternCrate(_, i)
            | Use(_, hir::UseKind::Single(i))
            | Static(_, i, ..)
            | Const(i, ..)
            | Fn { ident: i, .. }
            | Macro(i, ..)
            | Mod(i, ..)
            | TyAlias(i, ..)
            | Enum(i, ..)
            | Struct(i, ..)
            | Union(i, ..)
            | Trait(_, _, _, i, ..)
            | TraitAlias(_, i, ..) => i.name.to_ident_string(),
            Use(..) | ForeignMod { .. } | GlobalAsm { .. } | Impl { .. } => String::new(),
        };
        let s = &s.with_owner_id(self.owner_id.to_def_id());
        let tcx = s.base().tcx;
        let owner_id: DefId = self.owner_id.sinto(s);
        let def_id = Path::from(owner_id.clone())
            .ends_with(&[name])
            .then(|| owner_id.clone());
        Item {
            def_id,
            owner_id,
            span: self.span.sinto(s),
            vis_span: self.span.sinto(s),
            kind: self.kind.sinto(s),
            attributes: ItemAttributes::from_owner_id(s, self.owner_id),
            visibility: tcx.visibility(self.owner_id).sinto(s),
        }
    }
}

#[cfg(feature = "rustc")]
impl<'tcx, S: BaseState<'tcx>, Body: IsBody> SInto<S, Item<Body>> for hir::ItemId {
    fn sinto(&self, s: &S) -> Item<Body> {
        let tcx: rustc_middle::ty::TyCtxt = s.base().tcx;
        tcx.hir_item(*self).sinto(s)
    }
}

/// Reflects [`rustc_span::symbol::Ident`]
pub type Ident = (Symbol, Span);

#[cfg(feature = "rustc")]
impl<'tcx, S: BaseState<'tcx>> SInto<S, Ident> for rustc_span::symbol::Ident {
    fn sinto(&self, s: &S) -> Ident {
        (self.name.sinto(s), self.span.sinto(s))
    }
}

/// Reflects [`rustc_ast::AttrStyle`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<S>, from: rustc_ast::AttrStyle, state: S as _s)]
pub enum AttrStyle {
    Outer,
    Inner,
}

/// Reflects [`rustc_ast::Attribute`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: BaseState<'tcx>>, from: rustc_hir::Attribute, state: S as gstate)]
pub enum Attribute {
    Parsed(AttributeKind),
    Unparsed(AttrItem),
}
/// Reflects [`rustc_ast::ast::BindingMode`]
#[derive(AdtInto)]
#[args(<S>, from: rustc_ast::ast::BindingMode, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct BindingMode {
    #[value(self.0.sinto(s))]
    pub by_ref: ByRef,
    #[value(self.1.sinto(s))]
    pub mutability: Mutability,
}

/// Reflects [`rustc_ast::ast::ByRef`]
#[derive(AdtInto)]
#[args(<S>, from: rustc_ast::ast::ByRef, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum ByRef {
    Yes(Pinnedness, Mutability),
    No,
}

/// Reflects [`rustc_ast::ast::StrStyle`]
#[derive(AdtInto)]
#[args(<S>, from: rustc_ast::ast::StrStyle, state: S as gstate)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum StrStyle {
    Cooked,
    Raw(u8),
}

/// Reflects [`rustc_ast::ast::LitKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: BaseState<'tcx>>, from: rustc_ast::ast::LitKind, state: S as gstate)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LitKind {
    Str(Symbol, StrStyle),
    ByteStr(ByteSymbol, StrStyle),
    CStr(ByteSymbol, StrStyle),
    Byte(u8),
    Char(char),
    Int(
        #[serde(with = "serialize_int::unsigned")]
        #[schemars(with = "String")]
        u128,
        LitIntType,
    ),
    Float(Symbol, LitFloatType),
    Bool(bool),
    Err(ErrorGuaranteed),
}

#[cfg(feature = "rustc")]
impl<S> SInto<S, u128> for rustc_data_structures::packed::Pu128 {
    fn sinto(&self, _s: &S) -> u128 {
        self.0
    }
}

/// Reflects [`rustc_ast::token::CommentKind`]
#[derive(AdtInto)]
#[args(<S>, from: rustc_ast::token::CommentKind, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommentKind {
    Line,
    Block,
}

/// Reflects [`rustc_hir::AttrArgs`]
#[derive(AdtInto)]
#[args(<'tcx, S: BaseState<'tcx>>, from: rustc_hir::AttrArgs, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AttrArgs {
    Empty,
    Delimited(DelimArgs),
    Eq { eq_span: Span, expr: MetaItemLit },
}

/// Reflects [`rustc_ast::MetaItemLit`]
#[derive(AdtInto)]
#[args(<'tcx, S: BaseState<'tcx>>, from: rustc_ast::MetaItemLit, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MetaItemLit {
    pub symbol: Symbol,
    pub suffix: Option<Symbol>,
    pub kind: LitKind,
    pub span: Span,
}

/// Reflects [`rustc_hir::AttrItem`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: BaseState<'tcx>>, from: rustc_hir::AttrItem, state: S as gstate)]
pub struct AttrItem {
    #[map(x.to_string())]
    pub path: String,
    pub args: AttrArgs,
    pub span: Span,
}

#[cfg(feature = "rustc")]
impl<S> SInto<S, String> for rustc_ast::tokenstream::LazyAttrTokenStream {
    fn sinto(&self, st: &S) -> String {
        rustc_ast::tokenstream::TokenStream::new(self.to_attr_token_stream().to_token_trees())
            .sinto(st)
    }
}

sinto_todo!(rustc_hir, GenericArgs<'a> as HirGenericArgs);
sinto_todo!(rustc_hir, InlineAsm<'a>);
sinto_todo!(rustc_hir, MissingLifetimeKind);
sinto_todo!(rustc_hir, QPath<'tcx>);
sinto_todo!(rustc_hir, WhereRegionPredicate<'tcx>);
sinto_todo!(rustc_hir, WhereEqPredicate<'tcx>);
sinto_todo!(rustc_hir, OwnerId);
