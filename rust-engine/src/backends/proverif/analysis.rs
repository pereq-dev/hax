//! Facts about the whole crate that the ProVerif phases and printer consult.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use super::*;

/// A function or method declaration.
pub(crate) struct FnDecl<'a> {
    pub(crate) name: GlobalId,
    pub(crate) meta: &'a Metadata,
    pub(crate) params: &'a [Param],
    pub(crate) body: &'a Expr,
}

/// Whether the printer omits the body of the function with these `meta` and
/// `params`: opaque, `pv_constructor` and `pv_handwritten` functions. A
/// constant keeps its body.
pub(crate) fn hides_body(meta: &Metadata, params: &[Param]) -> bool {
    !params.is_empty()
        && meta.hax_attributes().any(|a| {
            matches!(
                a,
                AttrPayload::Erased | AttrPayload::PVConstructor | AttrPayload::PVHandwritten
            )
        })
}

/// The text of `quote`, with the global names it embeds (`${path}`) rendered,
/// so that the names it declares can be scanned. Other embedded fragments
/// stand as `_`.
pub(crate) fn quote_text(quote: &Quote) -> String {
    quote
        .0
        .iter()
        .map(|content| match content {
            QuoteContent::Verbatim(text) => text.clone(),
            QuoteContent::Expr(e) => match &*e.kind {
                ExprKind::GlobalId(g) => render_id(g),
                _ => "_".into(),
            },
            _ => "_".into(),
        })
        .collect()
}

/// Applies `item` and `impl_item` to what the printer emits of `items`: not
/// the late-skipped items, the trait declarations or the hidden bodies (see
/// [`hides_body`]). An impl is visited item by item.
pub(crate) fn visit_emitted(
    items: &mut [Item],
    mut item: impl FnMut(&mut Item),
    mut impl_item: impl FnMut(&mut ImplItem),
) {
    for it in items.iter_mut() {
        if crate::backends::is_late_skip(&it.meta) {
            continue;
        }
        match &mut it.kind {
            ItemKind::Trait { .. } => {}
            ItemKind::Fn { params, .. } if hides_body(&it.meta, params) => {}
            ItemKind::Impl { items, .. } => {
                for ii in items.iter_mut() {
                    match &ii.kind {
                        _ if crate::backends::is_late_skip(&ii.meta) => {}
                        ImplItemKind::Fn { params, .. } if hides_body(&ii.meta, params) => {}
                        _ => impl_item(ii),
                    }
                }
            }
            _ => item(it),
        }
    }
}

/// The function items and impl methods of `items`.
pub(crate) fn fn_decls(items: &[Item]) -> impl Iterator<Item = FnDecl<'_>> {
    items.iter().flat_map(|item| {
        let decls: Vec<FnDecl<'_>> = match &item.kind {
            ItemKind::Fn {
                name, params, body, ..
            } => vec![FnDecl {
                name: *name,
                meta: &item.meta,
                params,
                body,
            }],
            ItemKind::Impl { items, .. } => items
                .iter()
                .filter_map(|ii| match &ii.kind {
                    ImplItemKind::Fn { params, body } => Some(FnDecl {
                        name: ii.ident,
                        meta: &ii.meta,
                        params,
                        body,
                    }),
                    _ => None,
                })
                .collect(),
            _ => vec![],
        };
        decls
    })
}

/// How a name is used across the crate.
#[derive(Default)]
pub(crate) struct RefInfo {
    /// The maximum arity at a use site.
    pub(crate) arity: usize,
    /// Whether it is used as a constructor, so it must be declared `[data]`.
    pub(crate) is_constructor: bool,
    /// Whether it is applied somewhere (`f(..)`, possibly `f()`), which
    /// requires a `fun`/`letfun`: a `const` cannot be applied, not even as
    /// `f()`.
    pub(crate) applied: bool,
}

