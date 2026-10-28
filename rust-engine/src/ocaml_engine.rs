//! This module implements an interface to the OCaml hax engine. Via this
//! interface, the rust engine can communicate with the OCaml engine, and reuse
//! some of its components.

use std::{io::BufRead, sync::OnceLock};

use hax_frontend_exporter::{
    ThirBody,
    id_table::{Table, WithTable},
};
use hax_types::engine_api::protocol::{FromEngine, ToEngine};
use serde::Deserialize;

/// A query for the OCaml engine
#[derive(Debug, Clone, ::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize)]
pub struct Query {
    #[serde(flatten)]
    meta: Meta,
    /// The kind of query we want to send to the engine
    kind: QueryKind,
}

/// The metadata required to perform a query.
#[derive(Debug, Clone, ::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize)]
pub struct Meta {
    /// The version of hax currently used
    pub hax_version: String,
    /// Dictionary from `DefId`s to `impl_infos`
    pub impl_infos: Vec<(
        hax_frontend_exporter::DefId,
        hax_frontend_exporter::ImplInfos,
    )>,
    /// Enable debugging of phases in the OCaml engine
    pub debug_bind_phase: bool,
    /// Enable profiling in the OCaml engine
    pub profiling: bool,
}

static STATE: OnceLock<Meta> = OnceLock::new();

/// Initialize query metadata.
pub fn initialize(meta: Meta) {
    STATE
        .set(meta)
        .expect("`ocaml_engine::initialize` was called more than once")
}

/// The payload of the query. [`Response`] below mirrors this enum to represent
/// the response from the engine.
#[derive(Debug, Clone, ::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize)]
pub enum QueryKind {
    /// Ask the OCaml engine to import the given THIR from the frontend
    ImportThir {
        /// The input THIR items
        input: Vec<hax_frontend_exporter::Item<ThirBody>>,
        /// Translation options which contains include clauses (items filtering)
        translation_options: hax_types::cli_options::TranslationOptions,
    },

    /// Ask the OCaml engine to run given phases on given items
    ApplyPhases {
        /// The phases to run. See `untyped_phases.ml`.
        phases: Vec<String>,
        /// The items on which the phases will be applied.
        input: Vec<crate::ast::Item>,
    },

    /// Ask the OCaml engine to call an OCaml printer
    Print {
        /// Which printer to use
        printer: hax_types::cli_options::Backend,
        /// The items after applying the phases.
        input: Vec<crate::ast::Item>,
    },
}
/// A Response after a [`Query`]
#[derive(Debug, Clone, ::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize)]
pub enum Response {
    /// Return imported THIR as an internal AST from Rust engine
    ImportThir {
        /// The output Rust AST items
        output: Vec<crate::ast::Item>,
        /// The diagnostics reported while importing
        diagnostics: Vec<hax_types::diagnostics::Diagnostics>,
    },
    /// Return items after phase application
    ApplyPhases {
        /// The output Rust AST items after phases
        output: Vec<crate::ast::Item>,
        /// The diagnostics reported while applying the phases
        diagnostics: Vec<hax_types::diagnostics::Diagnostics>,
    },
    /// Printing was done successfully
    PrintOk,
}

/// Extends the common `FromEngine` messages with one extra case: `Response`.
#[derive(Debug, Clone, ::schemars::JsonSchema, ::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum ExtendedFromEngine {
    /// A standard `FromEngine` message
    FromEngine(FromEngine),
    /// A `Response`
    Response(Response),
}

impl QueryKind {
    /// Execute the query synchronously.
    pub fn execute(self, table: Option<Table>) -> Option<Response> {
        let query = Query {
            meta: STATE
                .get()
                .expect("`ocaml_engine::initialize` should be called first")
                .clone(),
            kind: self,
        };
        use std::io::Write;
        use std::process::Command;

        macro_rules! send {
            ($where: expr, $value:expr) => {
                serde_json::to_writer(&mut $where, $value).unwrap();
                $where.write_all(b"\n").unwrap();
                $where.flush().unwrap();
            };
        }

        let mut engine_subprocess =
            Command::new(std::env::var("HAX_ENGINE_BINARY").unwrap_or("hax-engine".into()))
                .arg("driver_rust_engine")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .unwrap();

        let mut stdin = std::io::BufWriter::new(
            engine_subprocess
                .stdin
                .as_mut()
                .expect("Could not write on stdin"),
        );

        if let Some(table) = table {
            WithTable::run(table, query, |with_table| {
                send!(stdin, with_table);
            });
        } else {
            send!(stdin, &(vec![] as Vec<()>, query));
        }

        let mut response = None;
        let stdout = std::io::BufReader::new(engine_subprocess.stdout.take().unwrap());
        // TODO: this should be streaming (i.e. use a `LineAsEOF` reader wrapper that consumes a reader until `\n` occurs)
        //       See https://github.com/pereq-dev/hax/issues/1537.
        for slice in stdout.split(b'\n') {
            let msg = (|| {
                let slice = slice.ok()?;
                let mut de = serde_json::Deserializer::from_slice(&slice);
                de.disable_recursion_limit();
                let de = serde_stacker::Deserializer::new(&mut de);
                let msg = ExtendedFromEngine::deserialize(de);
                msg.ok()
            })()
            .expect(
                "Hax engine sent an invalid json value. \
                                This might be caused by debug messages on stdout, \
                                which is reserved for JSON communication with cargo-hax",
            );

            match msg {
                ExtendedFromEngine::Response(res) => response = Some(res),
                ExtendedFromEngine::FromEngine(FromEngine::Exit) => break,
                // Proxy messages from the OCaml engine
                ExtendedFromEngine::FromEngine(from_engine) => {
                    crate::hax_io::write(&from_engine);
                    if from_engine.requires_response() {
                        let response: ToEngine = crate::hax_io::read_to_engine_message();
                        send!(stdin, &response);
                    }
                }
            }
        }
        drop(stdin);

        let exit_status = engine_subprocess.wait().unwrap();
        if !exit_status.success() {
            panic!("ocaml engine crashed");
        }

        response
    }
}
