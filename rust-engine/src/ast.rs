//! The core abstract syntax tree (AST) representation for hax.
//!
//! This module defines the primary data structures used to represent
//! typed syntax.
//!
//! The design of this AST is designed under the following constraints:
//!  1. Valid (cargo check) pretty-printed Rust can be produced out of it.
//!  2. The Rust THIR AST from the frontend can be imported into this AST.
//!  3. The AST defined in the OCaml engine can be imported into this AST.
//!  4. This AST can be exported to the OCaml engine.
//!  5. This AST should be suitable for AST transformations.

pub mod diagnostics;
pub mod fragment;
pub mod identifiers;
pub mod literals;
pub mod resugared;
pub mod span;
pub mod utils;
pub mod visitors;

use crate::{ast::diagnostics::Context, symbol::Symbol};
use diagnostics::Diagnostic;
use fragment::Fragment;
use hax_rust_engine_macros::*;
pub use identifiers::*;
use literals::*;
use resugared::*;
use span::Span;

/// Represents a generic value used in type applications (e.g., `T` in `Vec<T>`).
#[derive_group_for_ast]
pub enum GenericValue {
    /// A type-level generic value.
    ///
    /// # Example:
    /// `i32` in `Vec<i32>`
    Ty(Ty),
    /// A const-level generic value.
    ///
    /// # Example:
    /// `12` in `Foo<12>`
    Expr(Expr),
    /// A lifetime.
    ///
    /// # Example:
    /// `'a` in `foo<'a>`
    Lifetime,
}

/// Built-in primitive types.
#[derive_group_for_ast]
pub enum PrimitiveTy {
    /// The `bool` type.
    Bool,
    /// An integer type (e.g., `i32`, `u8`).
    Int(IntKind),
    /// A float type (e.g. `f32`)
    Float(FloatKind),
    /// The `char` type
    Char,
    /// The `str` type
    Str,
}

/// Represent a Rust lifetime region.
#[derive_group_for_ast]
pub struct Region;

/// A indirection for the representation of types.
#[derive_group_for_ast]
pub struct Ty(pub(crate) Box<TyKind>);

impl Ty {
    /// The type `bool`
    pub fn bool() -> Self {
        Self(Box::new(TyKind::Primitive(PrimitiveTy::Bool)))
    }
    /// The type `int`
    pub fn int(size: IntSize, signedness: Signedness) -> Self {
        Self(Box::new(TyKind::Primitive(PrimitiveTy::Int(IntKind {
            size,
            signedness,
        }))))
    }
    /// The `int` check
    pub fn is_int(&self) -> bool {
        let Self(b) = self;
        matches!(
            &**b,
            TyKind::Primitive(PrimitiveTy::Int(IntKind {
                size: _,
                signedness: _,
            }))
        )
    }
    /// The (hax) type `Prop`
    pub fn prop() -> Self {
        Self(Box::new(TyKind::App {
            head: crate::names::hax_lib::prop::Prop,
            args: vec![],
        }))
    }
}

/// Describes any Rust type (e.g., `i32`, `Vec<T>`, `fn(i32) -> bool`).
#[derive_group_for_ast]
pub enum TyKind {
    /// A primitive type.
    ///
    /// # Example:
    /// `i32`, `bool`
    Primitive(PrimitiveTy),

    /// A type application (generic type).
    ///
    /// # Example:
    /// `Vec<i32>`
    App {
        /// The type being applied (`Vec` in the example).
        head: GlobalId,
        /// The arguments (`[i32]` in the example).
        args: Vec<GenericValue>,
    },

    /// A function or closure type.
    ///
    /// # Example:
    /// `fn(i32) -> bool` or `Fn(i32) -> bool`
    Arrow {
        /// `i32` in the example
        inputs: Vec<Ty>,
        /// `bool` in the example
        output: Ty,
    },

    // TODO: Should we keep this type?
    /// A reference type.
    ///
    /// # Example:
    /// `&i32`, `&mut i32`
    Ref {
        /// The type inside the reference
        inner: Ty,
        /// Is the reference mutable?
        mutable: bool,
        /// The region of this reference
        region: Region,
    },

    /// A parameter type
    Param(LocalId),

    // TODO: Should we keep this type?
    /// A slice type.
    ///
    /// # Example:
    /// `&[i32]`
    Slice(Ty),

    /// An array type.
    ///
    /// # Example:
    /// `&[i32; 10]`
    Array {
        /// The type of the items of the array
        ty: Ty,
        /// The length of the array
        length: Box<Expr>,
    },

    /// A raw pointer type
    RawPointer,

    /// An associated type
    ///
    /// # Example:
    /// ```rust,ignore
    ///     fn f<T: Tr>() -> T::A {...}
    /// ```
    AssociatedType {
        /// Impl expr for `Tr<T>` in the example
        impl_: ImplExpr,
        /// `Tr::A` in the example
        item: GlobalId,
    },

    /// An opaque type
    ///
    /// # Example:
    /// ```rust,ignore
    /// type Foo = impl Bar;
    /// ```
    Opaque(GlobalId),

    /// A `dyn` type
    ///
    /// # Example:
    /// ```rust,ignore
    /// dyn Tr
    /// ```
    Dyn(Vec<DynTraitGoal>),

    /// A resugared type.
    /// This variant is introduced before printing only.
    /// Phases must not produce this variant.
    Resugared(ResugaredTyKind),

    /// Fallback constructor to carry errors.
    Error(ErrorNode),
}

#[derive_group_for_ast]
/// Represent a node of the AST where an error occurred.
pub struct ErrorNode {
    /// The node from the AST at the time something failed
    pub fragment: Box<Fragment>,
    /// The error(s) encountered.
    pub diagnostics: Vec<Diagnostic>,
}

