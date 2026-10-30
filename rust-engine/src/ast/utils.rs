//! This module provides a collection of utilities to work on AST.

use super::visitors::*;
use super::*;
use identifiers::*;
use std::collections::HashMap;

/// Useful visitor to map AST fragments.
pub mod mappers {
    use super::*;

    /// Visitor that substitutes local identifiers in ASTs.
    pub struct SubstLocalIds(HashMap<LocalId, LocalId>);

    impl SubstLocalIds {
        /// Create a substituer given one replacement couple.
        pub fn one(from: LocalId, to: LocalId) -> Self {
            Self::many([(from, to)])
        }
        /// Create a substituer given a bunch of replacement couples.
        pub fn many(replacements: impl IntoIterator<Item = (LocalId, LocalId)>) -> Self {
            Self(replacements.into_iter().collect())
        }
    }

    impl AstVisitorMut for SubstLocalIds {
        fn visit_local_id(&mut self, local_id: &mut LocalId) {
            if let Some(replacement) = self.0.get(local_id) {
                *local_id = replacement.clone();
            }
        }
    }

    /// How a [`ScopedRenamer`] renames binders.
    pub trait Renaming {
        /// A use of a local refers to the binder in scope with the same key.
        type Key: Eq + std::hash::Hash + Clone;
        /// The key of a local.
        fn key(&self, id: &LocalId) -> Self::Key;
        /// The new name of a binder of `id`.
        fn fresh(&mut self, id: &LocalId) -> LocalId;
    }

    /// Visitor that renames every binder of an expression with a [`Renaming`],
    /// and every use to the new name of the binder in scope. Free variables
    /// are left untouched.
    pub struct ScopedRenamer<R: Renaming> {
        /// The renaming policy.
        pub renaming: R,
        scope: HashMap<R::Key, LocalId>,
    }

    impl<R: Renaming> ScopedRenamer<R> {
        /// A renamer with an empty scope.
        pub fn new(renaming: R) -> Self {
            Self {
                renaming,
                scope: HashMap::new(),
            }
        }

        /// Rename the binders of `pat`, which then scope over what is visited
        /// next.
        pub fn bind(&mut self, pat: &mut Pat) {
            BindPat(self).visit_pat(pat)
        }

        /// Bring `id` into scope under its own name.
        pub fn declare(&mut self, id: LocalId) {
            self.scope.insert(self.renaming.key(&id), id);
        }

        fn bind_var(&mut self, var: &mut LocalId) {
            let new = self.renaming.fresh(var);
            self.scope.insert(self.renaming.key(var), new.clone());
            *var = new;
        }

        fn rename_use(&self, id: &mut LocalId) {
            if let Some(new) = self.scope.get(&self.renaming.key(id)) {
                *id = new.clone();
            }
        }

