//! An interactive debugger server for the rust engine.

use crate::ast::span::Span;
use crate::ast::*;
use crate::phase::Phase as _;
use crate::phase::PhaseKind;
use crate::printer::SourceMap;

macro_rules! declare_printers {
    {$($name:ident = $printer:expr),*$(,)?} => {
        /// Enumeration of all declared printers.
        #[derive(Clone, Debug, Copy, serde::Serialize, serde::Deserialize)]
        pub enum Printer {
            $($name,)*
            /// The printer of a backend
            Backend(Backend),
        }

        impl Printer {
            fn print_items(self, items: Vec<Item>) -> (String, SourceMap) {
                let module = Module {
                    ident: crate::names::rust_primitives::hax,
                    items,
                    meta: Metadata {
                        span: Span::dummy(),
                        attributes: vec![],
                    },
                };
                match self {
                    $(Self::$name => {
                        $printer.print(module)
                    }),*
                    Self::Backend(backend) => backend.print_module(module),
                }
            }
        }
    };
}
macro_rules! declare_backends {
    {$($name:ident = $backend:expr),*$(,)?} => {
        /// Enumeration of all declared backends.
        #[derive(Clone, Debug, Copy, serde::Serialize, serde::Deserialize)]
        pub enum Backend {
            $(
                #[doc = concat!("The ", stringify!($name), " backend.")]
                $name,
            )*
        }

        impl Backend {
            fn phases(self) -> Vec<PhaseKind> {
                use crate::backends::Backend;
                match self {
                    $(
                        Self::$name => $backend.phases(),
                    )*
                }
            }
            fn print_module(self, module: Module) -> (String, SourceMap) {
                use crate::backends::Backend;
                use crate::printer::Print;
                let item_graph = crate::attributes::LinkedItemGraph::new(&module.items, crate::ast::diagnostics::Context::Debugger) ;
                match self {
                    $(
                        Self::$name => $backend.printer(std::rc::Rc::new(item_graph)).print(module),
                    )*
                }
            }
        }
    };
}

declare_backends! {
    LegacyLean = crate::backends::legacy_lean::LeanBackend,
}

declare_printers! {}

/// A request to send to the debugger.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Request {
    /// Apply a given phase to the current items.
    ApplyPhase(PhaseKind),
    /// List the phases applied by a backend.
    ListPhases(Backend),
    /// Print the items with a given printer.
    Print(Printer),
    /// Dump the AST of the current items.
    DumpAst(DumpAstOptions),
}

/// Options one can set when dumping ASTs.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DumpAstOptions {
    /// Sort the items via their global id. The order is not alphabetical, it is just deterministic.
    pub sort_items_by_global_id: bool,
    /// Drop `Use` items.
    pub drop_use_items: bool,
    /// Drop `RustModule` items.
    pub drop_rust_modules_items: bool,
    /// Drop `NotImplementedYet` items.
    pub drop_not_implemented_yet_items: bool,
    /// Drop every attributes in the AST.
    pub drop_attributes: bool,
    /// Erases spans, replacing them by `"erased"`.
    /// Setting this to true will return untyped JSON.
    pub erase_spans: bool,
    /// Erases indices (e.g. local variable indices).
    /// Setting this to true will return untyped JSON.
    pub erase_indices: bool,
}

impl Default for DumpAstOptions {
    fn default() -> Self {
        Self {
            sort_items_by_global_id: true,
            drop_use_items: true,
            drop_rust_modules_items: true,
            drop_not_implemented_yet_items: true,
            drop_attributes: true,
            erase_spans: true,
            erase_indices: true,
        }
    }
}

/// Response given by the debugger.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Response {
    /// Response for `Request::ApplyPhase`: a phase have been applied.
    PhaseApplied(PhaseKind),
    /// Response for `Request::ListPhase`: list the phases for a backend.
    ListedPhases(Vec<PhaseKind>),
    /// Response for `Request::Print`: items have been printed.
    Printed {
        /// The rendered printed items.
        rendered: String,
        /// The sourcemap.
        source_map: SourceMap,
    },
    /// One of the possible response for `Request::DumpAst`. A AST was dumped as a typed JSON.
    TypedDumpedAst(Vec<Item>),
    /// One of the possible response for `Request::DumpAst`. A AST was dumped as an untyped JSON.
    DumpedAst(serde_json::Value),
    /// An error occured.
    Error(String),
}

