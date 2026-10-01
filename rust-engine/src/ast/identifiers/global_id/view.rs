//! Helpers to view and reason about **path segments** in Rust items.
//!
//! This module encodes a number of rustc invariants about which items are named vs.
//! unnamed and which items have parents. Those invariants are enforced at runtime and
//! will emit diagnostic if the invariants are broken.
//!
//! # What is a path segment?
//!
//! In Rust, every item lives inside a path. Every path begins with a crate root
//! (the crate where the item is defined).
//!
//! For example, imagine a crate called `my_crate` with this code:
//!
//! ```ignore
//! mod a {
//!     mod b {
//!         fn hello() {}
//!     }
//! }
//! ```
//!
//! The function `hello` has the full path `my_crate::a::b::hello`.
//! This path is made up of segments: `my_crate`, `a`, `b`, and `hello`.
//!
//! This module represents those segments as typed values, enriched with extra
//! information such as:
//! - whether the segment is named or unnamed (e.g. anonymous const or impl blocks),
//! - what kind of item it points to (a crate, module, struct, field, associated fn, etc.),
//! - its parent segment in the hierarchy (e.g. a field belongs to a constructor,
//!   which belongs to a type, which belongs to a module, which belongs to a crate).
//!
//! # Hierarchical nature of segments
//!
//! Segments form a hierarchy of ownership, starting from the crate root.
//! For example, in a crate called `my_crate`:
//!
//! ```ignore
//! struct Foo {
//!     bar: u32,
//! }
//! ```
//!
//! The field `bar` is represented as a `Field` segment. It knows its parent is
//! the constructor of `Foo`, which knows its parent is the type definition `Foo`,
//! which in turn belongs to the crate `my_crate`.
//!
//! The hierarchy looks like this:
//!
//! ```text
//! my_crate (crate)
//!  └── Foo (type)
//!       └── Foo (constructor)
//!            └── bar (field)
//! ```
//!
//! Similarly, with associated items in a crate `my_crate`:
//!
//! ```ignore
//! trait T {
//!     fn f();
//! }
//! ```
//!
//! The function `f` is represented as an `AssocItem` segment, whose parent is the
//! container `T` (a `Trait` segment), and ultimately the crate root:
//!
//! ```text
//! my_crate (crate)
//!  └── T (trait)
//!       └── f (assoc fn)
//! ```
//!
//! This hierarchical model makes it possible to:
//! - reliably find the **parent** of any segment (`bar` → constructor → type → crate),
//! - disambiguate names in backends (e.g. when two crates define constructors with the
//!   same name, the crate root keeps them separate),
//! - traverse full paths in a strongly-typed way (using [`View`] or [`PathSegment::parents`]).
//!
//! # Examples
//!
//! For this Rust code in crate `my_crate`:
//!
//! ```ignore
//! mod a {
//!     trait Foo {
//!         fn f() {
//!             enum T {
//!                 C { field: u8 },
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! We can represent various identifiers as hierarchical segments:
//!
//! | Path                               | Segments                                     |
//! |------------------------------------|----------------------------------------------|
//! | `my_crate`                         | `[my_crate]`                                 |
//! | `my_crate::a`                      | `[my_crate], [a]`                            |
//! | `my_crate::a::b::hello`            | `[my_crate], [a], [b], [hello]`              |
//! | `my_crate::a::Foo`                 | `[my_crate], [a], [Foo]`                     |
//! | `my_crate::a::Foo::f`              | `[my_crate], [a], [Foo::f]`                  |
//! | `my_crate::a::Foo::f::T`           | `[my_crate], [a], [Foo::f], [T]`             |
//! | `my_crate::a::Foo::f::T::C`        | `[my_crate], [a], [Foo::f], [T::C]`          |
//! | `my_crate::a::Foo::f::T::C::field` | `[my_crate], [a], [Foo::f], [T::C::field]`   |
//!

use hax_frontend_exporter::{CtorOf, DefKind, DefPathItem, ImplInfos};