impl ErrorNode {
    /// Creates an assertion failure out of an AST fragment and a message.
    pub fn assertion_failure(
        fragment: impl Into<Fragment> + HasMetadata,
        context: Context,
        message: impl Into<String>,
    ) -> Self {
        let span = fragment.span();
        let fragment = fragment.into();
        ErrorNode {
            diagnostics: vec![Diagnostic::new(
                fragment.clone(),
                diagnostics::DiagnosticInfo {
                    context,
                    span,
                    kind: hax_types::diagnostics::Kind::AssertionFailure {
                        details: message.into(),
                    },
                },
            )],
            fragment: Box::new(fragment),
        }
    }
}

/// A `dyn` trait. The generic arguments are known but the actual type
/// implementing the trait is known dynamically.
///
/// # Example:
/// ```rust,ignore
/// dyn Tr<A, B>
/// ```
#[derive_group_for_ast]
pub struct DynTraitGoal {
    /// `Tr` in the example above
    pub trait_: GlobalId,
    /// `A, B` in the example above
    pub non_self_args: Vec<GenericValue>,
}

/// Extra information attached to syntax nodes.
#[derive_group_for_ast]
pub struct Metadata {
    /// The location in the source code.
    pub span: Span,
    /// Rust attributes.
    pub attributes: Attributes,
    // TODO: add phase/desugar information
}

/// A typed expression with metadata.
#[derive_group_for_ast]
pub struct Expr {
    /// The kind of expression.
    pub kind: Box<ExprKind>,
    /// The type of this expression.
    pub ty: Ty,
    /// Source span and attributes.
    pub meta: Metadata,
}

/// A typed pattern with metadata.
#[derive_group_for_ast]
pub struct Pat {
    /// The kind of pattern.
    pub kind: Box<PatKind>,
    /// The type of this pattern.
    pub ty: Ty,
    /// Source span and attributes.
    pub meta: Metadata,
}

/// A pattern matching arm with metadata.
#[derive_group_for_ast]
pub struct Arm {
    /// The pattern of the arm.
    pub pat: Pat,
    /// The body of the arm.
    pub body: Expr,
    /// The optional guard of the arm.
    pub guard: Option<Guard>,
    /// Source span and attributes.
    pub meta: Metadata,
}

/// A pattern matching arm guard with metadata.
#[derive_group_for_ast]
pub struct Guard {
    /// The kind of guard.
    pub kind: GuardKind,
    /// Source span and attributes.
    pub meta: Metadata,
}

/// Represents different levels of borrowing.
#[derive_group_for_ast]
pub enum BorrowKind {
    /// Shared reference
    ///
    /// # Example:
    /// `&x`
    Shared,
    /// Unique reference: this is internal to rustc
    Unique,
    /// Mutable reference
    ///
    /// # Example:
    /// `&mut x`
    Mut,
}

/// Binding modes used in patterns.
#[derive_group_for_ast]
pub enum BindingMode {
    /// Binding by value
    ///
    /// # Example:
    /// `x`
    ByValue,
    /// Binding by reference
    ///
    /// # Example:
    /// `ref x`, `ref mut x`
    ByRef(BorrowKind),
}

/// Represents the various kinds of patterns.
#[derive_group_for_ast]
pub enum PatKind {
    /// Wildcard pattern
    ///
    /// # Example:
    /// `_`
    Wild,

    /// An ascription pattern
    ///
    /// # Example:
    /// `p : ty`
    Ascription {
        /// The inner pattern (`p` in the example)
        pat: Pat,
        /// The (spanned) type ascription (`ty` in the example)
        ty: SpannedTy,
    },

    /// An or pattern
    ///
    /// # Example:
    /// `p | q`
    /// Always contains at least 2 sub-patterns
    Or {
        /// A vector of sub-patterns
        sub_pats: Vec<Pat>,
    },

    /// An array pattern
    ///
    /// # Example:
    /// `[p, q]`
    Array {
        /// A vector of patterns
        args: Vec<Pat>,
    },

    /// A dereference pattern
    ///
    /// # Example:
    /// `&p`
    Deref {
        /// The inner pattern
        sub_pat: Pat,
    },

    /// A constant pattern
    ///
    /// # Example:
    /// `1`
    Constant {
        /// The literal
        lit: Literal,
    },

    /// A variable binding.
    ///
    /// # Examples:
    /// - `x` → `mutable: false`
    /// - `mut x` → `mutable: true`
    /// - `ref x` → `mode: ByRef(Shared)`
    Binding {
        /// Is the binding mutable? E.g. `x` is not mutable, `mut x` is.
        mutable: bool,
        /// The variable introduced by the binding pattern.
        var: LocalId,
        /// The binding mode, e.g. [`BindingMode::Shared`] for `ref x`.
        mode: BindingMode,
        /// The sub-pattern, if any.
        /// For example, this is `Some(inner_pat)` for the pattern `variable @ inner_pat`.
        sub_pat: Option<Pat>,
    },

    /// A constructor pattern
    ///
    /// # Example:
    /// ```rust,ignore
    /// Foo(x)
    /// ```
    Construct {
        /// The identifier of the constructor we are matching
        constructor: GlobalId,
        /// Are we constructing a record? E.g. a struct or a variant with named fields.
        is_record: bool,
        /// Is this a struct? (meaning, *not* a variant from an enum)
        is_struct: bool,
        /// A list of fields.
        fields: Vec<(GlobalId, Pat)>,
    },

    /// A resugared pattern.
    /// This variant is introduced before printing only.
    /// Phases must not produce this variant.
    Resugared(ResugaredPatKind),

    /// Fallback constructor to carry errors.
    Error(ErrorNode),
}

