//! Code generation backends.
//!
//! A backend is consititued of:
//!  - a list of AST transformations to apply, those are called phases.
//!  - and a printer.
//!
//! This top-level module is mostly an index of available backends and a
//! small prelude to make backend modules concise.
//!
//! # Adding a new backend
//! 1. Create a submodule under `src/backends/`, e.g. `foo.rs`.
//! 2. Put your printer and backend there.
//! 3. Re-export it here with `pub mod foo;`.
//!
//! See [`rust`] for an example implementation.

pub mod fstar;
pub mod legacy_lean;
pub mod proverif;
pub mod rust;

use std::{collections::HashMap, rc::Rc};

use crate::{
    ast::{Item, Metadata, Module, span::Span},
    attributes::LinkedItemGraph,
    phase::legacy::group_consecutive_ocaml_phases,
    printer::{HasLinkedItemGraph, Print, Printer},
};
use camino::Utf8PathBuf;
use hax_types::engine_api::File;

/// A hax backend.
///
/// A backend is responsible for turning the hax AST into sources of a target language.
/// It combines:
/// - a sequence of AST transformation phases, and
/// - a printer that generates textual output.
///
/// For example, we have F\*, Coq, and Lean backends.
/// Some are still in the old OCaml engine.
pub trait Backend {
    /// The printer type used by this backend.
    type Printer: Printer;

    /// Construct a new printer instance.
    ///
    /// By default this calls `Default::default` on the printer type.
    fn printer(&self, linked_item_graph: Rc<LinkedItemGraph>) -> Self::Printer {
        Self::Printer::default().with_linked_item_graph(linked_item_graph)
    }

    /// A short name identifying the backend.
    ///
    /// By default, this is delegated to the associated printer's [`Printer::NAME`].
    const NAME: &'static str = Self::Printer::NAME;

    /// The AST phases to apply before printing.
    ///
    /// Backends can override this to add transformations.
    /// The default is an empty list (no transformations).
    fn phases(&self) -> Vec<crate::phase::PhaseKind> {
        vec![]
    }

    /// A list of resugaring phases.
    fn resugaring_phases() -> Vec<Box<dyn prelude::Resugaring>> {
        vec![]
    }

    /// Group a flat list of items into modules.
    fn items_to_module(&self, items: Vec<Item>) -> Vec<Module> {
        let mut modules: HashMap<_, Vec<_>> = HashMap::new();
        for item in items {
            let module_ident = item.ident.mod_only_closest_parent();
            modules.entry(module_ident).or_default().push(item);
        }
        modules
            .into_iter()
            .map(|(ident, items)| Module {
                ident,
                items,
                meta: Metadata {
                    span: Span::dummy(),
                    attributes: vec![],
                },
            })
            .collect()
    }

    /// Print a list of modules into files
    fn modules_to_files(&self, modules: Vec<Module>, mut printer: Self::Printer) -> Vec<File> {
        modules
            .into_iter()
            .map(|module: Module| {
                let path = self.module_path(&module).into_string();
                let (contents, _) = printer.print(module);
                File {
                    path,
                    contents,
                    sourcemap: None,
                }
            })
            .collect()
    }

    /// Compute the relative filesystem path where a given module should be written.
    fn module_path(&self, module: &Module) -> Utf8PathBuf;
}

/// A backend can be interpreted as a phase
impl<B: Backend> crate::phase::Phase for B {
    fn apply(&self, items: &mut Vec<Item>) {
        for phase in group_consecutive_ocaml_phases(self.phases()) {
            phase.apply(items);
        }
    }
}

/// Whether an item is marked with a hax attribute whose payload deserializes to
/// `AttrPayload::ItemStatus(ItemStatus::Included { late_skip: true })`: it is
/// dropped right before printing.
pub(crate) fn is_late_skip(meta: &Metadata) -> bool {
    use hax_lib_macros_types::{AttrPayload, ItemStatus};
    meta.hax_attributes().any(|attr| {
        matches!(
            attr,
            AttrPayload::ItemStatus(ItemStatus::Included { late_skip: true })
        )
    })
}

/// Apply a backend to a collection of AST items, producing output files.
///
/// This runs all of the backend's [`Backend::phases`], groups the items into
/// modules via [`Backend::items_to_module`], and then uses the backend's printer
/// to generate source files with paths determined by [`Backend::module_path`].
pub fn apply_backend<B: Backend + 'static>(backend: B, mut items: Vec<Item>) -> Vec<File> {
    crate::phase::Phase::apply(&backend, &mut items);

    for mut resugaring_phase in B::resugaring_phases() {
        for item in &mut items {
            resugaring_phase.visit(item)
        }
    }

    let linked_items_graph = Rc::new(LinkedItemGraph::new(
        &items,
        prelude::diagnostics::Context::Printer(B::NAME.into()),
    ));

    /// Drop any item marked with a hax attribute whose payload deserializes to
    /// `AttrPayload::ItemStatus(ItemStatus::Included { late_skip: true })`.
    ///
    /// Items with such a "late-skip" attribute are typically generated by hax
    /// attributes.
    fn drop_skip_late_items(items: &mut Vec<Item>) {
        items.retain(|item| !is_late_skip(&item.meta));
        for item in items {
            if let crate::ast::ItemKind::Impl { items, .. } = &mut item.kind {
                items.retain(|impl_item| !is_late_skip(&impl_item.meta));
            }
        }
    }

    drop_skip_late_items(&mut items);

    let modules = backend.items_to_module(items);
    let printer = backend.printer(linked_items_graph.clone());
    backend.modules_to_files(modules, printer)
}

mod prelude {
    //! Small "bring-into-scope" set used by backend modules.
    //!
    //! Importing this prelude saves repetitive `use` lists in per-backend
    //! modules without forcing these names on downstream users.
    pub use super::Backend;
    pub use crate::ast::{identifiers::global_id::view::AnyKind, literals::*, resugared::*, *};
    pub use crate::printer::{
        pretty_ast::{DocBuilder, PrettyAst, ToDocument, install_pretty_helpers},
        render_view::*,
        *,
    };
    pub use crate::resugarings::*;
    pub use crate::symbol::Symbol;
    pub use hax_rust_engine_macros::{prepend_associated_functions_with, setup_printer_struct};
}
