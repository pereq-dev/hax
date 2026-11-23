//! This module contains the type definition for `DefId` and the types
//! `DefId` depends on.
//!
//! This is purposely a very small isolated module:
//! `hax-engine-names-extract` uses those types, but we don't want
//! `hax-engine-names-extract` to have a build dependency on the whole
//! frontend, that double the build times for the Rust part of hax.
//!
//! The feature `extract_names_mode` exists only in the crate
//! `hax-engine-names-extract`, and is used to turn off the derive
//! attributes `AdtInto` and `JsonSchema`.

use hax_adt_into::derive_group;

#[cfg(feature = "rustc")]
use crate::prelude::*;
#[cfg(not(feature = "extract_names_mode"))]
use crate::{AdtInto, JsonSchema};

#[cfg(feature = "rustc")]
use {rustc_hir as hir, rustc_hir::def_id::DefId as RDefId, rustc_middle::ty};

pub type Symbol = String;
#[cfg(not(feature = "extract_names_mode"))]
pub type ByteSymbol = Vec<u8>;

#[cfg(all(not(feature = "extract_names_mode"), feature = "rustc"))]
impl<'t, S> SInto<S, Symbol> for rustc_span::symbol::Symbol {
    fn sinto(&self, _s: &S) -> Symbol {
        self.to_ident_string()
    }
}

#[cfg(all(not(feature = "extract_names_mode"), feature = "rustc"))]
impl<'t, S> SInto<S, ByteSymbol> for rustc_span::symbol::ByteSymbol {
    fn sinto(&self, _s: &S) -> ByteSymbol {
        self.as_byte_str().to_owned()
    }
}

/// Reflects [`hir::Safety`]
#[cfg_attr(not(feature = "extract_names_mode"), derive(AdtInto, JsonSchema))]
#[cfg_attr(not(feature = "extract_names_mode"), args(<S>, from: hir::Safety, state: S as _s))]
#[derive_group(Serializers)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Safety {
    Unsafe,
    Safe,
}

pub type Mutability = bool;
#[cfg(not(feature = "extract_names_mode"))]
pub type Pinnedness = bool;

/// Reflects [`hir::def::CtorKind`]
#[derive_group(Serializers)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(not(feature = "extract_names_mode"), derive(JsonSchema, AdtInto))]
#[cfg_attr(not(feature = "extract_names_mode"), args(<S>, from: hir::def::CtorKind, state: S as _s))]
pub enum CtorKind {
    Fn,
    Const,
}

/// Reflects [`hir::def::CtorOf`]
#[derive_group(Serializers)]
#[derive(Debug, Copy, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(not(feature = "extract_names_mode"), derive(JsonSchema, AdtInto))]
#[cfg_attr(not(feature = "extract_names_mode"), args(<S>, from: hir::def::CtorOf, state: S as _s))]
pub enum CtorOf {
    Struct,
    Variant,
}

/// The id of a promoted MIR constant.
///
/// Reflects [`rustc_middle::mir::Promoted`].
#[derive_group(Serializers)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(not(feature = "extract_names_mode"), derive(JsonSchema, AdtInto))]
#[cfg_attr(not(feature = "extract_names_mode"), args(<S>, from: rustc_middle::mir::Promoted, state: S as _s))]
pub struct PromotedId {
    #[cfg_attr(not(feature = "extract_names_mode"), value(self.as_u32()))]
    pub id: u32,
}

#[cfg(feature = "rustc")]
impl PromotedId {
    pub fn as_rust_promoted_id(&self) -> rustc_middle::mir::Promoted {
        rustc_middle::mir::Promoted::from_u32(self.id)
    }
}