/// Represents the various kinds of pattern guards.
#[derive_group_for_ast]
pub enum GuardKind {
    /// An `if let` guard.
    ///
    /// # Example:
    /// ```rust,ignore
    /// match x {
    ///   Some(value) if let Some(x) = f(value) => x,
    ///   _ => ...,
    /// }
    /// ```
    IfLet {
        /// The left-hand side of the guard. `Some(x)` in the example.
        lhs: Pat,
        /// The right-hand side of the guard. `f(value)` in the example.
        rhs: Expr,
    },
}

// TODO: Replace by places, or just expressions
/// The left-hand side of an assignment.
#[derive_group_for_ast]
#[allow(missing_docs)]
pub enum Lhs {
    LocalVar {
        var: LocalId,
        ty: Ty,
    },
    VecRef {
        e: Box<Lhs>,
        ty: Ty,
    },
    ArbitraryExpr(Box<Expr>),
    FieldAccessor {
        e: Box<Lhs>,
        ty: Ty,
        field: GlobalId,
    },
    ArrayAccessor {
        e: Box<Lhs>,
        ty: Ty,
        index: Expr,
    },
}

/// An `ImplExpr` describes the full data of a trait implementation. Because of
/// generics, this may need to combine several concrete trait implementation
/// items. For example, `((1u8, 2u8), "hello").clone()` combines the generic
/// implementation of `Clone` for `(A, B)` with the concrete implementations for
/// `u8` and `&str`, represented as a tree.
#[derive_group_for_ast]
pub struct ImplExpr {
    /// The impl. expression itself.
    pub kind: Box<ImplExprKind>,
    /// The trait being implemented.
    pub goal: TraitGoal,
}

/// Represents all the kinds of impl expr.
///
/// # Example:
/// In the snippet below, the `clone` method on `x` corresponds to the implementation
/// of `Clone` derived for `Vec<T>` (`ImplApp`) given the `LocalBound` on `T`.
/// ```rust,ignore
/// fn f<T: Clone>(x: Vec<T>) -> Vec<T> {
///   x.clone()
/// }
/// ```
#[derive_group_for_ast]
pub enum ImplExprKind {
    /// The trait implementation being defined.
    ///
    /// # Example:
    /// The impl expr for `Type: Trait` used in `self.f()` is `Self_`.
    /// ```rust,ignore
    /// impl Trait for Type {
    ///     fn f(&self) {...}
    ///     fn g(&self) {self.f()}
    /// }
    /// ```
    Self_,
    /// A concrete `impl` block.
    ///
    /// # Example
    /// ```rust,ignore
    /// impl Clone for Type { // Consider this `impl` is called `impl0`
    ///     ...
    /// }
    /// fn f(x: Type) {
    ///     x.clone() // Here `clone` comes from `Concrete(impl0)`
    /// }
    /// ```
    Concrete(TraitGoal),
    /// A bound introduced by a generic clause.
    ///
    /// # Example:
    /// ```rust,ignore
    /// fn f<T: Clone>(x: T) -> T {
    ///   x.clone() // Here the method comes from the bound `T: Clone`
    /// }
    /// ```
    LocalBound {
        /// Local identifier to a bound.
        id: Symbol,
    },
    /// A parent implementation.
    ///
    /// # Example:
    /// ```rust,ignore
    /// trait SubTrait: Clone {}
    /// fn f<T: SubTrait>(x: T) -> T {
    ///   x.clone() // Here the method comes from the parent of the bound `T: SubTrait`
    /// }
    /// ```
    Parent {
        /// Parent implementation
        impl_: ImplExpr,
        /// Which implementation to pick in the parent
        ident: ImplIdent,
    },
    /// A projected associated implementation.
    ///
    /// # Example:
    /// In this snippet, `T::Item` is an `AssociatedType` where the subsequent `ImplExpr`
    /// is a type projection of `ITerator`.
    /// ```rust,ignore
    /// fn f<T: Iterator>(x: T) -> Option<T::Item> {
    ///     x.next()
    /// }
    /// ```
    Projection {
        /// The base implementation from which we project
        impl_: ImplExpr,
        /// The item in the trait implemented by `impl_`
        item: GlobalId,
        /// Which implementation to pick on the item
        ident: ImplIdent,
    },
    /// An instantiation of a generic implementation.
    ///
    /// # Example:
    /// ```rust,ignore
    /// fn f<T: Clone>(x: Vec<T>) -> Vec<T> {
    ///   x.clone() // The `Clone` implementation for `Vec` is instantiated with the local bound `T: Clone`
    /// }
    /// ```
    ImplApp {
        /// The head of the application
        impl_: ImplExpr,
        /// The arguments of the application
        args: Vec<ImplExpr>,
    },
    /// The implementation provided by a dyn.
    Dyn,
    /// A trait implemented natively by rust.
    Builtin(TraitGoal),
    /// Fallback constructor to carry errors.
    Error(ErrorNode),
}

/// Represents an impl item (associated type or function)
///
/// # Example:
/// ```rust,ignore
/// impl ... {
///   fn assoc_fn<T>(...) {...}
/// }
/// ```
#[derive_group_for_ast]
pub struct ImplItem {
    /// Metadata (span and attributes) for the impl item.
    pub meta: Metadata,
    /// Generics for this associated item. `T` in the example.
    pub generics: Generics,
    /// The associated item itself.
    pub kind: ImplItemKind,
    /// The unique identifier for this associated item.
    pub ident: GlobalId,
}