use crate::{
    ast::identifiers::global_id::{DefId, ExplicitDefId},
    symbol::Symbol,
};

#[derive(Debug, Clone)]
/// The kind of a type definition: `struct`, `enum`, or `union`.
pub enum TypeDefKind {
    /// A `struct` definition.
    Struct,
    /// An `enum` definition.
    Enum,
    /// A `union` definition.
    Union,
}

#[derive(Debug, Clone)]
/// The kind of a container for associated items (i.e., a `trait` or an `impl` block).
pub enum AssocItemContainerKind {
    /// An `impl` block.
    Impl {
        /// `true` if this is an inherent `impl` (no trait), `false` if it implements a trait.
        inherent: bool,
        /// Optional extra information about the impl (if available from the frontend).
        ///
        /// `None` when such information is not provided/collected.
        impl_infos: Option<ImplInfos>,
    },
    /// A `trait` definition.
    Trait {
        /// `true` if this is a trait alias (a type alias to a trait).
        trait_alias: bool,
    },
}

#[derive(Debug, Clone)]
/// The kind of a constructo (tuple struct/variant/struct-ctor function).
pub enum ConstructorKind {
    /// A constructor associated to a concrete type definition `ty`.
    Constructor {
        /// The type constructed
        ty: PathSegment<TypeDefKind>,
    },
}

#[derive(Debug, Clone)]
/// The kind of an associated item within a trait or impl.
pub enum AssocItemKind {
    /// An associated function.
    Fn,
    /// An associated constant.
    Const,
    /// An associated type.
    Ty,
}

#[derive(Debug, Clone)]
/// The kind of any item that can occur as a path segment.
///
/// This is a sum type that makes [`PathSegment<AnyKind>`] expressive enough to encode
/// precise parents (e.g., a field always has a constructor parent, an associated item
/// always has a trait/impl container, etc.).
pub enum AnyKind {
    /// A type definition (`struct`, `enum`, or `union`).
    TypeDef(TypeDefKind),
    /// A container of associated items (`trait` or `impl`).
    AssocItemContainer(AssocItemContainerKind),
    /// A constructor (for a struct or enum variant).
    Constructor(ConstructorKind),

    /// An associated item.
    AssocItem {
        /// Which associated item kind this is.
        kind: AssocItemKind,
        /// The parent container (trait or impl) of this associated item.
        container: PathSegment<AssocItemContainerKind>,
    },

    /// A standalone function.
    Fn,
    /// A standalone constant.
    Const,
    /// A `use` item.
    Use,
    /// An anonymous constant (e.g., `const _: T = ...;`).
    AnonConst,
    /// An inline constant (e.g., `let x = { const Y: i32 = 0; Y };`).
    InlineConst,
    /// A trait alias.
    TraitAlias,
    /// A foreign module (`extern "C" { ... }`).
    Foreign,
    /// A foreign type (`extern type T;`).
    ForeignTy,
    /// A type alias (`type Foo = Bar;`).
    TyAlias,
    /// An `extern crate` item.
    ExternCrate,
    /// An opaque item (e.g., `type Foo = impl Trait;`).
    Opaque,
    /// A `static` item.
    Static,
    /// A macro definition or export.
    Macro,
    /// A module or crate.
    Mod,
    /// A global assembly block.
    GlobalAsm,

    /// A field of a struct or a struct-like enum variant.
    Field {
        /// `true` if the field is *named* (e.g., `x` in `struct S { x: u8 }`);
        /// `false` if it is *unnamed* (tuple field like `0` in `struct T(u8)`).
        named: bool,
        /// The parent constructor that owns this field.
        ///
        /// Example: The parent of `x` is the constructor of `Foo` in:
        /// `struct Foo { x: u8 }`.
        parent: PathSegment<ConstructorKind>,
    },

    /// A closure expression item.
    Closure,
}