        /// Rename the binders of `pat` to the names `scope` already gives
        /// them, as for the alternatives of an or-pattern.
        fn rename_pat(&self, pat: &mut Pat) {
            struct RenamePat<'a, R: Renaming>(&'a ScopedRenamer<R>);
            impl<R: Renaming> AstVisitorMut for RenamePat<'_, R> {
                fn visit_pat_kind(&mut self, kind: &mut PatKind) {
                    if let PatKind::Binding { var, .. } = kind {
                        self.0.rename_use(var);
                    }
                    self.visit_inner(kind)
                }
            }
            RenamePat(self).visit_pat(pat)
        }

        fn scoped(&mut self, f: impl FnOnce(&mut Self)) {
            let saved = self.scope.clone();
            f(self);
            self.scope = saved;
        }
    }

    struct BindPat<'a, R: Renaming>(&'a mut ScopedRenamer<R>);

    impl<R: Renaming> AstVisitorMut for BindPat<'_, R> {
        fn visit_pat_kind(&mut self, kind: &mut PatKind) {
            match kind {
                PatKind::Binding { var, sub_pat, .. } => {
                    self.0.bind_var(var);
                    if let Some(sub_pat) = sub_pat {
                        self.visit_pat(sub_pat)
                    }
                }
                // The alternatives bind the same variables.
                PatKind::Or { sub_pats } => {
                    if let Some((first, rest)) = sub_pats.split_first_mut() {
                        self.visit_pat(first);
                        for p in rest {
                            self.0.rename_pat(p);
                        }
                    }
                }
                _ => self.visit_inner(kind),
            }
        }
    }

    impl<R: Renaming> AstVisitorMut for ScopedRenamer<R> {
        fn visit_expr(&mut self, expr: &mut Expr) {
            match &mut *expr.kind {
                ExprKind::LocalId(id) => self.rename_use(id),
                ExprKind::Let { lhs, rhs, body } => {
                    self.visit_expr(rhs);
                    self.scoped(|this| {
                        this.bind(lhs);
                        this.visit_expr(body)
                    })
                }
                ExprKind::Match { scrutinee, arms } => {
                    self.visit_expr(scrutinee);
                    for arm in arms {
                        self.scoped(|this| {
                            this.bind(&mut arm.pat);
                            if let Some(guard) = &mut arm.guard {
                                let GuardKind::IfLet { lhs, rhs } = &mut guard.kind;
                                this.visit_expr(rhs);
                                this.bind(lhs);
                            }
                            this.visit_expr(&mut arm.body)
                        })
                    }
                }
                ExprKind::Closure {
                    params,
                    body,
                    captures,
                } => {
                    for capture in captures {
                        self.visit_expr(capture);
                    }
                    self.scoped(|this| {
                        for param in params {
                            this.bind(param);
                        }
                        this.visit_expr(body)
                    })
                }
                ExprKind::Loop {
                    body, kind, state, ..
                } => {
                    match &mut **kind {
                        LoopKind::ForLoop { iterator, .. } => self.visit_expr(iterator),
                        LoopKind::ForIndexLoop { start, end, .. } => {
                            self.visit_expr(start);
                            self.visit_expr(end);
                        }
                        LoopKind::WhileLoop { .. } | LoopKind::UnconditionalLoop => {}
                    }
                    if let Some(state) = state {
                        self.visit_expr(&mut state.init);
                    }
                    self.scoped(|this| {
                        match &mut **kind {
                            LoopKind::ForLoop { pat, .. } => this.bind(pat),
                            LoopKind::ForIndexLoop { var, .. } => this.bind_var(var),
                            LoopKind::WhileLoop { .. } | LoopKind::UnconditionalLoop => {}
                        }
                        if let Some(state) = state {
                            this.bind(&mut state.body_pat);
                        }
                        if let LoopKind::WhileLoop { condition } = &mut **kind {
                            this.visit_expr(condition);
                        }
                        this.visit_expr(body)
                    })
                }
                ExprKind::Quote { contents } => {
                    for content in &mut contents.0 {
                        match content {
                            QuoteContent::Expr(e) => self.visit_expr(e),
                            QuoteContent::Pattern(p) => self.rename_pat(p),
                            QuoteContent::Verbatim(_) | QuoteContent::Ty(_) => {}
                        }
                    }
                }
                _ => self.visit_inner(expr),
            }
        }
    }
}

impl Expr {
    /// Create a tuple expression out of components.
    pub fn tuple(components: Vec<Expr>, span: Span) -> Self {
        let ty = TyKind::tuple(
            components
                .iter()
                .map(Typed::ty)
                .cloned()
                .map(GenericValue::Ty)
                .collect(),
        )
        .promote();
        ExprKind::tuple(components).promote(ty, span)
    }

    /// Create a unit (tuple of size 0) expression.
    pub fn unit(span: Span) -> Self {
        ExprKind::GlobalId(global_id::TupleId::Constructor { length: 0 }.into())
            .promote(TyKind::unit().promote(), span)
    }