/// Represents the kinds of impl items
#[derive_group_for_ast]
pub enum ImplItemKind {
    /// An instantiation of associated type
    ///
    /// # Example:
    /// The associated type `Error` in the following example.
    /// ```rust,ignore
    /// impl TryInto for ... {
    ///   type Error = u8;
    /// }
    /// ```
    Type {
        /// The type expression, `u8` in the example.
        ty: Ty,
        /// The parent bounds. In the example, there are none (in the definition
        /// of `TryInto`, there is no `Error: Something` in the associated type
        /// definition).
        parent_bounds: Vec<(ImplExpr, ImplIdent)>,
    },
    /// A definition for a trait function
    ///
    /// # Example:
    /// The associated function `into` in the following example.
    /// ```rust,ignore
    /// impl Into for T {
    ///   fn into(&self) -> T {...}
    /// }
    /// ```
    Fn {
        /// The body of the associated function (`...` in the example)
        body: Expr,
        /// The list of the argument for the associated function (`&self` in the example).
        params: Vec<Param>,
    },

    /// A resugared impl item.
    /// This variant is introduced before printing only.
    /// Phases must not produce this variant.
    Resugared(ResugaredImplItemKind),

    /// Fallback constructor to carry errors.
    Error(ErrorNode),
}

/// Represents a trait item (associated type, fn, or default)
#[derive_group_for_ast]
pub struct TraitItem {
    /// Source span and attributes.
    pub meta: Metadata,
    /// The kind of trait item we are dealing with (an associated type or function).
    pub kind: TraitItemKind,
    /// The generics this associated item carries.
    ///
    /// # Example:
    /// The generics `<B>` on `f`, **not** `<A>`.
    /// ```rust,ignore
    /// trait<A> ... {
    ///    fn f<B>(){}
    /// }
    /// ```
    pub generics: Generics,
    /// The identifier of the associated item.
    pub ident: GlobalId,
}

/// Represents the kinds of trait items
#[derive_group_for_ast]
pub enum TraitItemKind {
    /// An associated type
    Type(Vec<ImplIdent>),
    /// An associated function
    Fn(Ty),
    /// An associated function with a default body.
    /// A arrow type (like what is given in `TraitItemKind::Ty`) can be
    /// reconstructed using the types of the parameters and of the body.
    ///
    /// # Example:
    /// ```rust,ignore
    /// impl ... {
    ///   fn f(x: u8) -> u8 { x + 2 }
    /// }
    /// ```
    Default {
        /// The parameters of the associated function (`[x: u8]` in the example).
        params: Vec<Param>,
        /// The default body of the associated function (`x + 2` in the example).
        body: Expr,
    },

    /// A resugared trait item.
    /// This variant is introduced before printing only.
    /// Phases must not produce this variant.
    Resugared(ResugaredTraitItemKind),

    /// Fallback constructor to carry errors.
    Error(ErrorNode),
}

/// A QuoteContent is a component of a quote: it can be a verbatim string, a Rust expression to embed in the quote, a pattern etc.
///
/// # Example:
/// ```rust,ignore
/// fstar!("f ${x + 3} + 10")
/// ```
/// results in `[Verbatim("f"), Expr([[x + 3]]), Verbatim(" + 10")]`
#[derive_group_for_ast]
pub enum QuoteContent {
    /// A verbatim chunk of backend code.
    Verbatim(String),
    /// A Rust expression to inject in the quote.
    Expr(Expr),
    /// A Rust pattern to inject in the quote.
    Pattern(Pat),
    /// A Rust type to inject in the quote.
    Ty(Ty),
}

/// Represents an inlined piece of backend code
#[derive_group_for_ast]
pub struct Quote(pub Vec<QuoteContent>);

/// The origin of a quote item.
#[derive_group_for_ast]
pub struct ItemQuoteOrigin {
    /// From which kind of item this quote was placed on?
    pub item_kind: ItemQuoteOriginKind,
    /// From what item this quote was placed on?
    pub item_ident: GlobalId,
    /// What was the position of the quote?
    pub position: ItemQuoteOriginPosition,
}

/// The kind of a quote item's origin
#[derive_group_for_ast]
pub enum ItemQuoteOriginKind {
    /// A function
    Fn,
    /// A type alias
    TyAlias,
    /// A type definition (`enum`, `union`, `struct`)
    Type,
    /// A macro invocation
    /// TODO: drop
    MacroInvocation,
    /// A trait definition
    Trait,
    /// An `impl` block
    Impl,
    /// An alias
    Alias,
    /// A `use`
    Use,
    /// A quote
    Quote,
    /// An error
    HaxError,
    /// Something unknown
    NotImplementedYet,
}

/// The position of a quote item relative to its origin
#[derive_group_for_ast]
pub enum ItemQuoteOriginPosition {
    /// The quote was placed before an item
    Before,
    /// The quote was placed after an item
    After,
    /// The quote replaces an item
    Replace,
}

/// The kind of a loop (resugared by respective `Reconstruct...Loops` phases).
/// Useful for `FunctionalizeLoops`.
#[derive_group_for_ast]
pub enum LoopKind {
    /// An unconditional loop.
    ///
    /// # Example:
    /// `loop { ... }`
    UnconditionalLoop,
    /// A while loop.
    ///
    /// # Example:
    /// ```rust,ignore
    /// while(condition) { ... }
    /// ```
    WhileLoop {
        /// The boolean condition
        condition: Expr,
    },
    /// A for loop.
    ///
    /// # Example:
    /// ```rust,ignore
    /// for i in iterator { ... }
    /// ```
    ForLoop {
        /// The pattern of the for loop (`i` in the example).
        pat: Pat,
        /// The iterator we're looping on (`iterator` in the example).
        iterator: Expr,
    },
    /// A specialized for loop on a range.
    ///
    /// # Example:
    /// ```rust,ignore
    /// for i in start..end {
    ///   ...
    /// }
    /// ```
    ForIndexLoop {
        /// Where the range begins (`start` in the example).
        start: Expr,
        /// Where the range ends (`end` in the example).
        end: Expr,
        /// The binding used for the iteration.
        var: LocalId,
        /// The type of the binding `var`.
        var_ty: Ty,
    },
}