#[derive(Debug, Clone)]
/// Payloads used when a path segment is **unnamed**.
///
/// These correspond to items that do not contribute a user-facing identifier in the path.
pub enum UnnamedPathSegmentPayload {
    /// An `impl` block.
    Impl,
    /// An anonymous constant.
    AnonConst,
    /// An inline constant.
    InlineConst,
    /// A foreign module or crate.
    Foreign,
    /// A global assembly code block.
    GlobalAsm,
    /// A `use` item.
    Use,
    /// An opaque item (e.g., `type Foo = impl Trait;`).
    Opaque,
    /// A closure.
    Closure,
}

/// Each path segment carries a payload:
/// - [`PathSegmentPayload::Named`] with a user-decided name, or
/// - [`PathSegmentPayload::Unnamed`] for items that are anonymous in the path.
#[derive(Debug, Clone)]
pub enum PathSegmentPayload {
    /// A named segment (holds the name as a [`Symbol`]).
    Named(Symbol),
    /// An unnamed segment with a categorized payload.
    Unnamed(UnnamedPathSegmentPayload),
}

mod rustc_invariant_handling {
    //! This modules provides the function `error_dummy_value`, which emits errors.

    use std::any::{Any, type_name};
    use std::fmt::Debug;

    use super::*;
    use crate::{
        ast::{
            diagnostics::{Context, DiagnosticInfo},
            span::Span,
        },
        names,
    };
    use hax_types::diagnostics::Kind;

    #[derive(Clone, Copy)]
    /// Restrict [`ErrorDummyValue`] callers
    pub struct Permit(());

    pub trait ErrorDummyValue {
        fn error_dummy_value(_: Permit) -> Self;
    }

    impl ErrorDummyValue for PathSegmentPayload {
        fn error_dummy_value(_: Permit) -> Self {
            Self::Named(Symbol::new("hax_engine_view_fatal_error"))
        }
    }

    impl ErrorDummyValue for TypeDefKind {
        fn error_dummy_value(_: Permit) -> Self {
            TypeDefKind::Enum
        }
    }
    impl ErrorDummyValue for ConstructorKind {
        fn error_dummy_value(permit: Permit) -> Self {
            ConstructorKind::Constructor {
                ty: PathSegment::<TypeDefKind>::error_dummy_value(permit),
            }
        }
    }

    impl<K: ErrorDummyValue> ErrorDummyValue for PathSegment<K> {
        fn error_dummy_value(permit: Permit) -> Self {
            Self {
                identifier: DefId::error_dummy_value(permit),
                payload: PathSegmentPayload::error_dummy_value(permit),
                disambiguator: 0,
                kind: K::error_dummy_value(permit),
            }
        }
    }

    impl ErrorDummyValue for AnyKind {
        fn error_dummy_value(_: Permit) -> Self {
            Self::Fn
        }
    }

    impl ErrorDummyValue for DefId {
        fn error_dummy_value(_: Permit) -> Self {
            match names::rust_primitives::hax::failure.0.get() {
                crate::ast::identifiers::global_id::GlobalIdInner::Concrete(concrete_id) => {
                    concrete_id.def_id.def_id
                }
                // The error dummy value is generated by hax, with a concrete identifier
                _ => unreachable!("Hax generated name for failure is concrete"),
            }
        }
    }

    impl ErrorDummyValue for AssocItemContainerKind {
        fn error_dummy_value(_: Permit) -> Self {
            AssocItemContainerKind::Trait { trait_alias: false }
        }
    }

    impl ErrorDummyValue for bool {
        fn error_dummy_value(_: Permit) -> Self {
            true
        }
    }

    pub(super) fn error_dummy_value<T: ErrorDummyValue, V: Debug + Any>(
        message: &str,
        value: &V,
    ) -> T {
        let details = format!(
            "A rustc invariant about `DefId` was violated.\nContext: {message}.\nValue (type {}) is:\n{value:#?}",
            type_name::<T>()
        );
        DiagnosticInfo {
            context: Context::NameView,
            span: Span::dummy(),
            kind: Kind::AssertionFailure { details },
        }
        .emit();
        T::error_dummy_value(Permit(()))
    }
}
use rustc_invariant_handling::error_dummy_value;

