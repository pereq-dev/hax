//! The Lean backend
//!
//! This module defines the trait implementations to export the rust ast to
//! Pretty::Doc type, which can in turn be exported to string (or, eventually,
//! source maps).

use std::collections::HashSet;
use std::sync::OnceLock;

use super::prelude::*;
use crate::{
    ast::{
        identifiers::global_id::view::{ConstructorKind, PathSegment, TypeDefKind},
        span::Span,
    },
    attributes::hax_proof_attributes,
    names::rust_primitives::hax::{
        cast_op,
        explicit_monadic::{lift, pure},
    },
    phase::*,
};
use camino::Utf8PathBuf;
use hax_lib_macros_types::ProofMethod;
use hax_types::engine_api::File;

mod binops {
    pub use crate::names::core::cmp::PartialEq;
    pub use crate::names::core::ops::bit::*;
    pub use crate::names::core::ops::index::*;
    pub use crate::names::rust_primitives::arithmetic::neg;
    pub use crate::names::rust_primitives::hax::machine_int::*;
    pub use crate::names::rust_primitives::hax::{logical_op_and, logical_op_or};
}

const LIFT: GlobalId = lift;
const PURE: GlobalId = pure;
const CAST_OP: GlobalId = cast_op;

/// The Lean printer
#[setup_printer_struct]
#[derive(Default, Clone)]
pub struct LeanPrinter {
    current_namespace: Option<GlobalId>,
}

const INDENT: isize = 2;

const HEADER: &str = "
-- Legacy lean backend for Hax
-- The Hax prelude library can be found in hax/proof-libs/legacy-lean
import Hax
import Std.Tactic.Do
import Std.Do.Triple
import Std.Tactic.Do.Syntax
open Std.Do
open Std.Tactic

set_option mvcgen.warning false
set_option linter.unusedVariables false


";

impl RenderView for LeanPrinter {
    fn reserved_keywords() -> &'static HashSet<String> {
        static SET: OnceLock<HashSet<String>> = OnceLock::new();
        SET.get_or_init(|| {
            [
                // reserved for Lean:
                "end",
                "def",
                "abbrev",
                "theorem",
                "example",
                "inductive",
                "structure",
                "from",
                // reserved for hax encoding:
                "associatedTypes",
                "AssociatedTypes",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
        })
    }

    fn should_escape(id: &str) -> bool {
        Self::is_reserved_keyword(id)
            || id.starts_with(|c: char| c.is_ascii_digit())
            || id.starts_with("trait_constr_")
    }

    fn separator(&self) -> &str {
        "."
    }

    fn relativize_module_path<'a>(&self, module_path: &'a [PathSegment]) -> &'a [PathSegment] {
        if let Some(namespace) = self.current_namespace
            && namespace.view().segments() == module_path
        {
            &[]
        } else {
            module_path
        }
    }

    fn render_path_segment(&self, chunk: &PathSegment) -> Vec<String> {
        // Returning None indicates that the default rendering should be used
        (match chunk.kind() {
            AnyKind::Constructor(ConstructorKind::Constructor { ty })
                if matches!(ty.kind(), TypeDefKind::Struct) =>
            {
                Some(vec![
                    Self::escape(&self.render_path_segment_payload(chunk.payload())),
                    "mk".to_string(),
                ])
            }
            AnyKind::Field { named: _, parent } => match parent.kind() {
                ConstructorKind::Constructor { ty }
                    if matches!(&ty.kind(), TypeDefKind::Struct) =>
                {
                    chunk.parent().map(|parent| {
                        vec![
                            Self::escape(&self.render_path_segment_payload(parent.payload())),
                            Self::escape(&self.render_path_segment_payload(chunk.payload())),
                        ]
                    })
                }
                _ => None,
            },
            _ => None,
        })
        .unwrap_or(default::render_path_segment(self, chunk))
    }
}

impl Printer for LeanPrinter {}

/// The Lean backend
pub struct LeanBackend;

impl Backend for LeanBackend {
    type Printer = LeanPrinter;

    fn module_path(&self, module: &Module) -> Utf8PathBuf {
        let krate = module.ident.krate();
        Utf8PathBuf::from(krate).with_extension("lean")
    }

    fn items_to_module(&self, items: Vec<Item>) -> Vec<Module> {
        let mut modules: Vec<Module> = Vec::new();

        for item in items {
            let module_ident = item.ident.mod_only_closest_parent();

            if let Some(last_module) = modules.last_mut()
                && last_module.ident == module_ident
            {
                last_module.items.push(item);
            } else {
                modules.push(Module {
                    ident: module_ident,
                    items: vec![item],
                    meta: Metadata {
                        span: Span::dummy(),
                        attributes: vec![],
                    },
                });
            }
        }
        modules
    }

    fn modules_to_files(&self, modules: Vec<Module>, mut printer: Self::Printer) -> Vec<File> {
        if modules.is_empty() {
            return vec![];
        }
        let path = self.module_path(modules.first().unwrap()).to_string();
        let contents = modules
            .into_iter()
            .map(|module: Module| {
                let (c, _) = printer.print(module);
                c
            })
            .collect::<Vec<String>>()
            .join("\n");
        vec![File {
            path,
            contents: format!("{}{}", HEADER, contents),
            sourcemap: None,
        }]
    }

    fn phases(&self) -> Vec<PhaseKind> {
        use crate::phase::{PhaseKind::*, legacy::LegacyOCamlPhase::*};
        vec![
            RejectRawOrMutPointer.into(),
            RejectImplTypeMethod.into(),
            RewriteLocalSelf.into(),
            TransformHaxLibInline.into(),
            Specialize.into(),
            DropSizedTrait.into(),
            SimplifyQuestionMarks.into(),
            AndMutDefsite.into(),
            ReconstructAsserts.into(),
            ReconstructForLoops.into(),
            ReconstructWhileLoops.into(),
            DirectAndMut.into(),
            RejectArbitraryLhs.into(),
            DropBlocks.into(),
            DropMatchGuards.into(),
            DropReferences.into(),
            TrivializeAssignLhs.into(),
            HoistSideEffects.into(),
            HoistDisjunctivePatterns.into(),
            SimplifyMatchReturn.into(),
            LocalMutation.into(),
            RewriteControlFlow.into(),
            DropReturnBreakContinue.into(),
            FunctionalizeLoops.into(),
            RejectQuestionMark.into(),
            TraitsSpecs.into(),
            SimplifyHoisting.into(),
            NewtypeAsRefinement.into(),
            ReorderFields.into(),
            HoistAssociatedFns,
            SortItems.into(),
            FilterUnprintableItems,
            ExplicitMonadic,
        ]
    }

    fn resugaring_phases() -> Vec<Box<dyn Resugaring>> {
        vec![
            Box::new(RecursiveFunctions),
            Box::new(FunctionsToConstants),
            Box::new(LetPure),
            Box::new(RecordEllipsis),
        ]
    }
}

impl LeanPrinter {
    /// Checks if we are extracting core models to be able to use different namespeacing when
    /// referring to core.
    pub fn is_hax_core_models_extraction_mode(&self) -> bool {
        std::env::var("HAX_CORE_MODELS_EXTRACTION_MODE")
            .map(|v| v == "on")
            .unwrap_or(false)
    }

    /// Render a global id using the Rendering strategy of the Lean printer. Works for both concrete
    /// and projector ids. TODO: https://github.com/pereq-dev/hax/issues/1660
    pub fn render_id(&self, id: &GlobalId) -> String {
        let id = if !self.is_hax_core_models_extraction_mode() && id.krate() == "core" {
            id.rename_krate("core_models")
        } else {
            *id
        };
        self.render_string(&id.view())
    }

    /// Renders the last, most local part of an id. Used for named arguments of constructors.
    pub fn render_last(&self, id: &GlobalId) -> String {
        self.render(&id.view())
            .path
            .last()
            // TODO: Should be ensured by the rendering engine; see
            // https://github.com/pereq-dev/hax/issues/1660
            .expect("Segments should always be non-empty")
            .clone()
    }

