use crate::syn_ext::*;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::*;

/// The `RewriteSelf` structure is hidden in a module so that only its
/// method can mutate its fields.
mod rewrite_self {
    use super::*;
    use std::collections::HashSet;

    /// Small & dirty wrapper around spans to make them `Eq`,
    /// `PartialEq` and `Hash`
    #[derive(Clone, Debug)]
    struct SpanWrapper(Span);
    const _: () = {
        impl Eq for SpanWrapper {}
        impl PartialEq for SpanWrapper {
            fn eq(&self, other: &Self) -> bool {
                format!("{self:?}") == format!("{other:?}")
            }
        }
        use std::hash::*;
        impl Hash for SpanWrapper {
            fn hash<H: Hasher>(&self, state: &mut H) {
                format!("{self:?}").hash(state)
            }
        }
    };

    /// How to qualify `Self::Assoc` projections: `Self` is never in scope
    /// in the items we generate.
    #[derive(Clone, Default)]
    pub enum SelfProjection {
        /// `Self` is a concrete type: `Self::A` becomes `<TYPE as TRAIT>::A`,
        /// as Rust requires such projections to be fully qualified.
        Trait(Path),
        /// `Self` is a type parameter bounded by the trait: `Self::A` becomes
        /// `PARAM::A`. Not `<PARAM as TRAIT>::A`: that form rejects associated
        /// types inherited from supertraits.
        TypeParam,
        /// Projections cannot be qualified here, reject them.
        Unsupported,
        /// Nothing is known about `Self`: leave projections alone.
        #[default]
        Unknown,
    }

    /// A struct that carries informations for substituting `self` and
    /// `Self`. Note `typ` is an option:
    #[must_use]
    pub struct RewriteSelf {
        typ: Option<Type>,
        projection: SelfProjection,
        ident: Ident,
        self_spans: HashSet<SpanWrapper>,
        errors: Vec<Error>,
    }

    impl RewriteSelf {
        /// How `Self::Assoc` projections should be qualified.
        pub fn projection(&self) -> &SelfProjection {
            &self.projection
        }

        /// Rejects a `Self::Assoc` projection we cannot qualify.
        pub fn reject_projection(&mut self, span: Span) {
            self.errors.push(Error::new(
                span,
                "hax: an associated type of `Self` cannot be used in the signature of a \
                 trait method carrying a specification. Attach the specification to the \
                 implementations instead. See \
                 https://github.com/pereq-dev/hax/issues/2089.",
            ));
        }

        /// Consumes `RewriteSelf`, optionally outputing errors.
        pub fn get_error(self) -> Option<proc_macro2::TokenStream> {
            let Self {
                typ,
                self_spans,
                mut errors,
                ..
            } = self;
            if typ.is_none() && !self_spans.is_empty() {
                let mut error = Error::new(
                    Span::call_site(),
                    "This macro doesn't work on trait or impl items: you need to add a `#[hax_lib::attributes]` on the enclosing impl block or trait.",
                );
                for SpanWrapper(span) in self_spans {
                    let use_site = Error::new(
                        span,
                        "Here, the function you are trying to annotate has a `Self`.",
                    );
                    error.combine(use_site);
                }
                errors.push(error);
            }
            let mut errors = errors.into_iter();
            let mut error = errors.next()?;
            for other in errors {
                error.combine(other);
            }
            Some(error.to_compile_error())
        }

        fn self_detected(&mut self, span: Span) {
            self.self_spans.insert(SpanWrapper(span));
        }

        /// Requests the ident with which `self` should be substituted.
        pub fn self_ident(&mut self, span: Span) -> &Ident {
            self.self_detected(span);
            &self.ident
        }
        /// Requests the type with which `Self` should be substituted with.
        pub fn self_ty(&mut self, span: Span) -> Type {
            self.self_detected(span);
            self.typ.clone().unwrap_or_else(|| {
                parse_quote! {Self}
            })
        }
        /// Construct a rewritter
        pub fn new(ident: Ident, typ: Option<Type>, projection: SelfProjection) -> Self {
            Self {
                typ,
                projection,
                ident,
                self_spans: HashSet::new(),
                errors: Vec::new(),
            }
        }
    }
}
pub use rewrite_self::*;