/// The state against which the debugger is working.
pub struct State {
    /// An immutable vector of items.
    pub initial_items: Vec<Item>,
    /// A sequence of requests.
    pub requests: Vec<Request>,
}

impl State {
    /// Compute the items at the current state
    fn items(&self) -> Vec<Item> {
        let mut items = self.initial_items.clone();
        let phases = self.requests.iter().flat_map(|msg| match msg {
            Request::ApplyPhase(phase) => Some(phase.clone()),
            _ => None,
        });
        for phase in phases {
            phase.apply(&mut items);
        }
        items
    }

    /// Apply the request on a state.
    pub fn apply(&mut self, req: Request) -> Response {
        let mut items = self.items();
        match req {
            Request::ApplyPhase(phase) => {
                phase.apply(&mut items);
                Response::PhaseApplied(phase)
            }
            Request::Print(printer) => {
                let (rendered, source_map) = printer.print_items(items);
                Response::Printed {
                    rendered,
                    source_map,
                }
            }
            Request::DumpAst(options) => {
                let mut items: Vec<_> = items
                    .into_iter()
                    .filter(|it| {
                        let drop = match &it.kind {
                            ItemKind::Use { .. } => options.drop_use_items,
                            ItemKind::RustModule => options.drop_rust_modules_items,
                            ItemKind::NotImplementedYet => options.drop_not_implemented_yet_items,
                            _ => false,
                        };
                        !drop
                    })
                    .collect();
                if options.sort_items_by_global_id {
                    items.sort_by_key(|item| serde_json::to_string_pretty(&item.ident).ok());
                }
                if options.drop_attributes {
                    struct DropAttributes;
                    use crate::ast::visitors::AstVisitorMut;
                    impl AstVisitorMut for DropAttributes {
                        fn visit_metadata(&mut self, x: &mut Metadata) {
                            x.attributes = vec![];
                        }
                        fn visit_param(&mut self, x: &mut Param) {
                            x.attributes = vec![];
                        }
                        fn visit_variant(&mut self, x: &mut Variant) {
                            x.attributes = vec![];
                        }
                    }
                    DropAttributes.visit(&mut items);
                }
                if options.erase_indices || options.erase_spans {
                    let mut items = match serde_json::to_value(items) {
                        Ok(value) => value,
                        Err(err) => return Response::Error(err.to_string()),
                    };
                    use serde_json::Value;

                    fn visit_json<F>(value: &mut Value, f: &F)
                    where
                        F: Fn(&mut Value),
                    {
                        f(value);

                        match value {
                            Value::Array(arr) => {
                                for v in arr {
                                    visit_json(v, f);
                                }
                            }
                            Value::Object(map) => {
                                for (_k, v) in map {
                                    visit_json(v, f);
                                }
                            }
                            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
                        }
                    }

                    let erased = || Value::String("<erased>".to_string());
                    visit_json(&mut items, &|value| {
                        let Value::Object(map) = value else { return };
                        if options.erase_indices {
                            map.iter_mut()
                                .filter(|(k, _)| matches!(k.as_str(), "id" | "index"))
                                .for_each(|(_, value)| {
                                    *value = erased();
                                });
                        }
                        if options.erase_spans
                            && map.contains_key("data")
                            && map.contains_key("owner_hint")
                            && map.contains_key("id")
                        {
                            *value = erased();
                        }
                    });

                    Response::DumpedAst(items)
                } else {
                    Response::TypedDumpedAst(items)
                }
            }
            Request::ListPhases(backend) => Response::ListedPhases(backend.phases()),
        }
    }
}

/// Entrypoint for the interactive HTTP debugger.
pub fn http_interactive_debugger(items: Vec<Item>) {
    use axum::{Json, Router, extract, routing::post};
    use std::sync::Arc;

    async fn process(
        extract::State(items): extract::State<Arc<Vec<Item>>>,
        Json((messages, message)): Json<(Vec<Request>, Request)>,
    ) -> Json<Response> {
        let mut state = State {
            initial_items: items.to_vec(),
            requests: messages,
        };

        Json(state.apply(message))
    }

    async fn serve(items: Vec<Item>) {
        let state = Arc::new(items);

        let app = Router::new()
            .route("/process", post(process))
            .with_state(state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr: std::net::SocketAddr = listener.local_addr().unwrap();
        eprintln!("Listening on http://{addr}");
        axum::serve(listener, app).await.unwrap();
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(serve(items));
}
