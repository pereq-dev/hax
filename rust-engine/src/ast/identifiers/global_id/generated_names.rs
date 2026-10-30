/// We allow:
///  - `unused`: we don't use all the names present in the `engine/names` crate.
///    Filtering which `DefId` should be exposed would be complicated, and
///    dependent library may use some names. (for instance, the backend for
///    ProVerif may use names from `hax_lib_protocol` that are not needed
///    anywhere else in the engine)
///  - `non_snake_case`: we produce faithful names with respect to their
///    original definitions in Rust. We generate for instance `fn Some() ->
///    DefID {...}` that provides the `DefId` for the
///    `std::option::Option::Some`. We want the function to be named `Some`
///    here, not `some`.
///  - `broken_intra_doc_links`: we produce documentation that link the function
///    providing the `DefId` of a item to the item itself. Sometimes, we refer
///    to private items, to re-exported items or to items that are not in the
///    dependency closure of the engine: in such cases, `rustdoc` cannot link
///    properly.
#[allow(
    unused,
    non_snake_case,
    rustdoc::broken_intra_doc_links,
    missing_docs,
    clippy::module_inception,
    unused_qualifications,
    non_upper_case_globals
)]
pub mod root {
    include!("generated.rs");
}

/// Global identifiers are built around `DefId` that comes out of the hax
/// frontend. We use the Rust engine itself to produce the names: we run hax on
/// the `engine/names` crate, we extract identifiers from the resulting AST, and
/// we expose them back as Rust functions here.
pub mod codegen {
    use itertools::*;
    use std::iter;

    use crate::ast::Item;
    use crate::ast::identifiers::{
        GlobalId,
        global_id::{ExplicitDefId, compact_serialization},
    };
    use hax_frontend_exporter::DefKind;

    use std::collections::{HashMap, HashSet};

    /// Replace the crate name `"hax_engine_names"` with `"rust_primitives"` in the given `DefId`.
    fn rename_krate(def_id: &mut ExplicitDefId) {
        if def_id.def_id.krate == "hax_engine_names" {
            def_id.def_id = def_id.def_id.rename_krate("rust_primitives");
        }
    }

    /// Visit items and collect all the `DefId`s
    fn collect_def_ids(items: Vec<Item>) -> Vec<ExplicitDefId> {
        #[derive(Default)]
        struct DefIdCollector(HashSet<ExplicitDefId>);
        use crate::ast::visitors::*;
        impl AstVisitor for DefIdCollector {
            fn visit_global_id(&mut self, x: &GlobalId) {
                let mut current = x.0.explicit_def_id();
                while let Some(def_id) = current {
                    self.0.insert(def_id.clone());
                    current = def_id.parent();
                }
            }
        }

        // Collect names
        let mut names: Vec<_> = DefIdCollector::default()
            .visit_by_val(&items)
            .0
            .into_iter()
            .collect();

        // In the OCaml engine, `hax_engine_names` is renamed to `rust_primitives`.
        names.iter_mut().for_each(rename_krate);

        // We consume names after import by the OCaml engine. Thus, the OCaml
        // engine may have introduced already some hax-specific Rust names,
        // directly in `rust_primitives`. After renaming from `hax_engine_names`
        // to `rust_primitives`, such names may be duplicated. For instance,
        // that's the case of `unsize`: the crate `hax_engine_names` contains
        // expression with implicit unsize operations, thus the OCaml engine
        // inserts `rust_primitives::unsize`. In the same time,
        // `hax_engine_names::unsize` exists and was renamed to
        // `rust_primitives::unsize`. Whence the need to dedup here.
        names.sort();
        names.dedup();
        names
    }