impl PathSegmentPayload {
    /// Constructs a [`PathSegmentPayload`] from an [`ExplicitDefId`], assuming its last
    /// path segment is named.
    fn from_named(def_id: &ExplicitDefId) -> Self {
        Self::Named(match def_id.def_id.path.last() {
            Some(last) => match &last.data {
                DefPathItem::TypeNs(s)
                | DefPathItem::ValueNs(s)
                | DefPathItem::MacroNs(s)
                | DefPathItem::LifetimeNs(s) => Symbol::new(s),
                _ => return error_dummy_value("PathSegmentPayload::from_named", def_id),
            },
            None => Symbol::new(&def_id.def_id.krate),
        })
    }

    /// Constructs a [`PathSegmentPayload`] from an [`ExplicitDefId`], assuming its last
    /// path segment is unnamed.
    fn from_unnamed(def_id: &ExplicitDefId) -> Result<Self, &'static str> {
        match def_id.def_id.path.last() {
            Some(last) => match &last.data {
                DefPathItem::TypeNs(_)
                | DefPathItem::ValueNs(_)
                | DefPathItem::MacroNs(_)
                | DefPathItem::LifetimeNs(_) => {
                    return Err("PathSegmentPayload::from_unnamed, got name");
                }

                _ => (),
            },
            None => return Err("PathSegmentPayload::from_unnamed, got a root crate"),
        };
        Ok(Self::Unnamed(match &def_id.def_id.kind {
            DefKind::Use => UnnamedPathSegmentPayload::Use,
            DefKind::ForeignMod => UnnamedPathSegmentPayload::Foreign,
            DefKind::AnonConst => UnnamedPathSegmentPayload::AnonConst,
            // Path-wise, a rustc-promoted constant behaves like an inline one.
            DefKind::InlineConst | DefKind::PromotedConst => UnnamedPathSegmentPayload::InlineConst,
            DefKind::OpaqueTy => UnnamedPathSegmentPayload::Opaque,
            DefKind::GlobalAsm => UnnamedPathSegmentPayload::GlobalAsm,
            DefKind::Impl { .. } => UnnamedPathSegmentPayload::Impl,
            DefKind::Closure => UnnamedPathSegmentPayload::Closure,
            _ => return Err("PathSegmentPayload::from_unnamed, bad kind"),
        }))
    }

    /// Constructs a [`PathSegmentPayload`] from an [`ExplicitDefId`], dispatching to
    /// `from_named` or `from_unnamed` according to the item's [`DefKind`].
    ///
    /// This encodes rustc invariants about which kinds are name-bearing in paths.
    fn from_def_id(def_id: &ExplicitDefId) -> Self {
        match &def_id.def_id.kind {
            DefKind::Mod
            | DefKind::Struct
            | DefKind::Union
            | DefKind::Enum
            | DefKind::Variant
            | DefKind::Trait
            | DefKind::TyAlias
            | DefKind::ForeignTy
            | DefKind::TraitAlias
            | DefKind::AssocTy
            | DefKind::Fn
            | DefKind::Const
            | DefKind::Static { .. }
            | DefKind::Ctor { .. }
            | DefKind::AssocFn
            | DefKind::AssocConst
            | DefKind::Macro { .. }
            | DefKind::ExternCrate
            | DefKind::Field => Self::from_named(def_id),

            DefKind::Use
            | DefKind::ForeignMod
            | DefKind::AnonConst
            | DefKind::InlineConst
            | DefKind::PromotedConst
            | DefKind::OpaqueTy
            | DefKind::GlobalAsm
            | DefKind::Impl { .. }
            | DefKind::Closure => Self::from_unnamed(def_id)
                .unwrap_or_else(|message| error_dummy_value(message, def_id)),

            DefKind::TyParam
            | DefKind::ConstParam
            | DefKind::LifetimeParam
            | DefKind::SyntheticCoroutineBody => error_dummy_value(
                "PathSegmentPayload::from_def_id, kinds should never appear",
                def_id,
            ),
        }
    }
}