/// Facts about a crate. Names are compared rendered: ProVerif has one global
/// namespace, so two `GlobalId`s that render the same are one identifier.
#[derive(Default)]
pub(crate) struct Analysis {
    /// Const-generic parameters, rendered as `bitstring_default()`: ProVerif
    /// has no generic parameters.
    pub(crate) erased_const_generics: HashSet<LocalId>,
    /// How each name used in an expression or a pattern is used.
    pub(crate) references: HashMap<String, RefInfo>,
    /// Declaration → the names its body uses.
    /// The names the verbatim ProVerif text of the crate declares.
    pub(crate) quote_declared: HashSet<String>,
    pub(crate) calls: HashMap<String, HashSet<String>>,
    /// Name → the declarations using it, sorted.
    pub(crate) reference_sources: HashMap<String, Vec<String>>,
    /// Functions in a call-graph cycle. ProVerif `letfun`s are non-recursive,
    /// so these are rejected and opacified to uninterpreted `fun`s.
    pub(crate) recursive_fns: HashSet<String>,
    /// Nullary items (Rust `const`s, associated consts, no-argument `fn`s)
    /// whose body is printable, emitted as `letfun NAME() = <body>.`. Every
    /// reference to one is applied (`NAME()`): ProVerif has no unapplied
    /// letfuns.
    pub(crate) nullary_letfuns: HashSet<String>,
    /// Field-projection accessor → the constructor its `reduc` reduces on.
    pub(crate) accessor_owner: HashMap<String, String>,
    /// Every name declared as a `[data]` constructor: struct/enum variants,
    /// and the functions `#[hax_lib::opaque]` / `#[hax_lib::pv_constructor]`
    /// turn into free constructors.
    pub(crate) data_constructors: HashSet<String>,
    /// Nullary items that render as an opaque constant, with no constructor.
    pub(crate) opaque_consts: HashSet<String>,
    /// Function → the constructor skeletons of the patterns destructuring its
    /// results, sorted.
    pub(crate) result_shapes: HashMap<String, Vec<String>>,
}

