use hax_rust_engine::{
    backends,
    ocaml_engine::{self, Response},
};
use hax_types::{cli_options::Backend, engine_api::File};
use std::collections::HashMap;

fn main() {
    let (value, table) = hax_rust_engine::hax_io::read_engine_input_message().destruct();

    ocaml_engine::initialize(ocaml_engine::Meta {
        hax_version: value.hax_version,
        impl_infos: value.impl_infos,
        debug_bind_phase: value.backend.debug_engine.is_some(),
        profiling: value.backend.profile,
    });

    let items = match value.input {
        hax_types::driver_api::Items::Legacy(input) => {
            let query = hax_rust_engine::ocaml_engine::QueryKind::ImportThir {
                input,
                translation_options: value.backend.translation_options,
            };

            let Some(Response::ImportThir {
                output,
                diagnostics,
            }) = query.execute(Some(table))
            else {
                panic!()
            };
            for diagnostic in diagnostics {
                hax_rust_engine::hax_io::report_diagnostic(diagnostic);
            }
            output
        }
        hax_types::driver_api::Items::FullDef(items) => {
            let items: Vec<_> = items
                .into_iter()
                .filter(|item| {
                    !matches!(
                        item.kind,
                        hax_frontend_exporter::FullDefKind::Use(_)
                            | hax_frontend_exporter::FullDefKind::ExternCrate
                    )
                })
                .collect();
            let items_by_def_id = HashMap::from_iter(
                items
                    .iter()
                    .map(|item| (item.this.contents().def_id.clone(), item)),
            );
            items
                .iter()
                .flat_map(|item| hax_rust_engine::import_thir::import_item(item, &items_by_def_id))
                .collect()
        }
    };

    let files = match &value.backend.backend {
        Backend::Coq | Backend::Ssprove | Backend::Easycrypt | Backend::Lean { .. } => panic!(
            "The Rust engine cannot be called with backend {}.",
            value.backend.backend
        ),
        Backend::Fstar(_) => {
            let mut items = items;
            hax_rust_engine::phase::Phase::apply(&backends::fstar::FStarBackend, &mut items);

            let query = hax_rust_engine::ocaml_engine::QueryKind::Print {
                printer: value.backend.backend,
                input: items,
            };

            let Some(Response::PrintOk) = query.execute(None) else {
                panic!()
            };
            return;
        }
        Backend::LegacyLean => backends::apply_backend(backends::legacy_lean::LeanBackend, items),
        Backend::ProVerif(options) => backends::apply_backend(
            backends::proverif::ProVerifBackend {
                options: options.clone(),
            },
            items,
        ),
        Backend::Rust => backends::apply_backend(backends::rust::RustBackend, items),
        Backend::Debugger { interactive } => {
            use hax_rust_engine::debugger::*;

            if *interactive {
                http_interactive_debugger(items);
                vec![]
            } else {
                let mut state = State {
                    initial_items: items,
                    requests: vec![],
                };

                let contents = match state.apply(Request::DumpAst(DumpAstOptions::default())) {
                    Response::TypedDumpedAst(items) => {
                        serde_json::to_string_pretty(&items).unwrap()
                    }
                    Response::DumpedAst(value) => serde_json::to_string_pretty(&value).unwrap(),
                    _ => todo!(),
                };

                vec![File {
                    path: "ast.json".into(),
                    contents,
                    sourcemap: None,
                }]
            }
        }
        Backend::GenerateRustEngineNames => vec![File {
            path: "generated.rs".into(),
            contents: hax_rust_engine::names::codegen::export_def_ids_to_mod(items),
            sourcemap: None,
        }],
    };
    for file in files {
        hax_rust_engine::hax_io::write(&hax_types::engine_api::protocol::FromEngine::File(file));
    }
}