/// This is a marker to describe what control flow is present in a loop.
/// It is added by phase `DropReturnBreakContinue` and the information is used in
/// `FunctionalizeLoops`. We need it to replace the control flow nodes of the AST
/// by an encoding in the `ControlFlow` enum.
#[derive_group_for_ast]
pub enum ControlFlowKind {
    /// Contains no `return`, maybe some `break`s
    BreakOnly,
    /// Contains both at least one `return` and maybe some `break`s
    BreakOrReturn,
}

/// Represent explicit mutation context for a loop.
/// This is useful to make loops pure.
#[derive_group_for_ast]
pub struct LoopState {
    /// The initial state of the loop.
    pub init: Expr,
    /// The pattern that destructures the state of the loop.
    pub body_pat: Pat,
}

// TODO: Kill some nodes (e.g. `Array`)?
/// Describes the shape of an expression.
#[derive_group_for_ast]
pub enum ExprKind {
    /// If expression.
    ///
    /// # Example:
    /// `if x > 0 { 1 } else { 2 }`
    If {
        /// The boolean condition (`x > 0` in the example).
        condition: Expr,
        /// The then branch (`1` in the example).
        then: Expr,
        /// An optional else branch (`Some(2)`in the example).
        else_: Option<Expr>,
    },

    /// Function application.
    ///
    /// # Example:
    /// `f(x, y)`
    App {
        /// The head of the function application (or, which function do we apply?).
        head: Expr,
        /// The arguments applied to the function.
        args: Vec<Expr>,
        /// The generic arguments applied to the function.
        generic_args: Vec<GenericValue>,
        /// If the function requires generic bounds to be called, `bounds_impls`
        /// is a vector of impl. expressions for those bounds.
        bounds_impls: Vec<ImplExpr>,
        /// If we apply an associated function, contains the impl. expr used.
        trait_: Option<(ImplExpr, Vec<GenericValue>)>,
    },

    /// A literal value.
    ///
    /// # Example:
    /// `42`, `"hello"`
    Literal(Literal),

    /// An array literal.
    ///
    /// # Example:
    /// `[1, 2, 3]`
    Array(Vec<Expr>),

    /// A constructor application
    ///
    /// # Example:
    /// ```rust,ignore
    /// MyEnum::MyVariant { x : 1, ...base }
    /// ``````
    Construct {
        /// The identifier of the constructor we are building (`MyEnum::MyVariant` in the example).
        constructor: GlobalId,
        /// Are we constructing a record? E.g. a struct or a variant with named fields. (`true` in the example)
        is_record: bool,
        /// Is this a struct? Neaning, *not* a variant from an enum. (`false` in the example)
        is_struct: bool,
        /// A list of fields (`[(x, 1)]` in the example).
        fields: Vec<(GlobalId, Expr)>,
        /// The base expression, if any. (`Some(base)` in the example)
        base: Option<Expr>,
    },

    /// A `match`` expression.
    ///
    /// # Example:
    /// ```rust,ignore
    /// match x {
    ///     pat1 => expr1,
    ///     pat2 => expr2,
    /// }
    /// ```
    Match {
        /// The expression on which we are matching. (`x` in the example)
        scrutinee: Expr,
        /// The arms of the match. (`pat1 => expr1` and `pat2 => expr2` in the example)
        arms: Vec<Arm>,
    },

    /// A reference expression.
    ///
    /// # Examples:
    /// - `&x` → `mutable: false`
    /// - `&mut x` → `mutable: true`
    Borrow {
        /// Is the borrow mutable?
        mutable: bool,
        /// The expression we are borrowing
        inner: Expr,
    },

    /// Raw borrow
    ///
    /// # Example:
    /// `*const u8`
    AddressOf {
        /// Is the raw pointer mutable?
        mutable: bool,
        /// The expression on which we take a pointer
        inner: Expr,
    },

    /// A `let` expression used in expressions.
    ///
    /// # Example:
    /// `let x = 1; x + 1`
    Let {
        /// The left-hand side of the `let` expression. (`x` in the example)
        lhs: Pat,
        /// The right-hand side of the `let` expression. (`1` in the example)
        rhs: Expr,
        /// The body of the `let`. (`x + 1` in the example)
        body: Expr,
    },

    /// A global identifier.
    ///
    /// # Example:
    /// `std::mem::drop`
    GlobalId(GlobalId),

    /// A local variable.
    ///
    /// # Example:
    /// `x`
    LocalId(LocalId),

    /// Type ascription
    Ascription {
        /// The expression being ascribed.
        e: Expr,
        /// The type
        ty: Ty,
    },

    /// Variable mutation
    ///
    /// # Example:
    /// `x = 1`
    Assign {
        /// the left-hand side (place) of the assign
        lhs: Lhs,
        /// The value we are assigning
        value: Expr,
    },

    /// Loop
    ///
    /// # Example:
    /// `'label: loop { body }`
    Loop {
        /// The body of the loop.
        body: Expr,
        /// The kind of loop (e.g. `while`, `loop`, `for`...).
        kind: Box<LoopKind>,
        /// An optional loop state, that makes explicit the state mutated by the
        /// loop.
        state: Option<LoopState>,
        /// What kind of control flow is performed by this loop?
        control_flow: Option<ControlFlowKind>,
        /// Optional loop label.
        label: Option<Symbol>,
    },

    /// The `break` exppression, that breaks out of a loop.
    ///
    /// # Example:
    /// `break 'label 3`
    Break {
        /// The value we break with. By default, this is `()`.
        ///
        /// # Example:
        /// ```rust,ignore
        /// loop { break 3; } + 3
        /// ```
        value: Expr,
        /// What loop shall we break? By default, the parent enclosing loop.
        label: Option<Symbol>,
        /// When a loop has a state (see [`ExprKind::Loop::state`]), this field
        /// `state` is `Some(_)`. This carries the updated state for the loop.
        state: Option<Expr>,
    },