impl Analysis {
    pub(crate) fn new(items: &[Item]) -> Self {
        let render = |id: &GlobalId| render_id(id);
        let mut analysis = Self::default();

        // References, by the declaration the printer emits for them: a
        // top-level item by its own name, an impl by each of its items.
        let mut record = |source: String, collector: ExternRefCollector| {
            let calls = analysis.calls.entry(source.clone()).or_default();
            for r in collector.refs {
                let name = render(&r.id);
                let info = analysis.references.entry(name.clone()).or_default();
                info.arity = info.arity.max(r.arity);
                info.is_constructor |= r.is_constructor;
                info.applied |= r.applied;
                calls.insert(name);
            }
            for name in collector.lit_consts {
                analysis.references.entry(name.clone()).or_default();
                calls.insert(name);
            }
            calls.extend(collector.quoted);
            analysis.quote_declared.extend(collector.quote_declared);
            for (name, arity) in collector.unextracted {
                let info = analysis.references.entry(name.clone()).or_default();
                info.arity = info.arity.max(arity);
                info.is_constructor = true;
                info.applied = true;
                calls.insert(name);
            }
        };
        for item in items {
            match &item.kind {
                ItemKind::Impl { items, .. } => {
                    for ii in items {
                        let mut collector = ExternRefCollector::default();
                        match &ii.kind {
                            ImplItemKind::Fn { params, .. } if hides_body(&ii.meta, params) => {
                                params.iter().for_each(|p| collector.visit_param(p))
                            }
                            _ => collector.visit_impl_item(ii),
                        }
                        record(render(&ii.ident), collector);
                    }
                }
                ItemKind::Fn { params, .. } if hides_body(&item.meta, params) => {
                    let mut collector = ExternRefCollector::default();
                    params.iter().for_each(|p| collector.visit_param(p));
                    record(render(&item.ident), collector);
                }
                _ => {
                    let mut collector = ExternRefCollector::default();
                    collector.visit_item(item);
                    record(render(&item.ident), collector);
                }
            }
        }
        let mut sources: HashMap<String, Vec<String>> = HashMap::new();
        for (source, names) in &analysis.calls {
            for name in names {
                sources
                    .entry(name.clone())
                    .or_default()
                    .push(source.clone());
            }
        }
        for users in sources.values_mut() {
            users.sort();
        }
        analysis.reference_sources = sources;

        for item in items {
            let generics = match &item.kind {
                ItemKind::Fn { generics, .. }
                | ItemKind::Impl { generics, .. }
                | ItemKind::Trait { generics, .. }
                | ItemKind::Type { generics, .. } => Some(generics),
                _ => None,
            };
            let impl_generics = match &item.kind {
                ItemKind::Impl { items, .. } => items.iter().map(|ii| &ii.generics).collect(),
                _ => vec![],
            };
            for generics in generics.into_iter().chain(impl_generics) {
                for p in &generics.params {
                    if matches!(p.kind, GenericParamKind::Const { .. }) {
                        analysis.erased_const_generics.insert(p.ident.clone());
                    }
                }
            }
        }

        // A function without parameters is a Rust constant, which cannot
        // recurse; a Rust function without parameters takes `()`.
        let fns: HashSet<String> = fn_decls(items)
            .filter(|decl| !decl.params.is_empty())
            .map(|decl| render(&decl.name))
            .collect();
        analysis.recursive_fns = fns
            .iter()
            .filter(|f| analysis.reaches(f, f, &fns))
            .cloned()
            .collect();

        for decl in fn_decls(items) {
            let name = render(&decl.name);
            if decl.params.is_empty()
                && !body_is_hidden(decl.meta)
                && analysis.applied_fn_arity(&name).unwrap_or(0) == 0
                && nullary_body_is_printable(decl.body)
            {
                analysis.nullary_letfuns.insert(name);
            }
        }

        for item in items {
            if let ItemKind::Type { variants, .. } = &item.kind {
                for v in variants {
                    let constructor = render(&v.name);
                    for (field, _, _) in &v.arguments {
                        analysis
                            .accessor_owner
                            .insert(render(field), constructor.clone());
                    }
                    analysis.data_constructors.insert(constructor);
                }
            }
        }
        for decl in fn_decls(items) {
            let name = render(&decl.name);
            let is_free_constructor = decl
                .meta
                .hax_attributes()
                .any(|a| matches!(a, AttrPayload::Erased | AttrPayload::PVConstructor));
            if is_free_constructor {
                analysis.data_constructors.insert(name);
            } else if decl.params.is_empty()
                && !analysis.nullary_letfuns.contains(&name)
                && analysis.applied_fn_arity(&name).unwrap_or(0) == 0
            {
                // An atom with no constructor, whether it is declared as a
                // `const` or, applied as `NAME()`, as a nullary `fun`.
                analysis.opaque_consts.insert(name);
            }
        }

        let mut shapes = ResultShapes::default();
        for item in items {
            shapes.visit_item(item);
        }
        analysis.result_shapes = shapes
            .shapes
            .into_iter()
            .map(|(name, set)| {
                let mut shapes: Vec<String> = set.into_iter().collect();
                shapes.sort();
                (name, shapes)
            })
            .collect();
        analysis
    }

    /// Whether `from` reaches `to` through one call or more, following only
    /// calls to `within`.
    fn reaches(&self, from: &str, to: &str, within: &HashSet<String>) -> bool {
        let mut stack: Vec<&String> = self.callees(from, within).collect();
        let mut seen: HashSet<&String> = HashSet::new();
        while let Some(n) = stack.pop() {
            if n == to {
                return true;
            }
            if seen.insert(n) {
                stack.extend(self.callees(n, within));
            }
        }
        false
    }

