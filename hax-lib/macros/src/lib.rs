//! Proc-macros for hax.
//!
//! Proc-macros must reside in the root of the crate: this module defines all of
//! them, in every configuration, so that their documentation is always
//! available. Small ones carry their implementation here, right under their
//! documentation; bigger ones dispatch to `implementation`. Either way the
//! implementation is gated on `--cfg hax`, since it needs
//! `hax-lib-macros-types`, which is a `cfg(hax)`-only dependency.

#![cfg_attr(hax, feature(macro_metavar_expr_concat))]

mod hax_paths;

#[cfg(hax)]
mod impl_fn_decoration;
#[cfg(hax)]
mod implementation;
#[cfg(hax)]
mod quote;
#[cfg(hax)]
mod rewrite_self;
#[cfg(hax)]
mod syn_ext;
#[cfg(hax)]
mod utils;

#[cfg(hax)]
mod prelude {
    pub use crate::hax_paths::*;
    pub use crate::syn_ext::*;
    pub use proc_macro as pm;
    pub use proc_macro2::*;
    pub use quote::*;
    pub use std::collections::HashSet;
    pub use syn::spanned::Spanned;
    pub use syn::{visit_mut::VisitMut, *};

    pub use AttrPayload::Language as AttrHaxLang;
    pub use hax_lib_macros_types::*;
    pub type FnLike = syn::ImplItemFn;
}

#[cfg(not(hax))]
mod dummy;

use proc_macro::TokenStream;

