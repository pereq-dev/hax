use syn::visit::Visit;

use crate::prelude::*;
use crate::rewrite_self::*;

/// `HaxQuantifiers` makes polymorphic expression inlining functions available
pub struct HaxQuantifiers;
impl ToTokens for HaxQuantifiers {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote! {
            use ::hax_lib::fstar::prop as fstar;
            use ::hax_lib::coq::prop as coq;
            use ::hax_lib::legacy_lean::prop as lean;
            use ::hax_lib::proverif::prop as proverif;
        }
        .to_tokens(tokens)
    }
}

/// Meta informations about functions decorations
pub enum FnDecorationKind {
    Requires,
    Ensures { ret_binder: Pat, by_ref: bool },
    Decreases,
    SMTPat,
}

impl ToString for FnDecorationKind {
    fn to_string(&self) -> String {
        match self {
            FnDecorationKind::Requires => "requires".to_string(),
            FnDecorationKind::Ensures { .. } => "ensures".to_string(),
            FnDecorationKind::Decreases { .. } => "decreases".to_string(),
            FnDecorationKind::SMTPat { .. } => "SMTPat".to_string(),
        }
    }
}

impl From<FnDecorationKind> for AssociationRole {
    fn from(kind: FnDecorationKind) -> Self {
        match &kind {
            FnDecorationKind::Requires => AssociationRole::Requires,
            FnDecorationKind::Ensures { .. } => AssociationRole::Ensures,
            FnDecorationKind::Decreases => AssociationRole::Decreases,
            FnDecorationKind::SMTPat => AssociationRole::SMTPat,
        }
    }
}