    /// The names of `within` that the declaration `name` uses.
    pub(crate) fn callees<'a>(
        &'a self,
        name: &str,
        within: &'a HashSet<String>,
    ) -> impl Iterator<Item = &'a String> {
        self.calls
            .get(name)
            .into_iter()
            .flatten()
            .filter(move |callee| within.contains(*callee))
    }

    /// The maximum arity of the uses of `name`.
    pub(crate) fn arity(&self, name: &str) -> usize {
        self.references.get(name).map_or(0, |r| r.arity)
    }

    /// The arity of the applications of `name`, if it is applied.
    pub(crate) fn applied_fn_arity(&self, name: &str) -> Option<usize> {
        self.references
            .get(name)
            .filter(|r| r.applied)
            .map(|r| r.arity)
    }

    /// Whether evaluating `e` can never fail.
    pub(crate) fn cannot_fail(&self, e: &Expr) -> bool {
        match &*e.kind {
            ExprKind::LocalId(_) | ExprKind::Literal(_) => true,
            // A nullary item that kept its value renders as the call `NAME()`.
            ExprKind::GlobalId(g) => !self.nullary_letfuns.contains(&render_id(g)),
            ExprKind::Ascription { e, .. } => self.cannot_fail(e),
            _ => false,
        }
    }

    /// Why the projection `accessor(e)` can never reduce, if it cannot.
    ///
    /// A `reduc` destructor reduces on exactly one constructor, so a
    /// projection is dead when its argument provably has a different head: an
    /// opaque constant, which has no constructor at all, or an application of
    /// some *other* `[data]` constructor.
    pub(crate) fn dead_projection_reason(&self, accessor: &str, e: &Expr) -> Option<String> {
        let owner = self.accessor_owner.get(accessor)?;
        let head = match &*e.kind {
            ExprKind::App { head, .. } => head,
            _ => e,
        };
        let ExprKind::GlobalId(g) = &*head.kind else {
            return None;
        };
        let head_name = render_id(g);
        if self.opaque_consts.contains(&head_name) {
            return Some(format!(
                "is applied to `{head_name}`, an opaque constant with no constructor"
            ));
        }
        if self.data_constructors.contains(&head_name) && head_name != *owner {
            return Some(format!(
                "reduces only on `{owner}`, but is applied to a `{head_name}`"
            ));
        }
        None
    }
}

/// Order `items`, each declaring the ProVerif symbols `names` gives, so that
/// every referenced sibling is declared before its user: ProVerif is
/// single-pass, but Rust items may use each other in any source order.
///
/// The sort is stable, so items keep their source order unless a dependency
/// forces a move. The members of a recursive cycle are opacified to `fun`s
/// that reference nothing, so they have no dependency.
pub(crate) fn topo_sort<'a, T>(
    items: &'a [T],
    names: impl Fn(&T) -> Vec<String>,
    analysis: &Analysis,
) -> Vec<&'a T> {
    let names: Vec<Vec<String>> = items.iter().map(names).collect();
    let siblings: HashSet<String> = names.iter().flatten().cloned().collect();
    let index: HashMap<&String, usize> = names
        .iter()
        .enumerate()
        .flat_map(|(i, ns)| ns.iter().map(move |n| (n, i)))
        .collect();
    // `pending[i]`: the siblings item `i` uses that are not emitted yet;
    // `users[j]`: the items that use sibling `j`.
    let mut pending = vec![0usize; items.len()];
    let mut users: Vec<Vec<usize>> = vec![Vec::new(); items.len()];
    for (i, ns) in names.iter().enumerate() {
        for name in ns {
            if analysis.recursive_fns.contains(name) {
                continue;
            }
            for j in analysis
                .callees(name, &siblings)
                .map(|callee| index[callee])
            {
                if j != i {
                    pending[i] += 1;
                    users[j].push(i);
                }
            }
        }
    }
    // Kahn's algorithm, always taking the earliest ready item in source order.
    let mut ready: BinaryHeap<Reverse<usize>> = (0..items.len())
        .filter(|&i| pending[i] == 0)
        .map(Reverse)
        .collect();
    let mut order = Vec::with_capacity(items.len());
    while let Some(Reverse(i)) = ready.pop() {
        order.push(i);
        for &k in &users[i] {
            pending[k] -= 1;
            if pending[k] == 0 {
                ready.push(Reverse(k));
            }
        }
    }
    // Every cycle goes through a recursive item, so nothing is left here;
    // appending the rest just ensures no item is lost.
    let emitted: HashSet<usize> = order.iter().copied().collect();
    order.extend((0..items.len()).filter(|i| !emitted.contains(i)));
    order.into_iter().map(|i| &items[i]).collect()
}