#[derive(Debug, Clone)]
/// A typed path segment: one "piece" of a Rust path, with extra structure.
///
/// # What does that mean?
///
/// In Rust, every item (function, type, trait, field...) has a path starting at
/// its crate root. For example, in a crate called `my_crate`:
///
/// ```ignore
/// mod a {
///     mod b {
///         fn hello() {}
///     }
///     trait Foo {
///         fn f() {
///             enum T {
///                 C { field: u8 },
///             }
///         }
///     }
/// }
/// ```
///
/// Some paths and their **segments** are:
///
/// | Path                               | Segments                                     |
/// |------------------------------------|----------------------------------------------|
/// | `my_crate`                         | `[my_crate]`                                 |
/// | `my_crate::a`                      | `[my_crate], [a]`                            |
/// | `my_crate::a::b::hello`            | `[my_crate], [a], [b], [hello]`              |
/// | `my_crate::a::Foo`                 | `[my_crate], [a], [Foo]`                     |
/// | `my_crate::a::Foo::f`              | `[my_crate], [a], [Foo::f]`                  |
/// | `my_crate::a::Foo::f::T`           | `[my_crate], [a], [Foo::f], [T]`             |
/// | `my_crate::a::Foo::f::T::C`        | `[my_crate], [a], [Foo::f], [T::C]`          |
/// | `my_crate::a::Foo::f::T::C::field` | `[my_crate], [a], [Foo::f], [T::C::field]`   |
///
/// Each `[X]` here is a **path segment**.
///
/// # Hierarchy
///
/// Path segments form a hierarchy: each one knows its parent. For example, the
/// field `my_field` is inside the constructor of `MyVariant`, which is inside
/// the enum `MyEnum`, which lives inside the function `f`, and so on -- all the
/// way up to the crate root.
///
/// This parenthood is important:
/// - a field segment always has a constructor parent
///   (e.g. `my_field → MyVariant`).
/// - an associated item always has a trait/impl container parent
///   (e.g. `f → Foo`).
/// - everything ultimately has a **crate** as its top parent.
///
/// # Why does this matter?
///
/// This strong typing of segments lets tools:
/// - disambiguate names across contexts (e.g. two types with the same
///   constructor name),
/// - generate unique, human-readable names in other languages/backends,
/// - walk up the chain of parents to reconstruct full paths.
///
/// For example, with the F\* backend, constructors are not namespaced under the
/// name of their type, but live directly at top-level. Thus, they need to be
/// unique. Using the hierarchy, we can print them as `Foo_MyVariant` instead of
/// `Foo.MyVariant`.
pub struct PathSegment<Kind = AnyKind> {
    identifier: DefId,
    payload: PathSegmentPayload,
    disambiguator: u32,
    kind: Kind,
}

impl<K> PathSegment<K> {
    /// Returns the payload of this path segment (named vs. unnamed and why).
    pub fn payload(&self) -> PathSegmentPayload {
        self.payload.clone()
    }

    /// Returns the rustc path disambiguator for this segment.
    pub fn disambiguator(&self) -> u32 {
        self.disambiguator
    }

    /// Returns the kind of this segment as an [`K`].
    pub fn kind(&self) -> &K {
        &self.kind
    }

    /// Maps the segment's `kind` while preserving all other fields.
    fn map<U>(self, f: impl Fn(K, &DefId) -> U) -> PathSegment<U> {
        let Self {
            identifier,
            payload,
            disambiguator,
            kind,
        } = self;
        let kind = f(kind, &identifier);
        PathSegment {
            identifier,
            payload,
            disambiguator,
            kind,
        }
    }
}