    /// The value `constructor(fields)` of an enum variant.
    pub fn variant(
        constructor: GlobalId,
        fields: Vec<(GlobalId, Expr)>,
        ty: Ty,
        span: Span,
    ) -> Self {
        ExprKind::Construct {
            constructor,
            is_record: false,
            is_struct: false,
            fields,
            base: None,
        }
        .promote(ty, span)
    }

    /// The value `Some(value)` of type `ty`.
    pub fn some(value: Expr, ty: Ty, span: Span) -> Self {
        use crate::names::core::option::Option::Some;
        Self::variant(Some::Constructor, vec![(Some::_0, value)], ty, span)
    }

    /// The value `None` of type `ty`.
    pub fn none(ty: Ty, span: Span) -> Self {
        use crate::names::core::option::Option::None;
        Self::variant(None::Constructor, vec![], ty, span)
    }

    /// Whether the expression is a local, a global or a literal, under any
    /// ascriptions.
    pub fn is_leaf(&self) -> bool {
        match &*self.kind {
            ExprKind::LocalId(_) | ExprKind::GlobalId(_) | ExprKind::Literal(_) => true,
            ExprKind::Ascription { e, .. } => e.is_leaf(),
            _ => false,
        }
    }

    /// Whether the expression refers to the global `id`.
    pub fn mentions(&self, id: GlobalId) -> bool {
        struct Mentions(GlobalId, bool);
        impl AstVisitor for Mentions {
            fn enter_expr_kind(&mut self, kind: &ExprKind) {
                self.1 |= matches!(kind, ExprKind::GlobalId(g) if *g == self.0);
            }
        }
        let mut visitor = Mentions(id, false);
        visitor.visit_expr(self);
        visitor.1
    }

    /// Creates a `App` node for a standalone function.
    pub fn standalone_fn_app(
        head: impl Into<FnAppHead>,
        generic_args: Vec<GenericValue>,
        args: Vec<Expr>,
        output_type: Ty,
        span: Span,
    ) -> Self {
        ExprKind::standalone_fn_app(head, generic_args, args, output_type.clone(), span)
            .promote(output_type, span)
    }

    /// Creates a `App` node.
    pub fn fn_app(
        head: impl Into<FnAppHead>,
        generic_args: Vec<GenericValue>,
        args: Vec<Expr>,
        output_type: Ty,
        bounds_impls: Vec<ImplExpr>,
        trait_: Option<(ImplExpr, Vec<GenericValue>)>,
        span: Span,
    ) -> Self {
        ExprKind::fn_app(
            head,
            generic_args,
            args,
            output_type.clone(),
            bounds_impls,
            trait_,
            span,
        )
        .promote(output_type, span)
    }

    /// Removes a box
    pub fn unbox_once(&self) -> Option<&Expr> {
        if let ExprKind::App { head, args, .. } = self.kind()
            && let [arg] = &**args
            && let ExprKind::GlobalId(head) = head.kind()
            && let crate::names::alloc::boxed::Impl::new
            | crate::names::rust_primitives::hax::box_new = *head
        {
            Some(arg)
        } else {
            None
        }
    }

    /// Removes a deref
    pub fn underef_once(&self) -> Option<&Expr> {
        if let ExprKind::App { head, args, .. } = self.kind()
            && let [arg] = &**args
            && let ExprKind::GlobalId(head) = head.kind()
            && let crate::names::rust_primitives::hax::deref_op = *head
        {
            Some(arg)
        } else {
            None
        }
    }

    /// Removes all boxes and derefs wrapping the expression
    pub fn unbox_underef(&self) -> &Expr {
        let mut current = self;
        while let Some(e) = current.unbox_once().or_else(|| current.underef_once()) {
            current = e
        }
        current
    }
}