/// Reflects [`rustc_hir::def::DefKind`]
#[derive_group(Serializers)]
#[cfg_attr(not(feature = "extract_names_mode"), derive(JsonSchema, AdtInto))]
#[cfg_attr(not(feature = "extract_names_mode"), args(<S>, from: rustc_hir::def::DefKind, state: S as tcx))]
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum DefKind {
    Mod,
    Struct,
    Union,
    Enum,
    Variant,
    Trait,
    TyAlias,
    ForeignTy,
    TraitAlias,
    AssocTy,
    TyParam,
    Fn,
    Const,
    ConstParam,
    Static {
        safety: Safety,
        mutability: Mutability,
        nested: bool,
    },
    Ctor(CtorOf, CtorKind),
    AssocFn,
    AssocConst,
    Macro(MacroKinds),
    ExternCrate,
    Use,
    ForeignMod,
    AnonConst,
    InlineConst,
    #[cfg_attr(not(feature = "extract_names_mode"), disable_mapping)]
    /// Added by hax: promoted constants don't have def_ids in rustc but they do in hax.
    PromotedConst,
    OpaqueTy,
    Field,
    LifetimeParam,
    GlobalAsm,
    Impl {
        of_trait: bool,
    },
    Closure,
    SyntheticCoroutineBody,
}

#[derive_group(Serializers)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(not(feature = "extract_names_mode"), derive(JsonSchema))]
pub struct MacroKinds {
    bang: bool,
    attr: bool,
    derive: bool,
}

#[cfg(feature = "rustc")]
impl<S> SInto<S, MacroKinds> for rustc_hir::def::MacroKinds {
    fn sinto(&self, _s: &S) -> MacroKinds {
        MacroKinds {
            bang: self.contains(Self::BANG),
            attr: self.contains(Self::ATTR),
            derive: self.contains(Self::DERIVE),
        }
    }
}

/// Reflects [`rustc_hir::def_id::DefId`], augmented to also give ids to promoted constants (which
/// have their own ad-hoc numbering scheme in rustc for now).
#[derive_group(Serializers)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(not(feature = "extract_names_mode"), derive(JsonSchema))]
pub struct DefId {
    pub(crate) contents: crate::id_table::Node<DefIdContents>,
}

#[derive_group(Serializers)]
#[derive(Debug, Hash, Clone, Eq, Ord)]
#[cfg_attr(not(feature = "extract_names_mode"), derive(JsonSchema))]
pub struct DefIdContents {
    pub krate: String,
    pub path: Vec<DisambiguatedDefPathItem>,
    pub parent: Option<DefId>,
    /// Stores rustc's `CrateNum`, `DefIndex` and `Promoted` raw indices. This can be useful if one
    /// needs to convert a [`DefId`] into a [`rustc_hir::def_id::DefId`]. If the promoted id is
    /// `Some`, then this `DefId` indicates the nth promoted constant associated with the item,
    /// which doesn't have a real `rustc::DefId`.
    ///
    /// **Warning: this `index` field might not be safe to use**. They are valid only for one Rustc
    /// session. Please do not rely on those indices unless you cannot do otherwise.
    pub index: (u32, u32, Option<PromotedId>),
    pub is_local: bool,

    /// The kind of definition this `DefId` points to.
    pub kind: crate::DefKind,
}

impl PartialEq for DefIdContents {
    fn eq(&self, other: &Self) -> bool {
        self.krate == other.krate && self.path == other.path && self.kind == other.kind
    }
}

impl PartialOrd for DefIdContents {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.krate.partial_cmp(&other.krate) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.path.partial_cmp(&other.path) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.kind.partial_cmp(&other.kind)
    }
}

#[cfg(feature = "rustc")]
impl DefIdContents {
    pub fn make_def_id<'tcx, S: BaseState<'tcx>>(self, s: &S) -> DefId {
        let contents =
            s.with_global_cache(|cache| id_table::Node::new(self, &mut cache.id_table_session));
        DefId { contents }
    }
}

/// Returns the [`SyntheticItem`] encoded by a [rustc `DefId`](RDefId), if any.
#[cfg(feature = "rustc")]
pub fn def_id_as_synthetic<'tcx>(
    def_id: RDefId,
    s: &impl BaseState<'tcx>,
) -> Option<SyntheticItem> {
    s.with_global_cache(|c| c.reverse_synthetic_map.get(&def_id).copied())
}

#[cfg(feature = "rustc")]
impl DefId {
    /// The rustc def_id corresponding to this item, if there is one. Promoted constants don't have
    /// a rustc def_id.
    pub fn as_rust_def_id(&self) -> Option<RDefId> {
        let (_, _, promoted) = self.index;
        match promoted {
            None => Some(self.underlying_rust_def_id()),
            Some(_) => None,
        }
    }
    /// The def_id of this item or its parent if this is a promoted constant.
    pub fn underlying_rust_def_id(&self) -> RDefId {
        let (krate, index, _) = self.index;
        RDefId {
            krate: rustc_hir::def_id::CrateNum::from_u32(krate),
            index: rustc_hir::def_id::DefIndex::from_u32(index),
        }
    }