    /// Inject an identifier in before-last position while rendering
    /// TODO: use `DefIdInner::kind` for this instead (https://github.com/pereq-dev/hax/issues/1877)
    pub fn render_with_injection(&self, id: &GlobalId, injection: &String) -> String {
        let rendered = self.render(&id.view());
        let (last, butlast) = rendered
            .path
            .split_last()
            // TODO: Should be ensured by the rendering engine; see
            // https://github.com/pereq-dev/hax/issues/1660
            .expect("Segments should always be non-empty");
        let path: Vec<String> = butlast
            .iter()
            .chain(std::iter::once(injection))
            .chain(std::iter::once(last))
            .map(String::clone)
            .collect();
        self.rendered_to_string(Rendered {
            module: rendered.module,
            path,
        })
    }

    /// Escape a string for use in Lean string literals.
    /// Handles newlines, quotes, backslashes, and other special characters.
    fn escape_string(&self, s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '"' => result.push_str("\\\""),
                '\'' => result.push_str("\\'"),
                '\\' => result.push_str("\\\\"),
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                '\t' => result.push_str("\\t"),
                c if c.is_ascii_control() => {
                    result.push_str(&format!("\\x{:02x}", c as u8));
                }
                c => result.push(c),
            }
        }
        result
    }
}

/// Render parameters, adding a line after each parameter
impl<A: 'static + Clone> ToDocument<LeanPrinter, A> for Vec<Param> {
    fn to_document(&self, printer: &LeanPrinter) -> DocBuilder<A> {
        printer.params(self)
    }
}