    /// Return from a function.
    ///
    /// # Example:
    /// `return 1`
    Return {
        /// The expression we return (`1` in the example).
        value: Expr,
    },

    /// Continue (go to next loop iteration)
    ///
    /// # Example:
    /// `continue 'label`
    Continue {
        /// The loop we continue.
        label: Option<Symbol>,
        /// When a loop has a state (see [`ExprKind::Loop::state`]), this field
        /// `state` is `Some(_)`. This carries the updated state for the loop.
        state: Option<Expr>,
    },

    /// Closure (anonymous function)
    ///
    /// # Example:
    /// `|x| x`
    Closure {
        /// The parameters of the closure
        params: Vec<Pat>,
        /// The body of the closure
        body: Expr,
        /// The captured expressions
        captures: Vec<Expr>,
    },

    /// Block of safe or unsafe expression
    ///
    /// # Example:
    /// `unsafe { ... }`
    Block {
        /// The body of the block.
        body: Expr,
        /// The safety of the block.
        safety_mode: SafetyKind,
    },

    /// A quote is an inlined piece of backend code.
    Quote {
        /// The contents of the quote.
        contents: Quote,
    },

    /// A resugared expression.
    /// This variant is introduced before printing only.
    /// Phases must not produce this variant.
    Resugared(ResugaredExprKind),

    /// Fallback constructor to carry errors.
    Error(ErrorNode),
}

/// Represents the kinds of generic parameters
#[derive_group_for_ast]
pub enum GenericParamKind {
    /// A generic lifetime
    Lifetime,
    /// A generic type
    Type,
    /// A generic constant
    Const {
        /// The type of the generic constant
        ty: Ty,
    },
}

/// Represents an instantiated trait that needs to be implemented.
///
/// # Example:
/// A bound `_: std::ops::Add<u8>`
#[derive_group_for_ast]
pub struct TraitGoal {
    /// `std::ops::Add` in the example.
    pub trait_: GlobalId,
    /// `[u8]` in the example.
    pub args: Vec<GenericValue>,
}

/// Represents a trait bound in a generic constraint
#[derive_group_for_ast]
pub struct ImplIdent {
    /// The trait goal of this impl identifier
    pub goal: TraitGoal,
    /// The name itself
    pub name: Symbol,
}

/// A projection predicate expresses a constraint over an associated type:
/// ```rust,ignore
/// fn f<T: Foo<S = String>>(...)
/// ```
/// In this example `Foo` has an associated type `S`.
#[derive_group_for_ast]
pub struct ProjectionPredicate {
    /// The impl expression we project from
    pub impl_: ImplExpr,
    /// The associated type being projected
    pub assoc_item: GlobalId,
    /// The equality constraint on the associated type
    pub ty: Ty,
}

/// A generic constraint (lifetime, type-class or equality)
#[derive_group_for_ast]
pub enum GenericConstraint {
    /// A lifetime
    Lifetime(String), // TODO: Remove `String`
    /// A type-class constraint (e.g. `T: Foo`)
    TypeClass(ImplIdent),
    /// An equality constraint on an associated type (e.g. `T::Assoc = u8`)
    Equality(ProjectionPredicate),
}

/// A generic parameter (lifetime, type parameter or const parameter)
#[derive_group_for_ast]
pub struct GenericParam {
    /// The local identifier for the generic parameter
    pub ident: LocalId,
    /// Metadata (span and attributes) for the generic parameter.
    pub meta: Metadata,
    /// The kind of generic parameter.
    pub kind: GenericParamKind,
}

/// Generic parameters and constraints (contained between `<>` in function declarations)
#[derive_group_for_ast]
pub struct Generics {
    /// A vector of generic parameters.
    pub params: Vec<GenericParam>,
    /// A vector of generic constraints.
    pub constraints: Vec<GenericConstraint>,
}

/// Safety level of a function.
#[derive_group_for_ast]
pub enum SafetyKind {
    /// Safe function (default).
    Safe,
    /// Unsafe function.
    Unsafe,
}

/// Represents a single attribute.
#[derive_group_for_ast]
pub struct Attribute {
    /// The kind of attribute (a comment, a tool attribute?).
    pub kind: AttributeKind,
    /// The span of the attribute.
    pub span: Span,
}

/// Represents the kind of an attribute.
#[derive_group_for_ast]
pub enum AttributeKind {
    /// A tool attribute `#[path(tokens)]`
    Tool {
        /// The path to the tool
        path: String,
        /// The payload
        tokens: String,
    },
    /// A doc comment
    DocComment {
        /// What kind of comment? (single lines, block)
        kind: DocCommentKind,
        /// The contents of the comment
        body: String,
    },
    /// Hax attribute
    Hax(hax_lib_macros_types::AttrPayload),
}

/// Represents the kind of a doc comment.
#[derive_group_for_ast]
pub enum DocCommentKind {
    /// Single line comment (`//...`)
    Line,
    /// Block comment (`/*...*/`)
    Block,
}

/// A list of attributes.
pub type Attributes = Vec<Attribute>;

/// A type with its associated span.
#[derive_group_for_ast]
pub struct SpannedTy {
    /// The span of the type
    pub span: Span,
    /// The type itself
    pub ty: Ty,
}

/// A function or closure parameter.
///
/// # Example:
/// ```rust,ignore
/// (mut x, y): (T, u8)
/// ```
#[derive_group_for_ast]
pub struct Param {
    /// The pattern part (left-hand side) of a parameter (`(mut x, y)` in the example).
    pub pat: Pat,
    /// The type part (right-rand side) of a parameter (`(T, u8)` in the example).
    pub ty: Ty,
    /// The span of the type part (if available).
    pub ty_span: Option<Span>,
    /// Optionally, some attributes present on the parameter.
    pub attributes: Attributes,
}