/// Defines attribute proc-macros that forward to `implementation` under
/// `--cfg hax`, and are the identity otherwise.
macro_rules! passthrough_attributes {
    ($($(#[$meta:meta])* $name:ident;)*) => {
        $(
            $(#[$meta])*
            #[proc_macro_attribute]
            pub fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
                #[cfg(hax)]
                { implementation::$name(attr, item) }
                #[cfg(not(hax))]
                { let _ = attr; item }
            }
        )*
    };
}

/// Defines attribute proc-macros whose implementation is inlined below, next to
/// their documentation. The body is compiled only under `--cfg hax`; otherwise
/// the macro is the identity.
macro_rules! attribute_macros {
    ($($(#[$meta:meta])* fn $name:ident($attr:ident, $item:ident) $body:block)*) => {
        $(
            $(#[$meta])*
            #[proc_macro_attribute]
            pub fn $name($attr: TokenStream, $item: TokenStream) -> TokenStream {
                #[cfg(hax)]
                {
                    #[allow(unused_imports)]
                    use crate::{
                        impl_fn_decoration::*, prelude::*, rewrite_self::SelfProjection, utils::*,
                    };
                    $body
                }
                #[cfg(not(hax))]
                { let _ = $attr; $item }
            }
        )*
    };
}

passthrough_attributes! {
    /// When extracting to F*, inform about what is the current
    /// verification status for an item. It can either be `lax` or
    /// `panic_free`.
    fstar_verification_status;

    /// Add a logical postcondition to a function. Note you can use the
    /// `forall` and `exists` operators.
    ///
    /// You can use the (unqualified) macro `fstar!` (`BACKEND!` for any
    /// backend `BACKEND`) to inline F* (or Coq, ProVerif, etc.) code in
    /// the postcondition, e.g. `fstar!("true")`.
    ///
    /// # Example
    ///
    /// ```
    /// use hax_lib_macros::*;
    /// #[ensures(|result| result == x * 2)]
    /// pub fn twice(x: u64) -> u64 {
    ///     x + x
    /// }
    /// ```
    ensures;

    /// Same as [`macro@ensures`], but the closure takes the result by
    /// reference: the binder has type `&T` where `T` is the function's return
    /// type.
    ///
    /// # Example
    ///
    /// ```
    /// use hax_lib_macros::*;
    /// #[ensures_ref(|result| result.len() == 2)]
    /// pub fn pair(x: u64) -> Vec<u64> {
    ///     vec![x, x]
    /// }
    /// ```
    ensures_ref;

    /// Mark an item opaque: the extraction will assume the
    /// type without revealing its definition.
    #[deprecated(note = "Please use 'opaque' instead")]
    opaque_type;

    /// Mark an item opaque: the extraction will assume the
    /// type without revealing its definition.
    opaque;

    /// Marks a newtype `struct RefinedT(T);` as a refinement type. The
    /// struct should have exactly one unnamed private field.
    ///
    /// This macro takes one argument: a `Prop` proposition that refines
    /// values of type `SomeType`.
    ///
    /// For example, the following type defines bounded `u64` integers.
    ///
    /// ```
    /// #[hax_lib::refinement_type(|x| x >= MIN && x <= MAX)]
    /// pub struct BoundedU64<const MIN: u64, const MAX: u64>(u64);
    /// ```
    ///
    /// This macro will generate an implementation of the [`Deref`](core::ops::Deref)
    /// trait and of the `hax_lib::Refinement` trait. Those two traits are
    /// the only interface to this newtype: one is allowed only to
    /// construct or destruct refined type via those smart constructors
    /// and destructors, ensuring the abstraction.
    ///
    /// A refinement of a type `T` with a formula `f` can be seen as a box
    /// that contains a value of type `T` and a proof that this value
    /// satisfies the formula `f`.
    ///
    /// In debug mode, the refinement will be checked at run-time. This
    /// requires the base type `T` to implement `Clone`. Pass a first
    /// parameter `no_debug_runtime_check` to disable this behavior.
    ///
    /// When extracted via hax, this is interpreted in the backend as a
    /// refinement type: the use of such a type yields static proof
    /// obligations.
    refinement_type;

    /// β-inline every call to this ProVerif helper at the use site; the
    /// marked item itself drops out of the extracted model.
    pv_inline;

    /// Model a Rust function as a uniform-bitstring `extern__<fn>` ProVerif
    /// letfun, deriving the declaration arity and body-call arguments from the
    /// annotated function's signature. The symbol is named after the bare
    /// function name, which two modules can share; pass an explicit name
    /// (`#[pv_extern("mymod_encrypt")]`) when they do, or the two functions
    /// become one symbol in the model. Every parameter must be a plain
    /// identifier.
    pv_extern;

    /// Shorthand for `#[hax_lib::proverif::replace_body(<lit>)]`, for trivial
    /// constant stubs such as `#[pv_stub("nat_0")]`.
    pv_stub;

    /// Declare this function as the one-sided inverse of another, emitting
    /// `reduc forall x: bitstring; self__inverse(other(x)) = x.` and making
    /// `self` apply it. With `--result-errors`, a function returning a
    /// `Result` returns `Ok`, and an `Err` off the image of the other function.
    ///
    /// A ProVerif `reduc` rewrites on *constructors*, so the other function has
    /// to be one — annotate it `#[hax_lib::pv_constructor]` (or
    /// `#[hax_lib::opaque]`). Pointing this at a `letfun` produces a `reduc`
    /// ProVerif rejects.
    pv_inverse_of;
}

attribute_macros! {
    /// When extracting to F*, wrap this item in `#push-options "..."` and
    /// `#pop-options`.
    fn fstar_options(attr, item) {
        let item: TokenStream = item.into();
        let lit_str = parse_macro_input!(attr as LitStr);
        let payload = format!(r#"#push-options "{}""#, lit_str.value());
        let payload = LitStr::new(&payload, lit_str.span());
        quote! {
            #[::hax_lib::fstar::before(#payload)]
            #[::hax_lib::fstar::after(r#"#pop-options"#)]
            #item
        }
        .into()
    }

    /// Postprocess an item with a given tactic. This macro takes the tactic in
    /// parameter: this may be a Rust identifier or a raw snippet of F* code as a
    /// string literal.
    fn fstar_postprocess_with(attr, item) {
        let item: TokenStream = item.into();
        let payload: String = if let Ok(s) = syn::parse::<LitStr>(attr.clone()) {
            s.value()
        } else {
            let e = parse_macro_input!(attr as Expr);
            format!(" ${{ {} }} ", e.to_token_stream())
        };
        let payload = format!("[@@FStar.Tactics.postprocess_with ({payload})]");
        let payload: Lit = Lit::Str(syn::LitStr::new(&payload, Span::call_site()));
        quote! {#[::hax_lib::fstar::before(#payload)] #item}.into()
    }

    /// Allows to add SMT patterns to a lemma.
    /// For more informations about SMT patterns, please take a look here:
    /// <https://fstar-lang.org/tutorial/book/under_the_hood/uth_smt.html#designing-a-library-with-smt-patterns>.
    fn fstar_smt_pat(attr, item) {
        let phi: syn::Expr = parse_macro_input!(attr);
        let item: FnLike = parse_macro_input!(item);
        let (requires, attr) = make_fn_decoration(
            phi,
            item.sig.clone(),
            FnDecorationKind::SMTPat,
            None,
            None,
            SelfProjection::Unknown,
        );
        quote! {#requires #attr #item}.into()
    }

    /// Include this item in the Hax translation. This overrides any exclusion resulting of `-i` flag.
    fn include(attr, item) {
        let item: TokenStream = item.into();
        let _ = parse_macro_input!(attr as parse::Nothing);
        let attr = AttrPayload::ItemStatus(ItemStatus::Included { late_skip: false });
        quote! {#attr #item}.into()
    }

    /// Exclude this item from the Hax translation.
    fn exclude(attr, item) {
        let item: TokenStream = item.into();
        let _ = parse_macro_input!(attr as parse::Nothing);
        let attr = AttrPayload::ItemStatus(ItemStatus::Excluded { modeled_by: None });
        let charon = charon_attr(quote! {exclude});
        quote! {#attr #charon #item}.into()
    }

    /// Provide a measure for a function: this measure will be used once
    /// extracted in a backend for checking termination. The expression
    /// that decreases can be of any type. (TODO: this is probably as it
    /// is true only for F*, see #297)
    ///
    /// # Example
    ///
    /// ```
    /// use hax_lib_macros::*;
    /// #[decreases((m, n))]
    /// pub fn ackermann(m: u64, n: u64) -> u64 {
    ///     match (m, n) {
    ///         (0, _) => n + 1,
    ///         (_, 0) => ackermann(m - 1, 1),
    ///         _ => ackermann(m - 1, ackermann(m, n - 1)),
    ///     }
    /// }
    /// ```
    fn decreases(attr, item) {
        let phi: syn::Expr = parse_macro_input!(attr);
        let item: FnLike = parse_macro_input!(item);
        let (requires, attr) = make_fn_decoration(
            phi,
            item.sig.clone(),
            FnDecorationKind::Decreases,
            None,
            None,
            SelfProjection::Unknown,
        );
        quote! {#requires #attr #item}.into()
    }

    /// Add a logical precondition to a function.
    // Note you can use the `forall` and `exists` operators. (TODO: commented out for now, see #297)
    /// In the case of a function that has one or more `&mut` inputs, in
    /// the `ensures` clause, you can refer to such an `&mut` input `x` as
    /// `x` for its "past" value and `future(x)` for its "future" value.
    /// Where those future values sit relative to the result in the generated
    /// postcondition is backend-dependent, since each backend orders the tuple
    /// its functions return differently.
    ///
    /// You can use the (unqualified) macro `fstar!` (`BACKEND!` for any
    /// backend `BACKEND`) to inline F* (or Coq, ProVerif, etc.) code in
    /// the precondition, e.g. `fstar!("true")`.
    ///
    /// # Example
    ///
    /// ```
    /// use hax_lib_macros::*;
    /// #[requires(x.len() == y.len())]
    // #[requires(x.len() == y.len() && forall(|i: usize| i >= x.len() || y[i] > 0))] (TODO: commented out for now, see #297)
    /// pub fn div_pairwise(x: Vec<u64>, y: Vec<u64>) -> Vec<u64> {
    ///     x.iter()
    ///         .copied()
    ///         .zip(y.iter().copied())
    ///         .map(|(x, y)| x / y)
    ///         .collect()
    /// }
    /// ```
    fn requires(attr, item) {
        let phi: syn::Expr = parse_macro_input!(attr);
        let item: FnLike = parse_macro_input!(item);
        let (requires, attr) = make_fn_decoration(
            phi.clone(),
            item.sig.clone(),
            FnDecorationKind::Requires,
            None,
            None,
            SelfProjection::Unknown,
        );
        let mut item_with_debug = item.clone();
        item_with_debug
            .block
            .stmts
            .insert(0, parse_quote! {debug_assert!(#phi);});
        quote! {
            #requires #attr
            // TODO: disable `assert!`s for now (see #297)
            #item
            // #[cfg(    all(not(#HaxCfgOptionName),     debug_assertions )) ] #item_with_debug
            // #[cfg(not(all(not(#HaxCfgOptionName),     debug_assertions )))] #item
        }
        .into()
    }

    /// Mark an item transparent: the extraction will not
    /// make it opaque regardless of the `-i` flag default.
    fn transparent(_attr, item) {
        let item: Item = parse_macro_input!(item);
        let attr = AttrPayload::NeverErased;
        quote! {#attr #item}.into()
    }

    /// A marker indicating a `fn` as a ProVerif process read.
    fn process_read(_attr, item) {
        let item: ItemFn = parse_macro_input!(item);
        let attr = AttrPayload::ProcessRead;
        quote! {#attr #item}.into()
    }

    /// A marker indicating a `fn` as a ProVerif process write.
    fn process_write(_attr, item) {
        let item: ItemFn = parse_macro_input!(item);
        let attr = AttrPayload::ProcessWrite;
        quote! {#attr #item}.into()
    }

    /// A marker indicating a `fn` as a ProVerif process initialization.
    fn process_init(_attr, item) {
        let item: ItemFn = parse_macro_input!(item);
        let attr = AttrPayload::ProcessInit;
        quote! {#attr #item}.into()
    }

    /// A marker indicating an `enum` as describing the protocol messages.
    fn protocol_messages(_attr, item) {
        let item: ItemEnum = parse_macro_input!(item);
        let attr = AttrPayload::ProtocolMessages;
        quote! {#attr #item}.into()
    }

    /// A marker indicating a `fn` should be automatically translated to a ProVerif constructor.
    fn pv_constructor(_attr, item) {
        let item: ItemFn = parse_macro_input!(item);
        let attr = AttrPayload::PVConstructor;
        quote! {#attr #item}.into()
    }

    /// A marker indicating a `fn` requires manual modelling in ProVerif.
    fn pv_handwritten(_attr, item) {
        let item: ItemFn = parse_macro_input!(item);
        let attr = AttrPayload::PVHandwritten;
        quote! {#attr #item}.into()
    }

    /// This macro inserts a verbatim Lean proof into the extracted code.
    fn legacy_lean_proof(payload, item) {
        let item: ItemFn = parse_macro_input!(item);
        let payload = parse_macro_input!(payload as LitStr).value();
        let attr = AttrPayload::Proof(payload);
        quote! {#attr #item}.into()
    }

    /// This macro inserts a verbatim Lean proof showing that the `requires`-condition is panic-free.
    /// The proof is inserted into the `pureRequires` field of the Lean spec.
    fn legacy_lean_pure_requires_proof(payload, item) {
        let item: ItemFn = parse_macro_input!(item);
        let payload = parse_macro_input!(payload as LitStr).value();
        let attr = AttrPayload::PureRequiresProof(payload);
        quote! {#attr #item}.into()
    }

    /// This macro inserts a verbatim Lean proof showing that the `ensures`-condition is panic-free.
    /// The proof is inserted into the `pureEnsures` field of the Lean spec.
    fn legacy_lean_pure_ensures_proof(payload, item) {
        let item: ItemFn = parse_macro_input!(item);
        let payload = parse_macro_input!(payload as LitStr).value();
        let attr = AttrPayload::PureEnsuresProof(payload);
        quote! {#attr #item}.into()
    }

    /// Use the proof method `grind`. This influences the tactic and spec set used by Lean.
    fn legacy_lean_proof_method_grind(_attr, item) {
        let item: ItemFn = parse_macro_input!(item);
        let attr = AttrPayload::ProofMethod(hax_lib_macros_types::ProofMethod::Grind);
        quote! {#attr #item}.into()
    }

    /// Use the proof method `bv_decide`. This influences the tactic and spec set used by Lean.
    fn legacy_lean_proof_method_bv_decide(_attr, item) {
        let item: ItemFn = parse_macro_input!(item);
        let attr = AttrPayload::ProofMethod(hax_lib_macros_types::ProofMethod::BvDecide);
        quote! {#attr #item}.into()
    }
}

/// Mark a `Proof<{STATEMENT}>`-returning function as a lemma, where
/// `STATEMENT` is a `Prop` expression capturing any input
/// variable.
/// In the backends, this will generate a lemma with an empty proof.
///
/// # Example
///
/// ```
/// use hax_lib_macros::*;
// #[decreases((m, n))] (TODO: see #297)
/// pub fn ackermann(m: u64, n: u64) -> u64 {
///     match (m, n) {
///         (0, _) => n + 1,
///         (_, 0) => ackermann(m - 1, 1),
///         _ => ackermann(m - 1, ackermann(m, n - 1)),
///     }
/// }
///
/// #[lemma]
/// /// $`\forall n \in \mathbb{N}, \textrm{ackermann}(2, n) = 2 (n + 3) - 3`$
/// pub fn ackermann_property_m1(n: u64) -> Proof<{ ackermann(2, n) == 2 * (n + 3) - 3 }> {}
/// ```
#[proc_macro_attribute]
pub fn lemma(attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(hax)]
    {
        implementation::lemma(attr, item)
    }
    #[cfg(not(hax))]
    {
        let _ = (attr, item);
        TokenStream::new()
    }
}

/// Enable the following attributes in the annotated item and sub-items.
///
/// ### `refine` (on a field in a struct)
/// Refine a type with a logical formula.
///
/// ### `order` (on a named field in a struct or an enum)
/// Reorders a field in the extracted code.
///
/// Rust fields order matters for bit-level representation. Similarly, in some
/// situations, fields order matters in the backends: for instance in F*, one
/// may refine a field with a formula referring to a later field.
///
/// Those two orders may conflict. Adding `#[hax_lib::order(n)]` on a field with
/// override its order at extraction time.
///
/// By default, the order of a field is its index, e.g. the first field has
/// order 0, the i-th field has order i+1.
///
/// `refine` and `order` may also be written behind a `cfg_attr`, e.g.
/// `#[cfg_attr(hax_backend_fstar, refine(..))]` to refine a field for one
/// backend only. At most one `refine` and one `order` may be enabled per field.
///
/// ### `decreases`, `ensures` and `requires` (on a `fn` in an `impl`)
/// `decreases`, `ensures`, `requires`: behave exactly as documented above on
/// the proc attributes of the same name.
///
/// Those may also be written behind a `cfg_attr`, e.g.
/// `#[cfg_attr(hax, requires(..))]`: the predicate is preserved, so the
/// specification appears exactly when the predicate holds. This makes
/// `hax-lib` usable as a `cfg(hax)`-gated dependency.
///
/// # Example
///
/// ```
/// #[hax_lib_macros::attributes]
/// mod foo {
///     pub struct Hello {
///         pub x: u32,
///         #[refine(y > 3)]
///         pub y: u32,
///         #[refine(y + x + z > 3)]
///         pub z: u32,
///     }
///     impl Hello {
///         fn sum(&self) -> u32 {
///             self.x + self.y + self.z
///         }
///         #[ensures(|result| result - n == self.sum())]
///         fn plus(self, n: u32) -> u32 {
///             self.sum() + n
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn attributes(attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(hax)]
    {
        implementation::attributes(attr, item)
    }
    #[cfg(not(hax))]
    {
        let _ = attr;
        dummy::attributes(item)
    }
}

/// Create a mathematical integer. This macro expects a Rust integer
/// literal without suffix.
///
/// ## Examples:
/// - `int!(0x101010)`
/// - `int!(42)`
/// - `int!(0o52)`
/// - `int!(0h2A)`
#[proc_macro]
pub fn int(payload: TokenStream) -> TokenStream {
    #[cfg(hax)]
    {
        implementation::int(payload)
    }
    #[cfg(not(hax))]
    {
        dummy::int(payload)
    }
}

/// Add an invariant to a loop which deals with an index. The
/// invariant cannot refer to any variable introduced within the
/// loop. An invariant is a closure that takes one argument, the
/// index, and returns a proposition.
///
/// Note that loop invariants are unstable (this will be handled in a
/// better way in the future, see
/// <https://github.com/pereq-dev/hax/issues/858>) and only supported on
/// specific `for` loops with specific iterators:
///
///  - `for i in start..end {...}`
///  - `for i in (start..end).step_by(n) {...}`
///  - `for i in slice.enumerate() {...}`
///  - `for i in slice.chunks_exact(n).enumerate() {...}`
///
/// This function must be called on the first line of a loop body to
/// be effective. Note that in the invariant expression, `forall`,
/// `exists`, and `BACKEND!` (`BACKEND` can be `fstar`, `proverif`,
/// `coq`...) are in scope.
#[proc_macro]
pub fn loop_invariant(predicate: TokenStream) -> TokenStream {
    #[cfg(hax)]
    {
        implementation::loop_invariant(predicate)
    }
    #[cfg(not(hax))]
    {
        let _ = predicate;
        TokenStream::new()
    }
}

/// Must be used to prove termination of while loops. This takes an
/// expression that should be a usize that decreases at every iteration
///
/// This function must be called just after `loop_invariant`, or at the first
/// line of the loop if there is no invariant.
#[proc_macro]
pub fn loop_decreases(predicate: TokenStream) -> TokenStream {
    #[cfg(hax)]
    {
        implementation::loop_decreases(predicate)
    }
    #[cfg(not(hax))]
    {
        let _ = predicate;
        TokenStream::new()
    }
}

/// Internal macro for dealing with function decorations
/// (`#[decreases(...)]`, `#[ensures(...)]`, `#[requires(...)]`) on
/// `fn` items within an `impl` block. There is special handling since
/// such functions might have a `self` argument: in such cases, we
/// rewrite function decorations as `#[impl_fn_decoration(<KIND>,
/// <GENERICS>, <WHERE CLAUSE>, <SELF TYPE> [as <TRAIT>], <BODY>)]`, where
/// `<TRAIT>` is the trait implemented by the enclosing `impl` block.
#[proc_macro_attribute]
pub fn impl_fn_decoration(attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(hax)]
    {
        implementation::impl_fn_decoration(attr, item)
    }
    #[cfg(not(hax))]
    {
        let _ = (attr, item);
        dummy::internal_macro_misuse("impl_fn_decoration")
    }
}

/// Internal macro for dealing with function decorations on `fn` items within a
/// `trait`. See [`macro@impl_fn_decoration`].
#[proc_macro_attribute]
pub fn trait_fn_decoration(attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(hax)]
    {
        implementation::trait_fn_decoration(attr, item)
    }
    #[cfg(not(hax))]
    {
        let _ = (attr, item);
        dummy::internal_macro_misuse("trait_fn_decoration")
    }
}

/// Defines the item-level quoting attributes of a backend: `<BACKEND>_before`
/// and `<BACKEND>_after`.
macro_rules! item_quoting_proc_macros {
    ($backend:ident, $(($name:ident, $position:literal)),*) => {$(
        #[doc = concat!("This macro inlines verbatim ", stringify!($backend), " code ", $position, " a Rust item.")]
        ///
        /// This macro takes a string literal containing backend
        /// code. Just as backend expression macros, this literal can
        /// contains dollar-prefixed Rust names.
        ///
        /// Note: when targetting F*, you can prepend a first
        /// comma-separated argument: `interface`, `impl` or
        /// `both`. This controls where the code will apprear: in the
        /// `fst` or `fsti` files or both.
        #[proc_macro_attribute]
        pub fn $name(payload: TokenStream, item: TokenStream) -> TokenStream {
            #[cfg(hax)]
            { implementation::$name(payload, item) }
            #[cfg(not(hax))]
            { let _ = payload; item }
        }
    )*};
}

/// Defines every proc-macro attached to a given backend.
macro_rules! quoting_proc_macros {
    ($backend:ident, $expr:ident, $prop_expr:ident, $unsafe_expr:ident,
     $before:ident, $after:ident, $replace:ident, $replace_body:ident) => {
        #[doc = concat!("Embed ", stringify!($backend), " expression inside a Rust expression. This macro takes only one argument: some raw ", stringify!($backend), " code as a string literal.")]
        ///
        /// While it is possible to directly write raw backend code,
        /// sometimes it can be inconvenient. For example, referencing
        /// Rust names can be a bit cumbersome: for example, the name
        /// `my_crate::my_module::CONSTANT` might be translated
        /// differently in a backend (e.g. in the F* backend, it will
        /// probably be `My_crate.My_module.v_CONSTANT`).
        ///
        /// To facilitate this, you can write Rust names directly,
        /// using the prefix `$`: `f $my_crate::my_module__CONSTANT + 3`
        /// will be replaced with `f My_crate.My_module.v_CONSTANT + 3`
        /// in the F* backend for instance.
        ///
        /// If you want to refer to the Rust constructor
        /// `Enum::Variant`, you should write `$$Enum::Variant` (note
        /// the double dollar).
        ///
        /// If the name refers to something polymorphic, you need to
        /// signal it by adding _any_ type informations,
        /// e.g. `${my_module::function<()>}`. The curly braces are
        /// needed for such more complex expressions.
        ///
        /// You can also write Rust patterns with the `$?{SYNTAX}`
        /// syntax, where `SYNTAX` is a Rust pattern. The syntax
        /// `${EXPR}` also allows any Rust expressions
        /// `EXPR` to be embedded.
        ///
        /// Types can be refered to with the syntax `$:{TYPE}`.
        #[proc_macro]
        pub fn $expr(payload: TokenStream) -> TokenStream {
            #[cfg(hax)]
            { implementation::$expr(payload) }
            #[cfg(not(hax))]
            { let _ = payload; dummy::unit_expr() }
        }

        #[doc = concat!("The `Prop` version of `", stringify!($backend), "_expr`.")]
        #[proc_macro]
        pub fn $prop_expr(payload: TokenStream) -> TokenStream {
            #[cfg(hax)]
            { implementation::$prop_expr(payload) }
            #[cfg(not(hax))]
            { let _ = payload; dummy::prop_expr() }
        }

        #[doc = concat!("The unsafe (because polymorphic: even computationally relevant code can be inlined!) version of `", stringify!($backend), "_expr`.")]
        #[proc_macro]
        #[doc(hidden)]
        pub fn $unsafe_expr(payload: TokenStream) -> TokenStream {
            #[cfg(hax)]
            { implementation::$unsafe_expr(payload) }
            #[cfg(not(hax))]
            { let _ = payload; dummy::unsafe_expr() }
        }

        item_quoting_proc_macros!($backend, ($before, "before"), ($after, "after"));

        #[doc = concat!("Replaces a Rust item with some verbatim ", stringify!($backend)," code.")]
        #[proc_macro_attribute]
        pub fn $replace(payload: TokenStream, item: TokenStream) -> TokenStream {
            #[cfg(hax)]
            { implementation::$replace(payload, item) }
            #[cfg(not(hax))]
            { let _ = payload; item }
        }

        #[doc = concat!("Replaces the body of a Rust function with some verbatim ", stringify!($backend)," code.")]
        #[proc_macro_attribute]
        pub fn $replace_body(payload: TokenStream, item: TokenStream) -> TokenStream {
            #[cfg(hax)]
            { implementation::$replace_body(payload, item) }
            #[cfg(not(hax))]
            { let _ = payload; item }
        }
    };
}

quoting_proc_macros!(
    fstar,
    fstar_expr,
    fstar_prop_expr,
    fstar_unsafe_expr,
    fstar_before,
    fstar_after,
    fstar_replace,
    fstar_replace_body
);
quoting_proc_macros!(
    coq,
    coq_expr,
    coq_prop_expr,
    coq_unsafe_expr,
    coq_before,
    coq_after,
    coq_replace,
    coq_replace_body
);
quoting_proc_macros!(
    proverif,
    proverif_expr,
    proverif_prop_expr,
    proverif_unsafe_expr,
    proverif_before,
    proverif_after,
    proverif_replace,
    proverif_replace_body
);
quoting_proc_macros!(
    legacy_lean,
    legacy_lean_expr,
    legacy_lean_prop_expr,
    legacy_lean_unsafe_expr,
    legacy_lean_before,
    legacy_lean_after,
    legacy_lean_replace,
    legacy_lean_replace_body
);