impl RewriteSelf {
    /// Rewrites `Self::A::…` into a qualified path: `<TYPE>::A::…` or
    /// `<TYPE as TRAIT>::A::…`, see [`SelfProjection`].
    fn rewrite_self_projection(
        &mut self,
        expr_position: bool,
        qself: &mut Option<QSelf>,
        path: &mut Path,
    ) {
        let (
            None,
            Path {
                leading_colon: None,
                segments,
            },
        ) = (&*qself, &*path)
        else {
            return;
        };
        let mut segments = segments.iter();
        let Some(PathSegment {
            ident,
            arguments: PathArguments::None,
        }) = segments.next()
        else {
            return;
        };
        let suffix: Vec<_> = segments.collect();
        if ident != "Self" || suffix.is_empty() {
            return;
        }
        let as_trait = match self.projection().clone() {
            // On a concrete type, `as TRAIT` could change which item
            // `Self::X` resolves to: we qualify in type positions only,
            // where only associated types are at stake.
            SelfProjection::Trait(_) if expr_position => return,
            SelfProjection::Trait(trait_path) => trait_path,
            // Unlike `Trait`, also valid in expression position.
            SelfProjection::TypeParam => {
                let self_ty = self.self_ty(path.span());
                let rewritten: TypePath = parse_quote! { #self_ty::#(#suffix)::* };
                (*qself, *path) = (rewritten.qself, rewritten.path);
                return;
            }
            SelfProjection::Unsupported => return self.reject_projection(path.span()),
            SelfProjection::Unknown => return,
        };
        let self_ty = self.self_ty(path.span());
        let rewritten: TypePath = parse_quote! { <#self_ty as #as_trait>::#(#suffix)::* };
        (*qself, *path) = (rewritten.qself, rewritten.path);
    }
}

impl visit_mut::VisitMut for RewriteSelf {
    fn visit_type_path_mut(&mut self, tp: &mut TypePath) {
        visit_mut::visit_type_path_mut(self, tp);
        self.rewrite_self_projection(false, &mut tp.qself, &mut tp.path);
    }
    fn visit_expr_path_mut(&mut self, ep: &mut ExprPath) {
        visit_mut::visit_expr_path_mut(self, ep);
        self.rewrite_self_projection(true, &mut ep.qself, &mut ep.path);
    }
    fn visit_expr_mut(&mut self, e: &mut Expr) {
        visit_mut::visit_expr_mut(self, e);
        if e.is_ident("self") {
            let into = self.self_ident(e.span()).clone();
            *e = parse_quote! {#into}
        }
    }
    fn visit_type_mut(&mut self, ty: &mut Type) {
        visit_mut::visit_type_mut(self, ty);
        if ty.is_ident("Self") {
            *ty = self.self_ty(ty.span())
        }
    }
    fn visit_fn_arg_mut(&mut self, arg: &mut FnArg) {
        visit_mut::visit_fn_arg_mut(self, arg);
        let arg_span = arg.span();
        if let FnArg::Receiver(r) = arg {
            let span = r.self_token.span();
            *arg = FnArg::Typed(PatType {
                attrs: r.attrs.clone(),
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: self.self_ident(span).clone(),
                    subpat: None,
                })),
                colon_token: token::Colon(arg_span),
                ty: Box::new({
                    let ty = self.self_ty(span);
                    let (reference, lt) = r
                        .reference
                        .clone()
                        .map(|(r, lt)| (Some(r), lt))
                        .unwrap_or((None, None));
                    let mutability = reference.and(r.mutability.clone());
                    parse_quote! {#reference #lt #mutability #ty}
                }),
            });
        }
    }
    fn visit_item_impl_mut(&mut self, _i: &mut ItemImpl) {
        // Do nothing! We allow user to write self if it's nested in a impl block
    }
}