    /// Crafts a docstring for a `DefId`, hopefully (rustdoc) linking it back to
    /// its origin.
    fn docstring(explicit_id: &ExplicitDefId) -> String {
        let id = &explicit_id.def_id;
        let path = path_of_def_id(explicit_id);
        let (parent_path, def) = match &path[..] {
            [init @ .., last] => (init, last.clone()),
            _ => (&[] as &[_], id.krate.to_string()),
        };
        let parent_path_str = format!("::{}", parent_path.join("::"));
        let path_str = format!("::{}", path_of_def_id(explicit_id).join("::"));
        let subject = match &id.kind {
            DefKind::Mod => format!("module [`{path_str}`]"),
            DefKind::Struct => format!("struct [`{path_str}`]"),
            DefKind::Union => format!("union [`{path_str}`]"),
            DefKind::Enum => format!("enum [`{path_str}`]"),
            DefKind::Variant => format!("variant [`{path_str}`]"),
            DefKind::Trait => format!("trait [`{path_str}`]"),
            DefKind::TyAlias => format!("type alias [`{path_str}`]"),
            DefKind::ForeignTy => format!("foreign type [`{path_str}`]"),
            DefKind::TraitAlias => format!("trait alias [`{path_str}`]"),
            DefKind::AssocTy => format!("associated type [`{path_str}`]"),
            DefKind::TyParam => format!("type parameter from [`{parent_path_str}`]"),
            DefKind::Fn => format!("function [`{path_str}`]"),
            DefKind::Const => format!("const [`{path_str}`]"),
            DefKind::ConstParam => format!("const parameter from [`{parent_path_str}`]"),
            DefKind::Static { .. } => format!("static [`{path_str}`]"),
            DefKind::Ctor { .. } => format!("constructor for [`{parent_path_str}`]"),
            DefKind::AssocFn => format!("associated function [`{path_str}`]"),
            DefKind::AssocConst => format!("associated constant [`{path_str}`]"),
            DefKind::Macro { .. } => format!("macro [`{path_str}`]"),
            DefKind::ExternCrate => format!("extern crate [`{path_str}`]"),
            DefKind::Use => format!("use item [`{path_str}`]"),
            DefKind::ForeignMod => format!("foreign module [`{path_str}`]"),
            DefKind::AnonConst => return "This is an anonymous constant.".to_string(),
            DefKind::PromotedConst | DefKind::InlineConst => {
                format!("This is an inline const from [`{parent_path_str}`]")
            }
            DefKind::OpaqueTy => {
                return format!("This is an opaque type for [`{parent_path_str}`]");
            }
            DefKind::Field => format!("field [`{def}`] from {parent_path_str}"),
            DefKind::LifetimeParam => return "This is a lifetime parameter.".to_string(),
            DefKind::GlobalAsm => return "This is a global ASM block.".to_string(),
            DefKind::Impl { .. } => return "This is an impl block.".to_string(),
            DefKind::Closure => return "This is a closure.".to_string(),
            DefKind::SyntheticCoroutineBody => return "This is a coroutine body.".to_string(),
        };
        format!("This is the {subject}.")
    }

    /// Computes a string path for a `DefId`.
    fn path_of_def_id(explicit_id: &ExplicitDefId) -> Vec<String> {
        let id = &explicit_id.def_id;
        fn name_to_string(mut s: String) -> String {
            if s == "_" {
                s = "_anonymous".into();
            };
            if s.parse::<i32>().is_ok() {
                s = format!("_{s}");
            }
            s
        }
        iter::once(id.krate.to_string())
            .chain(id.path.iter().map(|item| {
                let data = match item.data.clone() {
                    hax_frontend_exporter::DefPathItem::CrateRoot { name } => name,
                    hax_frontend_exporter::DefPathItem::TypeNs(s)
                    | hax_frontend_exporter::DefPathItem::ValueNs(s)
                    | hax_frontend_exporter::DefPathItem::MacroNs(s)
                    | hax_frontend_exporter::DefPathItem::LifetimeNs(s) => s,
                    data => format!("{data:?}"),
                };
                if item.disambiguator == 0 {
                    data
                } else {
                    format!("{data}__{}", item.disambiguator)
                }
            }))
            .chain(if explicit_id.is_constructor {
                Some("Constructor".to_string())
            } else {
                None
            })
            .chain(if matches!(id.kind, DefKind::Ctor(..)) {
                // TODO: get rid of `ctor` #1657
                Some("ctor".to_string())
            } else {
                None
            })
            .map(name_to_string)
            .collect()
    }