    /// Returns the [`SyntheticItem`] encoded by a [rustc `DefId`](RDefId), if
    /// any.
    ///
    /// Note that this method relies on rustc indexes, which are session
    /// specific. See [`Self`] documentation.
    pub fn as_synthetic<'tcx>(&self, s: &impl BaseState<'tcx>) -> Option<SyntheticItem> {
        def_id_as_synthetic(self.underlying_rust_def_id(), s)
    }

    /// Iterate over this element and its parents.
    pub fn ancestry(&self) -> impl Iterator<Item = &Self> {
        std::iter::successors(Some(self), |def| def.parent.as_ref())
    }

    /// The `PathItem` corresponding to this item.
    pub fn path_item(&self) -> DisambiguatedDefPathItem {
        self.path
            .last()
            .cloned()
            .unwrap_or_else(|| DisambiguatedDefPathItem {
                disambiguator: 0,
                data: DefPathItem::CrateRoot {
                    name: self.krate.clone(),
                },
            })
    }

    /// Construct a hax `DefId` for the nth promoted constant of the current item. That `DefId` has
    /// no corresponding rustc `DefId`.
    pub fn make_promoted_child<'tcx, S: BaseState<'tcx>>(
        &self,
        s: &S,
        promoted_id: PromotedId,
    ) -> Self {
        let mut path = self.path.clone();
        path.push(DisambiguatedDefPathItem {
            data: DefPathItem::PromotedConst,
            // Reuse the promoted id as disambiguator, like for inline consts.
            disambiguator: promoted_id.id,
        });
        let (krate, index, _) = self.index;
        let contents = DefIdContents {
            krate: self.krate.clone(),
            path,
            parent: Some(self.clone()),
            is_local: self.is_local,
            index: (krate, index, Some(promoted_id)),
            kind: DefKind::PromotedConst,
        };
        contents.make_def_id(s)
    }
}

impl DefId {
    pub fn promoted_id(&self) -> Option<PromotedId> {
        let (_, _, promoted) = self.index;
        promoted
    }
}

impl std::ops::Deref for DefId {
    type Target = DefIdContents;
    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

#[cfg(not(feature = "rustc"))]
impl std::fmt::Debug for DefId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefId")
            .field("krate", &self.krate)
            .field("path", &self.path)
            .finish()
    }
}

#[cfg(feature = "rustc")]
impl std::fmt::Debug for DefId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use the more legible rustc debug implementation.
        write!(f, "{:?}", self.underlying_rust_def_id())?;
        if let Some(promoted) = self.promoted_id() {
            write!(f, "::promoted#{}", promoted.id)?;
        }
        Ok(())
    }
}

impl std::hash::Hash for DefId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // A `DefId` is basically an interned path; we only hash the path, discarding the rest of
        // the information.
        self.krate.hash(state);
        self.path.hash(state);
        self.promoted_id().hash(state);
    }
}

/// Gets the kind of the definition. Can't use `def_kind` directly because this crashes on the
/// crate root.
#[cfg(feature = "rustc")]
pub(crate) fn get_def_kind<'tcx>(tcx: ty::TyCtxt<'tcx>, def_id: RDefId) -> hir::def::DefKind {
    if def_id == rustc_span::def_id::CRATE_DEF_ID.to_def_id() {
        // Horrible hack: without this, `def_kind` crashes on the crate root. Presumably some table
        // isn't properly initialized otherwise.
        let _ = tcx.def_span(def_id);
    };
    tcx.def_kind(def_id)
}

/// The crate name under which synthetic items are exported under.
#[cfg(any(feature = "extract_names_mode", feature = "rustc"))]
pub(super) const SYNTHETIC_CRATE_NAME: &str = "<synthetic>";

