//! Copies of the relevant type-level types. These are semantically-rich representations of
//! type-level concepts such as types and trait references.
use crate::prelude::*;
use crate::sinto_as_usize;
use crate::sinto_todo;
use std::sync::Arc;

#[cfg(feature = "rustc")]
use rustc_middle::ty;

/// Generic container for decorating items with a type, a span,
/// attributes and other meta-data.
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decorated<T> {
    pub ty: Ty,
    pub span: Span,
    pub contents: Box<T>,
    pub hir_id: Option<(usize, usize)>,
    pub attributes: Vec<Attribute>,
}

/// Reflects [`ty::ParamTy`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::ParamTy, state: S as gstate)]
pub struct ParamTy {
    pub index: u32,
    pub name: Symbol,
}

/// Reflects [`ty::ParamConst`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<S>, from: ty::ParamConst, state: S as gstate)]
pub struct ParamConst {
    pub index: u32,
    pub name: Symbol,
}

/// A predicate without `Self`, for use in `dyn Trait`.
///
/// Reflects [`ty::ExistentialPredicate`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::ExistentialPredicate<'tcx>, state: S as state)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExistentialPredicate {
    /// E.g. `From<u64>`. Note that this isn't `T: From<u64>` with a given `T`, this is just
    /// `From<u64>`. Could be written `?: From<u64>`.
    Trait(ExistentialTraitRef),
    /// E.g. `Iterator::Item = u64`. Could be written `<? as Iterator>::Item = u64`.
    Projection(ExistentialProjection),
    /// E.g. `Send`.
    AutoTrait(DefId),
}

/// Reflects [`rustc_type_ir::ExistentialTraitRef`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: rustc_type_ir::ExistentialTraitRef<ty::TyCtxt<'tcx>>, state: S as state)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExistentialTraitRef {
    pub def_id: DefId,
    pub args: Vec<GenericArg>,
}

/// Reflects [`rustc_type_ir::ExistentialProjection`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: rustc_type_ir::ExistentialProjection<ty::TyCtxt<'tcx>>, state: S as state)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExistentialProjection {
    pub def_id: DefId,
    pub args: Vec<GenericArg>,
    pub term: Term,
}

/// Reflects [`ty::BoundTyKind`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::BoundTyKind, state: S as s)]
pub enum BoundTyKind {
    Anon,
    #[custom_arm(&FROM_TYPE::Param(def_id) => TO_TYPE::Param(def_id.sinto(s), s.base().tcx.item_name(def_id).sinto(s)),)]
    Param(DefId, Symbol),
}

/// Reflects [`ty::BoundTy`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::BoundTy, state: S as s)]
pub struct BoundTy {
    pub var: BoundVar,
    pub kind: BoundTyKind,
}

sinto_as_usize!(rustc_middle::ty, BoundVar);

/// Reflects [`ty::BoundRegionKind`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::BoundRegionKind, state: S as s)]
pub enum BoundRegionKind {
    Anon,
    NamedAnon(Symbol),
    #[custom_arm(&FROM_TYPE::Named(def_id) => {
        let tcx = s.base().tcx;
        TO_TYPE::Named {
            def_id: def_id.sinto(s),
            name: tcx.item_name(def_id).sinto(s),
            span: tcx.def_span(def_id).sinto(s),
            attributes: get_def_attrs(tcx, def_id, get_def_kind(tcx, def_id)).sinto(s),
        }
    })]
    Named {
        def_id: DefId,
        name: Symbol,
        span: Span,
        attributes: Vec<Attribute>,
    },
    ClosureEnv,
}

/// Reflects [`ty::BoundRegion`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::BoundRegion, state: S as s)]
pub struct BoundRegion {
    pub var: BoundVar,
    pub kind: BoundRegionKind,
}

/// Reflects [`ty::PlaceholderRegion`]
pub type PlaceholderRegion = Placeholder<BoundRegion>;
/// Reflects [`ty::PlaceholderConst`]
pub type PlaceholderConst = Placeholder<BoundVar>;
/// Reflects [`ty::PlaceholderType`]
pub type PlaceholderType = Placeholder<BoundTy>;

/// Reflects [`ty::Placeholder`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Placeholder<T> {
    pub bound: T,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>, T: SInto<S, U>, U> SInto<S, Placeholder<U>>
    for ty::Placeholder<T>
{
    fn sinto(&self, s: &S) -> Placeholder<U> {
        Placeholder {
            bound: self.bound.sinto(s),
        }
    }
}

/// Reflects [`rustc_middle::infer::canonical::Canonical`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct Canonical<T> {
    pub value: T,
}
/// Reflects [`ty::CanonicalUserType`]
pub type CanonicalUserType = Canonical<UserType>;

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>, T: SInto<S, U>, U> SInto<S, Canonical<U>>
    for rustc_middle::infer::canonical::Canonical<'tcx, T>
{
    fn sinto(&self, s: &S) -> Canonical<U> {
        Canonical {
            value: self.value.sinto(s),
        }
    }
}

/// Reflects [`ty::UserSelfTy`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::UserSelfTy<'tcx>, state: S as gstate)]
pub struct UserSelfTy {
    pub impl_def_id: DefId,
    pub self_ty: Ty,
}

/// Reflects [`ty::UserArgs`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::UserArgs<'tcx>, state: S as gstate)]
pub struct UserArgs {
    pub args: Vec<GenericArg>,
    pub user_self_ty: Option<UserSelfTy>,
}

/// Reflects [`ty::UserType`]: this is currently
/// disabled, and everything is printed as debug in the
/// [`UserType::Todo`] variant.
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::UserType<'tcx>, state: S as _s)]
pub enum UserType {
    // TODO: for now, we don't use user types at all.
    // We disable it for now, since it cause the following to fail:
    //
    //    pub const MY_VAL: u16 = 5;
    //    pub type Alias = MyStruct<MY_VAL>; // Using the literal 5, it goes through
    //
    //    pub struct MyStruct<const VAL: u16> {}
    //
    //    impl<const VAL: u16> MyStruct<VAL> {
    //        pub const MY_CONST: u16 = VAL;
    //    }
    //
    //    pub fn do_something() -> u32 {
    //        u32::from(Alias::MY_CONST)
    //    }
    //
    // In this case, we get a [ty::ConstKind::Bound] in
    // [do_something], which we are not able to translate.
    // See: https://github.com/pereq-dev/hax/pull/209

    // Ty(Ty),
    // TypeOf(DefId, UserArgs),
    #[todo]
    Todo(String),
}

/// Reflects [`ty::VariantDiscr`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::VariantDiscr, state: S as s)]
pub enum DiscriminantDefinition {
    #[custom_arm(FROM_TYPE::Explicit(did) => TO_TYPE::Explicit { def_id: did.sinto(s), span: s.base().tcx.def_span(did).sinto(s) },)]
    Explicit {
        def_id: DefId,
        span: Span,
    },
    Relative(u32),
}

/// Reflects [`ty::util::Discr`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::util::Discr<'tcx>, state: S as gstate)]
pub struct DiscriminantValue {
    pub val: u128,
    pub ty: Ty,
}

/// Reflects [`ty::Visibility`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum Visibility<Id> {
    Public,
    Restricted(Id),
}

#[cfg(feature = "rustc")]
impl<S, T: SInto<S, U>, U> SInto<S, Visibility<U>> for ty::Visibility<T> {
    fn sinto(&self, s: &S) -> Visibility<U> {
        use ty::Visibility as T;
        match self {
            T::Public => Visibility::Public,
            T::Restricted(id) => Visibility::Restricted(id.sinto(s)),
        }
    }
}

/// Reflects [`ty::FieldDef`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct FieldDef {
    pub did: DefId,
    /// Field definition of [tuple
    /// structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types)
    /// are anonymous, in that case `name` is [`None`].
    pub name: Option<Symbol>,
    pub vis: Visibility<DefId>,
    pub ty: Ty,
    pub span: Span,
    pub attributes: Vec<Attribute>,
}