/// A trait's own path, parameters turned back into arguments:
/// `trait T<'a, X, const N: usize>` yields `T<'a, X, N>`.
pub(crate) fn self_trait_path(item: &ItemTrait) -> Path {
    let args: Vec<GenericArgument> = item
        .generics
        .params
        .iter()
        .map(|param| match param {
            GenericParam::Lifetime(lt) => GenericArgument::Lifetime(lt.lifetime.clone()),
            GenericParam::Type(ty) => {
                let ident = &ty.ident;
                GenericArgument::Type(parse_quote! {#ident})
            }
            GenericParam::Const(c) => {
                let ident = &c.ident;
                GenericArgument::Const(parse_quote! {#ident})
            }
        })
        .collect();
    let ident = &item.ident;
    if args.is_empty() {
        parse_quote! {#ident}
    } else {
        parse_quote! {#ident<#(#args),*>}
    }
}

/// Builds the argument list of the internal `::hax_lib::impl_fn_decoration`
/// attribute: `<KIND>, <GENERICS>, <WHERE CLAUSE>, <SELF TYPE> [as <TRAIT>], <BODY>`.
pub(crate) fn impl_fn_decoration_args(
    decoration: &Ident,
    generics: &Generics,
    self_ty: &Type,
    as_trait: &Option<TokenStream>,
    tokens: &TokenStream,
) -> TokenStream {
    let where_clause = &generics.where_clause;
    quote! {#decoration, #generics, #where_clause, #self_ty #as_trait, #tokens}
}

/// Gates every item of `tokens` on `#[cfg(#pred)]`, if there is a `pred`.
pub(crate) fn cfg_gate(tokens: TokenStream, pred: Option<&Meta>) -> TokenStream {
    let Some(pred) = pred else {
        return tokens;
    };
    let Ok(file) = syn::parse2::<File>(tokens.clone()) else {
        return quote! {#[cfg(#pred)] const _: () = {#tokens};};
    };
    file.items
        .iter()
        .map(|item| quote! {#[cfg(#pred)] #item})
        .collect()
}

/// An item raising `error`, gated like [`cfg_gate`].
pub(crate) fn gated_error(error: Error, pred: Option<&Meta>) -> TokenStream {
    let error = error.to_compile_error();
    cfg_gate(quote! {const _: () = {#error};}, pred)
}

/// Emit one of charon's native `charon::*` markers. Only the lean backend drives
/// charon directly, bypassing the engine; every other backend gets nothing.
pub(crate) fn charon_attr(name: TokenStream) -> Option<TokenStream> {
    cfg!(hax_backend_lean).then(|| quote! {#[charon::#name]})
}

/// Whether the future value of mutable arguments (used by postconditions) come before or after the
/// result binder. The legacy engine expect future values before the return one, while the aeneas
/// engine expects the opposite.
pub(crate) fn future_args_last() -> bool {
    cfg!(hax_backend_lean)
}

/// Whether the decoration of a function without argument needs a padding unit argument. The legacy
/// engine translates functions of arity zero to functions taking exactly one unit argument, while
/// the aeneas engine keeps them of arity zero.
pub(crate) fn pad_nullary_sig() -> bool {
    !cfg!(hax_backend_lean)
}

/// Merge two `syn::Generics`, respecting lifetime orders
pub(crate) fn merge_generics(x: Generics, y: Generics) -> Generics {
    Generics {
        lt_token: x.lt_token.or(y.lt_token),
        gt_token: x.gt_token.or(y.gt_token),
        params: {
            let lts = x
                .lifetimes()
                .chain(y.lifetimes())
                .cloned()
                .map(GenericParam::Lifetime);
            let not_lts = x
                .params
                .clone()
                .into_iter()
                .filter(|p| !matches!(p, GenericParam::Lifetime(_)))
                .chain(
                    y.params
                        .clone()
                        .into_iter()
                        .filter(|p| !matches!(p, GenericParam::Lifetime(_))),
                );
            lts.chain(not_lts).collect()
        },
        where_clause: match (x.where_clause, y.where_clause) {
            (Some(wx), Some(wy)) => Some(syn::WhereClause {
                where_token: wx.where_token,
                predicates: wx.predicates.into_iter().chain(wy.predicates).collect(),
            }),
            (Some(w), None) | (None, Some(w)) => Some(w),
            (None, None) => None,
        },
    }
}

/// Transform every `x: &mut T` input into `x: &T` in a signature, and
/// returns a list of such transformed `x: &T` inputs
fn unmut_references_in_inputs(sig: &mut Signature) -> Vec<FnArg> {
    let mut mutable_inputs = vec![];
    for input in &mut sig.inputs {
        if let Some(mutability) = match input {
            FnArg::Receiver(syn::Receiver {
                reference: Some(_),
                mutability,
                ..
            }) => Some(mutability),
            FnArg::Typed(syn::PatType { ty, .. }) => {
                use std::borrow::BorrowMut;
                if let syn::Type::Reference(syn::TypeReference { mutability, .. }) = ty.borrow_mut()
                {
                    Some(mutability)
                } else {
                    None
                }
            }
            _ => None,
        } {
            if mutability.is_some() {
                *mutability = None;
                mutable_inputs.push(input.clone());
            }
        }
    }
    mutable_inputs
}

/// Expects a `FnArg` to be a simple variable pattern
fn expect_fn_arg_var_pat(arg: &FnArg) -> Option<(String, syn::Type)> {
    match arg {
        FnArg::Receiver(recv) => Some(("self".into(), *recv.ty.clone())),
        FnArg::Typed(pat_type) => match &*pat_type.pat {
            syn::Pat::Wild(_) => Some(("".into(), *pat_type.ty.clone())),
            syn::Pat::Ident(pat_ident) => {
                Some((format!("{}", pat_ident.ident), *pat_type.ty.clone()))
            }
            _ => None,
        },
    }
}

pub(crate) enum NotFutureExpr {
    BadNumberOfArgs,
    ArgNotIdent,
}

/// `expect_future_expr(e)` tries to match the pattern
/// `future(<syn::Ident>)` in expression `e`
pub(crate) fn expect_future_expr(e: &Expr) -> Option<std::result::Result<Ident, NotFutureExpr>> {
    if let Expr::Call(call) = e {
        if call.func.is_ident("future") {
            return Some(match call.args.iter().collect::<Vec<_>>().as_slice() {
                [arg] => arg.expect_ident().ok_or(NotFutureExpr::ArgNotIdent),
                _ => Err(NotFutureExpr::BadNumberOfArgs),
            });
        }
    }
    None
}

#[derive(Default)]
pub struct IdentCollector {
    pub idents: Vec<Ident>,
}

impl<'ast> syn::visit::Visit<'ast> for IdentCollector {
    fn visit_ident(&mut self, ident: &'ast Ident) {
        self.idents.push(ident.clone());
    }
}

impl IdentCollector {
    /// Returns a fresh identifier with the given prefix that is not in the collected identifiers.
    pub fn fresh_ident(&self, prefix: &str) -> Ident {
        let idents: HashSet<&Ident> = HashSet::from_iter(self.idents.iter());
        let mk = |s| Ident::new(s, Span::call_site());
        std::iter::once(mk(prefix))
            .chain((0u64..).map(|i| Ident::new(&format!("{}{}", prefix, i), Span::call_site())))
            .find(|ident| !idents.contains(ident))
            .unwrap()
    }
}

/// Rewrites `future(x)` nodes in an expression when (1) `x` is an
/// ident and (2) the ident `x` is contained in the HashSet.
struct RewriteFuture(HashSet<String>);
impl VisitMut for RewriteFuture {
    fn visit_expr_mut(&mut self, e: &mut Expr) {
        syn::visit_mut::visit_expr_mut(self, e);
        let error = match expect_future_expr(e) {
            Some(Ok(arg)) => {
                let arg = format!("{}", arg);
                if self.0.contains(&arg) {
                    let arg = create_future_ident(&arg);
                    *e = parse_quote! {#arg};
                    return;
                }
                Some(format!(
                    "Cannot find an input `{arg}` of type `&mut _`. In the context, `future` can be called on the following inputs: {:?}.",
                    self.0
                ))
            }
            Some(Err(error_kind)) => {
                let message = match error_kind {
                    NotFutureExpr::BadNumberOfArgs => {
                        "`future` can only be called with one argument: a `&mut` input name"
                    }
                    NotFutureExpr::ArgNotIdent => {
                        "`future` can only be called with an `&mut` input name"
                    }
                };
                let help_message = match self.0.iter().next() {
                    None => " In the context, there is no `&mut` input.".to_string(),
                    Some(var) => {
                        format!(" For example, in the context you can write `future({var})`.")
                    }
                };
                Some(format!("{message}.{}", help_message))
            }
            None => None,
        };
        if let Some(error) = error {
            *e = parse_quote! {::std::compile_error!(#error)};
        }
    }
}

fn create_future_ident(name: &str) -> syn::Ident {
    proc_macro2::Ident::new(&format!("{name}_future"), proc_macro2::Span::call_site())
}

/// The engine translates functions of arity zero to functions that
/// takes exactly one unit argument. The zero-arity functions we
/// generate are translated correctly as well. But in the case of a
/// `ensures` clause, that's an issue: we produce a function of arity
/// one, whose first argument is the result of the function. Instead,
/// we need a function of arity two.
/// `fix_signature_arity` adds a `unit` if needed.
fn add_unit_to_sig_if_needed(signature: &mut Signature) {
    if signature.inputs.is_empty() && pad_nullary_sig() {
        signature.inputs.push(parse_quote! {_: ()})
    }
}

/// Errors on the `Self::A` projections of `sig` whose associated type `A` is
/// not defined by the enclosing `impl` block: hax qualifies them as
/// `<Type as Trait>::A`, which is correct only for the block's own items.
pub fn foreign_self_projection_error(sig: &Signature, assoc: &[String]) -> Option<TokenStream> {
    struct Collector<'a> {
        assoc: &'a [String],
        errors: Vec<Error>,
    }
    impl<'ast> syn::visit::Visit<'ast> for Collector<'_> {
        fn visit_type_path(&mut self, tp: &'ast TypePath) {
            syn::visit::visit_type_path(self, tp);
            let mut segments = tp.path.segments.iter();
            let (Some(first), Some(assoc)) = (segments.next(), segments.next()) else {
                return;
            };
            if tp.qself.is_some() || first.ident != "Self" {
                return;
            }
            let name = assoc.ident.to_string();
            if !self.assoc.contains(&name) {
                self.errors.push(Error::new(
                    tp.span(),
                    format!(
                        "hax: `Self::{name}` is not defined by this `impl` block, hax cannot \
                         qualify it. Write `<Type as Trait>::{name}` explicitly. See \
                         https://github.com/pereq-dev/hax/issues/2089."
                    ),
                ));
            }
        }
    }
    let mut collector = Collector {
        assoc,
        errors: Vec::new(),
    };
    syn::visit::Visit::visit_signature(&mut collector, sig);
    let mut errors = collector.errors.into_iter();
    let mut error = errors.next()?;
    for other in errors {
        error.combine(other);
    }
    Some(error.to_compile_error())
}

/// Common logic when generating a function decoration
///
/// `self_type` substitutes `Self`, and `self_projection` says how to
/// qualify `Self::Assoc` projections (see [`SelfProjection`]).
pub fn make_fn_decoration(
    mut phi: Expr,
    mut signature: Signature,
    kind: FnDecorationKind,
    mut generics: Option<Generics>,
    self_type: Option<Type>,
    self_projection: SelfProjection,
) -> (TokenStream, AttrPayload) {
    let self_ident: Ident = {
        let mut idents = IdentCollector::default();
        idents.visit_expr(&phi);
        idents.visit_signature(&signature);
        idents.fresh_ident("self_")
    };
    let error = {
        let mut rewriter = RewriteSelf::new(self_ident, self_type, self_projection);
        rewriter.visit_expr_mut(&mut phi);
        rewriter.visit_signature_mut(&mut signature);
        if let Some(generics) = generics.as_mut() {
            rewriter.visit_generics_mut(generics);
        }
        rewriter.get_error()
    };
    let uid = ItemUid::fresh();
    let mut_ref_inputs = unmut_references_in_inputs(&mut signature);
    let decoration = {
        let decoration_sig = {
            let mut sig = signature.clone();
            sig.ident = format_ident!("{}", kind.to_string());
            if let FnDecorationKind::Ensures { ret_binder, by_ref } = &kind {
                add_unit_to_sig_if_needed(&mut sig);
                let output_typ = match sig.output {
                    syn::ReturnType::Default => parse_quote! {()},
                    syn::ReturnType::Type(_, t) => t,
                };
                let output_typ_tokens = if *by_ref {
                    quote! {&#output_typ}
                } else {
                    quote! {#output_typ}
                };
                let mut_ref_inputs = mut_ref_inputs
                    .iter()
                    .map(|mut_ref_input| {
                        expect_fn_arg_var_pat(mut_ref_input).expect(
                            "Every `&mut` input of a function annotated with a `ensures` clause is expected to be a simple variable pattern.",
                        )
                    });
                let mut rewrite_future =
                    RewriteFuture(mut_ref_inputs.clone().map(|x| x.0).collect());
                rewrite_future.visit_expr_mut(&mut phi);
                let (mut pats, mut tys): (Vec<_>, Vec<_>) = mut_ref_inputs
                    .map(|(name, ty)| {
                        (
                            create_future_ident(&name).to_token_stream(),
                            ty.to_token_stream(),
                        )
                    })
                    .unzip();

                let is_output_typ_unit = if let syn::Type::Tuple(tuple) = &*output_typ {
                    tuple.elems.is_empty()
                } else {
                    false
                };

                if !is_output_typ_unit || pats.is_empty() {
                    if future_args_last() {
                        pats.insert(0, ret_binder.to_token_stream());
                        tys.insert(0, output_typ_tokens);
                    } else {
                        pats.push(ret_binder.to_token_stream());
                        tys.push(output_typ_tokens);
                    }
                }

                sig.inputs
                    .push(syn::parse_quote! {(#(#pats),*): (#(#tys),*)});
            }
            if let Some(generics) = generics {
                sig.generics = merge_generics(generics, sig.generics);
            }
            sig.output = match &kind {
                FnDecorationKind::Decreases | FnDecorationKind::SMTPat => {
                    syn::parse_quote! { -> () }
                }
                _ => syn::parse_quote! { -> impl core::convert::Into<::hax_lib::Prop> },
            };
            sig
        };
        let uid_attr = AttrPayload::Uid(uid.clone());
        let late_skip = &AttrPayload::ItemStatus(ItemStatus::Included { late_skip: true });
        if let FnDecorationKind::Decreases | FnDecorationKind::SMTPat = &kind {
            phi = parse_quote! {::hax_lib::any_to_unit(#phi)};
        };
        let quantifiers = if let FnDecorationKind::Decreases = &kind {
            None
        } else {
            Some(HaxQuantifiers)
        };
        let future = if let FnDecorationKind::Ensures { .. } = &kind {
            quote! { #late_skip #AttrHaxLang fn future<T>(x: &mut T) -> &T { x } }
        } else {
            quote! {}
        };
        use AttrPayload::NeverErased;
        quote! {
            // ProVerif ignores contracts, and typechecking `#phi` here could
            // require `#[cfg(hax)]`-only helpers across the dependency tree.
            #[cfg(all(#DebugOrHaxCfgExpr, not(hax_backend_proverif)))]
            #late_skip
            const _: () = {
                #quantifiers
                #future
                #uid_attr
                #late_skip
                #[allow(unused)]
                #NeverErased
                #decoration_sig {
                    #phi
                }
            };
        }
    };

    let assoc_attr = AttrPayload::AssociatedItem {
        role: kind.into(),
        item: uid,
    };
    // On error the decoration is dropped: emitting it anyway would pile
    // rustc errors on top of ours.
    let decoration = match error {
        Some(error) => error,
        None => decoration,
    };
    (decoration, assoc_attr)
}