impl ExprKind {
    /// Creates a `App` node for a standalone function.
    pub fn standalone_fn_app(
        head: impl Into<FnAppHead>,
        generic_args: Vec<GenericValue>,
        args: Vec<Expr>,
        output_type: Ty,
        span: Span,
    ) -> Self {
        Self::fn_app(head, generic_args, args, output_type, vec![], None, span)
    }

    /// Creates a `App` node.
    pub fn fn_app(
        head: impl Into<FnAppHead>,
        generic_args: Vec<GenericValue>,
        args: Vec<Expr>,
        output_type: Ty,
        bounds_impls: Vec<ImplExpr>,
        trait_: Option<(ImplExpr, Vec<GenericValue>)>,
        span: Span,
    ) -> Self {
        let head = 'head: {
            let kind = match head.into() {
                FnAppHead::GlobalId(global_id) => ExprKind::GlobalId(global_id),
                FnAppHead::ExprKind(expr_kind) => expr_kind,
                FnAppHead::Expr(expr) => break 'head expr,
            };
            let head_ty = TyKind::Arrow {
                inputs: args.iter().map(Typed::ty).cloned().collect(),
                output: output_type.clone(),
            }
            .promote();
            kind.promote(head_ty, span)
        };

        Self::App {
            head,
            args,
            generic_args,
            bounds_impls,
            trait_,
        }
    }

    /// Creates a tuple out of a vector of components.
    pub fn tuple(components: Vec<Expr>) -> Self {
        let length = components.len();
        ExprKind::Construct {
            constructor: global_id::TupleId::Constructor { length }.into(),
            is_record: false,
            is_struct: true,
            fields: components
                .into_iter()
                .enumerate()
                .map(|(field, expr)| (global_id::TupleId::Field { length, field }.into(), expr))
                .collect(),
            base: None,
        }
    }

    /// Promote to an `Expr`
    pub fn promote(self, ty: Ty, span: Span) -> Expr {
        Expr {
            kind: Box::new(self),
            ty,
            meta: Metadata {
                span,
                attributes: Vec::new(),
            },
        }
    }
}

impl Metadata {
    /// Get an iterator over hax attributes for this AST fragment.
    pub fn hax_attributes(&self) -> impl Iterator<Item = &hax_lib_macros_types::AttrPayload> {
        crate::attributes::hax_attributes(&self.attributes)
    }
}

impl Pat {
    /// The variable bound by a plain binding pattern `x`, under any
    /// ascriptions.
    pub fn as_binder(&self) -> Option<&LocalId> {
        match &*self.kind {
            PatKind::Binding {
                var, sub_pat: None, ..
            } => Some(var),
            PatKind::Ascription { pat, .. } => pat.as_binder(),
            _ => None,
        }
    }

    /// The pattern `constructor(fields)` of an enum variant.
    pub fn variant(
        constructor: GlobalId,
        fields: Vec<(GlobalId, Pat)>,
        ty: Ty,
        span: Span,
    ) -> Self {
        PatKind::Construct {
            constructor,
            is_record: false,
            is_struct: false,
            fields,
        }
        .promote(ty, span)
    }

    /// The pattern `Some(inner)` of type `ty`.
    pub fn some(inner: Pat, ty: Ty, span: Span) -> Self {
        use crate::names::core::option::Option::Some;
        Self::variant(Some::Constructor, vec![(Some::_0, inner)], ty, span)
    }

    /// The pattern `None` of type `ty`.
    pub fn none(ty: Ty, span: Span) -> Self {
        use crate::names::core::option::Option::None;
        Self::variant(None::Constructor, vec![], ty, span)
    }

    /// Expects the pattern to be a simple binding `self`.
    pub fn expect_self(&self) -> Option<LocalId> {
        if let PatKind::Binding { var, .. } = self.kind()
            && var.is_self()
        {
            Some(var.clone())
        } else {
            None
        }
    }
}