#[cfg(feature = "rustc")]
impl FieldDef {
    pub fn sfrom<'tcx, S: UnderOwnerState<'tcx>>(
        s: &S,
        fdef: &ty::FieldDef,
        instantiate: ty::GenericArgsRef<'tcx>,
    ) -> FieldDef {
        let tcx = s.base().tcx;
        let ty = fdef.ty(tcx, instantiate).sinto(s);
        let name = {
            let name = fdef.name.sinto(s);
            let is_user_provided = {
                // SH: Note that the only way I found of checking if the user wrote the name or if it
                // is just an integer generated by rustc is by checking if it is just made of
                // numerals...
                name.parse::<usize>().is_err()
            };
            is_user_provided.then_some(name)
        };

        FieldDef {
            did: fdef.did.sinto(s),
            name,
            vis: fdef.vis.sinto(s),
            ty,
            span: tcx.def_span(fdef.did).sinto(s),
            attributes: get_def_attrs(tcx, fdef.did, get_def_kind(tcx, fdef.did)).sinto(s),
        }
    }
}

/// Reflects [`ty::VariantDef`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct VariantDef {
    pub def_id: DefId,
    pub ctor: Option<(CtorKind, DefId)>,
    pub name: Symbol,
    pub discr_def: DiscriminantDefinition,
    pub discr_val: DiscriminantValue,
    /// The definitions of the fields on this variant. In case of [tuple
    /// structs/variants](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types),
    /// the fields are anonymous, otherwise fields are named.
    pub fields: IndexVec<FieldIdx, FieldDef>,
    /// Span of the definition of the variant
    pub span: Span,
    pub attributes: Vec<Attribute>,
}

#[cfg(feature = "rustc")]
impl VariantDef {
    pub(crate) fn sfrom<'tcx, S: UnderOwnerState<'tcx>>(
        s: &S,
        def: &ty::VariantDef,
        discr_val: ty::util::Discr<'tcx>,
        instantiate: Option<ty::GenericArgsRef<'tcx>>,
    ) -> Self {
        let tcx = s.base().tcx;
        let instantiate =
            instantiate.unwrap_or_else(|| ty::GenericArgs::identity_for_item(tcx, def.def_id));
        VariantDef {
            def_id: def.def_id.sinto(s),
            ctor: def.ctor.sinto(s),
            name: def.name.sinto(s),
            discr_def: def.discr.sinto(s),
            discr_val: discr_val.sinto(s),
            fields: def
                .fields
                .iter()
                .map(|f| FieldDef::sfrom(s, f, instantiate))
                .collect(),
            span: s.base().tcx.def_span(def.def_id).sinto(s),
            attributes: get_def_attrs(tcx, def.def_id, get_def_kind(tcx, def.def_id)).sinto(s),
        }
    }
}

/// Reflects [`ty::EarlyParamRegion`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::EarlyParamRegion, state: S as s)]
pub struct EarlyParamRegion {
    pub index: u32,
    pub name: Symbol,
}

/// Reflects [`ty::LateParamRegion`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::LateParamRegion, state: S as s)]
pub struct LateParamRegion {
    pub scope: DefId,
    pub kind: LateParamRegionKind,
}

/// Reflects [`ty::LateParamRegionKind`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::LateParamRegionKind, state: S as s)]
pub enum LateParamRegionKind {
    Anon(u32),
    NamedAnon(u32, Symbol),
    #[custom_arm(&FROM_TYPE::Named(def_id) => TO_TYPE::Named(def_id.sinto(s), s.base().tcx.item_name(def_id).sinto(s)),)]
    Named(DefId, Symbol),
    ClosureEnv,
}

/// Reflects [`ty::RegionKind`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::RegionKind<'tcx>, state: S as gstate)]
pub enum RegionKind {
    ReEarlyParam(EarlyParamRegion),
    ReBound(BoundVarIndexKind, BoundRegion),
    ReLateParam(LateParamRegion),
    ReStatic,
    ReVar(RegionVid),
    RePlaceholder(PlaceholderRegion),
    ReErased,
    ReError(ErrorGuaranteed),
}

/// Reflects [`ty::BoundVarIndexKind`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Copy, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::BoundVarIndexKind, state: S as gstate)]
pub enum BoundVarIndexKind {
    Bound(DebruijnIndex),
    Canonical,
}

sinto_as_usize!(rustc_middle::ty, DebruijnIndex);
sinto_as_usize!(rustc_middle::ty, RegionVid);

/// Reflects [`ty::Region`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::Region<'tcx>, state: S as s)]
pub struct Region {
    #[value(self.kind().sinto(s))]
    pub kind: RegionKind,
}

/// Reflects both [`ty::GenericArg`] and [`ty::GenericArgKind`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::GenericArgKind<'tcx>, state: S as s)]
pub enum GenericArg {
    Lifetime(Region),
    Type(Ty),
    Const(ConstantExpr),
}

/// Contents of `ItemRef`.
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemRefContents {
    /// The item being refered to.
    pub def_id: DefId,
    /// The generics passed to the item. If `in_trait` is `Some`, these are only the generics of
    /// the method/type/const itself; generics for the traits are available in
    /// `in_trait.unwrap().trait`.
    pub generic_args: Vec<GenericArg>,
    /// Witnesses of the trait clauses required by the item, e.g. `T: Sized` for `Option<T>` or `B:
    /// ToOwned` for `Cow<'a, B>`. Same as above, for associated items this only includes clauses
    /// for the item itself.
    pub impl_exprs: Vec<ImplExpr>,
    /// If we're referring to a trait associated item, this gives the trait clause/impl we're
    /// referring to.
    pub in_trait: Option<ImplExpr>,
    /// Whether this contains any reference to a type/lifetime/const parameter.
    pub has_param: bool,
    /// Whether this contains any reference to a type/const parameter.
    pub has_non_lt_param: bool,
}

/// Reference to an item, with generics. Basically any mention of an item (function, type, etc)
/// uses this.
///
/// This can refer to a top-level item or to a trait associated item. Example:
/// ```ignore
/// trait MyTrait<TraitType, const TraitConst: usize> {
///   fn meth<MethType>(...) {...}
/// }
/// fn example_call<TraitType, SelfType: MyTrait<TraitType, 12>>(x: SelfType) {
///   x.meth::<String>(...)
/// }
/// ```
/// Here, in the call `x.meth::<String>(...)` we will build an `ItemRef` that looks like:
/// ```ignore
/// ItemRef {
///     def_id = MyTrait::meth,
///     generic_args = [String],
///     impl_exprs = [<proof of `String: Sized`>],
///     in_trait = Some(<proof of `SelfType: MyTrait<TraitType, 12>`>,
/// }
/// ```
/// The `in_trait` `ImplExpr` will have in its `trait` field a representation of the `SelfType:
/// MyTrait<TraitType, 12>` predicate, which looks like:
/// ```ignore
/// ItemRef {
///     def_id = MyTrait,
///     generic_args = [SelfType, TraitType, 12],
///     impl_exprs = [],
///     in_trait = None,
/// }
/// ```
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct ItemRef {
    pub(crate) contents: id_table::Node<ItemRefContents>,
}

impl ItemRefContents {
    #[cfg(feature = "rustc")]
    fn intern<'tcx, S: BaseState<'tcx>>(self, s: &S) -> ItemRef {
        s.with_global_cache(|cache| {
            let table_session = &mut cache.id_table_session;
            let contents = id_table::Node::new(self, table_session);
            ItemRef { contents }
        })
    }
}

impl ItemRef {
    /// The main way to obtain an `ItemRef`: from a `def_id` and generics.
    #[cfg(feature = "rustc")]
    pub fn translate<'tcx, S: UnderOwnerState<'tcx>>(
        s: &S,
        def_id: RDefId,
        generics: ty::GenericArgsRef<'tcx>,
    ) -> ItemRef {
        Self::translate_maybe_resolve_impl(
            s,
            s.base().options.item_ref_use_concrete_impl,
            def_id,
            generics,
        )
    }