#[prepend_associated_functions_with(install_pretty_helpers!(self: Self))]
const _: () = {
    // Emits a CLI error with a github issue number, and prints "sorry" in the lean output
    macro_rules! emit_error {($($tt:tt)*) => {disambiguated_todo!($($tt)*)};}

    // Insert a new line in a doc (pretty)
    macro_rules! line {($($tt:tt)*) => {disambiguated_line!($($tt)*)};}

    // Concatenate docs (pretty )
    macro_rules! concat {($($tt:tt)*) => {disambiguated_concat!($($tt)*)};}

    // Given an iterable `[A,B, ... , C]` and a separator `S`, create the doc `ASBS...CS`
    macro_rules! zip_right {
        ($a:expr, $sep:expr) => {
            docs![concat!($a.into_iter().map(|a| docs![a, $sep]))]
        };
    }

    // Given an iterable `[A,B, ... , C]` and a separator `S`, create the doc `SASB...SC`
    macro_rules! zip_left {
        ($sep:expr, $a:expr) => {
            docs![concat!($a.into_iter().map(|a| docs![$sep, a]))]
        };
    }

    // Prints a one-line comment
    macro_rules! comment {
        ($e:expr) => {
            docs!["-- ", $e]
        };
    }

    // Extra methods, specific to the LeanPrinter
    impl LeanPrinter {
        /// Prints arguments a variant or constructor of struct, using named or unamed arguments based
        /// on the `is_record` flag. Used for both expressions and patterns
        pub fn arguments<A: 'static + Clone, D>(
            &self,
            fields: &[(GlobalId, D)],
            is_record: &bool,
        ) -> DocBuilder<A>
        where
            D: ToDocument<Self, A>,
        {
            if *is_record {
                self.named_arguments(fields)
            } else {
                self.positional_arguments(fields)
            }
        }

        /// Prints fields of structures (when in braced notation)
        fn struct_fields<A: 'static + Clone, D>(&self, fields: &[(GlobalId, D)]) -> DocBuilder<A>
        where
            D: ToDocument<Self, A>,
        {
            docs![intersperse!(
                fields
                    .iter()
                    .map(|(id, e)| { docs![self.render_last(id), reflow!(" := "), e].group() }),
                docs![",", line!()]
            )]
            .group()
        }
        /// Prints named arguments (record) of a variant or constructor of struct
        fn named_arguments<A: 'static + Clone, D>(&self, fields: &[(GlobalId, D)]) -> DocBuilder<A>
        where
            D: ToDocument<Self, A>,
        {
            docs![zip_left!(
                line!(),
                fields.iter().map(|(id, e)| {
                    docs![self.render_last(id), reflow!(" := "), e]
                        .parens()
                        .group()
                })
            )]
            .group()
        }

        /// Prints positional arguments (tuple) of a variant or constructor of struct
        fn positional_arguments<A: 'static + Clone, D>(
            &self,
            fields: &[(GlobalId, D)],
        ) -> DocBuilder<A>
        where
            D: ToDocument<Self, A>,
        {
            docs![zip_left!(line!(), fields.iter().map(|(_, e)| e))].group()
        }

        /// Prints parameters of functions (items, trait items, impl items)
        fn params<A: 'static + Clone>(&self, params: &Vec<Param>) -> DocBuilder<A> {
            zip_left!(line!(), params)
        }

        /// Print parameters as function arguments
        fn params_as_args<A: 'static + Clone>(&self, params: &[Param]) -> DocBuilder<A> {
            zip_left!(
                line!(),
                params.iter().map(|param| {
                    let Ty(ty_kind) = &param.ty;
                    // We need to print arguments of type `Tuple0` as `⟨⟩` instead of `_`
                    // https://github.com/pereq-dev/hax/issues/1856
                    if let TyKind::App { head, .. } = **ty_kind
                        && let Some(global_id::TupleId::Type { length: 0 }) = head.expect_tuple()
                    {
                        docs!["⟨⟩"]
                    } else {
                        docs![param]
                    }
                })
            )
        }

        /// Renders expressions with an explicit ascription `(e : RustM ty)`. Used for the body of closure, for
        /// numeric literals, etc.
        fn expr_typed_result<A: 'static + Clone>(&self, expr: &Expr) -> DocBuilder<A> {
            docs![
                expr,
                softline!(),
                ":",
                line!(),
                docs!["RustM", line!(), &expr.ty].group()
            ]
            .group()
        }

        fn pat_typed<A: 'static + Clone>(&self, pat: &Pat) -> DocBuilder<A> {
            docs![pat, reflow!(" :"), line!(), &pat.ty].parens().group()
        }

        fn do_block<A: 'static + Clone, D: ToDocument<Self, A>>(&self, body: D) -> DocBuilder<A> {
            docs!["do", line!(), body].group()
        }

        /// Produces a name for a constraint on an trait-level constraint, or an associated
        /// type. The name is obtained by combining the type it applies to and the name of the
        /// constraint (and should be unique)
        fn constraint_name(&self, type_name: &String, constraint: &ImplIdent) -> String {
            format!("trait_constr_{}_{}", type_name, constraint.name)
        }

        /// Renders a named argument for associated types with equality constraints
        /// (aka projections). If there are no equality constraints, returns None.
        fn associated_type_projections<A: 'static + Clone>(
            &self,
            impl_ident: &ImplIdent,
            projections: Vec<DocBuilder<A>>,
        ) -> Option<DocBuilder<A>> {
            (!projections.is_empty()).then_some(
                docs![
                    "(associatedTypes := {",
                    line!(),
                    docs![
                        "show",
                        line!(),
                        impl_ident.goal.trait_,
                        ".AssociatedTypes",
                        zip_left!(line!(), impl_ident.goal.args.iter()),
                    ]
                    .group()
                    .nest(INDENT),
                    line!(),
                    reflow!("by infer_instance"),
                    line!(),
                    docs![
                        "with",
                        line!(),
                        intersperse!(projections, docs![",", line!()]),
                    ]
                    .group()
                    .nest(INDENT),
                    "})"
                ]
                .group()
                .nest(INDENT),
            )
        }

        /// Turns an expression of type `RustM T` into one of type `T` (out of the monad), providing
        /// reflexivity as a proof witness.
        fn monad_extract<A: 'static + Clone>(&self, expr: &Expr) -> DocBuilder<A> {
            if let ExprKind::App { head, args, .. } = expr.kind()
                && let ExprKind::GlobalId(PURE) = head.kind()
                && let [pure_expr] = &args[..]
                && let ExprKind::Literal(_) | ExprKind::GlobalId(_) | ExprKind::LocalId(_) =
                    pure_expr.kind()
            {
                // Pure values are displayed directly. Note that constructors, while pure, may
                // contain sub-expressions that are not, so they must be wrapped in a do-block
                docs![pure_expr]
            } else if let ExprKind::App { head, args, .. } = expr.kind()
                && let ExprKind::GlobalId(PURE) = head.kind()
                && let [pure_expr] = &args[..]
                && let ExprKind::App { args, .. } = pure_expr.kind()
                && args.is_empty()
            {
                // Constants are pure values
                docs![pure_expr]
            } else {
                // All other expressions are wrapped in a do-block, and extracted out of the monad
                docs![
                    "RustM.of_isOk",
                    line!(),
                    self.do_block(expr).parens(),
                    line!(),
                    "(by rfl)"
                ]
                .group()
                .nest(INDENT)
            }
        }

        /// Print trait items, adding trait-level params as extra arguments
        fn trait_item_with_trait_params<A: 'static + Clone>(
            &self,
            trait_generics: &[GenericParam],
            TraitItem {
                meta: _,
                kind,
                generics: item_generics,
                ident,
            }: &TraitItem,
        ) -> DocBuilder<A> {
            {
                let name = self.render_last(ident);
                let trait_generics = zip_left!(
                    softline!(),
                    trait_generics
                        .iter()
                        .map(|GenericParam { ident, .. }| docs![ident].parens())
                );
                docs![match kind {
                    TraitItemKind::Fn(ty) => {
                        docs![
                            name,
                            trait_generics,
                            self.generics(item_generics, &self.render_last(ident)),
                            softline!(),
                            ":",
                            line!(),
                            ty
                        ]
                        .group()
                        .nest(INDENT)
                    }
                    TraitItemKind::Type(_) => {
                        docs![name, softline!(), ":", line!(), "Type"]
                            .group()
                            .nest(INDENT)
                    }
                    TraitItemKind::Default { params, body } => docs![
                        docs![
                            name,
                            trait_generics,
                            self.generics(item_generics, &self.render_last(ident)),
                            zip_left!(line!(), params).group(),
                            softline!(),
                            ":",
                            if params.is_empty() {
                                docs![body.ty, softline!(), reflow!(":=")]
                            } else {
                                docs!["RustM", softline!(), body.ty, softline!(), reflow!(":= do")]
                                    .group()
                            }
                        ]
                        .group(),
                        line!(),
                        if params.is_empty() {
                            self.monad_extract(body)
                        } else {
                            docs![body]
                        },
                    ]
                    .group()
                    .nest(INDENT),
                    TraitItemKind::Resugared(_) => {
                        unreachable!("This backend has no resugaring for trait items")
                    }
                    TraitItemKind::Error(e) => docs![e],
                }]
            }
        }

        // Print generics, using `name` as a prefix for constraint names
        fn generics<A: 'static + Clone>(
            &self,
            generics: &Generics,
            name: &String,
        ) -> DocBuilder<A> {
            docs![
                zip_left!(line!(), &generics.params),
                zip_left!(
                    line!(),
                    generics.type_class_constraints().map(|impl_ident| {
                        let projections = generics
                            .equality_constraints()
                            .filter(|p| !matches!(&*p.impl_.kind, ImplExprKind::LocalBound { id } if *id != impl_ident.name ))
                            .map(|p| {
                                if let ImplExprKind::LocalBound { .. } = &*p.impl_.kind {
                                    docs![p]
                                } else if let ImplExprKind::Parent { .. } = &*p.impl_.kind {
                                    emit_error!(issue 1923, "Unsupported equality constraints on associated types of parent trait")
                                } else {
                                    emit_error!(issue 1924, "Unsupported variant of associated type projection")
                                }
                            })
                            .collect::<Vec<_>>();
                        docs![
                            docs![
                                self.constraint_name(&format!("{}_associated_type", name), impl_ident),
                                reflow!(" : "),
                                impl_ident.goal.trait_,
                                ".AssociatedTypes",
                                concat!(
                                    impl_ident.goal.args.iter().map(|arg| docs![line!(), arg])
                                )
                            ]
                            .brackets()
                            .group()
                            .nest(INDENT),
                            line!(),
                            docs![
                                self.constraint_name(name, impl_ident),
                                reflow!(" : "),
                                impl_ident.goal.trait_,
                                concat!(
                                    impl_ident.goal.args.iter().map(|arg| docs![line!(), arg])
                                ),
                                line!(),
                                self.associated_type_projections(impl_ident, projections)
                            ]
                            .brackets()
                            .nest(INDENT)
                            .group()
                        ]
                        .group()
                    })
                ),
            ]
            .group()
        }

        /// Print spec of an item
        fn spec<A: 'static + Clone>(
            &self,
            item: &Item,
            name: &GlobalId,
            generics: &Generics,
            params: &Vec<Param>,
        ) -> DocBuilder<A> {
            let linked_items = HasLinkedItemGraph::linked_item_graph(self);
            let spec = linked_items.fn_like_linked_expressions(item, item.self_id());
            if !linked_items.has_spec(item) {
                nil!()
            } else {
                match hax_proof_attributes(item) {
                    Err(message) => emit_error!("{message}"),
                    Ok(proof_attributes) => {
                        let (tactic, specset) = match proof_attributes.proof_method {
                            Some(ProofMethod::Grind) => ("grind", "int"),
                            Some(ProofMethod::BvDecide) | None => ("bv_decide", "bv"),
                        };
                        let pure_requires_proof = proof_attributes
                            .pure_requires_proof
                            .unwrap_or(format!("by hax_construct_pure <;> {tactic}"));
                        let pure_ensures_proof = proof_attributes
                            .pure_ensures_proof
                            .unwrap_or(format!("by hax_construct_pure <;> {tactic}"));
                        let proof = proof_attributes.proof.map(|s| docs![s]).unwrap_or(docs![
                            "by hax_mvcgen [",
                            name,
                            "] <;> ",
                            tactic
                        ]);
                        {
                            docs![
                                hardline!(),
                                hardline!(),
                                docs!["set_option hax_mvcgen.specset \"", specset, "\" in"],
                                hardline!(),
                                "@[hax_spec]",
                                hardline!(),
                                docs![
                                    docs![
                                        "def",
                                        line!(),
                                        name,
                                        ".spec",
                                        self.generics(generics, &self.render_last(name)),
                                        params,
                                        softline!(),
                                        ":"
                                    ]
                                    .group()
                                    .nest(INDENT),
                                    line!(),
                                    docs![
                                        "Spec",
                                        line!(),
                                        docs![
                                            "requires",
                                            softline!(),
                                            ":= do",
                                            line!(),
                                            spec.precondition
                                                .map_or(reflow!("pure True"), |p| docs![p])
                                        ]
                                        .parens()
                                        .group()
                                        .nest(INDENT),
                                        line!(),
                                        docs![
                                            "ensures := ",
                                            spec.postcondition.map_or(
                                                reflow!("fun _ => pure True"),
                                                |p| docs![
                                                    "fun",
                                                    line!(),
                                                    p.result_binder,
                                                    softline!(),
                                                    "=> do",
                                                    line!(),
                                                    p.body,
                                                ]
                                                .group()
                                                .nest(INDENT)
                                            ),
                                        ]
                                        .parens()
                                        .group()
                                        .nest(INDENT),
                                        line!(),
                                        docs![
                                            name,
                                            zip_left!(line!(), &generics.params),
                                            self.params_as_args(params)
                                        ]
                                        .parens()
                                        .group()
                                        .nest(INDENT)
                                    ]
                                    .group()
                                    .nest(INDENT),
                                    softline!(),
                                    ":=",
                                ]
                                .group()
                                .nest(2 * INDENT),
                                softline!(),
                                docs![
                                    hardline!(),
                                    docs!["pureRequires :=", softline!(), pure_requires_proof],
                                    hardline!(),
                                    docs!["pureEnsures :=", softline!(), pure_ensures_proof],
                                    hardline!(),
                                    docs!["contract :=", softline!(), proof]
                                        .group()
                                        .nest(INDENT),
                                    hardline!(),
                                ]
                                .nest(INDENT)
                                .braces(),
                            ]
                        }
                    }
                }
            }
        }
    }

    impl<A: 'static + Clone> ToDocument<LeanPrinter, A> for (Vec<GenericParam>, &TraitItem) {
        fn to_document(&self, printer: &LeanPrinter) -> DocBuilder<A> {
            printer.trait_item_with_trait_params(&self.0, self.1)
        }
    }

    impl<A: 'static + Clone> PrettyAst<A> for LeanPrinter {
        const NAME: &'static str = "Lean";

        /// Produce a non-panicking placeholder document. In general, prefer the use of the helper macro [`todo_document!`].
        fn todo_document(&self, message: &str, issue_id: Option<u32>) -> DocBuilder<A> {
            <Self as PrettyAst<A>>::emit_diagnostic(
                self,
                hax_types::diagnostics::Kind::Unimplemented {
                    issue_id,
                    details: Some(message.into()),
                },
            );
            text!("sorry")
        }

        fn module(&self, module: &Module) -> DocBuilder<A> {
            let current_namespace = module.ident;
            let new_printer = LeanPrinter {
                current_namespace: Some(current_namespace),
                ..self.clone()
            };
            let items = &module.items;
            docs![
                "namespace ",
                current_namespace,
                hardline!(),
                hardline!(),
                intersperse!(
                    items.iter().map(|item| { item.to_document(&new_printer) }),
                    docs![hardline!(), hardline!()]
                ),
                hardline!(),
                hardline!(),
                "end ",
                current_namespace,
                hardline!(),
                hardline!(),
            ]
        }

        fn global_id(&self, global_id: &GlobalId) -> DocBuilder<A> {
            docs![self.render_id(global_id)]
        }

        fn generics(&self, generics: &Generics) -> DocBuilder<A> {
            self.generics(generics, &String::new())
        }

        fn generic_constraint(&self, _: &GenericConstraint) -> DocBuilder<A> {
            unreachable!(
                "Generic constraints are rendered inline because they must contain associated type projections."
            )
        }

        fn generic_param(&self, generic_param: &GenericParam) -> DocBuilder<A> {
            match generic_param.kind() {
                GenericParamKind::Type => docs![&generic_param.ident, reflow!(" : Type")]
                    .parens()
                    .group(),
                GenericParamKind::Lifetime => unreachable_by_invariant!(Drop_references),
                GenericParamKind::Const { ty } => docs![&generic_param.ident, reflow!(" : "), ty]
                    .parens()
                    .group(),
            }
        }

        fn generic_value(&self, generic_value: &GenericValue) -> DocBuilder<A> {
            match generic_value {
                GenericValue::Ty(ty) => docs![ty],
                GenericValue::Expr(expr) => docs![expr].parens(),
                GenericValue::Lifetime => unreachable_by_invariant!(Drop_references),
            }
        }

        fn expr(&self, Expr { kind, ty, meta: _ }: &Expr) -> DocBuilder<A> {
            match &**kind {
                ExprKind::If {
                    condition,
                    then,
                    else_,
                } => {
                    if let Some(else_branch) = else_ {
                        docs![
                            docs!["if", line!(), condition, reflow!(" then do")].group(),
                            docs![line!(), then].nest(INDENT),
                            line!(),
                            reflow!("else do"),
                            docs![line!(), else_branch].nest(INDENT)
                        ]
                        .group()
                    } else {
                        unreachable_by_invariant!(Local_mutation)
                    }
                }
                ExprKind::App {
                    head,
                    args,
                    generic_args,
                    bounds_impls: _,
                    trait_,
                } => {
                    match (&args[..], &generic_args[..], head.kind()) {
                        ([arg], [], ExprKind::GlobalId(LIFT)) => docs![reflow!("← "), arg].parens(),
                        ([arg], [], ExprKind::GlobalId(PURE)) => {
                            docs![reflow!("pure "), arg].parens()
                        }
                        ([arg], [], ExprKind::GlobalId(CAST_OP)) => docs![
                            // Add type annotation for `cast_op`:
                            docs![head, line!(), arg],
                            softline!(),
                            ":",
                            line!(),
                            "RustM",
                            softline!(),
                            ty
                        ]
                        .parens()
                        .group()
                        .nest(INDENT),
                        // TODO: Replace this match pattern with an `if let` guard when the feature stabilizes
                        // Tracking PR: https://github.com/rust-lang/rust/pull/141295
                        (
                            [arg],
                            [],
                            ExprKind::GlobalId(op @ (binops::neg | binops::not | binops::Not::not)),
                        ) if arg.ty == Ty::bool() || arg.ty.is_int() => {
                            let symbol = match *op {
                                binops::neg => "-?",
                                binops::not => "~?",
                                binops::Not::not => "!?",
                                _ => unreachable!(),
                            };
                            docs![symbol, softline!(), arg].parens()
                        }
                        ([lhs, rhs], [], ExprKind::GlobalId(binops::Index::index)) => {
                            docs![lhs, "[", line_!(), rhs, line_!(), "]_?"]
                                .nest(INDENT)
                                .group()
                        }
                        // TODO: Replace this match pattern with an `if let` guard when the feature stabilizes
                        // Tracking PR: https://github.com/rust-lang/rust/pull/141295
                        (
                            [lhs, rhs],
                            [],
                            ExprKind::GlobalId(
                                op @ (binops::add
                                | binops::sub
                                | binops::mul
                                | binops::div
                                | binops::rem
                                | binops::shr
                                | binops::shl
                                | binops::bitand
                                | binops::BitAnd::bitand
                                | binops::bitor
                                | binops::BitOr::bitor
                                | binops::bitxor
                                | binops::BitXor::bitxor
                                | binops::logical_op_and
                                | binops::logical_op_or
                                | binops::eq
                                | binops::PartialEq::eq
                                | binops::lt
                                | binops::le
                                | binops::gt
                                | binops::ge
                                | binops::ne
                                | binops::PartialEq::ne),
                            ),
                        ) if (lhs.ty == Ty::bool() && rhs.ty == Ty::bool())
                            || (rhs.ty.is_int() && lhs.ty.is_int()) =>
                        {
                            let symbol = match *op {
                                binops::add => "+?",
                                binops::sub => "-?",
                                binops::mul => "*?",
                                binops::div => "/?",
                                binops::rem => "%?",
                                binops::shr => ">>>?",
                                binops::shl => "<<<?",
                                binops::bitand => "&&&?",
                                binops::BitAnd::bitand => "&&?",
                                binops::bitor => "|||?",
                                binops::BitOr::bitor => "||?",
                                binops::bitxor => "^^^?",
                                binops::BitXor::bitxor => "^^?",
                                binops::logical_op_and => "&&?",
                                binops::logical_op_or => "||?",
                                binops::eq => "==?",
                                binops::PartialEq::eq => "==?",
                                binops::lt => "<?",
                                binops::le => "<=?",
                                binops::gt => ">?",
                                binops::ge => ">=?",
                                binops::ne => "!=?",
                                binops::PartialEq::ne => "!=?",
                                _ => unreachable!(),
                            };
                            docs![lhs, line!(), docs![symbol, softline!(), rhs].group()]
                                .group()
                                .nest(INDENT)
                                .parens()
                        }
                        _ => {
                            // Fallback for any application
                            docs![
                                head,
                                trait_
                                    .as_ref()
                                    .map(|(impl_expr, _)| zip_left!(line!(), &impl_expr.goal.args)),
                                zip_left!(line!(), generic_args).group(),
                                zip_left!(line!(), args).group(),
                            ]
                            .parens()
                            .nest(INDENT)
                            .group()
                        }
                    }
                }
                ExprKind::Literal(numeric_lit @ (Literal::Float { .. } | Literal::Int { .. })) => {
                    docs![numeric_lit, reflow!(" : "), ty].parens().group()
                }
                ExprKind::Literal(literal) => docs![literal],
                ExprKind::Array(exprs) => docs![
                    "RustArray.ofVec #v[",
                    intersperse!(exprs, docs![",", line!()])
                        .nest(INDENT)
                        .group()
                        .align(),
                    "]"
                ]
                .parens()
                .group(),
                ExprKind::Construct {
                    constructor,
                    is_record,
                    is_struct,
                    fields,
                    base,
                } => {
                    if fields.is_empty() && base.is_none() {
                        docs![constructor]
                    } else if let Some(base) = base {
                        if !(*is_record && *is_struct) {
                            unreachable!(
                                "Constructors with base expressions are necessarily structs with record-like arguments"
                            )
                        }
                        docs![base, line!(), reflow!("with "), self.struct_fields(fields)]
                            .braces()
                            .group()
                    } else {
                        docs![constructor, self.arguments(fields, is_record)]
                            .nest(INDENT)
                            .parens()
                            .group()
                    }
                }
                ExprKind::Let { lhs, rhs, body }
                | ExprKind::Resugared(ResugaredExprKind::LetPure { lhs, rhs, body }) => {
                    let binder = if matches!(**kind, ExprKind::Let { .. }) {
                        " ←"
                    } else {
                        " :="
                    };
                    docs![
                        docs![
                            docs![
                                "let",
                                line!(),
                                // TODO: Improve treatment of patterns in general. see
                                // https://github.com/pereq-dev/hax/issues/1712
                                match *lhs.kind.clone() {
                                    PatKind::Ascription { .. } =>
                                        docs![lhs, reflow!(" : "), &lhs.ty],
                                    PatKind::Binding {
                                        mutable: false,
                                        var,
                                        mode: BindingMode::ByValue,
                                        sub_pat: None,
                                    } => docs![&var, reflow!(" : "), &lhs.ty],
                                    _ => docs![lhs],
                                },
                            ]
                            .group(),
                            binder,
                            line!(),
                            rhs,
                            ";"
                        ]
                        .nest(INDENT)
                        .group(),
                        line!(),
                        body,
                    ]
                }
                ExprKind::GlobalId(global_id) => docs![global_id],
                ExprKind::LocalId(local_id) => docs![local_id],
                ExprKind::Ascription { e, ty } => docs![e, reflow!(" : "), ty].parens().group(),
                ExprKind::Closure {
                    params,
                    body,
                    captures: _,
                } => docs![
                    docs![
                        reflow!("fun"),
                        zip_left!(line!(), params),
                        softline!(),
                        "=>"
                    ]
                    .group(),
                    line!(),
                    self.do_block(self.expr_typed_result(body)).parens()
                ]
                .parens()
                .group()
                .nest(INDENT),

                ExprKind::Resugared(ResugaredExprKind::Tuple { .. }) => {
                    unreachable!("This printer doesn't use the tuple resugaring")
                }
                ExprKind::Match { scrutinee, arms } => docs![
                    docs![
                        "match",
                        docs![line!(), scrutinee].nest(INDENT),
                        line!(),
                        "with"
                    ]
                    .group(),
                    docs![line!(), intersperse!(arms, line!())]
                        .group()
                        .nest(INDENT),
                ]
                .group(),

                ExprKind::Borrow { .. } => {
                    unreachable_by_invariant!(Drop_references)
                }
                ExprKind::AddressOf { .. } => unreachable_by_invariant!(Reject_raw_or_mut_pointer),
                ExprKind::Assign { .. } => unreachable_by_invariant!(Local_mutation),
                ExprKind::Loop { .. } => unreachable_by_invariant!(Functionalize_loops),
                ExprKind::Break { .. } | ExprKind::Return { .. } | ExprKind::Continue { .. } => {
                    unreachable_by_invariant!(Drop_break_continue_return)
                }
                ExprKind::Block { .. } => unreachable_by_invariant!(Drop_blocks),
                ExprKind::Quote { contents } => docs![contents],
                ExprKind::Error(error_node) => docs![error_node],
            }
        }

        fn arm(&self, arm: &Arm) -> DocBuilder<A> {
            if let Some(_guard) = &arm.guard {
                unreachable_by_invariant!(Drop_match_guards)
            } else {
                docs![
                    reflow!("| "),
                    &arm.pat,
                    softline!(),
                    "=>",
                    softline!(),
                    "do",
                    line!(),
                    &arm.body
                ]
                .nest(INDENT)
                .group()
            }
        }

        fn pat(&self, pat: &Pat) -> DocBuilder<A> {
            match &*pat.kind {
                PatKind::Wild => docs!["_"],
                PatKind::Ascription { pat, ty: _ } => docs![pat],
                PatKind::Binding {
                    mutable,
                    var,
                    mode,
                    sub_pat,
                } => match (mutable, mode, sub_pat) {
                    (true, _, _) => unreachable_by_invariant!(Local_mutation),
                    (false, BindingMode::ByRef(_), _) => unreachable_by_invariant!(Drop_references),
                    (false, BindingMode::ByValue, None) => docs![var],
                    (false, BindingMode::ByValue, Some(pat)) => {
                        docs![var, "@", softline_!(), pat].group()
                    }
                },
                PatKind::Or { sub_pats } => docs![intersperse!(sub_pats, reflow!(" | "))].group(),
                PatKind::Array { .. } => {
                    emit_error!(issue 1712, "Unsupported pattern-matching on arrays")
                }
                PatKind::Deref { .. } => unreachable_by_invariant!(Drop_references),
                PatKind::Constant {
                    lit: Literal::Float { .. },
                } => {
                    emit_error!(issue 1788, "Unsupported pattern-matching on floats")
                }
                PatKind::Constant { lit } => docs![lit],
                PatKind::Construct {
                    constructor,
                    is_record,
                    is_struct,
                    fields,
                } => {
                    if *is_struct {
                        if !*is_record {
                            // Tuple-like structure, using positional arguments
                            docs![
                                "⟨",
                                intersperse!(
                                    fields.iter().map(|field| { docs![&field.1] }),
                                    docs![",", line!()]
                                )
                                .align()
                                .group(),
                                "⟩"
                            ]
                            .align()
                            .group()
                        } else {
                            // Record-like structure, using named arguments
                            docs![intersperse!(
                                fields.iter().map(|(id, pat)| {
                                    docs![self.render_last(id), reflow!(" :="), line!(), pat]
                                        .group()
                                }),
                                docs![",", line!()]
                            )]
                            .align()
                            .braces()
                            .group()
                        }
                    } else {
                        // Variant
                        docs![
                            constructor,
                            line!(),
                            self.arguments(fields, is_record).align()
                        ]
                        .parens()
                        .group()
                        .nest(INDENT)
                    }
                }
                PatKind::Resugared(ResugaredPatKind::ConstructWithEllipsis {
                    constructor,
                    is_struct,
                    fields,
                }) => {
                    if *is_struct {
                        // Struct: render as `{f1 := pat, f2 := pat, ..}` or `_`
                        if fields.is_empty() {
                            docs!["_"]
                        } else {
                            docs![intersperse!(
                                fields
                                    .iter()
                                    .map(|(id, pat)| {
                                        docs![self.render_last(id), reflow!(" :="), line!(), pat]
                                            .group()
                                    })
                                    .chain(std::iter::once(docs![".."])),
                                docs![",", line!()]
                            )]
                            .align()
                            .braces()
                            .group()
                        }
                    } else {
                        // Enum variant with named fields: (f1 := pat) (f2 := pat) ..
                        let record_part = if fields.is_empty() {
                            docs!["_"]
                        } else {
                            docs![intersperse!(
                                fields.iter().map(|(id, pat)| {
                                    docs![self.render_last(id), reflow!(" :="), line!(), pat]
                                        .group()
                                        .parens()
                                }),
                                line!()
                            )]
                            .align()
                            .group()
                        };
                        docs![constructor, line!(), record_part, " .."]
                            .parens()
                            .group()
                            .nest(INDENT)
                    }
                }
                PatKind::Error(_) => {
                    // TODO : Should be made unreachable by https://github.com/pereq-dev/hax/pull/1672
                    text!("sorry")
                }
            }
        }

        fn ty(&self, ty: &Ty) -> DocBuilder<A> {
            match ty.kind() {
                TyKind::Primitive(primitive_ty) => docs![primitive_ty],
                TyKind::App { head, args } => {
                    if args.is_empty() {
                        docs![head]
                    } else {
                        docs![head, zip_left!(line!(), args)]
                            .parens()
                            .group()
                            .nest(INDENT)
                    }
                }
                TyKind::Arrow { inputs, output } => docs![
                    zip_right!(inputs, docs![softline!(), "->", line!()]),
                    "RustM",
                    softline!(),
                    output
                ]
                .parens()
                .group(),
                TyKind::Param(local_id) => docs![local_id],
                TyKind::Slice(ty) => docs!["RustSlice", line!(), ty].parens().group(),
                TyKind::Array { ty, length } => docs!["RustArray", line!(), ty, line!(), {
                    if let ExprKind::Literal(int_lit @ Literal::Int { .. }) = length.kind() {
                        docs![int_lit]
                    } else if let ExprKind::LocalId(local_id) = length.kind() {
                        docs![local_id]
                    } else {
                        unreachable!(
                            "Only arrays with integer literal or const param size are supported"
                        )
                    }
                }]
                .parens()
                .group(),
                TyKind::AssociatedType { impl_, item } => {
                    let kind = impl_.kind();
                    match &kind {
                        ImplExprKind::Self_ => docs!["associatedTypes.", self.render_last(item)],
                        ImplExprKind::Parent { ident, .. }
                        | ImplExprKind::Projection { ident, .. } => {
                            docs![item, zip_left!(line!(), ident.goal.args.iter())]
                                .parens()
                                .group()
                                .nest(INDENT)
                        }
                        ImplExprKind::LocalBound { .. } => {
                            docs![item, zip_left!(line!(), impl_.goal.args.iter())]
                                .parens()
                                .group()
                                .nest(INDENT)
                        }
                        _ => {
                            emit_error!(issue 1922, "Unsupported variant of associated type")
                        }
                    }
                }
                TyKind::Ref { .. } => unreachable_by_invariant!(Drop_references),
                TyKind::RawPointer => unreachable_by_invariant!(Reject_raw_or_mut_pointer),
                TyKind::Opaque(_) => emit_error!(issue 1714, "Unsupported opaque type definitions"),
                TyKind::Dyn(_) => emit_error!(issue 1708, "Unsupported `dyn` traits"),
                TyKind::Resugared(resugared_ty_kind) => match resugared_ty_kind {
                    ResugaredTyKind::Tuple(_) => {
                        unreachable!("This backend does not use tuple resugaring (yet)")
                    }
                },
                TyKind::Error(e) => docs![e],
            }
        }

        fn literal(&self, literal: &Literal) -> DocBuilder<A> {
            docs![match literal {
                Literal::String(symbol) => format!("\"{}\"", self.escape_string(symbol)),
                Literal::Char(c) => format!("'{c}'"),
                Literal::Bool(b) => format!("{b}"),
                Literal::Int {
                    value,
                    negative,
                    kind: _,
                } => format!("{}{value}", if *negative { "-" } else { "" }),
                Literal::Float {
                    value,
                    negative,
                    kind: _,
                } => format!("{}{value}", if *negative { "-" } else { "" }),
            }]
        }

        fn local_id(&self, local_id: &LocalId) -> DocBuilder<A> {
            // TODO: should be done by name rendering, see https://github.com/pereq-dev/hax/issues/1630
            docs![Self::escape(&local_id.0)]
        }

        fn spanned_ty(&self, spanned_ty: &SpannedTy) -> DocBuilder<A> {
            docs![&spanned_ty.ty]
        }

        fn primitive_ty(&self, primitive_ty: &PrimitiveTy) -> DocBuilder<A> {
            match primitive_ty {
                PrimitiveTy::Bool => docs!["Bool"],
                PrimitiveTy::Int(int_kind) => docs![int_kind],
                PrimitiveTy::Float(float_kind) => docs![float_kind],
                PrimitiveTy::Char => docs!["Char"],
                PrimitiveTy::Str => docs!["String"],
            }
        }

        fn int_kind(&self, int_kind: &IntKind) -> DocBuilder<A> {
            docs![match (&int_kind.signedness, &int_kind.size) {
                (Signedness::Signed, IntSize::S8) => "i8",
                (Signedness::Signed, IntSize::S16) => "i16",
                (Signedness::Signed, IntSize::S32) => "i32",
                (Signedness::Signed, IntSize::S64) => "i64",
                (Signedness::Signed, IntSize::S128) => "i128",
                (Signedness::Signed, IntSize::SSize) => "isize",
                (Signedness::Unsigned, IntSize::S8) => "u8",
                (Signedness::Unsigned, IntSize::S16) => "u16",
                (Signedness::Unsigned, IntSize::S32) => "u32",
                (Signedness::Unsigned, IntSize::S64) => "u64",
                (Signedness::Unsigned, IntSize::S128) => "u128",
                (Signedness::Unsigned, IntSize::SSize) => "usize",
            }]
        }

        fn float_kind(&self, float_kind: &FloatKind) -> DocBuilder<A> {
            docs![match float_kind {
                FloatKind::F32 => "f32",
                FloatKind::F64 => "f64",
                _ => emit_error!(issue 1787, "The only supported float types are `f32` and `f64`."),
            }]
        }

        fn quote_content(&self, quote_content: &QuoteContent) -> DocBuilder<A> {
            match quote_content {
                QuoteContent::Verbatim(s) => {
                    intersperse!(s.lines().map(|x| x.to_string()), hardline!())
                }
                QuoteContent::Expr(expr) => docs![expr],
                QuoteContent::Pattern(pat) => docs![pat],
                QuoteContent::Ty(ty) => docs![ty],
            }
        }

        fn quote(&self, quote: &Quote) -> DocBuilder<A> {
            concat![&quote.0]
        }

        fn param(&self, param: &Param) -> DocBuilder<A> {
            if matches!(
                *param.pat.kind,
                PatKind::Wild | PatKind::Ascription { .. } | PatKind::Binding { sub_pat: None, .. }
            ) {
                self.pat_typed(&param.pat)
            } else {
                emit_error!(issue 1791, "Function parameters must not contain patterns")
            }
        }

        fn item(&self, item @ Item { ident, kind, meta }: &Item) -> DocBuilder<A> {
            let body = match kind {
                ItemKind::Fn {
                    name,
                    generics,
                    body,
                    params,
                    safety: _,
                } => {
                    let opaque = item.is_opaque();
                    let linked_items = HasLinkedItemGraph::linked_item_graph(self);
                    docs![
                        if opaque || linked_items.has_spec(item) {
                            nil!()
                        } else {
                            // Function should be unfolded by `mvcgen`
                            docs!["@[spec]", hardline!()]
                        },
                        docs![
                            docs![
                                docs![
                                    docs![if opaque { "opaque" } else { "def" }, line!(), name]
                                        .group(),
                                    self.generics(generics, &self.render_last(name)),
                                    params,
                                    softline!(),
                                    ":"
                                ]
                                .group(),
                                line!(),
                                docs![
                                    "RustM",
                                    line!(),
                                    &body.ty,
                                    if opaque {
                                        nil!()
                                    } else {
                                        docs![line!(), ":= do"]
                                    }
                                ]
                                .group(),
                            ]
                            .group()
                            .nest(INDENT),
                            if opaque { nil!() } else { docs![line!(), body] }
                        ]
                        .group()
                        .nest(INDENT),
                        if opaque {
                            nil!()
                        } else {
                            docs![&self.spec(item, name, generics, params)]
                        }
                    ]
                }
                ItemKind::TyAlias { name, generics, ty } => docs![
                    "abbrev ",
                    name,
                    self.generics(generics, &self.render_last(name)),
                    softline!(),
                    ":",
                    line!(),
                    "Type",
                    softline!(),
                    ":=",
                    line!(),
                    ty
                ]
                .nest(INDENT)
                .group(),
                ItemKind::RustModule | ItemKind::Use { .. } => nil!(),
                ItemKind::Quote { quote, origin: _ } => docs![quote],
                ItemKind::NotImplementedYet => {
                    emit_error!(issue 1706, "Item unsupported by the Hax engine (unimplemented yet)")
                }
                ItemKind::Type {
                    name,
                    generics,
                    variants,
                    is_struct,
                } => {
                    if item.is_opaque() {
                        docs![
                            reflow!("opaque "),
                            name,
                            self.generics(generics, &self.render_last(name)),
                            softline!(),
                            ":",
                            line!(),
                            "Type"
                        ]
                        .group()
                        .nest(INDENT)
                    }
                    // TODO: use a resugaring, see https://github.com/pereq-dev/hax/issues/1668
                    else if *is_struct {
                        // Structures
                        let Some(variant) = variants.first() else {
                            unreachable!(
                                "Structures should always have a constructor (even empty ones)"
                            )
                        };
                        let args = if variant.arguments.is_empty() {
                            comment!["no fields"]
                        } else if !variant.is_record {
                            // Tuple-like structure, using positional arguments
                            intersperse!(
                                variant.arguments.iter().enumerate().map(|(i, (_, ty, _))| {
                                    docs![format!("_{i} :"), line!(), ty].group().nest(INDENT)
                                }),
                                hardline!()
                            )
                        } else {
                            // Structure-like structure, using named arguments
                            intersperse!(
                                variant.arguments.iter().map(|(id, ty, _)| {
                                    docs![self.render_last(id), reflow!(" : "), ty]
                                        .group()
                                        .nest(INDENT)
                                }),
                                hardline!()
                            )
                        };
                        docs![
                            docs![
                                reflow!("structure "),
                                name,
                                self.generics(generics, &self.render_last(name)),
                                line!(),
                                "where"
                            ]
                            .group(),
                            docs![hardline!(), args],
                        ]
                        .nest(INDENT)
                        .group()
                    } else {
                        // Enums
                        let applied_name: DocBuilder<A> = if generics.params.is_empty()
                            && generics.constraints.is_empty()
                        {
                            docs![name]
                        } else {
                            docs![name, self.generics(generics, &self.render_last(name))].group()
                        };
                        docs![
                            docs![
                                "inductive ",
                                name,
                                self.generics(generics, &self.render_last(name)),
                                softline!(),
                                ":",
                                line!(),
                                "Type"
                            ]
                            .group(),
                            hardline!(),
                            intersperse!(
                                variants.iter().map(|variant| docs![
                                    "| ",
                                    variant,
                                    applied_name.clone()
                                ]
                                .group()
                                .nest(INDENT)),
                                hardline!()
                            ),
                        ]
                    }
                }
                ItemKind::Trait {
                    name,
                    generics,
                    items,
                    safety: _,
                } => {
                    let generic_types = generics.type_class_constraints().collect::<Vec<_>>();
                    if generic_types.len() < generics.constraints.len() {
                        emit_error!(issue 1921, "Unsupported equality constraints on associated types")
                    }
                    docs![
                        // A trait is encoded as two Lean type classes: one holding the associated types,
                        // and one holding all other fields.
                        // This is the type class holding the associated types:
                        docs![
                            docs![
                                docs![reflow!("class "), name, ".AssociatedTypes"],
                                zip_left!(line!(), &generics.params).group(),
                                line!(),
                                "where"
                            ]
                            .group(),
                            zip_left!(
                                hardline!(),
                                generic_types.iter().map(|impl_ident| docs![
                                    self.constraint_name(&self.render_last(name), impl_ident),
                                    " :",
                                    line!(),
                                    &impl_ident.goal.trait_,
                                    ".AssociatedTypes",
                                    line!(),
                                    intersperse!(&impl_ident.goal.args, line!())
                                ]
                                .group()
                                .brackets())
                            ),
                            zip_left!(
                                hardline!(),
                                items
                                    .iter()
                                    .filter(|item| { matches!(item.kind, TraitItemKind::Type(_)) })
                                    .map(|item| docs![(generics.params.clone(), item)])
                            ),
                        ]
                        .nest(INDENT),
                        // We add the `[instance]` attribute to the contained constraints to make
                        // them available for type inference:
                        zip_left!(
                            docs![hardline!(), hardline!()],
                            generic_types.iter().map(|impl_ident| docs![
                                "attribute [instance_reducible, instance]",
                                line!(),
                                name,
                                ".AssociatedTypes.",
                                self.constraint_name(&self.render_last(name), impl_ident),
                            ]
                            .group()
                            .nest(INDENT))
                        ),
                        zip_left!(
                            docs![hardline!(), hardline!()],
                            items
                                .iter()
                                .filter(|item| { matches!(item.kind, TraitItemKind::Type(_)) })
                                .map(|item| docs![
                                    "attribute [reducible]",
                                    line!(),
                                    self.render_with_injection(
                                        &item.ident,
                                        &"AssociatedTypes".to_string()
                                    )
                                ]
                                .group()
                                .nest(INDENT))
                        ),
                        // When referencing associated types, we would like to refer to them as
                        // `TraitName.TypeName` instead of `TraitName.AssociatedTypes.TypeName`:
                        zip_left!(
                            docs![hardline!(), hardline!()],
                            items
                                .iter()
                                .filter(|item| { matches!(item.kind, TraitItemKind::Type(_)) })
                                .map(|item| {
                                    docs![
                                        "abbrev ",
                                        name,
                                        ".",
                                        self.render_last(&item.ident),
                                        " :=",
                                        line!(),
                                        name,
                                        ".AssociatedTypes",
                                        ".",
                                        self.render_last(&item.ident),
                                    ]
                                    .nest(INDENT)
                                })
                        ),
                        hardline!(),
                        hardline!(),
                        // This is the type class holding all other fields:
                        docs![
                            docs![
                                docs![reflow!("class "), name],
                                docs![
                                    // Type parameters are also parameters of the class, but constraints are fields of the class
                                    docs![zip_left!(line!(), &generics.params)].group(),
                                    line!(),
                                    // The collection of associated types is an extra parameter so that we can encode
                                    // equality constraints on associated types.
                                    docs![
                                        reflow!("associatedTypes :"),
                                        softline!(),
                                        "outParam",
                                        softline!(),
                                        docs![
                                            name,
                                            ".AssociatedTypes",
                                            softline!(),
                                            intersperse!(&generics.params, softline!()),
                                        ]
                                        .parens()
                                        .nest(INDENT)
                                    ]
                                    .brackets()
                                    .nest(INDENT)
                                ]
                                .group(),
                                line!(),
                                "where"
                            ]
                            .group(),
                            // Lean's `extends` does not work for us because one cannot implement
                            // different functions of the same name on the super- and on the
                            // subclass. So we treat supertraits like any other constraint:
                            zip_left!(
                                hardline!(),
                                generic_types.iter().map(|impl_ident| docs![
                                    self.constraint_name(&self.render_last(name), impl_ident),
                                    softline!(),
                                    ":",
                                    line!(),
                                    impl_ident.goal.trait_,
                                    zip_left!(line!(), impl_ident.goal.args.iter())
                                ]
                                .group()
                                .brackets())
                            ),
                            // We also add constraints on associated types here:
                            concat!(
                                items
                                    .iter()
                                    .filter(|item| { matches!(item.kind, TraitItemKind::Type(_)) })
                                    .map(|item| docs![
                                        self.generics(
                                            &item.generics,
                                            &self.render_last(&item.ident)
                                        )
                                    ])
                            ),
                            // Finally the regular trait items:
                            zip_left!(
                                hardline!(),
                                items.iter().filter(|item| {!(
                                    // TODO: should be treated directly by name rendering, see :
                                    // https://github.com/pereq-dev/hax/issues/1646
                                    item.ident.is_precondition() || item.ident.is_postcondition() ||
                                    // Associated types are encoded in a separate type class.
                                    matches!(item.kind, TraitItemKind::Type(_))
                                )}).map(|item| docs![(generics.params.clone(), item)] )
                            ),
                        ]
                        .nest(INDENT),
                        // We add the `[instance]` attribute to the contained constraints to make
                        // them available for type inference:
                        zip_left!(
                            docs![hardline!(), hardline!()],
                            generic_types.iter().map(|impl_ident| docs![
                                "attribute [instance_reducible, instance]",
                                line!(),
                                name,
                                ".",
                                self.constraint_name(&self.render_last(name), impl_ident),
                            ]
                            .group()
                            .nest(INDENT))
                        ),
                    ]
                }
                ItemKind::Impl {
                    generics,
                    self_ty: _,
                    of_trait: TraitGoal { trait_, args },
                    items,
                    parent_bounds: _,
                } => {
                    let opaque = item.is_opaque();
                    docs![
                        // An impl is encoded as two Lean instances:
                        // One for the associated types...
                        docs![
                            docs![
                                if opaque {
                                    reflow!("@[instance] opaque ")
                                } else {
                                    reflow!("@[reducible] instance ")
                                },
                                ident,
                                ".AssociatedTypes",
                                self.generics(generics, &self.render_last(ident)),
                                softline!(),
                                ":"
                            ]
                            .group(),
                            line!(),
                            docs![trait_, ".AssociatedTypes", zip_left!(line!(), args)].group(),
                            if opaque {
                                docs![
                                    softline!(),
                                    ":=",
                                    line!(),
                                    reflow!("by constructor <;> exact Inhabited.default")
                                ]
                            } else {
                                docs![line!(), "where"]
                            },
                        ]
                        .group()
                        .nest(INDENT),
                        if opaque {
                            nil!()
                        } else {
                            docs![zip_left!(
                                hardline!(),
                                items.iter().filter(|item| {
                                    matches!(item.kind, ImplItemKind::Type { .. })
                                })
                            )]
                            .nest(INDENT)
                        },
                        hardline!(),
                        hardline!(),
                        // ...and one for all other fields:
                        docs![
                            docs![
                                if opaque {
                                    reflow!("@[instance] opaque ")
                                } else {
                                    reflow!("instance ")
                                },
                                ident,
                                self.generics(generics, &self.render_last(ident)),
                                softline!(),
                                ":"
                            ]
                            .group(),
                            line!(),
                            docs![trait_, zip_left!(line!(), args.iter())].group(),
                            if opaque {
                                docs![
                                    softline!(),
                                    ":=",
                                    line!(),
                                    reflow!("by constructor <;> exact Inhabited.default")
                                ]
                            } else {
                                docs![line!(), "where"]
                            },
                        ]
                        .group()
                        .nest(INDENT),
                        if opaque {
                            nil!()
                        } else {
                            docs![zip_left!(
                                hardline!(),
                                items.iter().filter(|item| {
                                    !(
                                        // TODO: should be treated directly by name rendering, see :
                                        // https://github.com/pereq-dev/hax/issues/1646
                                        item.ident.is_precondition() || item.ident.is_postcondition() ||
                                        // Associated types are encoded into a separate type class
                                        matches!(item.kind, ImplItemKind::Type { .. })
                                    )
                                })
                            )]
                            .nest(INDENT)
                        },
                    ]
                }
                ItemKind::Resugared(resugared_item_kind) => match resugared_item_kind {
                    ResugaredItemKind::Constant {
                        name,
                        body,
                        generics,
                    } => docs![
                        docs![
                            docs![
                                docs!["def", line!(), name].group(),
                                self.generics(generics, &self.render_last(ident)),
                                softline!(),
                                ":",
                            ]
                            .group(),
                            line!(),
                            &body.ty,
                            line!(),
                            ":="
                        ]
                        .group(),
                        line!(),
                        self.monad_extract(body),
                    ]
                    .group()
                    .nest(INDENT),
                    ResugaredItemKind::RecursiveFn {
                        name,
                        generics,
                        body,
                        params,
                        safety,
                    } => {
                        // Render the item with an appended `partial_fixpoint`:
                        let item = Item {
                            ident: item.ident,
                            kind: ItemKind::Fn {
                                name: *name,
                                generics: generics.clone(),
                                body: body.clone(),
                                params: params.clone(),
                                safety: safety.clone(),
                            },
                            meta: item.meta.clone(),
                        };
                        return docs![item, hardline!(), "partial_fixpoint"];
                    }
                },
                ItemKind::Alias { .. } => {
                    // aliases are introduced when creating bundles. Those should not appear in
                    // Lean, as items can be named correctly in any file.
                    emit_error!(issue 1658, "Unsupported alias item")
                }
                ItemKind::Error(e) => docs![e],
            };
            docs![meta, body]
        }

        fn impl_item(
            &self,
            ImplItem {
                meta: _,
                generics,
                kind,
                ident,
            }: &ImplItem,
        ) -> DocBuilder<A> {
            let name = self.render_last(ident);
            match kind {
                ImplItemKind::Type {
                    ty,
                    parent_bounds: _,
                } => docs![name, reflow!(" := "), ty],
                ImplItemKind::Fn { body, params } => docs![
                    docs![
                        name,
                        softline!(),
                        ":=",
                        line!(),
                        docs![
                            "fun",
                            self.generics(generics, &self.render_last(ident)),
                            zip_left!(line!(), params).group(),
                            softline!(),
                            "=>",
                            softline!(),
                            "do"
                        ]
                        .group()
                        .nest(INDENT)
                    ]
                    .group(),
                    line!(),
                    body
                ]
                .group()
                .nest(INDENT),
                ImplItemKind::Resugared(ResugaredImplItemKind::Constant { body })
                    if generics.params.is_empty() =>
                {
                    docs![
                        name,
                        softline!(),
                        ":=",
                        softline!(),
                        self.monad_extract(body)
                    ]
                }
                ImplItemKind::Resugared(ResugaredImplItemKind::Constant { body }) => docs![
                    docs![
                        name,
                        softline!(),
                        ":=",
                        line!(),
                        docs!["fun", line!(), generics, line!(), "=>",]
                            .group()
                            .nest(INDENT)
                    ]
                    .group(),
                    line!(),
                    self.monad_extract(body)
                ]
                .group()
                .nest(INDENT),
                ImplItemKind::Error(err) => docs!(err),
            }
        }

        fn impl_ident(&self, ImplIdent { .. }: &ImplIdent) -> DocBuilder<A> {
            unreachable!(
                "`ImplIdent`s are rendered inline because we have multiple variants of how they must be rendered."
            )
        }

        fn trait_goal(&self, TraitGoal { .. }: &TraitGoal) -> DocBuilder<A> {
            unreachable!(
                "`TraitGoal`s are rendered inline because we have multiple variants of how they must be rendered."
            )
        }

        fn variant(
            &self,
            Variant {
                name,
                arguments,
                is_record,
                attributes,
            }: &Variant,
        ) -> DocBuilder<A> {
            docs![
                concat!(attributes),
                self.render_last(name),
                softline!(),
                // args
                if *is_record {
                    // Use named the arguments, keeping only the head of the identifier
                    docs![
                        intersperse!(
                            arguments.iter().map(|(id, ty, _)| {
                                docs![self.render_last(id), reflow!(" : "), ty]
                                    .parens()
                                    .group()
                            }),
                            line!()
                        )
                        .align()
                        .nest(INDENT),
                        line!(),
                        reflow!(": "),
                    ]
                    .group()
                } else {
                    // Use anonymous arguments
                    docs![
                        reflow!(": "),
                        concat!(
                            arguments
                                .iter()
                                .map(|(_, ty, _)| { docs![ty, reflow!(" -> ")] })
                        )
                    ]
                }
            ]
            .group()
            .nest(INDENT)
        }

        fn symbol(&self, symbol: &Symbol) -> DocBuilder<A> {
            docs![Self::escape(symbol)]
        }

        fn metadata(
            &self,
            Metadata {
                span: _,
                attributes,
            }: &Metadata,
        ) -> DocBuilder<A> {
            concat!(attributes)
        }

        fn lhs(&self, _lhs: &Lhs) -> DocBuilder<A> {
            unreachable_by_invariant!(Local_mutation)
        }

        fn safety_kind(&self, _safety_kind: &SafetyKind) -> DocBuilder<A> {
            nil!()
        }

        fn binding_mode(&self, _binding_mode: &BindingMode) -> DocBuilder<A> {
            unreachable!("This backend handle binding modes directly inside patterns")
        }

        fn region(&self, _region: &Region) -> DocBuilder<A> {
            unreachable_by_invariant!(Drop_references)
        }

        fn dyn_trait_goal(&self, _dyn_trait_goal: &DynTraitGoal) -> DocBuilder<A> {
            emit_error!(issue 1708, "`dyn` traits are unsupported")
        }

        fn attribute(&self, Attribute { kind, span: _ }: &Attribute) -> DocBuilder<A> {
            match kind {
                AttributeKind::Tool { .. } | AttributeKind::Hax { .. } => {
                    nil!()
                }
                AttributeKind::DocComment {
                    kind: DocCommentKind::Line,
                    body,
                } => comment!(body.clone()).append(hardline!()),
                AttributeKind::DocComment {
                    kind: DocCommentKind::Block,
                    body,
                } => docs![
                    "/--",
                    line!(),
                    intersperse!(body.lines().map(|line| line.to_string()), line!()),
                    line!(),
                    "-/"
                ]
                .nest(INDENT)
                .group()
                .append(hardline!()),
            }
        }

        fn borrow_kind(&self, _borrow_kind: &BorrowKind) -> DocBuilder<A> {
            unreachable_by_invariant!(Drop_references)
        }

        fn guard(&self, _guard: &Guard) -> DocBuilder<A> {
            unreachable_by_invariant!(Drop_match_guards)
        }

        fn projection_predicate(
            &self,
            projection_predicate: &ProjectionPredicate,
        ) -> DocBuilder<A> {
            docs![
                self.render_last(&projection_predicate.assoc_item),
                softline!(),
                ":=",
                line!(),
                projection_predicate.ty,
            ]
            .group()
            .nest(INDENT)
        }

        fn error_node(&self, _error_node: &ErrorNode) -> DocBuilder<A> {
            // TODO : Should be made unreachable by https://github.com/pereq-dev/hax/pull/1672
            text!("sorry")
        }

        // Impl expressions

        fn impl_expr(&self, _impl_expr: &ImplExpr) -> DocBuilder<A> {
            emit_error!(issue 1716, "Explicit impl expressions are unsupported")
        }
    }
};