#[cfg(feature = "rustc")]
fn translate_def_id<'tcx, S: BaseState<'tcx>>(s: &S, def_id: RDefId) -> DefId {
    let tcx = s.base().tcx;
    let path = {
        // Set the def_id so the `CrateRoot` path item can fetch the crate name.
        let state_with_id = s.with_owner_id(def_id);
        tcx.def_path(def_id)
            .data
            .iter()
            .map(|x| x.sinto(&state_with_id))
            .collect()
    };
    let contents = DefIdContents {
        path,
        krate: if def_id_as_synthetic(def_id, s).is_some() {
            SYNTHETIC_CRATE_NAME.to_string()
        } else {
            tcx.crate_name(def_id.krate).to_string()
        },
        parent: tcx.opt_parent(def_id).sinto(s),
        index: (
            rustc_hir::def_id::CrateNum::as_u32(def_id.krate),
            rustc_hir::def_id::DefIndex::as_u32(def_id.index),
            None,
        ),
        is_local: def_id.is_local(),
        kind: get_def_kind(tcx, def_id).sinto(s),
    };
    contents.make_def_id(s)
}

#[cfg(all(not(feature = "extract_names_mode"), feature = "rustc"))]
impl<'s, S: BaseState<'s>> SInto<S, DefId> for RDefId {
    fn sinto(&self, s: &S) -> DefId {
        if let Some(def_id) = s.with_item_cache(*self, |cache| cache.def_id.clone()) {
            return def_id;
        }
        let def_id = translate_def_id(s, *self);
        s.with_item_cache(*self, |cache| cache.def_id = Some(def_id.clone()));
        def_id
    }
}

#[cfg(not(feature = "extract_names_mode"))]
pub type Path = Vec<String>;

#[cfg(all(not(feature = "extract_names_mode"), feature = "rustc"))]
impl std::convert::From<DefId> for Path {
    fn from(v: DefId) -> Vec<String> {
        std::iter::once(&v.krate)
            .chain(v.path.iter().filter_map(|item| match &item.data {
                DefPathItem::TypeNs(s)
                | DefPathItem::ValueNs(s)
                | DefPathItem::MacroNs(s)
                | DefPathItem::LifetimeNs(s) => Some(s),
                _ => None,
            }))
            .cloned()
            .collect()
    }
}

#[cfg(not(feature = "extract_names_mode"))]
pub type GlobalIdent = DefId;

#[cfg(all(not(feature = "extract_names_mode"), feature = "rustc"))]
impl<'tcx, S: BaseState<'tcx>> SInto<S, GlobalIdent> for rustc_hir::def_id::LocalDefId {
    fn sinto(&self, st: &S) -> DefId {
        self.to_def_id().sinto(st)
    }
}

/// Reflects [`rustc_hir::definitions::DefPathData`]
#[derive_group(Serializers)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(not(feature = "extract_names_mode"), derive(AdtInto, JsonSchema))]
#[cfg_attr(not(feature = "extract_names_mode"), args(<'ctx, S: UnderOwnerState<'ctx>>, from: rustc_hir::definitions::DefPathData, state: S as s))]
pub enum DefPathItem {
    CrateRoot {
        #[cfg_attr(not(feature = "extract_names_mode"), value(s.base().tcx.crate_name(s.owner_id().krate).sinto(s)))]
        name: Symbol,
    },
    Impl,
    ForeignMod,
    Use,
    GlobalAsm,
    TypeNs(Symbol),
    ValueNs(Symbol),
    MacroNs(Symbol),
    LifetimeNs(Symbol),
    Closure,
    Ctor,
    LateAnonConst,
    AnonConst,
    #[cfg_attr(not(feature = "extract_names_mode"), disable_mapping)]
    PromotedConst,
    DesugaredAnonymousLifetime,
    OpaqueTy,
    OpaqueLifetime(Symbol),
    AnonAssocTy(Symbol),
    SyntheticCoroutineBody,
    NestedStatic,
}

#[derive_group(Serializers)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(not(feature = "extract_names_mode"), derive(AdtInto, JsonSchema))]
#[cfg_attr(not(feature = "extract_names_mode"), args(<'a, S: UnderOwnerState<'a>>, from: rustc_hir::definitions::DisambiguatedDefPathData, state: S as s))]
/// Reflects [`rustc_hir::definitions::DisambiguatedDefPathData`]
pub struct DisambiguatedDefPathItem {
    pub data: DefPathItem,
    pub disambiguator: u32,
}