    /// Makes a `ItemRef` from a `def_id` and generics.
    ///
    /// If `resolve_impl == true` and `(def_id, generics)` points to a trait item that
    /// can be resolved to a specific `impl`, `translate` rewrites `def_id` to the
    /// concrete associated item from that `impl` and re-bases the generics.
    ///
    /// For instance, [`<u32 as From<u8>>::from`] produces a [`ItemRef`] with a
    /// [`DefId`] looking like `core::convert::num::Impl#42::from` when
    /// `resolve_impl` is `true`, `core::convert::From::from` otherwise.
    #[cfg(feature = "rustc")]
    fn translate_maybe_resolve_impl<'tcx, S: UnderOwnerState<'tcx>>(
        s: &S,
        resolve_impl: bool,
        mut def_id: RDefId,
        mut generics: ty::GenericArgsRef<'tcx>,
    ) -> ItemRef {
        use rustc_infer::infer::canonical::ir::TypeVisitableExt;
        let key = (def_id, generics);
        if let Some(item) = s.with_cache(|cache| cache.item_refs.get(&key).cloned()) {
            return item;
        }

        let tcx = s.base().tcx;
        // If this is an associated item, resolve the trait reference.
        let mut trait_info = self_clause_for_item(s, def_id, generics);

        // If the reference is a known trait impl and the impl implements the target item, we can
        // point directly to the implemented item.
        if resolve_impl
            && let Some(tinfo) = &trait_info
            && let ImplExprAtom::Concrete(impl_ref) = &tinfo.r#impl
            && let impl_def_id = impl_ref.def_id.as_rust_def_id().unwrap()
            && let Some(implemented_item) = tcx
                .associated_items(impl_def_id)
                .in_definition_order()
                .find(|item| item.trait_item_def_id() == Some(def_id))
        {
            let trait_def_id = tcx.parent(def_id);
            def_id = implemented_item.def_id;
            generics = generics.rebase_onto(tcx, trait_def_id, impl_ref.rustc_args(s));
            trait_info = None;
        }

        let hax_def_id = def_id.sinto(s);
        let mut hax_generics = generics.sinto(s);
        let mut impl_exprs = solve_item_required_traits(s, def_id, generics);

        // Fixup the generics.
        if let Some(tinfo) = &trait_info {
            // The generics are split in two: the arguments of the trait and the arguments of the
            // method/associated item.
            //
            // For instance, if we have:
            // ```
            // trait Foo<T> {
            //     fn baz<U>(...) { ... }
            // }
            //
            // fn test<T : Foo<u32>(x: T) {
            //     x.baz(...);
            //     ...
            // }
            // ```
            // The generics for the call to `baz` will be the concatenation: `<T, u32, U>`, which we
            // split into `<T, u32>` and `<U>`.
            let trait_ref = tinfo.r#trait.hax_skip_binder_ref();
            let num_trait_generics = trait_ref.generic_args.len();
            hax_generics.drain(0..num_trait_generics);
            let mut num_trait_trait_clauses = trait_ref.impl_exprs.len();
            // Items other than associated types get an extra `Self: Trait` clause as the first
            // clause, we skip that one too. Note: that clause is the same as `tinfo`.
            if !matches!(hax_def_id.kind, DefKind::AssocTy) {
                num_trait_trait_clauses += 1;
            };
            impl_exprs.drain(0..num_trait_trait_clauses);
        }

        let content = ItemRefContents {
            def_id: hax_def_id,
            generic_args: hax_generics,
            impl_exprs,
            in_trait: trait_info,
            has_param: generics.has_param()
                || generics.has_escaping_bound_vars()
                || generics.has_free_regions(),
            has_non_lt_param: generics.has_param(),
        };
        let item = content.intern(s);
        s.with_cache(|cache| {
            cache.item_refs.insert(key, item.clone());
        });
        s.with_global_cache(|cache| {
            cache.reverse_item_refs_map.insert(item.id(), generics);
        });
        item
    }

    /// Construct an `ItemRef` for items that can't have generics (e.g. modules).
    #[cfg(feature = "rustc")]
    pub fn dummy_without_generics<'tcx, S: BaseState<'tcx>>(s: &S, def_id: DefId) -> ItemRef {
        let content = ItemRefContents {
            def_id,
            generic_args: Default::default(),
            impl_exprs: Default::default(),
            in_trait: Default::default(),
            has_param: false,
            has_non_lt_param: false,
        };
        let item = content.intern(s);
        s.with_global_cache(|cache| {
            cache
                .reverse_item_refs_map
                .insert(item.id(), ty::GenericArgsRef::default());
        });
        item
    }

    /// For an `ItemRef` that refers to a trait, this returns values for each of the non-gat
    /// associated types of this trait and its parents, in a fixed order.
    #[cfg(feature = "rustc")]
    pub fn trait_associated_types<'tcx, S: UnderOwnerState<'tcx>>(&self, s: &S) -> Vec<Ty> {
        if !matches!(self.def_id.kind, DefKind::Trait | DefKind::TraitAlias) {
            panic!("`ItemRef::trait_associated_types` expected a trait")
        }
        let tcx = s.base().tcx;
        let typing_env = s.typing_env();
        let def_id = self.def_id.as_rust_def_id().unwrap();
        let generics = self.rustc_args(s);
        let tref = ty::TraitRef::new(tcx, def_id, generics);
        rustc_utils::assoc_tys_for_trait(tcx, typing_env, tref)
            .into_iter()
            .map(|alias_ty| ty::Ty::new_alias(tcx, ty::Projection, alias_ty))
            .map(|ty| normalize(tcx, typing_env, ty))
            .map(|ty| ty.sinto(s))
            .collect()
    }

    /// Erase lifetimes from the generic arguments of this item.
    #[cfg(feature = "rustc")]
    pub fn erase<'tcx, S: UnderOwnerState<'tcx>>(&self, s: &S) -> Self {
        let def_id = self.def_id.underlying_rust_def_id();
        let args = self.rustc_args(s);
        let args = erase_and_norm(s.base().tcx, s.typing_env(), args);
        Self::translate(s, def_id, args).with_def_id(s, &self.def_id)
    }

    pub fn contents(&self) -> &ItemRefContents {
        &self.contents
    }

    /// Get a unique id identitying this `ItemRef`.
    pub fn id(&self) -> id_table::Id {
        self.contents.id()
    }

    /// Recover the original rustc args that generated this `ItemRef`. Will panic if the `ItemRef`
    /// was built by hand instead of using `translate_item_ref`.
    #[cfg(feature = "rustc")]
    pub fn rustc_args<'tcx, S: BaseState<'tcx>>(&self, s: &S) -> ty::GenericArgsRef<'tcx> {
        s.with_global_cache(|cache| *cache.reverse_item_refs_map.get(&self.id()).unwrap())
    }

    /// Mutate the `DefId`, keeping the same generic args.
    #[cfg(feature = "rustc")]
    pub fn mutate_def_id<'tcx, S: BaseState<'tcx>>(
        &self,
        s: &S,
        f: impl FnOnce(&mut DefId),
    ) -> Self {
        let args = self.rustc_args(s);
        let mut contents = self.contents().clone();
        f(&mut contents.def_id);
        let new = contents.intern(s);
        s.with_global_cache(|cache| {
            cache.reverse_item_refs_map.insert(new.id(), args);
        });
        new
    }

    /// Set the `DefId`, keeping the same generic args.
    #[cfg(feature = "rustc")]
    pub fn with_def_id<'tcx, S: BaseState<'tcx>>(&self, s: &S, def_id: &DefId) -> Self {
        self.mutate_def_id(s, |d| *d = def_id.clone())
    }
}