/// The messages reporting any two items whose ProVerif names collide.
///
/// `escape` is not injective and `__` path joining is ambiguous, so two
/// distinct Rust items can render to one ProVerif symbol, which the model
/// would silently identify.
pub(crate) fn name_collisions(items: &[Item]) -> Vec<String> {
    let mut seen: HashMap<String, String> = HashMap::new();
    let mut collisions = Vec::new();
    let mut check = |id: &GlobalId| {
        let rendered = render_id(id);
        let source = id.to_debug_string();
        match seen.get(&rendered) {
            Some(previous) if *previous != source => collisions.push(format!(
                "`{previous}` and `{source}` both render to the ProVerif name \
                 `{rendered}`; the model would identify them"
            )),
            Some(_) => {}
            None => {
                seen.insert(rendered, source);
            }
        }
    };
    for item in items {
        match &item.kind {
            ItemKind::Fn { name, .. } => check(name),
            ItemKind::Type { name, variants, .. } => {
                check(name);
                for v in variants {
                    check(&v.name);
                    for (field, _, _) in &v.arguments {
                        check(field);
                    }
                }
            }
            ItemKind::Impl { items, .. } => {
                for ii in items {
                    check(&ii.ident);
                }
            }
            _ => {}
        }
    }
    collisions
}

/// Whether a nullary item's body can be printed as a `letfun` body, so the
/// constant keeps its value in the model instead of becoming an opaque atom.
/// A body containing an `Error` node, or the `dropped_body` marker of a body
/// that was never imported, keeps the opaque form instead.
fn nullary_body_is_printable(body: &Expr) -> bool {
    !has_error(body) && !matches!(&*body.kind, ExprKind::GlobalId(g) if *g == names::dropped_body)
}

/// Whether the item's body is not printed: `#[hax_lib::opaque]`,
/// `#[hax_lib::pv_handwritten]` (or `--assume-items`) and
/// `#[hax_lib::pv_constructor]` all say "do not use this body".
pub(crate) fn body_is_hidden(meta: &Metadata) -> bool {
    meta.hax_attributes().any(|a| {
        matches!(
            a,
            AttrPayload::Erased | AttrPayload::PVHandwritten | AttrPayload::PVConstructor
        )
    })
}

/// Whether `e` contains an `Error` node: a fragment the frontend or an earlier
/// phase could not translate.
pub(crate) fn has_error(e: &Expr) -> bool {
    struct Finder(bool);
    impl AstVisitor for Finder {
        fn enter_expr_kind(&mut self, kind: &ExprKind) {
            self.0 |= matches!(kind, ExprKind::Error(_));
        }
    }
    let mut finder = Finder(false);
    finder.visit_expr(e);
    finder.0
}

/// For each function whose result is destructured at a call site (`let PAT =
/// f(..)` or `match f(..)`, possibly through `let x = f(..)`), the constructor
/// skeletons of those patterns. Recorded against a symbol declared in
/// `missingdecl.pvl`, this is the result shape a hand-written definition must
/// produce.
#[derive(Default)]
struct ResultShapes {
    shapes: HashMap<String, HashSet<String>>,
    /// Locals of the current function bound to the result of a call, and the
    /// function called.
    results: HashMap<LocalId, String>,
}