impl PathSegment<ConstructorKind> {
    /// Lift a `PathSegment` of kind `ConstructorKind` to a `PathSegment` of kind `AnyKind`.
    pub fn lift(&self) -> PathSegment<AnyKind> {
        self.clone().map(|kind, _| AnyKind::Constructor(kind))
    }
}
impl PathSegment<TypeDefKind> {
    /// Lift a `PathSegment` of kind `TypeDefKind` to a `PathSegment` of kind `AnyKind`.
    pub fn lift(&self) -> PathSegment<AnyKind> {
        self.clone().map(|kind, _| AnyKind::TypeDef(kind))
    }
}
impl PathSegment<AssocItemContainerKind> {
    /// Lift a `PathSegment` of kind `AssocItemContainerKind` to a `PathSegment` of kind `AnyKind`.
    pub fn lift(&self) -> PathSegment<AnyKind> {
        self.clone()
            .map(|kind, _| AnyKind::AssocItemContainer(kind))
    }
}

impl PartialEq<PathSegment> for PathSegment {
    fn eq(&self, other: &PathSegment) -> bool {
        self.identifier == other.identifier && self.disambiguator == other.disambiguator
    }
}

impl PathSegment {
    /// Asserts that this segment is a [`TypeDefKind`] and narrows the type.
    ///
    /// Emits a diagnostic if it doesn
    fn assert_type_def(self) -> PathSegment<TypeDefKind> {
        self.map(|kind, did| match kind {
            AnyKind::TypeDef(inner) => inner,
            _ => error_dummy_value(&format!("expected TypeDefKind, got {kind:#?}"), did),
        })
    }

    /// Asserts that this segment is an [`AssocItemContainerKind`] and narrows the type.
    fn assert_assoc_item_container(self) -> PathSegment<AssocItemContainerKind> {
        self.map(|kind, did| match kind {
            AnyKind::AssocItemContainer(inner) => inner,
            _ => error_dummy_value(
                &format!("expected AssocItemContainerKind, got {kind:#?}"),
                did,
            ),
        })
    }

    /// Asserts that this segment is a [`ConstructorKind`] and narrows the type.
    fn assert_constructor(self) -> PathSegment<ConstructorKind> {
        self.map(|kind, did| match kind {
            AnyKind::Constructor(inner) => inner,
            _ => error_dummy_value(&format!("expected ConstructorKind, got {kind:#?}"), did),
        })
    }