impl std::ops::Deref for ItemRef {
    type Target = ItemRefContents;
    fn deref(&self) -> &Self::Target {
        self.contents()
    }
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, GenericArg> for ty::GenericArg<'tcx> {
    fn sinto(&self, s: &S) -> GenericArg {
        self.kind().sinto(s)
    }
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, Vec<GenericArg>> for ty::GenericArgsRef<'tcx> {
    fn sinto(&self, s: &S) -> Vec<GenericArg> {
        self.iter().map(|v| v.kind().sinto(s)).collect()
    }
}

/// Reflects both [`ty::GenericArg`] and [`ty::GenericArgKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: BaseState<'tcx>>, from: rustc_ast::ast::LitIntType, state: S as gstate)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LitIntType {
    Signed(IntTy),
    Unsigned(UintTy),
    Unsuffixed,
}

/// Reflects partially [`ty::InferTy`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S>, from: ty::InferTy, state: S as gstate)]
pub enum InferTy {
    #[custom_arm(FROM_TYPE::TyVar(..) => TO_TYPE::TyVar,)]
    TyVar, /*TODO?*/
    #[custom_arm(FROM_TYPE::IntVar(..) => TO_TYPE::IntVar,)]
    IntVar, /*TODO?*/
    #[custom_arm(FROM_TYPE::FloatVar(..) => TO_TYPE::FloatVar,)]
    FloatVar, /*TODO?*/
    FreshTy(u32),
    FreshIntTy(u32),
    FreshFloatTy(u32),
}

/// Reflects [`rustc_type_ir::IntTy`]
#[derive(AdtInto)]
#[args(<S>, from: rustc_type_ir::IntTy, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Copy, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntTy {
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
}

/// Reflects [`rustc_type_ir::FloatTy`]
#[derive(AdtInto)]
#[args(<S>, from: rustc_type_ir::FloatTy, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Copy, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FloatTy {
    F16,
    F32,
    F64,
    F128,
}

/// Reflects [`rustc_type_ir::UintTy`]
#[derive(AdtInto)]
#[args(<S>, from: rustc_type_ir::UintTy, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Copy, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum UintTy {
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
}

impl ToString for IntTy {
    fn to_string(&self) -> String {
        use IntTy::*;
        match self {
            Isize => "isize".to_string(),
            I8 => "i8".to_string(),
            I16 => "i16".to_string(),
            I32 => "i32".to_string(),
            I64 => "i64".to_string(),
            I128 => "i128".to_string(),
        }
    }
}

impl ToString for UintTy {
    fn to_string(&self) -> String {
        use UintTy::*;
        match self {
            Usize => "usize".to_string(),
            U8 => "u8".to_string(),
            U16 => "u16".to_string(),
            U32 => "u32".to_string(),
            U64 => "u64".to_string(),
            U128 => "u128".to_string(),
        }
    }
}

/// Reflects [`ty::TypeAndMut`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::TypeAndMut<'tcx>, state: S as gstate)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeAndMut {
    pub ty: Box<Ty>,
    pub mutbl: Mutability,
}

#[cfg(feature = "rustc")]
impl<S, U, T: SInto<S, U>> SInto<S, Vec<U>> for ty::List<T> {
    fn sinto(&self, s: &S) -> Vec<U> {
        self.iter().map(|x| x.sinto(s)).collect()
    }
}

/// Reflects [`ty::Variance`]
#[derive(AdtInto)]
#[args(<S>, from: ty::Variance, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum Variance {
    Covariant,
    Invariant,
    Contravariant,
    Bivariant,
}

/// Reflects [`ty::GenericParamDef`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::GenericParamDef, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct GenericParamDef {
    pub name: Symbol,
    pub def_id: DefId,
    pub index: u32,
    pub pure_wrt_drop: bool,
    #[value(
        match self.kind {
            ty::GenericParamDefKind::Lifetime => GenericParamDefKind::Lifetime,
            ty::GenericParamDefKind::Type { has_default, synthetic } => GenericParamDefKind::Type { has_default, synthetic },
            ty::GenericParamDefKind::Const { has_default, .. } => {
                let ty = s.base().tcx.type_of(self.def_id).instantiate_identity().sinto(s);
                GenericParamDefKind::Const { has_default, ty }
            },
        }
    )]
    pub kind: GenericParamDefKind,
    /// Variance of this type parameter, if sensible.
    #[value({
        use rustc_hir::def::DefKind::*;
        let tcx = s.base().tcx;
        let parent = tcx.parent(self.def_id);
        match tcx.def_kind(parent) {
            Fn | AssocFn | Enum | Struct | Union | Ctor(..) | OpaqueTy => {
                tcx.variances_of(parent).get(self.index as usize).sinto(s)
            }
            _ => None
        }
    })]
    pub variance: Option<Variance>,
    #[value(s.base().tcx.def_span(self.def_id).sinto(s))]
    pub span: Span,
    #[value({
        let tcx = s.base().tcx;
        get_def_attrs(tcx, self.def_id, get_def_kind(tcx, self.def_id)).sinto(s)
    })]
    pub attributes: Vec<Attribute>,
}

/// Reflects [`ty::GenericParamDefKind`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum GenericParamDefKind {
    Lifetime,
    Type { has_default: bool, synthetic: bool },
    Const { has_default: bool, ty: Ty },
}

/// Reflects [`ty::Generics`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::Generics, state: S as state)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct TyGenerics {
    pub parent: Option<DefId>,
    pub parent_count: usize,
    #[from(own_params)]
    pub params: Vec<GenericParamDef>,
    // pub param_def_id_to_index: FxHashMap<DefId, u32>,
    pub has_self: bool,
    pub has_late_bound_regions: Option<Span>,
}

#[cfg(feature = "rustc")]
impl TyGenerics {
    pub(crate) fn count_total_params(&self) -> usize {
        self.parent_count + self.params.len()
    }
}

/// This type merges the information from
/// `rustc_type_ir::AliasKind` and `ty::AliasTy`
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Alias {
    pub kind: AliasKind,
    pub args: Vec<GenericArg>,
    pub def_id: DefId,
}

/// Reflects [`ty::AliasKind`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AliasKind {
    /// The projection of a trait type: `<Ty as Trait<...>>::Type<...>`
    Projection {
        /// The `impl Trait for Ty` in `Ty: Trait<..., Type = U>`.
        impl_expr: ImplExpr,
        /// The `Type` in `Ty: Trait<..., Type = U>`.
        assoc_item: AssocItem,
    },
    /// An associated type in an inherent impl.
    Inherent,
    /// An `impl Trait` opaque type.
    Opaque {
        /// The real type hidden inside this opaque type.
        hidden_ty: Ty,
    },
    /// A type alias that references opaque types. Likely to always be normalized away.
    Free,
}