impl ResultShapes {
    fn callee(&self, e: &Expr) -> Option<String> {
        match &*e.kind {
            ExprKind::App { head, .. } => match &*head.kind {
                ExprKind::GlobalId(g) => Some(render_id(g)),
                _ => None,
            },
            ExprKind::LocalId(x) => self.results.get(x).cloned(),
            _ => None,
        }
    }

    fn record(&mut self, e: &Expr, pat: &Pat) {
        if let Some(callee) = self.callee(e) {
            let shape = shape_of_pat(pat);
            // A trivial destructure carries no information.
            if shape != "_" {
                self.shapes.entry(callee).or_default().insert(shape);
            }
        }
    }
}

impl AstVisitor for ResultShapes {
    // Locals are unique within a function only.
    fn enter_item(&mut self, _: &Item) {
        self.results.clear();
    }
    fn enter_impl_item(&mut self, _: &ImplItem) {
        self.results.clear();
    }

    fn enter_expr_kind(&mut self, kind: &ExprKind) {
        match kind {
            ExprKind::Let { lhs, rhs, .. } => {
                if let (Some(x), ExprKind::App { .. }) = (lhs.as_binder(), &*rhs.kind)
                    && let Some(callee) = self.callee(rhs)
                {
                    self.results.insert(x.clone(), callee);
                }
                self.record(rhs, lhs)
            }
            ExprKind::Match { scrutinee, arms } => {
                for arm in arms {
                    self.record(scrutinee, &arm.pat);
                }
            }
            _ => {}
        }
    }
}

/// The constructor skeleton of a pattern: each `Construct` becomes
/// `Name(<inner>, ...)` and every binder, wildcard, or constant becomes `_`.
/// `Tuple2(a, Tuple2(b, c))` yields `Tuple2(_, Tuple2(_, _))`.
fn shape_of_pat(pat: &Pat) -> String {
    match &*pat.kind {
        PatKind::Construct {
            constructor,
            fields,
            ..
        } => {
            let inner: Vec<String> = fields.iter().map(|(_, p)| shape_of_pat(p)).collect();
            format!("{}({})", render_id(constructor), inner.join(", "))
        }
        PatKind::Ascription { pat, .. } => shape_of_pat(pat),
        _ => "_".to_string(),
    }
}

/// Visitor recording every `GlobalId` used in expression or pattern position,
/// with the arity at the use site and whether the use requires a `[data]`
/// constructor.
#[derive(Default)]
struct ExternRefCollector {
    refs: Vec<Ref>,
    /// The constants standing for literals, which need a declaration.
    lit_consts: Vec<String>,
    /// The identifiers of verbatim ProVerif text, which may name a symbol
    /// that another item declares.
    quoted: Vec<String>,
    /// The opaque constructors standing for methods of impls that are not
    /// extracted, with their arity.
    unextracted: Vec<(String, usize)>,
    /// The names verbatim ProVerif text declares.
    quote_declared: Vec<String>,
}

struct Ref {
    id: GlobalId,
    arity: usize,
    is_constructor: bool,
    applied: bool,
}

impl ExternRefCollector {
    fn push(&mut self, id: GlobalId, arity: usize, is_constructor: bool, applied: bool) {
        self.refs.push(Ref {
            id,
            arity,
            is_constructor,
            applied,
        });
    }
}

impl AstVisitor for ExternRefCollector {
    fn visit_expr(&mut self, e: &Expr) {
        if let ExprKind::App { head, args, .. } = &*e.kind
            && let ExprKind::GlobalId(g) = &*head.kind
            && is_unextracted_call(&e.meta)
        {
            self.unextracted
                .push((format!("{}{UNEXTRACTED_SUFFIX}", render_id(g)), args.len()));
            args.iter().for_each(|arg| self.visit_expr(arg));
            return;
        }
        self.visit_inner(e)
    }