    /// Internal constructor that consumes an iterator of [`ExplicitDefId`]s (from child
    /// to parents) and builds a single [`PathSegment`] at a time, honoring rustc
    /// invariants and wiring proper parents for kinds that require them
    /// (constructors, fields, associated items).
    ///
    /// Returns `None` when the iterator is exhausted.
    fn from_iterator(it: &mut impl Iterator<Item = ExplicitDefId>) -> Option<Self> {
        let def_id = it.next()?;
        let mut from_iterator = |context: &str| match Self::from_iterator(it) {
            Some(value) => value,
            None => error_dummy_value(
                &format!("PathSegment::from_iterator, expected parent for {context}."),
                &def_id,
            ),
        };
        let payload = PathSegmentPayload::from_def_id(&def_id);

        let kind = match &def_id.def_id.kind {
            // Struct constructor path segment special-casing (struct-as-ctor).
            DefKind::Ctor(CtorOf::Struct, _) | DefKind::Struct if def_id.is_constructor => {
                let parent_def_id = ExplicitDefId {
                    is_constructor: false,
                    def_id: def_id.def_id,
                };
                let parent = match Self::from_iterator(&mut std::iter::once(parent_def_id)) {
                    Some(value) => value,
                    None => error_dummy_value(
                        "PathSegment::from_iterator, expected parent for Struct/Ctor.",
                        &def_id,
                    ),
                };
                AnyKind::Constructor(ConstructorKind::Constructor {
                    ty: parent.assert_type_def(),
                })
            }
            // Non-ctor struct item.
            DefKind::Ctor(CtorOf::Struct, _) => AnyKind::TypeDef(TypeDefKind::Struct),
            // Enum variants and non-struct ctors.
            DefKind::Variant | DefKind::Ctor(_, _) => {
                AnyKind::Constructor(ConstructorKind::Constructor {
                    ty: from_iterator("Variant/Ctor").assert_type_def(),
                })
            }
            DefKind::Struct => AnyKind::TypeDef(TypeDefKind::Struct),
            DefKind::Union => AnyKind::TypeDef(TypeDefKind::Union),
            DefKind::Enum => AnyKind::TypeDef(TypeDefKind::Enum),
            DefKind::Trait => {
                AnyKind::AssocItemContainer(AssocItemContainerKind::Trait { trait_alias: false })
            }
            DefKind::Impl { of_trait } => AnyKind::AssocItemContainer(
                AssocItemContainerKind::Impl { inherent: !of_trait, impl_infos: /* intentionally left None; fill where available */ None },
            ),

            // Simple leaf kinds.
            DefKind::Mod => AnyKind::Mod,
            DefKind::Fn => AnyKind::Fn,
            DefKind::Const => AnyKind::Const,
            DefKind::Static { .. } => AnyKind::Static,
            DefKind::Use => AnyKind::Use,
            DefKind::TyAlias => AnyKind::TyAlias,
            DefKind::TraitAlias => AnyKind::TraitAlias,
            DefKind::ForeignTy => AnyKind::ForeignTy,
            DefKind::ForeignMod => AnyKind::Foreign,
            DefKind::Macro { .. } => AnyKind::Macro,
            DefKind::AnonConst => AnyKind::AnonConst,
            DefKind::InlineConst | DefKind::PromotedConst => AnyKind::InlineConst,
            DefKind::OpaqueTy => AnyKind::Opaque,
            DefKind::GlobalAsm => AnyKind::GlobalAsm,
            DefKind::Closure => AnyKind::Closure,
            DefKind::ExternCrate => AnyKind::ExternCrate,

            // Field: requires a constructor parent and conveys whether it's named.
            DefKind::Field => AnyKind::Field {
                parent: from_iterator("Field").assert_constructor(),
                named: match &payload {
                    PathSegmentPayload::Named(symbol) => {
                        // Tuple fields are numbered; parse success => unnamed field.
                        str::parse::<usize>(symbol.as_ref()).is_ok()
                    }
                    PathSegmentPayload::Unnamed(_) => {
                        error_dummy_value("Field should carry a ValueNs payload.", &def_id)
                    }
                },
            },

            // Associated items: require a container parent.
            DefKind::AssocTy => AnyKind::AssocItem {
                container: from_iterator("AssocTy").assert_assoc_item_container(),
                kind: AssocItemKind::Ty,
            },
            DefKind::AssocFn => AnyKind::AssocItem {
                container: from_iterator("AssocFn").assert_assoc_item_container(),
                kind: AssocItemKind::Fn,
            },
            DefKind::AssocConst => AnyKind::AssocItem {
                container: from_iterator("AssocConst").assert_assoc_item_container(),
                kind: AssocItemKind::Const,
            },

            _ => error_dummy_value("PathSegment::from_iterator_opt", &def_id),
        };
        let identifier = def_id.def_id;
        let disambiguator = identifier.path.last().map(|d| d.disambiguator).unwrap_or(0);
        Some(Self {
            identifier,
            payload,
            disambiguator,
            kind,
        })
    }
}

impl PathSegment {
    /// Returns the parent path segment, if any.
    ///
    /// Parents exist only for:
    /// - [`AnyKind::Constructor`] (parent is its [`TypeDefKind`]),
    /// - [`AnyKind::AssocItem`] (parent is its container `trait`/`impl`),
    /// - [`AnyKind::Field`] (parent is its constructor).
    ///
    /// All other kinds return `None`.
    pub fn parent(&self) -> Option<PathSegment> {
        Some(match self.kind.clone() {
            AnyKind::Constructor(ConstructorKind::Constructor { ty }) => {
                ty.map(|kind, _| AnyKind::TypeDef(kind))
            }
            AnyKind::AssocItem { container, .. } => {
                container.map(|kind, _| AnyKind::AssocItemContainer(kind))
            }
            AnyKind::Field { parent, .. } => parent.map(|kind, _| AnyKind::Constructor(kind)),
            _ => return None,
        })
    }