#[cfg(feature = "rustc")]
impl Alias {
    #[tracing::instrument(level = "trace", skip(s))]
    fn from<'tcx, S: UnderOwnerState<'tcx>>(
        s: &S,
        alias_kind: &rustc_type_ir::AliasTyKind,
        alias_ty: &ty::AliasTy<'tcx>,
    ) -> TyKind {
        let tcx = s.base().tcx;
        let typing_env = s.typing_env();
        use rustc_type_ir::AliasTyKind as RustAliasKind;

        // Try to normalize the alias first.
        let ty = ty::Ty::new_alias(tcx, *alias_kind, *alias_ty);
        let ty = crate::traits::normalize(tcx, typing_env, ty);
        let ty::Alias(alias_kind, alias_ty) = ty.kind() else {
            let ty: Ty = ty.sinto(s);
            return ty.kind().clone();
        };

        let kind = match alias_kind {
            RustAliasKind::Projection => {
                let trait_ref = alias_ty.trait_ref(tcx);
                // In a case like:
                // ```
                // impl<T, U> Trait for Result<T, U>
                // where
                //     for<'a> &'a Result<T, U>: IntoIterator,
                //     for<'a> <&'a Result<T, U> as IntoIterator>::Item: Copy,
                // {}
                // ```
                // the `&'a Result<T, U> as IntoIterator` trait ref has escaping bound variables
                // yet we dont have a binder around (could even be several). Binding this correctly
                // is therefore difficult. Since our trait resolution ignores lifetimes anyway, we
                // just erase them. See also https://github.com/pereq-dev/hax/issues/747.
                let trait_ref = crate::traits::erase_free_regions(tcx, trait_ref);
                let item = tcx.associated_item(alias_ty.def_id);
                AliasKind::Projection {
                    assoc_item: AssocItem::sfrom(s, &item),
                    impl_expr: solve_trait(s, ty::Binder::dummy(trait_ref)),
                }
            }
            RustAliasKind::Inherent => AliasKind::Inherent,
            RustAliasKind::Opaque => {
                // Reveal the underlying `impl Trait` type.
                let ty = tcx.type_of(alias_ty.def_id).instantiate(tcx, alias_ty.args);
                AliasKind::Opaque {
                    hidden_ty: ty.sinto(s),
                }
            }
            RustAliasKind::Free => AliasKind::Free,
        };
        TyKind::Alias(Alias {
            kind,
            args: alias_ty.args.sinto(s),
            def_id: alias_ty.def_id.sinto(s),
        })
    }
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, Box<Ty>> for ty::Ty<'tcx> {
    fn sinto(&self, s: &S) -> Box<Ty> {
        Box::new(self.sinto(s))
    }
}

/// Reflects [`rustc_middle::ty::Ty`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct Ty {
    pub(crate) kind: id_table::Node<TyKind>,
}

impl Ty {
    #[cfg(feature = "rustc")]
    pub fn new<'tcx, S: BaseState<'tcx>>(s: &S, kind: TyKind) -> Self {
        s.with_global_cache(|cache| {
            let table_session = &mut cache.id_table_session;
            let kind = id_table::Node::new(kind, table_session);
            Ty { kind }
        })
    }

    pub fn inner(&self) -> &Arc<TyKind> {
        self.kind.inner()
    }

    pub fn kind(&self) -> &TyKind {
        self.inner().as_ref()
    }
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, Ty> for rustc_middle::ty::Ty<'tcx> {
    fn sinto(&self, s: &S) -> Ty {
        if let Some(ty) = s.with_cache(|cache| cache.tys.get(self).cloned()) {
            return ty;
        }
        let kind: TyKind = self.kind().sinto(s);
        let ty = Ty::new(s, kind);
        s.with_cache(|cache| {
            cache.tys.insert(*self, ty.clone());
        });
        ty
    }
}

/// Reflects [`ty::TyKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::TyKind<'tcx>, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TyKind {
    Bool,
    Char,
    Int(IntTy),
    Uint(UintTy),
    Float(FloatTy),

    #[custom_arm(
        ty::TyKind::FnDef(fun_id, generics) => {
            let item = translate_item_ref(s, *fun_id, generics);
            let tcx = s.base().tcx;
            let fn_sig = tcx.fn_sig(*fun_id).instantiate(tcx, generics);
            let fn_sig = Box::new(fn_sig.sinto(s));
            TyKind::FnDef { item, fn_sig }
        },
    )]
    /// Reflects [`ty::TyKind::FnDef`]
    FnDef {
        item: ItemRef,
        fn_sig: Box<PolyFnSig>,
    },

    #[custom_arm(
        ty::TyKind::FnPtr(tys, header) => {
            let sig = tys.map_bound(|tys| ty::FnSig {
                inputs_and_output: tys.inputs_and_output,
                c_variadic: header.c_variadic,
                safety: header.safety,
                abi: header.abi,
            });
            TyKind::Arrow(Box::new(sig.sinto(s)))
        },
    )]
    /// Reflects [`ty::TyKind::FnPtr`]
    Arrow(Box<PolyFnSig>),

    #[custom_arm(
        ty::TyKind::Closure (def_id, generics) => {
            let closure = generics.as_closure();
            TyKind::Closure(ClosureArgs::sfrom(s, *def_id, closure))
        },
    )]
    Closure(ClosureArgs),

    #[custom_arm(FROM_TYPE::Adt(adt_def, generics) => TO_TYPE::Adt(translate_item_ref(s, adt_def.did(), generics)),)]
    Adt(ItemRef),
    #[custom_arm(FROM_TYPE::Foreign(def_id) => TO_TYPE::Foreign(translate_item_ref(s, *def_id, Default::default())),)]
    Foreign(ItemRef),
    /// The `ItemRef` uses the fake `Array` def_id.
    #[custom_arm(FROM_TYPE::Array(ty, len) => TO_TYPE::Array({
        let def_id = s.with_global_cache(|c| c.get_synthetic_def_id(s, SyntheticItem::Array));
        let args = s.base().tcx.mk_args(&[(*ty).into(), (*len).into()]);
        ItemRef::translate(s, def_id, args)
    }),)]
    Array(ItemRef),
    /// The `ItemRef` uses the fake `Slice` def_id.
    #[custom_arm(FROM_TYPE::Slice(ty) => TO_TYPE::Slice({
        let def_id = s.with_global_cache(|c| c.get_synthetic_def_id(s, SyntheticItem::Slice));
        let args = s.base().tcx.mk_args(&[(*ty).into()]);
        ItemRef::translate(s, def_id, args)
    }),)]
    Slice(ItemRef),
    /// The `ItemRef` uses the fake `Tuple` def_id.
    #[custom_arm(FROM_TYPE::Tuple(tys) => TO_TYPE::Tuple({
        let def_id = s.with_global_cache(|c| c.get_synthetic_def_id(s, SyntheticItem::Tuple(tys.len())));
        let args = s.base().tcx.mk_args_from_iter(tys.into_iter().map(ty::GenericArg::from));
        ItemRef::translate(s, def_id, args)
    }),)]
    Tuple(ItemRef),
    Str,
    RawPtr(Box<Ty>, Mutability),
    Ref(Region, Box<Ty>, Mutability),
    #[custom_arm(FROM_TYPE::Dynamic(preds, region) => make_dyn(s, preds, region),)]
    Dynamic(
        /// Fresh type parameter that we use as the `Self` type in the prediates below.
        ParamTy,
        /// Clauses that define the trait object. These clauses use the fresh type parameter above
        /// as `Self` type.
        GenericPredicates,
        Region,
    ),
    #[custom_arm(FROM_TYPE::Coroutine(def_id, generics) => TO_TYPE::Coroutine(translate_item_ref(s, *def_id, generics)),)]
    Coroutine(ItemRef),
    Never,
    #[custom_arm(FROM_TYPE::Alias(alias_kind, alias_ty) => Alias::from(s, alias_kind, alias_ty),)]
    Alias(Alias),
    Param(ParamTy),
    Bound(BoundVarIndexKind, BoundTy),
    Placeholder(PlaceholderType),
    Infer(InferTy),
    #[custom_arm(FROM_TYPE::Error(..) => TO_TYPE::Error,)]
    Error,
    #[todo]
    Todo(String),
}