    fn visit_quote(&mut self, quote: &Quote) {
        let text = quote_text(quote);
        self.quote_declared.extend(scan_declared_names(&text));
        self.quoted.extend(
            text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .filter(|word| word.starts_with(|c: char| c.is_ascii_alphabetic()))
                .map(str::to_string),
        );
        // A name spliced in as the prefix of a longer identifier, such as
        // `${f}__inverse`, does not refer to the item.
        for (i, content) in quote.0.iter().enumerate() {
            let glued = matches!(quote.0.get(i + 1), Some(QuoteContent::Verbatim(next))
                if next.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_'));
            match content {
                QuoteContent::Expr(e) if glued && matches!(&*e.kind, ExprKind::GlobalId(_)) => {}
                QuoteContent::Expr(e) => self.visit_expr(e),
                QuoteContent::Pattern(p) => self.visit_pat(p),
                QuoteContent::Ty(t) => self.visit_ty(t),
                QuoteContent::Verbatim(_) => {}
            }
        }
    }

    fn enter_expr_kind(&mut self, kind: &ExprKind) {
        match kind {
            ExprKind::App { head, args, .. } => {
                if let ExprKind::GlobalId(g) = &*head.kind {
                    self.push(*g, args.len(), false, true);
                }
            }
            ExprKind::GlobalId(g) => self.push(*g, 0, false, false),
            ExprKind::Construct {
                constructor,
                fields,
                ..
            } => self.push(*constructor, fields.len(), true, false),
            ExprKind::Literal(lit) => self.lit_consts.extend(lit_const_name(lit)),
            _ => {}
        }
    }

    fn enter_pat_kind(&mut self, kind: &PatKind) {
        match kind {
            // A field-less struct pattern is printed as a binder.
            PatKind::Construct {
                constructor,
                fields,
                is_struct,
                ..
            } if !(*is_struct && fields.is_empty()) => {
                self.push(*constructor, fields.len(), true, false)
            }
            // A constant pattern references the same opaque literal constant
            // an expression would.
            PatKind::Constant { lit } => self.lit_consts.extend(lit_const_name(lit)),
            _ => {}
        }
    }
}

/// Every name that a `fun`, `letfun`, `const` or `reduc` declaration of the
/// ProVerif `text` introduces.
pub(crate) fn scan_declared_names(text: &str) -> Vec<String> {
    scan_declarations(text)
        .into_iter()
        .map(|(_, name)| name)
        .collect()
}

/// Every `fun`, `letfun`, `const` or `reduc` declaration of the ProVerif
/// `text`, as its keyword and the name it introduces.
pub(crate) fn scan_declarations(text: &str) -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    for line in text.lines().map(str::trim_start) {
        for kw in ["fun", "letfun", "const"] {
            if let Some(rest) = line.strip_prefix(kw).and_then(|r| r.strip_prefix(' ')) {
                let end = rest.find(['(', ':', ' ', '\t']).unwrap_or(rest.len());
                if end > 0 {
                    out.push((kw, rest[..end].to_string()));
                }
            }
        }
    }
    // A `reduc forall …; NAME(…) = …` header may span several lines.
    let mut rest = text;
    while let Some(i) = rest.find("reduc") {
        let starts_line = rest[..i]
            .rsplit('\n')
            .next()
            .is_none_or(|indent| indent.trim().is_empty());
        rest = &rest[i + "reduc".len()..];
        if !starts_line || !rest.trim_start().starts_with("forall") {
            continue;
        }
        let Some((_, tail)) = rest.split_once(';') else {
            break;
        };
        let head = tail.trim_start();
        let end = head
            .find(|c: char| c == '(' || c.is_whitespace())
            .unwrap_or(head.len());
        if end > 0 {
            out.push(("reduc", head[..end].to_string()));
        }
        rest = tail;
    }
    out
}