/// Helper enum that describes what can serve as function application heads.
/// This is an helper that is useful for [`ExprKind::fn_application`].
pub enum FnAppHead {
    /// A global identifier
    GlobalId(GlobalId),
    /// An expression kind
    ExprKind(ExprKind),
    /// A full blown expression
    Expr(Expr),
}

impl From<GlobalId> for FnAppHead {
    fn from(value: GlobalId) -> Self {
        Self::GlobalId(value)
    }
}
impl From<ExprKind> for FnAppHead {
    fn from(value: ExprKind) -> Self {
        Self::ExprKind(value)
    }
}
impl From<Expr> for FnAppHead {
    fn from(value: Expr) -> Self {
        Self::Expr(value)
    }
}

impl Generics {
    /// Concatenate two generics
    pub fn concat(mut self, other: Self) -> Self {
        self.constraints.extend(other.constraints);
        self.params.extend(other.params);
        use std::cmp::Ordering;
        self.params.sort_by(|a, b| match (a.kind(), b.kind()) {
            (GenericParamKind::Lifetime, GenericParamKind::Lifetime) => Ordering::Equal,
            (GenericParamKind::Lifetime, _) => Ordering::Less,
            (_, GenericParamKind::Lifetime) => Ordering::Greater,
            _ => Ordering::Equal,
        });
        self
    }
    /// Empty generics
    pub fn empty() -> Self {
        Self {
            params: Vec::new(),
            constraints: Vec::new(),
        }
    }
}

impl Item {
    /// Returns a `LocalId` named `self` if the item is a standalone function
    /// whose first argument is the keyword `self`. In other words, this
    /// function returns a local identifier only for associated methods from
    /// inherent `impl` blocks.
    pub fn self_id(&self) -> Option<LocalId> {
        if let ItemKind::Fn { params, .. } = self.kind()
            && let [first, ..] = &params[..]
            && let Some(self_id) = first.pat.expect_self()
        {
            Some(self_id.clone())
        } else {
            None
        }
    }
}

impl ItemKind {
    /// Promote to an item
    pub fn promote(self, ident: GlobalId, span: Span) -> Item {
        Item {
            ident,
            kind: self,
            meta: Metadata {
                span,
                attributes: Vec::new(),
            },
        }
    }
}

impl GenericValue {
    /// Tries to extract a [`Ty`] out of a [`GenericValue`].
    pub fn expect_ty(&self) -> Option<&Ty> {
        let Self::Ty(ty) = self else { return None };
        Some(ty)
    }
}

impl TyKind {
    /// Tuple type
    pub fn tuple(args: Vec<GenericValue>) -> Self {
        let head = global_id::TupleId::Type { length: args.len() }.into();
        Self::App { head, args }
    }
    /// Unit type
    pub fn unit() -> Self {
        Self::tuple(Vec::new())
    }
    /// Promote to a Ty
    pub fn promote(self) -> Ty {
        Ty(Box::new(self))
    }
}

impl Arm {
    /// Create a non-guarded arm
    pub fn non_guarded(pat: Pat, body: Expr, span: Span) -> Self {
        Self {
            pat,
            body,
            guard: None,
            meta: Metadata {
                span,
                attributes: Vec::new(),
            },
        }
    }
}

impl PatKind {
    /// Pattern for binding to a single variable
    pub fn var_pat(var: LocalId) -> Self {
        Self::Binding {
            mutable: false,
            var,
            mode: BindingMode::ByValue,
            sub_pat: None,
        }
    }
    /// Promote to a `Pat`
    pub fn promote(self, ty: Ty, span: Span) -> Pat {
        Pat {
            kind: Box::new(self),
            ty,
            meta: Metadata {
                span,
                attributes: Vec::new(),
            },
        }
    }
}

impl Variant {
    /// Whether a variant has fields or not.
    /// See https://doc.rust-lang.org/reference/items/enumerations.html#field-less-enum.
    pub fn is_fieldless(&self) -> bool {
        self.arguments.is_empty()
    }
}