/// Transform existential predicates into properly resolved predicates.
#[cfg(feature = "rustc")]
fn make_dyn<'tcx, S: UnderOwnerState<'tcx>>(
    s: &S,
    epreds: &'tcx ty::List<ty::Binder<'tcx, ty::ExistentialPredicate<'tcx>>>,
    region: &ty::Region<'tcx>,
) -> TyKind {
    let tcx = s.base().tcx;
    let def_id = s.owner_id();
    let span = rustc_span::DUMMY_SP.sinto(s);

    // Pretend there's an extra type in the environment.
    let new_param_ty = {
        let generics = tcx.generics_of(def_id);
        let param_count = generics.parent_count + generics.own_params.len();
        ty::ParamTy::new(param_count as u32 + 1, rustc_span::Symbol::intern("_dyn"))
    };
    let new_ty = new_param_ty.to_ty(tcx);

    // Set the new type as the `Self` parameter of our predicates.
    let clauses: Vec<ty::Clause<'_>> = epreds
        .iter()
        .map(|epred| epred.with_self_ty(tcx, new_ty))
        .collect();

    // Populate a predicate searcher that knows about the `dyn` clauses.
    let mut predicate_searcher = s.with_predicate_searcher(|ps| ps.clone());
    predicate_searcher
        .insert_bound_predicates(clauses.iter().filter_map(|clause| clause.as_trait_clause()));
    predicate_searcher.set_param_env(rustc_trait_selection::traits::normalize_param_env_or_error(
        tcx,
        ty::ParamEnv::new(
            tcx.mk_clauses_from_iter(
                s.param_env()
                    .caller_bounds()
                    .iter()
                    .chain(clauses.iter().copied()),
            ),
        ),
        rustc_trait_selection::traits::ObligationCause::dummy(),
    ));

    // Using the predicate searcher, translate the predicates. Only the projection predicates need
    // to be handled specially.
    let predicates = clauses
        .into_iter()
        .map(|clause| {
            let clause = match clause.as_projection_clause() {
                // Translate normally
                None => clause.sinto(s),
                // Translate by hand using our predicate searcher. This does the same as
                // `clause.sinto(s)` except that it uses our predicate searcher to resolve the
                // projection `ImplExpr`.
                Some(proj) => {
                    let bound_vars = proj.bound_vars().sinto(s);
                    let proj = {
                        let alias_ty = &proj.skip_binder().projection_term.expect_ty(tcx);
                        let impl_expr = {
                            let poly_trait_ref = proj.rebind(alias_ty.trait_ref(tcx));
                            predicate_searcher
                                .resolve(&poly_trait_ref, &|_| {})
                                .s_unwrap(s)
                                .sinto(s)
                        };
                        let Term::Ty(ty) = proj.skip_binder().term.sinto(s) else {
                            unreachable!()
                        };
                        let item = tcx.associated_item(alias_ty.def_id);
                        ProjectionPredicate {
                            impl_expr,
                            assoc_item: AssocItem::sfrom(s, &item),
                            ty,
                        }
                    };
                    let kind = Binder {
                        value: ClauseKind::Projection(proj),
                        bound_vars,
                    };
                    let id = kind.clone().map(PredicateKind::Clause).predicate_id();
                    Clause { kind, id }
                }
            };
            (clause, span.clone())
        })
        .collect();

    let predicates = GenericPredicates { predicates };
    let param_ty = new_param_ty.sinto(s);
    let region = region.sinto(s);
    TyKind::Dynamic(param_ty, predicates, region)
}

/// Reflects [`ty::CanonicalUserTypeAnnotation`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::CanonicalUserTypeAnnotation<'tcx>, state: S as gstate)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub struct CanonicalUserTypeAnnotation {
    pub user_ty: CanonicalUserType,
    pub span: Span,
    pub inferred_ty: Ty,
}

/// Reflects [`ty::AdtKind`]
#[derive_group(Serializers)]
#[derive(Copy, Clone, Debug, JsonSchema)]
pub enum AdtKind {
    Struct,
    Union,
    Enum,
    /// We sometimes pretend arrays are an ADT and generate a `FullDef` for them.
    Array,
    /// We sometimes pretend slices are an ADT and generate a `FullDef` for them.
    Slice,
    /// We sometimes pretend tuples are an ADT and generate a `FullDef` for them.
    Tuple,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, AdtKind> for ty::AdtKind {
    fn sinto(&self, _s: &S) -> AdtKind {
        match self {
            ty::AdtKind::Struct => AdtKind::Struct,
            ty::AdtKind::Union => AdtKind::Union,
            ty::AdtKind::Enum => AdtKind::Enum,
        }
    }
}

sinto_todo!(rustc_middle::ty, AdtFlags);

/// Reflects [`ty::ReprOptions`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: rustc_abi::ReprOptions, state: S as s)]
pub struct ReprOptions {
    /// Whether an explicit integer representation was specified.
    #[value(self.int.is_some())]
    pub int_specified: bool,
    /// The actual discriminant type resulting from the representation options.
    #[value({
        use crate::rustc_middle::ty::util::IntTypeExt;
        self.discr_type().to_ty(s.base().tcx).sinto(s)
    })]
    pub typ: Ty,
    pub align: Option<Align>,
    pub pack: Option<Align>,
    #[value(ReprFlags { is_c: self.c(), is_transparent: self.transparent(), is_simd: self.simd() })]
    pub flags: ReprFlags,
}

/// The representation flags without the ones irrelevant outside of rustc.
#[derive_group(Serializers)]
#[derive(Default, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReprFlags {
    pub is_c: bool,
    pub is_transparent: bool,
    pub is_simd: bool,
}

/// Reflects [`ty::Align`], but directly stores the number of bytes as a u64.
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: BaseState<'tcx>>, from: rustc_abi::Align, state: S as _s)]
pub struct Align {
    #[value({
        self.bytes()
    })]
    pub bytes: u64,
}

/// Reflects [`ty::adjustment::PointerCoercion`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum PointerCoercion {
    ReifyFnPointer,
    UnsafeFnPointer,
    ClosureFnPointer(Safety),
    MutToConstPointer,
    ArrayToPointer,
    Unsize(UnsizingMetadata),
}

/// The metadata to attach to the newly-unsized ptr.
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema)]
pub enum UnsizingMetadata {
    Length(ConstantExpr),
    VTablePtr(ImplExpr),
    Unknown,
}

#[cfg(feature = "rustc")]
impl PointerCoercion {
    pub fn sfrom<'tcx, S: UnderOwnerState<'tcx>>(
        s: &S,
        coercion: ty::adjustment::PointerCoercion,
        src_ty: ty::Ty<'tcx>,
        tgt_ty: ty::Ty<'tcx>,
    ) -> PointerCoercion {
        match coercion {
            ty::adjustment::PointerCoercion::ReifyFnPointer => PointerCoercion::ReifyFnPointer,
            ty::adjustment::PointerCoercion::UnsafeFnPointer => PointerCoercion::UnsafeFnPointer,
            ty::adjustment::PointerCoercion::ClosureFnPointer(x) => {
                PointerCoercion::ClosureFnPointer(x.sinto(s))
            }
            ty::adjustment::PointerCoercion::MutToConstPointer => {
                PointerCoercion::MutToConstPointer
            }
            ty::adjustment::PointerCoercion::ArrayToPointer => PointerCoercion::ArrayToPointer,
            ty::adjustment::PointerCoercion::Unsize => {
                // We only support unsizing behind references, pointers and boxes for now.
                let meta = match (src_ty.builtin_deref(true), tgt_ty.builtin_deref(true)) {
                    (Some(src_ty), Some(tgt_ty)) => {
                        let tcx = s.base().tcx;
                        let typing_env = s.typing_env();
                        let (src_ty, tgt_ty) =
                            tcx.struct_lockstep_tails_raw(src_ty, tgt_ty, |ty| {
                                normalize(tcx, typing_env, ty)
                            });
                        match tgt_ty.kind() {
                            ty::Slice(_) | ty::Str => match src_ty.kind() {
                                ty::Array(_, len) => {
                                    let len = len.sinto(s);
                                    UnsizingMetadata::Length(len)
                                }
                                _ => UnsizingMetadata::Unknown,
                            },
                            ty::Dynamic(preds, ..) => {
                                let pred = preds[0].with_self_ty(tcx, src_ty);
                                let clause = pred.as_trait_clause().expect(
                                    "the first `ExistentialPredicate` of `TyKind::Dynamic` \
                                        should be a trait clause",
                                );
                                let tref = clause.rebind(clause.skip_binder().trait_ref);
                                let impl_expr = solve_trait(s, tref);
                                UnsizingMetadata::VTablePtr(impl_expr)
                            }
                            _ => UnsizingMetadata::Unknown,
                        }
                    }
                    _ => UnsizingMetadata::Unknown,
                };
                PointerCoercion::Unsize(meta)
            }
        }
    }
}