/// A variant of an enum or struct.
/// In our representation structs always have one variant with an argument for each field.
#[derive_group_for_ast]
pub struct Variant {
    /// Name of the variant
    pub name: GlobalId,
    /// Fields of this variant (named or anonymous)
    pub arguments: Vec<(GlobalId, Ty, Attributes)>,
    /// True if fields are named
    pub is_record: bool,
    // TODO Missing span
    /// Attributes of the variant
    pub attributes: Attributes,
}

/// A top-level item in the module.
#[derive_group_for_ast]
pub enum ItemKind {
    /// A function or constant item.
    ///
    /// # Example:
    /// ```rust,ignore
    /// fn add<T: Clone>(x: i32, y: i32) -> i32 {
    ///     x + y
    /// }
    /// ```
    /// Constants are represented as functions of arity zero, while functions always have a non-zero arity.
    Fn {
        /// The identifier of the function.
        ///
        /// # Example:
        /// `add`
        name: GlobalId,

        /// The generic arguments and constraints of the function.
        ///
        /// # Example:
        /// the generic type `T` and the constraint `T: Clone`
        generics: Generics,

        /// The body of the function
        ///
        /// # Example:
        /// `x + y`
        body: Expr,

        /// The parameters of the function.
        ///
        /// # Example:
        /// `x: i32, y: i32`
        params: Vec<Param>,

        /// The safety of the function.
        safety: SafetyKind,
    },

    /// A type alias.
    ///
    /// # Example:
    /// ```rust,ignore
    /// type A = u8;
    /// ```
    TyAlias {
        /// Name of the alias
        ///
        /// # Example:
        /// `A`
        name: GlobalId,

        /// Generic arguments and constraints
        generics: Generics,

        /// Original type
        ///
        /// # Example:
        /// `u8`
        ty: Ty,
    },

    /// A type definition (struct or enum)
    ///
    /// # Example:
    /// ```rust,ignore
    /// enum A {B, C}
    /// struct S {f: u8}
    /// ```
    Type {
        /// Name of this type
        ///
        /// # Example:
        /// `A`, `S`
        name: GlobalId,

        /// Generic parameters and constraints
        generics: Generics,

        /// Variants
        ///
        /// # Example:
        /// `{B, C}`
        variants: Vec<Variant>,

        /// Is this a struct (or an enum)
        is_struct: bool,
    },

    /// A trait definition.
    ///
    /// # Example:
    /// ```rust,ignore
    /// trait T<A> {
    ///     type Assoc;
    ///     fn m(x: Self::Assoc, y: Self) -> A;
    /// }
    /// ```
    Trait {
        /// Name of this trait
        ///
        /// # Example:
        /// `T`
        name: GlobalId,

        /// Generic parameters and constraints
        ///
        /// # Example:
        /// `<A>`
        generics: Generics,

        /// Items required to implement the trait
        ///
        /// # Example:
        /// `type Assoc;`, `fn m ...;`
        items: Vec<TraitItem>,

        /// Safe or unsafe
        safety: SafetyKind,
    },

    /// A trait implementation.
    ///
    /// # Example:
    /// ```rust,ignore
    /// impl T<u8> for u16 {
    ///     type Assoc = u32;
    ///     fn m(x: u32, y: u16) -> u8 {
    ///         (x as u8) + (y as u8)
    ///     }
    /// }
    /// ```
    Impl {
        /// Generic arguments and constraints
        generics: Generics,

        /// The type we implement the trait for
        ///
        /// # Example:
        /// `u16`
        self_ty: Ty,

        /// Instantiated trait that is being implemented
        ///
        /// # Example:
        /// `T<u8>`
        of_trait: TraitGoal,

        /// Items in this impl
        ///
        /// # Example:
        /// `fn m ...`, `type Assoc ...`
        items: Vec<ImplItem>,

        /// Implementations of traits required for this impl
        parent_bounds: Vec<(ImplExpr, ImplIdent)>,
    },

    /// Internal node introduced by phases, corresponds to an alias to any item.
    Alias {
        /// New name
        name: GlobalId,
        /// Original name
        item: GlobalId,
    },

    // TODO: Should we keep `Use`?
    /// A `use` statement
    Use {
        /// Path to used item(s)
        path: Vec<String>,

        /// Comes from external crate
        is_external: bool,

        /// Optional `as`
        rename: Option<String>,
    },

    /// A `Quote` node is inserted by phase TransformHaxLibInline to deal with some `hax_lib` features.
    /// For example insertion of verbatim backend code.
    Quote {
        /// Content of the quote
        quote: Quote,

        /// Description of the quote target position
        origin: ItemQuoteOrigin,
    },

    /// A Rust module (`mod`, inline or not).
    /// This exists solely because modules can have attributes relevant to the hax engine.
    RustModule,

    /// Fallback constructor to carry errors.
    Error(ErrorNode),

    /// A resugared item.
    /// This variant is introduced before printing only.
    /// Phases must not produce this variant.
    Resugared(ResugaredItemKind),

    /// Item that is not implemented yet
    NotImplementedYet,
}

/// A top-level item with metadata.
#[derive_group_for_ast]
pub struct Item {
    /// The global identifier of the item.
    pub ident: GlobalId,
    /// The kind of the item.
    pub kind: ItemKind,
    /// Source span and attributes.
    pub meta: Metadata,
}