    /// Given a list of `DefId`, this will create a Rust code source that provides those names.
    ///
    /// For example, given `krate::module::f` and `krate::g`, this will produce something like:
    /// ```rust,ignore
    /// mod krate {
    ///    mod module {
    ///       fn f() -> DefId {...}
    ///    }
    ///    fn g() -> DefId {...}
    /// }
    /// ```
    fn generate_names_hierachy(def_ids: Vec<ExplicitDefId>) -> String {
        /// Helper struct: a graph of module and definitions.
        #[derive(Debug, Default)]
        struct Module {
            attached_def_id: Option<ExplicitDefId>,
            submodules: HashMap<String, Module>,
            definitions: Vec<(String, ExplicitDefId)>,
        }
        impl Module {
            fn new(def_ids: Vec<ExplicitDefId>) -> Self {
                let mut node = Self::default();
                for def_id in &def_ids {
                    node.insert(def_id);
                }
                for def_id in def_ids {
                    let modpath = path_of_def_id(&def_id);
                    if let Some(module) = node.find_module(&modpath) {
                        module.attached_def_id = Some(def_id.clone());
                    }
                }
                node
            }
            /// Insert a `DefId` in our module tree
            fn insert(&mut self, def_id: &ExplicitDefId) {
                let fullpath = path_of_def_id(def_id);
                let [modpath @ .., def] = &fullpath[..] else {
                    return;
                };

                let mut node = self;
                for chunk in modpath {
                    node = node.submodules.entry(chunk.clone()).or_default();
                }

                node.definitions.push((def.clone(), def_id.clone()));
            }
            /// Get a mutable borrow to the submodule denoted by `modpath`, if it exists
            fn find_module(&mut self, modpath: &Vec<String>) -> Option<&mut Self> {
                let mut node = self;
                for chunk in modpath {
                    node = node.submodules.get_mut(chunk)?;
                }
                Some(node)
            }
            /// Render the module tree as a string
            fn render(self, path: String, indexes: &HashMap<ExplicitDefId, usize>) -> String {
                /// Computes the visibility restriction for a given path.
                fn restriction(path: &str) -> &'static str {
                    // Tuples are encoded directly in `GlobalIdInner::Tuple`.
                    // The names here exist so that tuple identifiers can be handled in the exact same way as other identifiers.
                    // But the canonical representation of tuples is not `names::rust_primitives::hax::Tuple*`.
                    // Whence this visibility restriction.
                    if path.starts_with("::rust_primitives::hax::Tuple") {
                        "(in crate::ast::identifiers::global_id)"
                    } else {
                        ""
                    }
                }
                let Self {
                    submodules,
                    definitions,
                    attached_def_id,
                } = self;
                let submodules = submodules
                    .into_iter()
                    .sorted_by(|(a, _), (b, _)| a.cmp(b))
                    .map(|(name, contents)| {
                        let path = format!("{path}::{name}");
                        let restriction = restriction(&path);
                        format!(
                            r###"pub{restriction} mod {name} {{ {} }}"###,
                            contents.render(path, indexes)
                        )
                    });
                let definitions = definitions
                    .into_iter()
                    .sorted_by(|(a, _), (b, _)| a.cmp(b))
                    .map(|(name, def_id)| {
                        let docstring = docstring(&def_id);
                        let index = indexes.get(&def_id).unwrap();
                        let restriction = restriction(&format!("{path}::{name}"));
                        format!(r###"
                            #[doc = r##"{docstring}"##]
                            pub{restriction} const {name}: crate::ast::identifiers::global_id::GlobalId = crate::ast::identifiers::global_id::GlobalId(root::INTERNED_GLOBAL_IDS[{index}]);
                        "###)
                    });
                let docstring = attached_def_id
                    .iter()
                    .map(docstring)
                    .map(|s| format!(r###"#![doc=r##"{s}"##]"###));
                docstring
                    .chain(iter::once("use super::root;".to_string()))
                    .chain(submodules)
                    .chain(definitions)
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
        let enumerated_def_ids = def_ids
            .iter()
            .cloned()
            .enumerate()
            .map(|(n, def_id)| (def_id, n))
            .collect::<Vec<_>>();
        let indexes = HashMap::from_iter(enumerated_def_ids.iter().cloned());
        let tree = Module::new(def_ids).render(String::new(), &indexes);
        let functions = {
            enumerated_def_ids.iter().map(|(did, i)| {
                let serialized = compact_serialization::serialize(did);
                let parent = did.parent().as_ref().map(|parent| *indexes.get(parent).unwrap()).map(|parent| format!("Some(did_{parent}())")).unwrap_or("None".into());
                format!(r###"fn did_{i}() -> ExplicitDefId {{deserialize(r##"{serialized}"##, {parent})}}"###)
            }).collect::<Vec<_>>().join("\n")
        };
        let array_literal = enumerated_def_ids
            .iter()
            .map(|(_, i)| format!("did_{i}().into_global_id_inner()"))
            .collect::<Vec<_>>()
            .join(",");
        let n = indexes.len();
        format!(
            r#"// This file was generated by `cargo hax into generate-rust-engine-names`.
// To regenerate it, please use `just regenerate-names`. Under the hood, `cargo
// hax into generate-rust-engine-names` runs the Rust engine, which in turn
// calls `rust_engine::names::export_def_ids_to_mod`.

static TABLE_AND_INTERNED_GLOBAL_IDS: (crate::interning::LazyLockNewWithValue<crate::ast::identifiers::global_id::GlobalIdInner, {n}>, [crate::interning::Interned<crate::ast::identifiers::global_id::GlobalIdInner>; {n}]) = {{
    crate::interning::InterningTable::new_with_values(|| {{
        use crate::ast::identifiers::global_id::ExplicitDefId;
        use crate::ast::identifiers::global_id::compact_serialization::deserialize;
        {functions}
        [{array_literal}]
    }})
}};

static INTERNED_GLOBAL_IDS: [crate::interning::Interned<crate::ast::identifiers::global_id::GlobalIdInner>; {n}] = TABLE_AND_INTERNED_GLOBAL_IDS.1;

impl crate::interning::Internable for crate::ast::identifiers::global_id::GlobalIdInner {{
    fn interning_table() -> &'static std::sync::Mutex<crate::interning::InterningTable<Self>> {{
        &TABLE_AND_INTERNED_GLOBAL_IDS.0
    }}
}}

{tree}
"#
        )
    }

    /// Finds all `DefId`s in `items`, and produce a Rust module exposing them.
    pub fn export_def_ids_to_mod(items: Vec<Item>) -> String {
        generate_names_hierachy(collect_def_ids(items))
    }
}