/// Reflects [`ty::FnSig`]
#[derive_group(Serializers)]
#[derive(AdtInto, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::FnSig<'tcx>, state: S as s)]
pub struct TyFnSig {
    #[value(self.inputs().sinto(s))]
    pub inputs: Vec<Ty>,
    #[value(self.output().sinto(s))]
    pub output: Ty,
    pub c_variadic: bool,
    pub safety: Safety,
    pub abi: ExternAbi,
}

/// Reflects [`ty::PolyFnSig`]
pub type PolyFnSig = Binder<TyFnSig>;

/// Reflects [`ty::TraitRef`]
/// Contains the def_id and arguments passed to the trait. The first type argument is the `Self`
/// type. The `ImplExprs` are the _required_ predicate for this trait; currently they are always
/// empty because we consider all trait predicates as implied.
/// `self.in_trait` is always `None` because a trait can't be associated to another one.
pub type TraitRef = ItemRef;

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, TraitRef> for ty::TraitRef<'tcx> {
    fn sinto(&self, s: &S) -> TraitRef {
        translate_item_ref(s, self.def_id, self.args)
    }
}

/// Reflects [`ty::TraitPredicate`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::TraitPredicate<'tcx>, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TraitPredicate {
    pub trait_ref: TraitRef,
    #[map(*x == ty::PredicatePolarity::Positive)]
    #[from(polarity)]
    pub is_positive: bool,
}

/// Reflects [`ty::OutlivesPredicate`] as a named struct
/// instead of a tuple struct. This is because the script converting
/// JSONSchema types to OCaml doesn't support tuple structs, and this
/// is the only tuple struct in the whole AST.
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OutlivesPredicate<T> {
    pub lhs: T,
    pub rhs: Region,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>, T, U> SInto<S, OutlivesPredicate<U>>
    for ty::OutlivesPredicate<'tcx, T>
where
    T: SInto<S, U>,
{
    fn sinto(&self, s: &S) -> OutlivesPredicate<U> where {
        OutlivesPredicate {
            lhs: self.0.sinto(s),
            rhs: self.1.sinto(s),
        }
    }
}

/// Reflects [`ty::RegionOutlivesPredicate`]
pub type RegionOutlivesPredicate = OutlivesPredicate<Region>;
/// Reflects [`ty::TypeOutlivesPredicate`]
pub type TypeOutlivesPredicate = OutlivesPredicate<Ty>;

/// Reflects [`ty::Term`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Term {
    Ty(Ty),
    Const(ConstantExpr),
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, Term> for ty::Term<'tcx> {
    fn sinto(&self, s: &S) -> Term {
        use ty::TermKind;
        match self.kind() {
            TermKind::Ty(ty) => Term::Ty(ty.sinto(s)),
            TermKind::Const(c) => Term::Const(c.sinto(s)),
        }
    }
}

/// Expresses a constraints over an associated type.
///
/// For instance:
/// ```text
/// fn f<T : Foo<S = String>>(...)
///              ^^^^^^^^^^
/// ```
/// (provided the trait `Foo` has an associated type `S`).
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectionPredicate {
    /// The `impl Trait for Ty` in `Ty: Trait<..., Type = U>`.
    pub impl_expr: ImplExpr,
    /// The `Type` in `Ty: Trait<..., Type = U>`.
    pub assoc_item: AssocItem,
    /// The type `U` in `Ty: Trait<..., Type = U>`.
    pub ty: Ty,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderBinderState<'tcx>> SInto<S, ProjectionPredicate>
    for ty::ProjectionPredicate<'tcx>
{
    fn sinto(&self, s: &S) -> ProjectionPredicate {
        let tcx = s.base().tcx;
        let alias_ty = &self.projection_term.expect_ty(tcx);
        let poly_trait_ref = s.binder().rebind(alias_ty.trait_ref(tcx));
        let Term::Ty(ty) = self.term.sinto(s) else {
            unreachable!()
        };
        let item = tcx.associated_item(alias_ty.def_id);
        ProjectionPredicate {
            impl_expr: solve_trait(s, poly_trait_ref),
            assoc_item: AssocItem::sfrom(s, &item),
            ty,
        }
    }
}

/// Reflects [`ty::ClauseKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderBinderState<'tcx>>, from: ty::ClauseKind<'tcx>, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClauseKind {
    Trait(TraitPredicate),
    RegionOutlives(RegionOutlivesPredicate),
    TypeOutlives(TypeOutlivesPredicate),
    Projection(ProjectionPredicate),
    ConstArgHasType(ConstantExpr, Ty),
    WellFormed(Term),
    ConstEvaluatable(ConstantExpr),
    HostEffect(HostEffectPredicate),
    UnstableFeature(Symbol),
}

sinto_todo!(rustc_middle::ty, HostEffectPredicate<'tcx>);

/// Reflects [`ty::Clause`] and adds a hash-consed predicate identifier.
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Clause {
    pub kind: Binder<ClauseKind>,
    pub id: PredicateId,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, Clause> for ty::Clause<'tcx> {
    fn sinto(&self, s: &S) -> Clause {
        let kind = self.kind().sinto(s);
        let id = kind.clone().map(PredicateKind::Clause).predicate_id();
        Clause { kind, id }
    }
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, Clause> for ty::PolyTraitPredicate<'tcx> {
    fn sinto(&self, s: &S) -> Clause {
        let kind: Binder<_> = self.sinto(s);
        let kind: Binder<ClauseKind> = kind.map(ClauseKind::Trait);
        let id = kind.clone().map(PredicateKind::Clause).predicate_id();
        Clause { kind, id }
    }
}

/// Reflects [`ty::Predicate`] and adds a hash-consed predicate identifier.
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Predicate {
    pub kind: Binder<PredicateKind>,
    pub id: PredicateId,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, Predicate> for ty::Predicate<'tcx> {
    fn sinto(&self, s: &S) -> Predicate {
        let kind = self.kind().sinto(s);
        let id = kind.predicate_id();
        Predicate { kind, id }
    }
}

/// Reflects [`ty::BoundVariableKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::BoundVariableKind, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoundVariableKind {
    Ty(BoundTyKind),
    Region(BoundRegionKind),
    Const,
}

/// Reflects [`ty::Binder`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Binder<T> {
    pub value: T,
    pub bound_vars: Vec<BoundVariableKind>,
}

impl<T> Binder<T> {
    pub fn as_ref(&self) -> Binder<&T> {
        Binder {
            value: &self.value,
            bound_vars: self.bound_vars.clone(),
        }
    }

    pub fn hax_skip_binder(self) -> T {
        self.value
    }

    pub fn hax_skip_binder_ref(&self) -> &T {
        &self.value
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Binder<U> {
        Binder {
            value: f(self.value),
            bound_vars: self.bound_vars,
        }
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn rebind<U>(&self, value: U) -> Binder<U> {
        self.as_ref().map(|_| value)
    }
}

/// Reflects [`ty::GenericPredicates`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::GenericPredicates<'tcx>, state: S as s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, Default, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenericPredicates {
    #[value(self.predicates.iter().map(|x| x.sinto(s)).collect())]
    pub predicates: Vec<(Clause, Span)>,
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>> SInto<S, GenericPredicates>
    for crate::traits::Predicates<'tcx>
{
    fn sinto(&self, s: &S) -> GenericPredicates {
        GenericPredicates {
            predicates: self.as_ref().sinto(s),
        }
    }
}

#[cfg(feature = "rustc")]
impl<'tcx, S: UnderOwnerState<'tcx>, T1, T2> SInto<S, Binder<T2>> for ty::Binder<'tcx, T1>
where
    T1: SInto<StateWithBinder<'tcx>, T2>,
{
    fn sinto(&self, s: &S) -> Binder<T2> {
        let bound_vars = self.bound_vars().sinto(s);
        let value = {
            let under_binder_s = &s.with_binder(self.as_ref().map_bound(|_| ()));
            self.as_ref().skip_binder().sinto(under_binder_s)
        };
        Binder { value, bound_vars }
    }
}

/// Reflects [`ty::SubtypePredicate`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::SubtypePredicate<'tcx>, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubtypePredicate {
    pub a_is_expected: bool,
    pub a: Ty,
    pub b: Ty,
}

/// Reflects [`ty::CoercePredicate`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::CoercePredicate<'tcx>, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CoercePredicate {
    pub a: Ty,
    pub b: Ty,
}