    /// Returns an iterator over `self` and all its ancestors, walking up via
    /// [`Self::parent`] until no parent remains.
    pub fn parents(&self) -> impl Iterator<Item = Self> {
        std::iter::successors(Some(self.clone()), |seg| seg.parent())
    }
}

mod view_encapsulation {
    //! Encapsulation module to scope [`View`]'s invariants
    use crate::ast::{
        identifiers::global_id::{FreshModule, ReservedSuffix},
        span::Span,
    };

    use super::*;
    /// A view for an [`ExplicitDefId`], materialized as a list of typed
    /// [`PathSegment`]s ordered from the crate root/module towards the item.
    pub struct View(Vec<PathSegment>, Option<ReservedSuffix>);

    impl View {
        /// Returns the full list of segments (non-empty).
        pub fn segments(&self) -> &[PathSegment] {
            &self.0
        }

        /// Returns the last (most specific) segment.
        pub fn last(&self) -> &PathSegment {
            self.0
                .last()
                .expect("Broken invariant: a view always contains at least one path path segments.")
        }

        /// Returns the first (outermost) segment.
        pub fn first(&self) -> &PathSegment {
            self.0
                .first()
                .expect("Broken invariant: a view always contains at least one path path segments.")
        }

        /// Splits the view at the boundary between (Rust) modules and the first non-module
        /// segment.
        ///
        /// Returns `(modules, rest)`, where `modules` is the (non empty) prefix of
        /// `mod` segments (e.g., the crate/module path), and `rest` is the remaining
        /// segments starting at the first non-`mod`.
        pub fn split_at_module(&self) -> (&[PathSegment], &[PathSegment]) {
            let position = self
                .segments()
                .iter()
                .enumerate()
                .find(|(_, seg)| !matches!(seg.kind(), AnyKind::Mod))
                .map(|(i, _)| i)
                .unwrap_or(self.segments().len());
            self.segments().split_at(position)
        }

        /// Get the first parent which is a proper module (all its parent are modules as well).
        pub fn module(&self) -> &PathSegment {
            self.0
                .iter()
                .take_while(|seg| !matches!(seg.kind(), AnyKind::Mod))
                .last()
                .expect("Broken invariant, a name has at least a crate")
        }

        /// Get the optional suffix of this view
        pub fn suffix(&self) -> &Option<ReservedSuffix> {
            &self.1
        }

        /// Add a suffix to a view
        pub fn with_suffix(mut self, suffix: Option<ReservedSuffix>) -> Self {
            self.1 = suffix;
            self
        }
    }

    impl From<ExplicitDefId> for View {
        /// Builds a [`View`] from an [`ExplicitDefId`], reconstructing segments by walking
        /// up the parent chain and then reversing to obtain the canonical outer→inner order.
        fn from(value: ExplicitDefId) -> Self {
            let mut it = value.parents();
            let mut inner =
                std::iter::from_fn(|| PathSegment::from_iterator(&mut it)).collect::<Vec<_>>();
            inner.reverse();
            debug_assert!(!inner.is_empty()); // invariant: non-empty
            Self(inner, None)
        }
    }

    impl From<FreshModule> for View {
        fn from(value: FreshModule) -> Self {
            use crate::ast::diagnostics::{Context, DiagnosticInfo};
            (DiagnosticInfo {
                context: Context::NameView,
                span: Span::dummy(),
                kind: hax_types::diagnostics::Kind::Unimplemented {
                    issue_id: Some(1779),
                    details: Some(
                        "Fresh modules are not implemented yet in the Rust engine".into(),
                    ),
                },
            })
            .emit();
            // dummy value
            value
                .hints
                .first()
                .expect("The list of hints should be non-empty")
                .clone()
                .into()
        }
    }
}
pub use view_encapsulation::View;