impl Item {
    /// Checks whether the item was marked opaque using `hax_lib::opaque`
    pub fn is_opaque(&self) -> bool {
        self.meta.attributes.iter().any(|a| {
            matches!(
                a.kind,
                AttributeKind::Hax(hax_lib_macros_types::AttrPayload::Erased)
            )
        })
    }
}

/// A "flat" module: this contains only non-module items.
#[derive_group_for_ast]
pub struct Module {
    /// The global identifier of the module.
    pub ident: GlobalId,
    /// The list of items that belongs to this module.
    pub items: Vec<Item>,
    /// Source span and attributes.
    pub meta: Metadata,
}

impl Generics {
    /// Returns Iterator over all type-class constraints (`GenericConstraint::TypeClass`)
    pub fn type_class_constraints(&self) -> impl Iterator<Item = &ImplIdent> {
        self.constraints.iter().filter_map(|c| match c {
            GenericConstraint::TypeClass(impl_id) => Some(impl_id),
            _ => None,
        })
    }
    /// Returns Iterator over all equality constraints (`GenericConstraint::Equality`)
    pub fn equality_constraints(&self) -> impl Iterator<Item = &ProjectionPredicate> {
        self.constraints.iter().filter_map(|c| match c {
            GenericConstraint::Equality(pp) => Some(pp),
            _ => None,
        })
    }
}

/// Traits for utilities on AST data types
pub mod traits {
    use super::*;
    /// Marks AST data types that carry metadata (span + attributes)
    pub trait HasMetadata {
        /// Get metadata
        fn metadata(&self) -> &Metadata;
        /// Get mutable borrow on metadata
        fn metadata_mut(&mut self) -> &mut Metadata;
    }
    /// Marks AST data types that carry a span
    pub trait HasSpan {
        /// Get span
        fn span(&self) -> Span;
        /// Mutable borrow on the span
        fn span_mut(&mut self) -> &mut Span;
    }
    /// Marks AST data types that carry a Type
    pub trait Typed {
        /// Get type
        fn ty(&self) -> &Ty;
    }
    impl<T: HasMetadata> HasSpan for T {
        fn span(&self) -> Span {
            self.metadata().span
        }
        fn span_mut(&mut self) -> &mut Span {
            &mut self.metadata_mut().span
        }
    }

    /// Marks types of the AST that carry a kind (an enum for the actual content)
    pub trait HasKind {
        /// Type carrying the kind, should be named `<Self>Kind`
        type Kind;
        /// Get kind
        fn kind(&self) -> &Self::Kind;
        /// Get mutable borrow on kind
        fn kind_mut(&mut self) -> &mut Self::Kind;
    }

    macro_rules! derive_has_metadata {
        ($($ty:ty),*) => {
            $(impl HasMetadata for $ty {
                fn metadata(&self) -> &Metadata {
                    &self.meta
                }
                fn metadata_mut(&mut self) -> &mut Metadata {
                    &mut self.meta
                }
            })*
        };
    }
    macro_rules! derive_has_kind {
        ($($ty:ty => $kind:ty),*) => {
            $(impl HasKind for $ty {
                type Kind = $kind;
                fn kind(&self) -> &Self::Kind {
                    &self.kind
                }
                fn kind_mut(&mut self) -> &mut Self::Kind {
                    &mut self.kind
                }
            })*
        };
    }

    derive_has_metadata!(
        Item,
        Expr,
        Pat,
        Guard,
        Arm,
        ImplItem,
        TraitItem,
        GenericParam
    );
    derive_has_kind!(
        Item => ItemKind, Expr => ExprKind, Pat => PatKind, Guard => GuardKind,
        GenericParam => GenericParamKind, ImplItem => ImplItemKind, TraitItem => TraitItemKind, ImplExpr => ImplExprKind
    );

    impl HasSpan for Attribute {
        fn span(&self) -> Span {
            self.span
        }
        fn span_mut(&mut self) -> &mut Span {
            &mut self.span
        }
    }

    impl Typed for Expr {
        fn ty(&self) -> &Ty {
            &self.ty
        }
    }
    impl Typed for Pat {
        fn ty(&self) -> &Ty {
            &self.ty
        }
    }
    impl Typed for SpannedTy {
        fn ty(&self) -> &Ty {
            &self.ty
        }
    }

    impl HasSpan for SpannedTy {
        fn span(&self) -> Span {
            self.span
        }
        fn span_mut(&mut self) -> &mut Span {
            &mut self.span
        }
    }

    /// Manual implementation of HasKind as the Ty struct contains a Box<TyKind>
    /// instead of a TyKind directly.
    impl HasKind for Ty {
        type Kind = TyKind;

        fn kind(&self) -> &Self::Kind {
            &self.0
        }
        fn kind_mut(&mut self) -> &mut Self::Kind {
            &mut self.0
        }
    }

    /// Fragments of the AST on which we can store an `ErrorNode`.
    pub trait FallibleAstNode {
        /// Replace the current node with an error.
        fn set_error(&mut self, error_node: ErrorNode);
        /// Extract an error if any.
        fn get_error(&self) -> Option<&ErrorNode>;
    }
    macro_rules! derive_error_node {
        ($($ty:ident => $kind:ident),*) => {$(
            impl FallibleAstNode for $ty {
                fn set_error(&mut self, mut error_node: ErrorNode) {
                    if let Some(base) = self.get_error().cloned() {
                        error_node.diagnostics.extend_from_slice(&base.diagnostics);
                    }
                    *self.kind_mut() = $kind::Error(error_node)
                }
                fn get_error(&self) -> Option<&ErrorNode> {
                    match &self.kind() {
                        $kind::Error(error_node) => Some(error_node),
                        _ => None,
                    }
                }
            }
        )*};
    }

    derive_error_node!(Item => ItemKind, Pat => PatKind, Expr => ExprKind, Ty => TyKind);
}
pub use traits::*;