/// Reflects [`ty::AliasRelationDirection`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::AliasRelationDirection, state: S as _tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AliasRelationDirection {
    Equate,
    Subtype,
}

/// Reflects [`ty::ClosureArgs`]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, JsonSchema)]
#[derive_group(Serializers)]
pub struct ClosureArgs {
    pub item: ItemRef,
    /// The base kind of this closure. The kinds are ordered by inclusion: any `Fn` works as an
    /// `FnMut`, and any `FnMut` works as an `FnOnce`.
    pub kind: ClosureKind,
    /// The signature of the function that the closure implements, e.g. `fn(A, B, C) -> D`.
    pub fn_sig: PolyFnSig,
    /// The set of captured variables. Together they form the state of the closure.
    pub upvar_tys: Vec<Ty>,
}

impl ClosureArgs {
    /// Iterate over the upvars that are borrows with erased regions. These may require allocating
    /// fresh regions.
    pub fn iter_upvar_borrows(&self) -> impl Iterator<Item = &Ty> {
        self.upvar_tys.iter().filter(|ty| {
            matches!(
                ty.kind(),
                TyKind::Ref(
                    Region {
                        kind: RegionKind::ReErased
                    },
                    ..
                )
            )
        })
    }
}

#[cfg(feature = "rustc")]
impl ClosureArgs {
    // Manual implementation because we need the `def_id` of the closure.
    pub(crate) fn sfrom<'tcx, S>(
        s: &S,
        def_id: RDefId,
        from: ty::ClosureArgs<ty::TyCtxt<'tcx>>,
    ) -> Self
    where
        S: UnderOwnerState<'tcx>,
    {
        let tcx = s.base().tcx;
        let sig = from.sig();
        let item = {
            // The closure has no generics of its own: it inherits its parent generics and could
            // have late-bound args but these are part of the signature.
            let parent_args = tcx.mk_args(from.parent_args());
            translate_item_ref(s, def_id, parent_args)
        };
        ClosureArgs {
            item,
            kind: from.kind().sinto(s),
            fn_sig: tcx
                .signature_unclosure(sig, rustc_hir::Safety::Safe)
                .sinto(s),
            upvar_tys: from.upvar_tys().sinto(s),
        }
    }
}

/// Reflects [`ty::ClosureKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderOwnerState<'tcx>>, from: ty::ClosureKind, state: S as _tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClosureKind {
    Fn,
    FnMut,
    FnOnce,
}

sinto_todo!(rustc_middle::ty, NormalizesTo<'tcx>);

/// Reflects [`ty::PredicateKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: UnderBinderState<'tcx>>, from: ty::PredicateKind<'tcx>, state: S as tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PredicateKind {
    Clause(ClauseKind),
    DynCompatible(DefId),
    Subtype(SubtypePredicate),
    Coerce(CoercePredicate),
    ConstEquate(ConstantExpr, ConstantExpr),
    Ambiguous,
    AliasRelate(Term, Term, AliasRelationDirection),
    NormalizesTo(NormalizesTo),
}

/// Reflects [`ty::AssocItem`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssocItem {
    pub def_id: DefId,
    /// This is `None` for RPTITs.
    pub name: Option<Symbol>,
    pub kind: AssocKind,
    pub container: AssocItemContainer,
    /// Whether this item has a value (e.g. this is `false` for trait methods without default
    /// implementations).
    pub has_value: bool,
}

#[cfg(feature = "rustc")]
impl AssocItem {
    pub fn sfrom<'tcx, S: BaseState<'tcx>>(s: &S, item: &ty::AssocItem) -> AssocItem {
        Self::sfrom_instantiated(s, item, None)
    }

    /// Translate an `AssocItem` and optionally instantiate it with the provided arguments.
    pub fn sfrom_instantiated<'tcx, S: BaseState<'tcx>>(
        s: &S,
        item: &ty::AssocItem,
        item_args: Option<ty::GenericArgsRef<'tcx>>,
    ) -> AssocItem {
        let tcx = s.base().tcx;
        // We want to solve traits in the context of this item.
        let s = &s.with_owner_id(item.def_id);
        let item_args =
            item_args.unwrap_or_else(|| ty::GenericArgs::identity_for_item(tcx, item.def_id));
        let container_id = item.container_id(tcx);
        let container_args = item_args.truncate_to(tcx, tcx.generics_of(container_id));
        let container = match item.container {
            ty::AssocContainer::Trait => {
                let trait_ref =
                    ty::TraitRef::new_from_args(tcx, container_id, container_args).sinto(s);
                AssocItemContainer::TraitContainer { trait_ref }
            }
            ty::AssocContainer::TraitImpl(implemented_item_id) => {
                let implemented_item_id = implemented_item_id.unwrap();
                let item = translate_item_ref(s, container_id, container_args);
                let implemented_trait_ref = tcx
                    .impl_trait_ref(container_id)
                    .instantiate(tcx, container_args);
                let implemented_trait_item = translate_item_ref(
                    s,
                    implemented_item_id,
                    item_args.rebase_onto(tcx, container_id, implemented_trait_ref.args),
                );
                AssocItemContainer::TraitImplContainer {
                    impl_: item,
                    implemented_trait_ref: implemented_trait_ref.sinto(s),
                    implemented_trait_item,
                    overrides_default: tcx.defaultness(implemented_item_id).has_value(),
                }
            }
            ty::AssocContainer::InherentImpl => AssocItemContainer::InherentImplContainer {
                impl_id: container_id.sinto(s),
            },
        };
        AssocItem {
            def_id: item.def_id.sinto(s),
            name: item.opt_name().sinto(s),
            kind: item.kind.sinto(s),
            container,
            has_value: item.defaultness(tcx).has_value(),
        }
    }
}

/// Reflects [`ty::AssocKind`]
#[derive(AdtInto)]
#[args(<'tcx, S: BaseState<'tcx>>, from: ty::AssocKind, state: S as _tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssocKind {
    Const { name: Symbol },
    Fn { name: Symbol, has_self: bool },
    Type { data: AssocTypeData },
}

/// Reflects [`ty::AssocTypeData`]
#[derive(AdtInto)]
#[args(<'tcx, S: BaseState<'tcx>>, from: ty::AssocTypeData, state: S as _tcx)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssocTypeData {
    Normal(Symbol),
    Rpitit(ImplTraitInTraitData),
}

#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssocItemContainer {
    TraitContainer {
        trait_ref: TraitRef,
    },
    TraitImplContainer {
        /// Reference to the def_id of the impl block.
        impl_: ItemRef,
        /// The trait ref implemented by the impl block.
        implemented_trait_ref: TraitRef,
        /// The the associated item (in the trait declaration) that is being implemented.
        implemented_trait_item: ItemRef,
        /// Whether the corresponding trait item had a default (and therefore this one overrides
        /// it).
        overrides_default: bool,
    },
    InherentImplContainer {
        impl_id: DefId,
    },
}

/// Reflects [`ty::ImplTraitInTraitData`]
#[derive(AdtInto)]
#[args(<'tcx, S: BaseState<'tcx>>, from: ty::ImplTraitInTraitData, state: S as _s)]
#[derive_group(Serializers)]
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImplTraitInTraitData {
    Trait {
        fn_def_id: DefId,
        opaque_def_id: DefId,
    },
    Impl {
        fn_def_id: DefId,
    },
}
