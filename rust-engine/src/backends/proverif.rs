//! The ProVerif backend.
//!
//! Prints the hax AST to a single ProVerif `lib.pvl` library file under a
//! uniform-bitstring model: every Rust type collapses to `bitstring`, so the
//! output keeps the term structure (constructors, function calls, control
//! flow) while scalar values become opaque symbolic terms. Names referenced
//! but not defined here are declared by the shipped `primitives.pvl` /
//! `cryptolib.pvl` libraries, or listed in a companion `missingdecl.pvl`
//! diagnostic. The preamble is built by [`header`].

use std::collections::HashSet;
use std::rc::Rc;
use std::sync::OnceLock;

use super::prelude::*;
use crate::ast::identifiers::global_id::TupleId;
use crate::ast::span::Span;
use crate::ast::visitors::AstVisitor;
use crate::phase::*;
use camino::Utf8PathBuf;
use hax_lib_macros_types::AttrPayload;
use hax_types::cli_options::ProVerifOptions;
use hax_types::engine_api::File;

pub(crate) mod analysis;
use analysis::{Analysis, has_error, name_collisions, scan_declared_names, topo_sort};

/// Constructor IDs used by the ProVerif special-cases.
mod names {
    pub use crate::names::core::clone::Clone::clone;
    pub use crate::names::core::convert::Into::into;
    pub use crate::names::core::ops::control_flow::ControlFlow::{
        Break::Constructor as ControlFlowBreak, Continue::Constructor as ControlFlowContinue,
    };
    pub use crate::names::core::ops::deref::Deref::deref;
    pub use crate::names::core::option::Option::None::Constructor as OptionNone;
    pub use crate::names::core::option::Option::Some::Constructor as OptionSome;
    pub use crate::names::rust_primitives::hax::dropped_body;
    pub use crate::names::rust_primitives::hax::never_to_any;
    pub use crate::names::rust_primitives::hax::{array_cons, array_nil};
    pub use crate::names::rust_primitives::hax::{cast_op, logical_op_and, logical_op_or};
    pub use crate::names::rust_primitives::unsize;
}

/// The ProVerif printer.
#[setup_printer_struct]
#[derive(Default, Clone)]
pub struct ProVerifPrinter {
    /// Facts about the crate being printed.
    analysis: Rc<Analysis>,
}

/// Bundled `primitives.pvl`, parsed so its names are not re-declared in
/// `missingdecl.pvl`.
const PRIMITIVES_PVL: &str = include_str!("../../../hax-lib/proof-libs/proverif/primitives.pvl");

/// Bundled `result.pvl` — the default Result model.
const RESULT_PVL: &str = include_str!("../../../hax-lib/proof-libs/proverif/result.pvl");

/// Bundled `result_errors.pvl` — the Result model of `--result-errors`.
const RESULT_ERRORS_PVL: &str =
    include_str!("../../../hax-lib/proof-libs/proverif/result_errors.pvl");

/// The library that models `Result` in the chosen mode, without extension.
fn result_lib(result_errors: bool) -> &'static str {
    if result_errors {
        "result_errors"
    } else {
        "result"
    }
}

/// Preamble baked into every `lib.pvl` file: how to run it and which
/// libraries it needs.
///
/// It ends with a declaration naming a constant that only the matching Result
/// library defines, so loading the other one is an error rather than a model
/// with the wrong semantics. The declaration is named after `krate`, so the
/// models of several crates can be loaded together.
fn header(result_errors: bool, krate: &str) -> String {
    let result_lib = result_lib(result_errors);
    let result_model = if result_errors {
        "  This model was extracted with --result-errors: Ok and Err are
  constructors, so error paths stay reachable."
    } else {
        "  This model was extracted without --result-errors: a Result is its
  Ok payload and producing an Err ends the trace, so error paths are
  pruned from the model."
    };
    format!(
        "\
(*
  Run with:
    proverif -lib <hax>/hax-lib/proof-libs/proverif/primitives.pvl \\
             -lib <hax>/hax-lib/proof-libs/proverif/{result_lib}.pvl \\
             -lib <hax>/hax-lib/proof-libs/proverif/cryptolib.pvl  \\
             -lib missingdecl.pvl \\
             -lib lib.pvl your_analysis.pv

  These libraries ship with hax.

  primitives.pvl declares the symbols this file uses but does not
  define: the public channel c, the construct_fail sink,
  bitstring_default and bitstring_err, the Some/None and True/False
  constructors, the nat_0 to nat_16 literal constants, and the
  rust_primitives, core, and hax_lib helpers.

  {result_lib}.pvl models Result and its methods.
{result_model}

  cryptolib.pvl is the shared Dolev-Yao crypto model (hash, AEAD,
  KDF, DH, KEM, signatures, MAC, serialization) that a crate wires
  itself to with #[hax_lib::proverif::replace_body(\"crypto__...\")].
  It is needed whenever the crate does so, and harmless otherwise.

  missingdecl.pvl, extracted next to this file, declares every symbol
  this file uses that none of these libraries defines, such as string
  literals and integer literals above 16. Each is left abstract:
  replace its declaration by a definition — by hand or from another
  crate's extraction — or keep it as an ideal-functionality boundary.
*)

(* Only typechecks with {result_lib}.pvl loaded. *)
letfun hax_result_model_check__{krate}() = hax_result_model__{result_lib}.

"
    )
}

/// Prefix for the banner [`ProVerifBackend::modules_to_files`] prepends when
/// the printer could not express part of the crate.
///
/// ProVerif answers a query it cannot reach with "no attack found", so a model
/// with a hole reads exactly like a verified one. The banner is for humans; the
/// `hax_extraction_error` query makes every ProVerif run report a failure.
const ERROR_BANNER: &str = "\
(*****************************************************************)
(*  INCOMPLETE MODEL — this extraction did not fully succeed.    *)
(*                                                               *)
(*  Each `HAX ERROR` below marks a construct the ProVerif        *)
(*  backend could not express; it renders as `bitstring_err()`,  *)
(*  so the enclosing declaration has NO value and every process  *)
(*  reaching it dies. Queries over the dead part come back       *)
(*  `true` for lack of any trace, NOT because they hold.         *)
(*                                                               *)
(*  The `hax_extraction_error__*` query below fails on purpose,  *)
(*  so this shows up in the verification summary. Fix the errors *)
(*  (or model the items by hand) before trusting any result.     *)
(*                                                               *)
";

/// Closes [`ERROR_BANNER`], after the one-line-per-error list. The marker is
/// named after `krate`, so the models of several crates can be loaded together.
fn error_banner_end(krate: &str) -> String {
    format!(
        "\
(*****************************************************************)
free hax_extraction_error__{krate}: bitstring.
query attacker(hax_extraction_error__{krate}).

"
    )
}

const INDENT: isize = 2;

/// The unit value `()`.
const UNIT: &str = "rust_primitives__hax__Tuple0__Tuple0";

/// The kinds of `hax__{kind}{n}` binders `ProverifBindFailable` introduces.
const GENERATED_BINDER_KINDS: [&str; 4] = ["scrut", "cond", "arm", "val"];

/// Marks the locals the ProVerif phases generate before
/// `ProverifSimplifyBindings`. No Rust identifier contains it, so a generated
/// local never equals a user's; [`render_local`] drops it.
const GENERATED_LOCAL_MARK: char = '%';

/// A local generated by a ProVerif phase, rendered as `name`.
pub(crate) fn generated_local(name: &str) -> LocalId {
    LocalId(Symbol::new(format!("{GENERATED_LOCAL_MARK}{name}")))
}

/// Whether `s` is an output of [`sanitize_string_literal`].
fn is_sanitized_literal(s: &str) -> bool {
    if s == "_empty" {
        return true;
    }
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_alphanumeric() {
            i += 1;
        } else if bytes[i] == b'_'
            && bytes.get(i + 1) == Some(&b'x')
            && bytes
                .get(i + 2..i + 4)
                .is_some_and(|h| h.iter().all(u8::is_ascii_hexdigit))
        {
            i += 4;
        } else {
            return false;
        }
    }
    !s.is_empty()
}

/// Whether `name` is a global the printer emits inside bodies: a literal
/// constant or a name of `primitives.pvl`. A local of that name would shadow it.
fn is_emitted_global(name: &str) -> bool {
    static NAMES: OnceLock<HashSet<String>> = OnceLock::new();
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    name.strip_prefix("nat_")
        .is_some_and(|n| digits(n.strip_prefix("neg_").unwrap_or(n)))
        || ["string_lit__", "char_lit__", "float_lit__"]
            .iter()
            .any(|prefix| {
                name.strip_prefix(prefix).is_some_and(|rest| {
                    is_sanitized_literal(rest.strip_prefix("neg_").unwrap_or(rest))
                })
            })
        || NAMES
            .get_or_init(|| scan_declared_names(PRIMITIVES_PVL).into_iter().collect())
            .contains(name)
}

/// Whether `name` has the form of a binder the ProVerif phases or printer
/// introduce, or of a global the printer emits, which a Rust local must not
/// take.
pub(crate) fn is_generated_binder(name: &str) -> bool {
    let numbered = |prefix: &str| {
        name.strip_prefix(prefix)
            .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
    };
    name == "wildcard"
        || is_emitted_global(name)
        || numbered("wildcard_")
        || name == "bl__xh"
        || name == "bl__xt"
        || [
            "bl__seq", "bl__acc", "bl__unit", "bl__cf", "bl__brk", "bl__bv", "bl__out",
        ]
        .into_iter()
        .any(numbered)
        || GENERATED_BINDER_KINDS
            .iter()
            .any(|kind| numbered(&format!("hax__{kind}")))
}

/// Depth to which `for`-loops over a collection are unrolled, since ProVerif
/// has no loops or recursion. The model is precise up to this many iterations;
/// see [`ProVerifPrinter::unroll_for_loop_level`].
const LOOP_BOUND: usize = 3;

/// `s` with every `(*` and `*)` broken up, so that it can sit inside a
/// ProVerif comment, which nests.
fn defuse_comment(s: &str) -> String {
    s.replace("*)", "* )").replace("(*", "( *")
}

/// The comment marking a construct the printer could not express. The error
/// banner lists every such comment of the model.
fn error_marker(message: &str) -> String {
    let message = defuse_comment(&message.replace('\n', " "));
    format!("(* HAX ERROR: {message} *)")
}

/// What the diagnostics of an error node report.
fn error_details(error: &ErrorNode) -> String {
    // The OCaml engine's messages start with a generic header line.
    const HEADERS: [&str; 2] = [
        "something is not implemented yet.",
        "Explicit rejection by a phase in the Hax engine:",
    ];
    let details: Vec<String> = error
        .diagnostics
        .iter()
        .filter_map(|d| match &d.info().kind {
            hax_types::diagnostics::Kind::Unimplemented {
                details: Some(details),
                ..
            } => Some(details.clone()),
            hax_types::diagnostics::Kind::ExplicitRejection { reason, .. } => Some(reason.clone()),
            hax_types::diagnostics::Kind::OcamlEngineErrorPayload(payload) => payload
                .lines()
                .map(str::trim)
                .find(|line| !line.is_empty() && !HEADERS.contains(line))
                .map(str::to_string),
            _ => None,
        })
        .collect();
    if details.is_empty() {
        "error node".to_string()
    } else {
        details.join("; ")
    }
}

/// What the error nodes of `e` report, or `fallback` when they carry no
/// details.
fn nested_error_details(e: &Expr, fallback: &str) -> String {
    struct Collect(Vec<String>);
    impl AstVisitor for Collect {
        fn enter_expr_kind(&mut self, kind: &ExprKind) {
            if let ExprKind::Error(error) = kind
                && !error.diagnostics.is_empty()
            {
                let details = error_details(error);
                if !self.0.contains(&details) {
                    self.0.push(details);
                }
            }
        }
    }
    let mut collect = Collect(Vec::new());
    collect.visit_expr(e);
    if collect.0.is_empty() {
        fallback.to_string()
    } else {
        collect.0.join("; ")
    }
}

/// The tool attribute a phase puts on an item or expression to make the
/// printer emit a warning, whose message is the payload. The OCaml engine
/// passes it through unparsed.
const PV_WARNING: &str = "_hax::pv_warning";

/// The tool attribute marking a call of a method of an impl that is not
/// extracted. The printer renders it as an opaque constructor, named after
/// the method with [`UNEXTRACTED_SUFFIX`].
const PV_UNEXTRACTED: &str = "_hax::pv_unextracted_impl";

/// The suffix of the opaque symbol standing for a method whose impl is not
/// extracted.
pub(crate) const UNEXTRACTED_SUFFIX: &str = "__unextracted_impl";

/// The attribute carrying the warning `message`; see [`PV_WARNING`].
pub(crate) fn warning_attribute(message: &str, span: Span) -> Attribute {
    Attribute {
        kind: AttributeKind::Tool {
            path: PV_WARNING.into(),
            tokens: message.into(),
        },
        span,
    }
}

/// The attribute marking a call to a method whose impl is not extracted; see
/// [`PV_UNEXTRACTED`].
pub(crate) fn unextracted_attribute(span: Span) -> Attribute {
    Attribute {
        kind: AttributeKind::Tool {
            path: PV_UNEXTRACTED.into(),
            tokens: String::new(),
        },
        span,
    }
}

/// Whether `meta` carries the tool attribute `path`, and its payload.
fn tool_attribute<'a>(meta: &'a Metadata, path: &str) -> Option<&'a str> {
    meta.attributes.iter().find_map(|a| match &a.kind {
        AttributeKind::Tool { path: p, tokens } if p == path => Some(tokens.as_str()),
        _ => None,
    })
}

/// The tool attribute marking a call that `--keep-trait-calls` keeps, which a
/// hand-written library defines.
const PV_KEPT_CALL: &str = "_hax::pv_kept_trait_call";

/// The attribute marking a kept trait call; see [`PV_KEPT_CALL`].
pub(crate) fn kept_call_attribute(span: Span) -> Attribute {
    Attribute {
        kind: AttributeKind::Tool {
            path: PV_KEPT_CALL.into(),
            tokens: String::new(),
        },
        span,
    }
}

/// Whether `meta` marks a call to a method whose impl is not extracted.
pub(crate) fn is_unextracted_call(meta: &Metadata) -> bool {
    tool_attribute(meta, PV_UNEXTRACTED).is_some()
}

/// The comment marking a branch the model may not follow as Rust does. The
/// warning banner lists every such comment of the model.
fn warning_marker(message: &str) -> String {
    let message = defuse_comment(&message.replace('\n', " "));
    format!("(* HAX WARNING: {message} *)")
}

/// Opens the banner listing the warning markers of a model.
const WARNING_BANNER: &str = "\
(*****************************************************************)
(*  WARNINGS — branches the model may not follow as Rust does.   *)
(*                                                               *)
(*  Each `HAX WARNING` below marks an `if` or `match` on a value *)
(*  the model cannot always evaluate. When it cannot, no arm of  *)
(*  a `match` is taken (the trace ends) and an `if` takes its    *)
(*  `else` branch, whatever Rust does. A query about the other   *)
(*  branch may then hold for lack of any trace.                  *)
(*                                                               *)
";

/// The names the shipped libraries declare, of either Result model.
fn library_names() -> &'static HashSet<String> {
    static NAMES: OnceLock<HashSet<String>> = OnceLock::new();
    NAMES.get_or_init(|| {
        [PRIMITIVES_PVL, RESULT_PVL, RESULT_ERRORS_PVL]
            .into_iter()
            .flat_map(scan_declared_names)
            .collect()
    })
}

/// The width in bits of an integer type, taking pointer-sized ones as 64-bit.
fn int_bits(size: &IntSize) -> u32 {
    match size {
        IntSize::S8 => 8,
        IntSize::S16 => 16,
        IntSize::S32 => 32,
        IntSize::S64 | IntSize::SSize => 64,
        IntSize::S128 => 128,
    }
}

/// Whether the cast `from as to` preserves the value, so that the identity
/// models it: a widening between integers of the same signedness.
fn is_lossless_cast(from: &Ty, to: &Ty) -> bool {
    match (from.kind(), to.kind()) {
        (TyKind::Primitive(PrimitiveTy::Int(from)), TyKind::Primitive(PrimitiveTy::Int(to))) => {
            from.signedness == to.signedness && int_bits(&from.size) <= int_bits(&to.size)
        }
        _ => false,
    }
}

/// The ordering comparisons, which reduce only between small literals.
const ORDERING_COMPARISONS: [&str; 4] = [
    "rust_primitives__hax__machine_int__lt",
    "rust_primitives__hax__machine_int__le",
    "rust_primitives__hax__machine_int__gt",
    "rust_primitives__hax__machine_int__ge",
];

/// The messages of the markers starting with `prefix` in `model`, in order.
fn markers<'a>(model: &'a str, prefix: &str) -> Vec<&'a str> {
    model
        .split(prefix)
        .skip(1)
        .filter_map(|rest| rest.split_once(" *)"))
        .map(|(message, _)| message.trim())
        .collect()
}

/// The messages of the error markers of `model`, in order.
fn error_markers(model: &str) -> Vec<&str> {
    markers(model, "(* HAX ERROR: ")
}

impl RenderView for ProVerifPrinter {
    fn reserved_keywords() -> &'static HashSet<String> {
        static SET: OnceLock<HashSet<String>> = OnceLock::new();
        SET.get_or_init(|| {
            // ProVerif's reserved keywords.
            [
                "among",
                "axiom",
                "channel",
                "choice",
                "clauses",
                "const",
                "def",
                "diff",
                "do",
                "elimtrue",
                "else",
                "equation",
                "equivalence",
                "event",
                "expand",
                "fail",
                "for",
                "forall",
                "foreach",
                "free",
                "fun",
                "get",
                "if",
                "implementation",
                "in",
                "inj-event",
                "insert",
                "lemma",
                "let",
                "letfun",
                "letproba",
                "new",
                "noninterf",
                "noselect",
                "not",
                "nounif",
                "or",
                "otherwise",
                "out",
                "param",
                "phase",
                "pred",
                "proba",
                "process",
                "proof",
                "public_vars",
                "putbegin",
                "query",
                "reduc",
                "restriction",
                "secret",
                "select",
                "set",
                "suchthat",
                "sync",
                "table",
                "then",
                "type",
                "weaksecret",
                "yield",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
        })
    }

    /// Namespaces are flattened with `__`.
    fn separator(&self) -> &str {
        "__"
    }

    /// Escape so the rendered string is a *valid ProVerif identifier*.
    ///
    /// A ProVerif identifier is `[A-Za-z0-9_]+`, must not start with `_`, and
    /// must not be a keyword. Every other byte becomes `_xHH`, a leading `_`
    /// gets a `u` prefix, and a keyword gets a `_kw` suffix.
    ///
    /// It is **not** injective: a literal `_x20` and a space both escape to
    /// `u_x20`, `let` and a literal `let_kw` both to `let_kw`, and joining
    /// segments with `__` cannot tell `mod a__b { fn c }` from
    /// `mod a { fn b__c }`. Two Rust items that collide this way would silently
    /// become one ProVerif symbol, so [`ProVerifBackend::modules_to_files`]
    /// checks the rendered names for duplicates and reports any it finds rather
    /// than relying on the escape alone.
    fn escape(id: &str) -> String {
        let mut escaped = String::with_capacity(id.len());
        for b in id.bytes() {
            if b.is_ascii_alphanumeric() || b == b'_' {
                escaped.push(b as char);
            } else {
                escaped.push_str(&format!("_x{b:02x}"));
            }
        }
        if escaped.is_empty() {
            return "_ERROR_EMPTY_ID_".to_string();
        }
        let escaped = if escaped.starts_with('_') {
            format!("u{escaped}")
        } else {
            escaped
        };
        if Self::is_reserved_keyword(&escaped) {
            format!("{escaped}_kw")
        } else {
            escaped
        }
    }
}

/// The ProVerif identifier of a global. Rendering does not depend on the
/// printer's state.
pub(crate) fn render_id(id: &GlobalId) -> String {
    thread_local!(static PRINTER: ProVerifPrinter = ProVerifPrinter::default());
    PRINTER.with(|printer| printer.render_string(&id.view()))
}

/// The ProVerif identifier of a local. An `impl Trait` parameter is named
/// after its bound.
pub(crate) fn render_local(id: &LocalId) -> String {
    let name = id.0.to_string();
    let name = match name.strip_prefix(GENERATED_LOCAL_MARK) {
        Some(rest) => rest.to_string(),
        None => name,
    };
    let name = match name.strip_prefix("impl ") {
        Some(rest) => format!("impl_{}", rest.replace([' ', '+'], "_")),
        None => name,
    };
    ProVerifPrinter::escape(&name)
}

impl Printer for ProVerifPrinter {
    // ProVerif's flattened identifiers (`bertie__tls13crypto__AeadKeyIV__…`)
    // are long, so an 80-column page forces nearly every declaration to
    // break. A wider page lets short declarations/calls collapse onto one
    // line while still breaking the genuinely long ones.
    const RENDER_WIDTH: usize = 100;
}

impl ProVerifPrinter {
    fn render_id(&self, id: &GlobalId) -> String {
        render_id(id)
    }
}

#[prepend_associated_functions_with(install_pretty_helpers!(self: Self))]
const _: () = {
    macro_rules! line {
        ($($tt:tt)*) => {
            disambiguated_line!($($tt)*)
        };
    }
    macro_rules! concat {
        ($($tt:tt)*) => {
            disambiguated_concat!($($tt)*)
        };
    }

    // Print a separated list of ToDocument values, joined by `, `.
    macro_rules! comma_sep {
        ($it:expr) => {
            intersperse!($it, docs![",", line!()])
        };
    }

    // Wrap a document in parentheses, on one line when it fits and otherwise
    // broken after `(` with the contents indented.
    macro_rules! paren_doc {
        ($d:expr) => {
            docs!["(", docs![line_!(), $d].nest(INDENT), line_!(), ")"].group()
        };
    }

    // Build a grouped, parenthesized, comma-separated list from an iterator
    // of `ToDocument` values. Combines [`comma_sep!`] and [`paren_doc!`].
    macro_rules! paren_list {
        ($it:expr) => {
            paren_doc!(comma_sep!($it))
        };
    }

    impl ProVerifPrinter {
        /// Declaration for the degenerate `f(..) = f(..)` stub the frontend
        /// emits when it cannot resolve an external trait method. ProVerif has
        /// no recursive letfuns, so the name becomes a free (injective, one-way)
        /// symbol — a modelling assumption that is recorded as an error, since
        /// an unresolved helper on a hashed or signed path can hide an attack.
        fn unresolved_trait_method_stub<A: 'static + Clone>(
            &self,
            rendered_name: &str,
            params: &[Param],
        ) -> DocBuilder<A> {
            <Self as PrettyAst<A>>::emit_diagnostic(
                self,
                hax_types::diagnostics::Kind::Unimplemented {
                    issue_id: None,
                    details: Some(format!(
                        "`{rendered_name}` is an unresolved external trait method; its body is \
                         dropped and the name is modelled as an uninterpreted one-way symbol"
                    )),
                },
            );
            let arg_types = comma_sep!(params.iter().map(|p| docs![&p.ty]));
            docs![
                error_marker(&format!(
                    "unresolved external trait method `{rendered_name}`; its body is dropped"
                )),
                hardline!(),
                "(* It is modelled as an uninterpreted injective one-way symbol. *)",
                hardline!(),
                "fun ",
                rendered_name.to_string(),
                paren_doc!(arg_types),
                ": bitstring."
            ]
        }

        /// The placeholder for a Rust pattern with no ProVerif form.
        ///
        /// It must be **refutable**. `hax_unsupported_pattern` is a private
        /// name in `primitives.pvl` that is never output, so the pattern
        /// `(=hax_unsupported_pattern)` matches nothing: the arm is skipped
        /// and the `let … else` chain falls through to the next one. A
        /// `wildcard` binder would do the opposite — turn the arm into a
        /// catch-all that fires on *every* scrutinee, silently widening what
        /// the model accepts.
        fn unsupported_pattern<A: 'static + Clone>(&self, message: &str) -> DocBuilder<A> {
            <Self as PrettyAst<A>>::emit_diagnostic(
                self,
                hax_types::diagnostics::Kind::Unimplemented {
                    issue_id: None,
                    details: Some(message.into()),
                },
            );
            docs!["(=hax_unsupported_pattern) ", error_marker(message)]
        }

        /// Declaration for a nullary item (a Rust `const`/`static`, a no-argument
        /// `fn`, or an associated const).
        ///
        /// When the initializer is printable — which is the common case, since
        /// a `const` body is a constant expression — it is emitted as
        /// `letfun NAME() = <body>.` so the *value* survives into the model.
        /// Dropping it would make the constant an opaque atom, which is unsound
        /// in two ways: a field projection off it can never reduce (the whole
        /// enclosing letfun goes dead), and two constants that are equal in Rust
        /// become distinct terms, so `x == SOME_CONST` is spuriously false under
        /// the structural `PartialEq::eq` model.
        ///
        /// Only a body that cannot be printed (an `Error` node, or the engine's
        /// `dropped_body` marker for an item whose body was never imported)
        /// falls back to an opaque `const NAME: bitstring.`; and if some call
        /// site *applies* the name with arguments — a const closure that is
        /// invoked (`(NO_BLOCK)()`), or an associated const indexed by a
        /// const-generic (`A::<1>`) — neither form fits, so it is declared as an
        /// opaque `fun` of the observed arity.
        fn const_or_applied_fun<A: 'static + Clone>(
            &self,
            rendered_name: &str,
            body: Option<&Expr>,
        ) -> DocBuilder<A> {
            if self.analysis.nullary_letfuns.contains(rendered_name)
                && let Some(body) = body
            {
                return docs![
                    "letfun ",
                    rendered_name.to_string(),
                    "() =",
                    docs![line!(), docs![body]].nest(INDENT).group(),
                    "."
                ];
            }
            // An erroneous body is dropped, so the constant is opaque.
            let marker = match body {
                Some(body) if has_error(body) => {
                    docs![
                        error_marker(&nested_error_details(
                            body,
                            "Constant with an untranslatable body"
                        )),
                        hardline!()
                    ]
                }
                _ => nil!(),
            };
            let decl = match self.analysis.applied_fn_arity(rendered_name) {
                Some(arity) => {
                    // Applied somewhere — declare a `fun` of that arity. A
                    // zero-argument application (`AC()`) yields `fun AC(): ...`.
                    let params: DocBuilder<A> = if arity == 0 {
                        docs!["()"]
                    } else {
                        paren_doc!(comma_sep!((0..arity).map(|_| docs!["bitstring"])))
                    };
                    docs![
                        "(* nullary const applied as a function *)",
                        hardline!(),
                        "fun ",
                        rendered_name.to_string(),
                        params,
                        ": bitstring."
                    ]
                }
                None => docs!["const ", rendered_name.to_string(), ": bitstring."],
            };
            docs![marker, decl]
        }

        /// Opaque `fun` declaration for a function ProVerif cannot express
        /// because it sits in a call-graph cycle (self- or mutually-recursive):
        /// ProVerif letfuns are non-recursive. The body is dropped and the name
        /// declared as a free symbol, so callers stay well-formed.
        ///
        /// This is **not** a sound over-approximation. A free ProVerif symbol is
        /// injective and one-way, which the real function need not be: if it is
        /// constant, or collides on two inputs, the model claims a disequality
        /// that does not hold and hides whatever an attacker could do with that
        /// collision. It is a *modelling assumption* — "treat this computation
        /// as an unrelated one-way function" — and it is recorded as an error so
        /// the file's `hax_extraction_error` marker fires.
        fn recursive_opaque_fun<A: 'static + Clone>(
            &self,
            rendered_name: &str,
            params: &[Param],
            span: Span,
        ) -> DocBuilder<A> {
            // Report against the item's explicit span: the contextual span is
            // not reliably the enclosing item in item position.
            use crate::printer::pretty_ast::HasContextualSpan;
            <Self as PrettyAst<A>>::emit_diagnostic(
                &self.with_span(span),
                hax_types::diagnostics::Kind::Unimplemented {
                    issue_id: None,
                    details: Some(
                        "ProVerif has no recursive functions; this function is in a \
                         call-graph cycle, so its body is dropped and the name is modelled \
                         as an uninterpreted (injective, one-way) symbol"
                            .into(),
                    ),
                },
            );
            let arg_types = comma_sep!(params.iter().map(|p| docs![&p.ty]));
            docs![
                error_marker(&format!(
                    "`{rendered_name}` is recursive; its body is dropped"
                )),
                hardline!(),
                "(* It is modelled as an uninterpreted injective one-way symbol, which *)",
                hardline!(),
                "(* the real function need not be. Replace with a hand-written model  *)",
                hardline!(),
                "(* before trusting any result that depends on it.                    *)",
                hardline!(),
                "fun ",
                rendered_name.to_string(),
                paren_doc!(arg_types),
                ": bitstring."
            ]
        }

        /// A "trivial" binder is one whose rendered pattern is just `x`
        /// (or `_`) — an irrefutable pattern that always matches, so it needs
        /// no `else` fallback.
        ///
        /// We mirror the special cases in `pat`: `Ascription`, and `Ok(inner)`
        /// under the default Result model, are transparent (they just render
        /// their inner pat), so trivialness propagates through them.
        fn is_trivial_binder(&self, pat: &Pat) -> bool {
            match &*pat.kind {
                PatKind::Wild => true,
                PatKind::Binding {
                    sub_pat: None,
                    mode: BindingMode::ByValue,
                    mutable: false,
                    ..
                } => true,
                PatKind::Ascription { pat, .. } => self.is_trivial_binder(pat),
                _ => false,
            }
        }

        /// Render `expr` in a position that would otherwise be parenthesized,
        /// dropping the parens when [`expr_is_atomic`] proves they are
        /// redundant. Used for `let` RHSs, the `if`-rewrite branches, and
        /// match-arm bodies — turning `let x = (f(a)) in` into the readable
        /// `let x = f(a) in` while keeping parens around compound terms.
        fn paren_unless_atomic<A: 'static + Clone>(&self, expr: &Expr) -> DocBuilder<A> {
            if expr_is_atomic(expr) {
                docs![expr]
            } else {
                docs![expr].parens()
            }
        }

        /// Render a match arm's pattern in `let`-binder position: a constant
        /// matches by equality (`(=lit)`), any other pattern binds
        /// structurally.
        fn let_pat<A: 'static + Clone>(&self, pat: &Pat) -> DocBuilder<A> {
            match &*pat.kind {
                PatKind::Constant { lit } => docs!["=", lit].parens(),
                _ => docs![pat],
            }
        }

        /// Render `if <cond> { <then> } else { <else_> }`. `cond` cannot fail:
        /// ProVerif takes the `else` branch also when it fails.
        /// Why the model may be unable to evaluate the branching value `e`:
        /// it contains an ordering comparison or a call of a function the
        /// model does not define.
        fn opaque_branch_reason(&self, e: &Expr) -> Option<String> {
            struct Find<'a> {
                analysis: &'a Analysis,
                reason: Option<String>,
            }
            impl AstVisitor for Find<'_> {
                // A library defines the kept trait calls.
                fn visit_expr(&mut self, e: &Expr) {
                    if tool_attribute(&e.meta, PV_KEPT_CALL).is_some() {
                        if let ExprKind::App { args, .. } = &*e.kind {
                            args.iter().for_each(|arg| self.visit_expr(arg));
                        }
                        return;
                    }
                    self.visit_inner(e)
                }

                fn enter_expr_kind(&mut self, kind: &ExprKind) {
                    if self.reason.is_some() {
                        return;
                    }
                    if let ExprKind::App { head, .. } = kind
                        && let ExprKind::GlobalId(g) = &*head.kind
                    {
                        let name = render_id(g);
                        if ORDERING_COMPARISONS.contains(&name.as_str()) {
                            self.reason = Some(
                                "branch on an ordering comparison, which the model decides only between literals up to 16"
                                    .into(),
                            );
                        } else if !self.analysis.calls.contains_key(&name)
                            && !self.analysis.data_constructors.contains(&name)
                            && !self.analysis.accessor_owner.contains_key(&name)
                            && !self.analysis.quote_declared.contains(&name)
                            && !library_names().contains(&name)
                            && !is_passthrough(head)
                        {
                            self.reason = Some(format!(
                                "branch on a result of `{name}`, which the model does not define"
                            ));
                        }
                    }
                }
            }
            let mut find = Find {
                analysis: &self.analysis,
                reason: None,
            };
            find.visit_expr(e);
            find.reason
        }

        /// `doc`, preceded by a warning if the branching value `e` may be
        /// opaque; see [`Self::opaque_branch_reason`].
        fn warn_opaque_branch<A: 'static + Clone>(
            &self,
            e: &Expr,
            doc: DocBuilder<A>,
        ) -> DocBuilder<A> {
            match self.opaque_branch_reason(e) {
                Some(reason) => docs![warning_marker(&reason), hardline!(), doc],
                None => doc,
            }
        }

        fn if_then_else<A: 'static + Clone>(
            &self,
            cond: &Expr,
            then: DocBuilder<A>,
            else_: DocBuilder<A>,
        ) -> DocBuilder<A> {
            docs![
                "let (=True()) = ",
                self.paren_unless_atomic(cond),
                line!(),
                "in ",
                then,
                line!(),
                "else ",
                else_
            ]
            .nest(INDENT)
            .group()
        }

        /// Render an (already or-pattern-expanded and truncated) arm list as a
        /// ProVerif `let … in … else …` chain.
        ///
        /// The chain is built right-to-left so each arm closes over its
        /// *continuation*: the code reached when the arm's pattern does not
        /// destructure the scrutinee. When no arm matches, the chain bottoms
        /// out in `bitstring_err()`.
        ///
        /// A failable arm's body is wrapped in `( … )` so the `else` after it
        /// binds to *this* arm's `let`, not to an inner destructure in the
        /// body. An arm whose pattern is a trivial binder (e.g. `x`, or `Ok(x)`
        /// under the default Result model) always matches, so it ends the
        /// chain with no `else`.
        fn render_arm_chain<A: 'static + Clone>(
            &self,
            scrutinee: &Expr,
            arms: &[&Arm],
        ) -> DocBuilder<A> {
            let Some((arm, rest)) = arms.split_first() else {
                return docs!["bitstring_err()"];
            };
            if arm.guard.is_some() {
                unreachable_by_invariant!(Proverif_bind_failable)
            }
            match &*arm.pat.kind {
                // A bare wildcard is the fallback: its body runs unconditionally.
                PatKind::Wild => docs![&arm.body],
                _ => {
                    let pat = self.let_pat(&arm.pat);
                    if self.is_trivial_binder(&arm.pat) {
                        let body = docs![&arm.body];
                        docs![
                            "let ",
                            pat,
                            " = ",
                            scrutinee,
                            " in",
                            docs![line!(), body].nest(INDENT).group()
                        ]
                    } else {
                        let body = self.paren_unless_atomic(&arm.body);
                        docs![
                            "let ",
                            pat,
                            " = ",
                            scrutinee,
                            " in",
                            docs![line!(), body].nest(INDENT).group(),
                            hardline!(),
                            "else ",
                            self.render_arm_chain(scrutinee, rest)
                        ]
                    }
                }
            }
        }

        /// Emit one level `k` of the bounded unrolling of
        /// `for <pat> in <iterator> { <body> }`; see [`Self::unroll_for_loop`].
        ///
        /// Level `k` destructures the head element off the Seq `bl__seq{k}`
        /// (`let array_cons(<pat>, bl__seq{k+1}) = bl__seq{k}`), runs the body
        /// once and recurses. With a `state`, the accumulator is threaded
        /// through `bl__acc{k}`; without, the loop yields unit. The prefix has
        /// no leading underscore, which ProVerif's lexer rejects.
        ///
        /// When the loop `breaks`, the body yields a `ControlFlow`: `Continue`
        /// carries the next accumulator, and `Break(((), acc))` ends the loop
        /// with `acc`.
        fn unroll_for_loop_level<A: 'static + Clone>(
            &self,
            k: usize,
            pat: &Pat,
            body: &Expr,
            state: Option<&LoopState>,
            breaks: bool,
        ) -> DocBuilder<A> {
            let cons = self.render_id(&names::array_cons);
            let nil = self.render_id(&names::array_nil);
            let (seq, next_seq) = (format!("bl__seq{k}"), format!("bl__seq{}", k + 1));
            let (acc, next_acc) = (format!("bl__acc{k}"), format!("bl__acc{}", k + 1));
            // The head did not destructure. That is "the Seq is empty" only when
            // it really is `array_nil`; any other term (a byte blob, an opaque
            // slice) is a collection we cannot walk, and returning the
            // accumulator there would claim the loop is done.
            let not_cons = format!(
                "(let {nil}() = {seq} in {acc} else hax_loop_opaque_seq({acc}, {seq}) \
                 (* iterator is not a cons-list *))"
            );
            if k >= LOOP_BOUND {
                return match state {
                    // A remaining head means the collection is longer than the
                    // unrolling: the accumulator is an unknown function of what
                    // is left.
                    Some(_) => docs![format!(
                        "let {cons}(bl__xh, bl__xt) = {seq} in hax_loop_unbounded({acc}, {seq}) \
                         (* more than LOOP_BOUND elements *) else {not_cons}"
                    )],
                    // A loop with no accumulator yields unit whatever the
                    // remaining iterations do, so the model skips them. Ending
                    // the trace instead would make every query past the loop hold
                    // vacuously; skipping ignores an iteration that would have
                    // failed.
                    None => docs![UNIT, " (* more than LOOP_BOUND elements *)"],
                };
            }
            let inner = self.unroll_for_loop_level(k + 1, pat, body, state, breaks);
            let head = docs![
                format!("let {cons}("),
                pat,
                format!(", {next_seq}) = {seq} in (")
            ];
            let (bind_state, next, otherwise) = match state {
                Some(state) => (
                    docs!["let ", &state.body_pat, format!(" = {acc} in"), hardline!()],
                    next_acc,
                    not_cons,
                ),
                None => (nil!(), format!("bl__unit{k}"), format!("({UNIT})")),
            };
            let run = if breaks {
                let (cf, brk) = (format!("bl__cf{k}"), format!("bl__brk{k}"));
                let exit = match state {
                    Some(_) => format!(
                        "(let {}(bl__bv{k}, bl__out{k}) = {brk} in bl__out{k} else bitstring_err())",
                        self.render_id(&TupleId::Constructor { length: 2 }.into())
                    ),
                    None => format!("({UNIT})"),
                };
                docs![
                    format!("let {cf} = ("),
                    body,
                    ") in",
                    hardline!(),
                    format!(
                        "let {}({brk}) = {cf} in {exit}",
                        self.render_id(&names::ControlFlowBreak)
                    ),
                    hardline!(),
                    format!(
                        "else let {}({next}) = {cf} in (",
                        self.render_id(&names::ControlFlowContinue)
                    ),
                    inner,
                    ") else bitstring_err()"
                ]
            } else {
                docs![format!("let {next} = ("), body, ") in", hardline!(), inner]
            };
            docs![head, bind_state, run, ") else ", otherwise]
        }

        /// The bounded unrolling of `for <pat> in <iterator> { <body> }`, since
        /// ProVerif has no loops. The iterator is a cons-list ("Seq") over
        /// `array_cons`/`array_nil`, walked one element per level for
        /// `LOOP_BOUND` levels; no integers are involved.
        fn unroll_for_loop<A: 'static + Clone>(
            &self,
            pat: &Pat,
            iterator: &Expr,
            body: &Expr,
            state: Option<&LoopState>,
            breaks: bool,
        ) -> DocBuilder<A> {
            let init = match state {
                Some(state) => docs!["let bl__acc0 = (", &state.init, ") in "],
                None => nil!(),
            };
            docs![
                "(let bl__seq0 = (",
                iterator,
                ") in ",
                init,
                hardline!(),
                self.unroll_for_loop_level(0, pat, body, state, breaks),
                ")"
            ]
        }

        /// Attach `span` as a `pretty` document annotation, but only when the
        /// printer's annotation type `A` is actually `Span` — which is the case
        /// during real rendering, since the [`Printer`] trait pins `A = Span`.
        /// The `PrettyAst<A>` methods are generic in `A`, so this guarded helper
        /// lets them record per-item provenance (harvested by
        /// the printer's [`crate::printer::SourceMap`]) without constraining `A`;
        /// for any other `A` it is a no-op. Annotations never affect layout, so
        /// the rendered text is unchanged either way.
        fn annotate_with_span<A: 'static + Clone>(doc: DocBuilder<A>, span: Span) -> DocBuilder<A> {
            match (&span as &dyn std::any::Any).downcast_ref::<A>() {
                Some(a) => doc.annotate(a.clone()),
                None => doc,
            }
        }

        /// Emit a source-provenance comment
        /// `(* src: file:lo_line:lo_col-hi_line:hi_col name *)` above a
        /// generated declaration, tying the ProVerif output back to the Rust
        /// definition it came from (the side-by-side / explainability anchor).
        /// The full span keeps items traceable even after the declaration order
        /// is topologically sorted. Only emitted for an item with a real source
        /// span in the extracted crate: phase-synthesized nodes
        /// (`Span::dummy()`) have no file, and the path of a file elsewhere is
        /// absolute, which would make the output depend on where it is built.
        fn src_comment<A: 'static + Clone>(&self, span: Span, name: &GlobalId) -> DocBuilder<A> {
            if let Some(fs) = span.as_frontend_spans().first()
                && let Some(path) = fs.filename.to_path()
                && path.is_relative()
            {
                return docs![
                    format!(
                        "(* src: {}:{}:{}-{}:{} {} *)",
                        path.display(),
                        fs.lo.line,
                        fs.lo.col,
                        fs.hi.line,
                        fs.hi.col,
                        self.render_id(name)
                    ),
                    hardline!()
                ];
            }
            nil!()
        }

        /// Render the leading Rust doc comments (`///` / `//!`) of an item's
        /// attribute list as ProVerif `(* … *)` comment lines, so the
        /// generated model carries the same prose that documents the Rust
        /// source — the readability goal of side-by-side translation. Tool
        /// and Hax attributes carry no ProVerif meaning and are skipped.
        ///
        /// Each physical line of the doc body becomes its own `(* … *)` so we
        /// never hand the `pretty` layout engine a string with embedded
        /// newlines (it counts columns and would mislay the rest of the line).
        /// Any `(*` / `*)` appearing inside the prose is defused (`( *` / `* )`)
        /// so it can neither open nor prematurely close the surrounding
        /// ProVerif comment.
        fn doc_comments<A: 'static + Clone>(&self, attrs: &[Attribute]) -> DocBuilder<A> {
            let mut doc = nil!();
            for a in attrs {
                if let AttributeKind::DocComment { body, .. } = &a.kind {
                    if body.is_empty() {
                        doc = docs![doc, "(* *)", hardline!()];
                        continue;
                    }
                    for line in body.lines() {
                        doc = docs![
                            doc,
                            "(*",
                            if line.is_empty() {
                                String::new()
                            } else {
                                format!(" {} ", defuse_comment(line.trim_end()))
                            },
                            "*)",
                            hardline!()
                        ];
                    }
                }
            }
            doc
        }

        /// The declaration of a function or method.
        fn fn_decl<A: 'static + Clone>(
            &self,
            name: &GlobalId,
            params: &[Param],
            body: &Expr,
            meta: &Metadata,
        ) -> DocBuilder<A> {
            if let Some(message) = tool_attribute(meta, PV_WARNING) {
                let mut plain = meta.clone();
                plain.attributes.retain(
                    |a| !matches!(&a.kind, AttributeKind::Tool { path, .. } if path == PV_WARNING),
                );
                return docs![
                    warning_marker(message),
                    hardline!(),
                    self.fn_decl(name, params, body, &plain)
                ];
            }
            let rendered = self.render_id(name);
            let has = |attr: fn(&AttrPayload) -> bool| meta.hax_attributes().any(attr);
            let as_constructor = has(|a| matches!(a, AttrPayload::PVConstructor));
            let is_erased = has(|a| matches!(a, AttrPayload::Erased));
            let as_handwritten = has(|a| matches!(a, AttrPayload::PVHandwritten));
            let is_self_call = matches!(
                &*body.kind,
                ExprKind::App { head, .. } if matches!(&*head.kind, ExprKind::GlobalId(g) if g == name)
            );
            if params.is_empty() {
                // Rust consts; see `const_or_applied_fun`.
                self.const_or_applied_fun(&rendered, Some(body))
            } else if as_constructor || is_erased {
                // `#[hax_lib::pv_constructor]` and `#[hax_lib::opaque]` both
                // render as a free `[data]` constructor: an opaque function
                // with no equations.
                let header = if as_constructor {
                    "(* marked as constructor *)"
                } else {
                    "(* opaque *)"
                };
                docs![
                    header,
                    hardline!(),
                    "fun ",
                    rendered,
                    paren_doc!(comma_sep!(params.iter().map(|p| docs![&p.ty]))),
                    ": bitstring [data]."
                ]
            } else if is_self_call && !as_handwritten {
                // `f(..) = f(..)` is the stub the frontend emits for an
                // unresolvable external trait method.
                self.unresolved_trait_method_stub(&rendered, params)
            } else if self.analysis.recursive_fns.contains(&rendered) {
                self.recursive_opaque_fun(&rendered, params, meta.span)
            } else {
                let params = paren_doc!(comma_sep!(params.iter().map(|p| docs![p])));
                let (comment, body) = if as_handwritten {
                    (
                        docs![
                            error_marker(&format!("REPLACE `{rendered}` by a hand-written model")),
                            hardline!()
                        ],
                        docs!["bitstring_default()"],
                    )
                } else {
                    (nil!(), docs![body])
                };
                docs![
                    comment,
                    "letfun ",
                    rendered,
                    params,
                    " =",
                    docs![line!(), body].nest(INDENT).group(),
                    "."
                ]
            }
        }

        /// Print the `fun ... [data].` declaration and the matching `reduc`
        /// lines that recover each field. Every constructor returns
        /// `bitstring` because the surface type collapses uniformly.
        fn fun_and_reduc<A: 'static + Clone>(&self, variant: &Variant) -> DocBuilder<A> {
            let constructor_name = self.render_id(&variant.name);
            let typed_args_vec: Vec<(GlobalId, Ty)> = variant
                .arguments
                .iter()
                .map(|(id, ty, _)| (*id, ty.clone()))
                .collect();
            let arg_types_doc = paren_list!(typed_args_vec.iter().map(|(_, ty)| docs![ty]));

            let fun_line = docs![
                "fun ",
                constructor_name.clone(),
                arg_types_doc,
                ": bitstring [data]."
            ];

            if typed_args_vec.is_empty() {
                return fun_line;
            }

            // A field projection `self.f` is an `App` whose head is the field's
            // `GlobalId`, so each destructor is named exactly like its field.
            // That means the `reduc forall ...;` header must NOT bind any
            // variable with the same rendered name, or ProVerif emits a
            // "rebound" warning and shadows the destructor function name
            // inside the rule. The forall-bound variables are LOCAL to the
            // rule and only need to be fresh w.r.t. the (fully-qualified,
            // global) accessor/constructor function symbols used in it, so we
            // give them short positional names `v_0`, `v_1`, ….
            let bind_names: Vec<String> = (0..typed_args_vec.len())
                .map(|i| format!("v_{i}"))
                .collect();
            let fun_args_full: DocBuilder<A> = comma_sep!(
                typed_args_vec
                    .iter()
                    .enumerate()
                    .map(|(i, (_, ty))| { docs![bind_names[i].clone(), ": ", ty] })
            );
            let fun_args_names: DocBuilder<A> =
                comma_sep!((0..typed_args_vec.len()).map(|i| docs![bind_names[i].clone()]));

            let reduc_pieces: Vec<DocBuilder<A>> = typed_args_vec
                .iter()
                .enumerate()
                .map(|(i, (id, _ty))| {
                    let accessor = self.render_id(id);
                    let constructor_call =
                        docs![constructor_name.clone(), paren_doc!(fun_args_names.clone())];
                    let accessor_call = docs![accessor, paren_doc!(constructor_call)];
                    docs![
                        docs!["reduc forall ", fun_args_full.clone()]
                            .nest(INDENT)
                            .group(),
                        ";",
                        docs![line!(), accessor_call, " = ", bind_names[i].clone()]
                            .nest(INDENT)
                            .group()
                    ]
                })
                .collect();
            let reduc_doc = intersperse!(reduc_pieces, docs![".", hardline!()]);

            docs![fun_line, hardline!(), reduc_doc, "."]
        }
    }

    impl<A: 'static + Clone> PrettyAst<A> for ProVerifPrinter {
        const NAME: &'static str = "ProVerif";

        /// The placeholder for anything the printer cannot express.
        ///
        /// It must be a *term*: a bare `(* ... *)` is not one, and a letfun
        /// body made of a comment does not parse. `bitstring_err()` is
        /// `primitives.pvl`'s "value we do not have", so the enclosing declaration
        /// stays well-formed while the trailing comment names the gap. The
        /// enclosing letfun then has no value, which is why every such gap also
        /// arms the file-level `hax_extraction_error` query — on its own, a
        /// failing letfun is indistinguishable from a verified one.
        fn todo_document(&self, message: &str, issue_id: Option<u32>) -> DocBuilder<A> {
            <Self as PrettyAst<A>>::emit_diagnostic(
                self,
                hax_types::diagnostics::Kind::Unimplemented {
                    issue_id,
                    details: Some(message.into()),
                },
            );
            let issue = issue_id
                .map(|id| format!(" (see hax issue #{id})"))
                .unwrap_or_default();
            docs![
                "bitstring_err() ",
                error_marker(&format!("{message}{issue}"))
            ]
        }

        fn unimplemented_method(
            &self,
            method: &str,
            _ast: fragment::FragmentRef<'_>,
        ) -> DocBuilder<A> {
            self.todo_document(
                &format!("the ProVerif printer does not implement `{method}`"),
                None,
            )
        }

        fn module(&self, module: &Module) -> DocBuilder<A> {
            // ProVerif is single-file. Each module just lays its items down,
            // separated by hard newlines, each after the functions and types it
            // uses.
            let names = |item: &Item| match &item.kind {
                ItemKind::Impl { items, .. } => {
                    items.iter().map(|ii| self.render_id(&ii.ident)).collect()
                }
                ItemKind::Fn { name, .. } => vec![self.render_id(name)],
                ItemKind::Quote { quote, .. } => {
                    let mut names = scan_declared_names(&analysis::quote_text(quote));
                    names.push(self.render_id(&item.ident));
                    names
                }
                ItemKind::Type { variants, .. } => variants
                    .iter()
                    .flat_map(|v| {
                        std::iter::once(v.name)
                            .chain(v.arguments.iter().map(|(field, _, _)| *field))
                    })
                    .map(|id| self.render_id(&id))
                    .collect(),
                _ => vec![],
            };
            let items = topo_sort(&module.items, names, &self.analysis);
            docs![intersperse!(
                items.into_iter().map(|item| { item.to_document(self) }),
                hardline!()
            )]
        }

        fn global_id(&self, global_id: &GlobalId) -> DocBuilder<A> {
            docs![self.render_id(global_id)]
        }

        fn local_id(&self, local_id: &LocalId) -> DocBuilder<A> {
            if self.analysis.erased_const_generics.contains(local_id) {
                return docs!["bitstring_default()"];
            }
            docs![render_local(local_id)]
        }

        fn literal(&self, literal: &Literal) -> DocBuilder<A> {
            match literal {
                Literal::Bool(true) => docs!["True()"],
                Literal::Bool(false) => docs!["False()"],
                // ProVerif has no string, char, float or large integer terms:
                // each such literal is an opaque constant named after its value.
                _ => docs![lit_const_name(literal).unwrap_or_default()],
            }
        }

        fn primitive_ty(&self, _primitive_ty: &PrimitiveTy) -> DocBuilder<A> {
            // Booleans and integers are the `True()`/`False()`/`nat_N`
            // terms of `primitives.pvl`.
            docs!["bitstring"]
        }

        fn int_kind(&self, _int_kind: &IntKind) -> DocBuilder<A> {
            docs!["bitstring"]
        }

        fn float_kind(&self, _float_kind: &FloatKind) -> DocBuilder<A> {
            docs!["bitstring"]
        }

        fn symbol(&self, symbol: &Symbol) -> DocBuilder<A> {
            docs![Self::escape(symbol.as_ref())]
        }

        fn ty(&self, ty: &Ty) -> DocBuilder<A> {
            // Uniform-bitstring: every type renders as `bitstring`.
            // `Param(_)` survives — Specialize doesn't monomorphize generic
            // *functions*, only specific named arithmetic/conversion calls —
            // but a `Param` in output position is meaningless to ProVerif, so
            // we still flatten to `bitstring`.
            match ty.kind() {
                TyKind::Ref { .. } => unreachable_by_invariant!(Drop_references),
                TyKind::RawPointer => unreachable_by_invariant!(Reject_raw_or_mut_pointer),
                TyKind::Dyn(_) => unreachable_by_invariant!(Reject_dyn),
                // A type must stay a type.
                TyKind::Error(_) => docs!["bitstring ", error_marker("error type")],
                _ => docs!["bitstring"],
            }
        }

        fn pat(&self, pat: &Pat) -> DocBuilder<A> {
            match &*pat.kind {
                PatKind::Wild => unreachable_by_invariant!(Proverif_name_wildcards),
                PatKind::Constant { lit } => docs!["=", lit].parens(),
                PatKind::Binding {
                    mutable,
                    var,
                    mode,
                    sub_pat,
                } => match (mutable, mode, sub_pat) {
                    (true, _, _) => unreachable_by_invariant!(Local_mutation),
                    (false, BindingMode::ByRef(_), _) => unreachable_by_invariant!(Drop_references),
                    (false, BindingMode::ByValue, None) => docs![var],
                    (false, BindingMode::ByValue, Some(p)) => docs![var, ": ", &p.ty],
                },
                PatKind::Ascription { pat, ty: _ } => docs![pat],
                PatKind::Construct {
                    constructor,
                    fields: _,
                    is_record: _,
                    is_struct: _,
                } if *constructor == names::OptionNone => docs!["None()"],
                PatKind::Construct {
                    constructor,
                    fields,
                    is_record: _,
                    is_struct: _,
                } if *constructor == names::OptionSome => {
                    // Every payload is already a `bitstring`, so the inner
                    // pattern is rendered directly.
                    if let Some((_, inner)) = fields.first() {
                        docs!["Some", docs![inner].parens()]
                    } else {
                        docs!["Some()"]
                    }
                }
                PatKind::Construct {
                    constructor,
                    fields,
                    is_record: _,
                    is_struct: _,
                } => {
                    // Every field is rendered (`_` is a named binder by now), so a
                    // multi-field pattern always keeps the constructor's arity.
                    // ProVerif rejects a 0-arg `Constructor()` on a multi-field
                    // constructor (e.g. a `(_, _)` tuple catch-all), so a full
                    // `Tuple2(wildcard, wildcard)` is the always-matching form.
                    let args = if fields.is_empty() {
                        nil!()
                    } else {
                        comma_sep!(fields.iter().map(|(_, p)| {
                            // A bare binding carries its type ascription.
                            match &*p.kind {
                                PatKind::Binding {
                                    sub_pat: None,
                                    var,
                                    mutable: false,
                                    mode: BindingMode::ByValue,
                                } => docs![var, ": ", &p.ty],
                                _ => docs![p],
                            }
                        }))
                    };
                    if fields.is_empty() {
                        docs![constructor, "()"]
                    } else {
                        docs![constructor, paren_doc!(args)]
                    }
                }
                PatKind::Or { .. } => {
                    self.unsupported_pattern("ProVerif backend does not support or-patterns")
                }
                PatKind::Array { .. } => {
                    self.unsupported_pattern("ProVerif backend does not support array patterns")
                }
                PatKind::Deref { .. } => unreachable_by_invariant!(Drop_references),
                PatKind::Resugared(_) => {
                    self.unsupported_pattern("ProVerif backend does not support resugared patterns")
                }
                // A phase already reported why this node is an error.
                PatKind::Error(error) => {
                    docs![
                        "(=hax_unsupported_pattern) ",
                        error_marker(&error_details(error))
                    ]
                }
            }
        }

        fn expr(&self, expr: &Expr) -> DocBuilder<A> {
            if let Some(message) = tool_attribute(&expr.meta, PV_WARNING) {
                let mut plain = expr.clone();
                plain.meta.attributes.retain(
                    |a| !matches!(&a.kind, AttributeKind::Tool { path, .. } if path == PV_WARNING),
                );
                return docs![warning_marker(message), " ", &plain];
            }
            match &*expr.kind {
                ExprKind::App { head, args, .. }
                    if is_unextracted_call(&expr.meta)
                        && matches!(&*head.kind, ExprKind::GlobalId(_)) =>
                {
                    let ExprKind::GlobalId(g) = &*head.kind else {
                        unreachable!()
                    };
                    docs![
                        format!("{}{UNEXTRACTED_SUFFIX}", self.render_id(g)),
                        paren_doc!(comma_sep!(args.iter().map(|a| docs![a])))
                    ]
                }
                // A cast that may change the value is still the identity: a
                // documented approximation, with a warning.
                ExprKind::App { head, args, .. }
                    if matches!(&*head.kind, ExprKind::GlobalId(g) if *g == names::cast_op)
                        && matches!(&args[..], [arg] if !is_lossless_cast(&arg.ty, &expr.ty)) =>
                {
                    let arg = &args[0];
                    docs![
                        warning_marker(
                            "`as` cast modelled as the identity, although it may change the value"
                        ),
                        " ",
                        arg
                    ]
                }
                ExprKind::App { head, args, .. } if is_passthrough(head) => {
                    args.first().map(|a| docs![a]).unwrap_or(nil!())
                }
                ExprKind::App { head, .. } if matches!(&*head.kind, ExprKind::GlobalId(g) if *g == names::never_to_any) =>
                {
                    docs!["bitstring_err()"]
                }

                // `&&` and `||` evaluate their right operand only when the left
                // one does not decide the result.
                ExprKind::App { head, args, .. }
                    if args.len() == 2
                        && matches!(&*head.kind, ExprKind::GlobalId(g)
                            if *g == names::logical_op_and || *g == names::logical_op_or) =>
                {
                    let is_and =
                        matches!(&*head.kind, ExprKind::GlobalId(g) if *g == names::logical_op_and);
                    let rhs = self.paren_unless_atomic(&args[1]);
                    let (on_true, on_false, opaque) = if is_and {
                        (rhs, docs!["False()"], "logical_and_opaque")
                    } else {
                        (docs!["True()"], rhs, "logical_or_opaque")
                    };
                    // A left operand that is neither `True()` nor `False()` (an
                    // un-modelled comparison, say) gives an opaque result. It
                    // cannot fail, so it can be tested twice.
                    let lhs = self.paren_unless_atomic(&args[0]);
                    docs![
                        "let (=True()) = ",
                        lhs.clone(),
                        line!(),
                        "in ",
                        on_true,
                        line!(),
                        "else let (=False()) = ",
                        lhs.clone(),
                        line!(),
                        "in ",
                        on_false,
                        line!(),
                        "else ",
                        opaque,
                        "(",
                        lhs,
                        ")"
                    ]
                    .nest(INDENT)
                    .group()
                }

                // ===== Construct: Some / None / generic =====
                ExprKind::Construct { constructor, .. } if *constructor == names::OptionNone => {
                    docs!["None()"]
                }
                ExprKind::Construct {
                    constructor,
                    fields,
                    ..
                } if *constructor == names::OptionSome => {
                    if let Some((_, inner)) = fields.first() {
                        docs!["Some", docs![inner].parens()]
                    } else {
                        docs!["Some()"]
                    }
                }
                ExprKind::Construct {
                    constructor,
                    fields,
                    ..
                } => {
                    if fields.is_empty() {
                        docs![constructor, "()"]
                    } else {
                        let payload = comma_sep!(fields.iter().map(|(_, v)| docs![v]));
                        docs![constructor, paren_doc!(payload)]
                    }
                }

                // ===== Match → if-let chain =====
                //
                // ProVerif evaluates `letfun` bodies eagerly, so any
                // destructure that can fail must have an `else` clause —
                // otherwise the whole letfun call aborts, even from arms
                // that weren't taken. The arms are split on or-patterns and
                // truncated at the first arm that always matches, then
                // `render_arm_chain` builds the `let … in … else …` chain,
                // threading each arm's fall-through continuation and ending
                // in `bitstring_err()` when nothing matches.
                //
                // A match with no arms (uninhabited scrutinee: `match never {}`,
                // or the discriminant cast of a variant-less enum) destructures
                // nothing and is dynamically unreachable. ProVerif cannot spell an
                // empty `let`-chain — the arms would render to nothing, leaving a
                // bare `else bitstring_err()` (a syntax error) — so model the point
                // directly as the error sink.
                ExprKind::Match { arms, .. } if arms.is_empty() => {
                    docs!["bitstring_err()"]
                }
                ExprKind::Match { scrutinee, arms } => {
                    let arm_always_matches = |arm: &Arm| {
                        matches!(*arm.pat.kind, PatKind::Wild) || self.is_trivial_binder(&arm.pat)
                    };
                    // ProVerif has no or-patterns. After
                    // `HoistDisjunctivePatterns` the alternatives sit at the top
                    // of the arm pattern, so split each `A | B => e` arm into one
                    // arm per alternative (sharing the body); the if-let chain
                    // below then renders one `let pat = scrutinee in body else ...`
                    // clause each.
                    let expanded: Vec<Arm> = arms
                        .iter()
                        .flat_map(|arm| match &*arm.pat.kind {
                            PatKind::Or { sub_pats } => sub_pats
                                .iter()
                                .map(|sp| Arm {
                                    pat: sp.clone(),
                                    body: arm.body.clone(),
                                    guard: arm.guard.clone(),
                                    meta: arm.meta.clone(),
                                })
                                .collect::<Vec<_>>(),
                            _ => vec![arm.clone()],
                        })
                        .collect();
                    // Take arms up to and including the first one that always
                    // matches; later arms are dynamically unreachable.
                    let mut truncated: Vec<&Arm> = Vec::new();
                    for arm in expanded.iter() {
                        truncated.push(arm);
                        if arm_always_matches(arm) {
                            break;
                        }
                    }
                    // The chain re-tests the scrutinee in every arm, which
                    // cannot fail; see `ProverifBindFailable`.
                    self.warn_opaque_branch(scrutinee, self.render_arm_chain(scrutinee, &truncated))
                }

                // ===== If / Let =====
                //
                // ProVerif's `if`/`then`/`else` requires a `bool`-typed
                // condition, but under the uniform-bitstring collapse every
                // condition is `bitstring`, so an `if` is rendered by
                // `if_then_else` as `let (=True()) = cond in then_ else else_`.
                //
                // An `if let` chain (`if let Some(x) = a && let Some(y) = b`)
                // has no first-order ProVerif encoding: the binding tests become
                // `Error` nodes and their bindings (`x`, `y`) are lost, yet the
                // then-branch still references them. When the condition contains
                // an `Error` node the whole `if` is unmodelable, so collapse it
                // to the error sink rather than emit a body over unbound
                // variables. (The `Error` nodes already carry their own
                // diagnostics.)
                ExprKind::If { condition, .. } if has_error(condition) => {
                    docs![
                        "bitstring_err() ",
                        error_marker(&nested_error_details(
                            condition,
                            "Untranslatable `if` condition"
                        ))
                    ]
                }
                ExprKind::If {
                    condition,
                    then,
                    else_,
                } => self.warn_opaque_branch(
                    condition,
                    self.if_then_else(
                        condition,
                        self.paren_unless_atomic(then),
                        match else_ {
                            Some(e) => self.paren_unless_atomic(e),
                            // `if c { e }` is unit when `c` does not hold.
                            None => docs![UNIT],
                        },
                    ),
                ),
                //
                // A failable (refutable) pattern requires an `else` clause:
                // ProVerif evaluates letfun bodies eagerly, so a destructure
                // that fails aborts the entire letfun call (even when nested
                // inside an unreached Match arm) unless an `else` catches it.
                // Wrap the body in parens so any `else` chain *inside* `body`
                // doesn't rebind to this outer `let`. A trivial (irrefutable)
                // binder (`let x = e`) always matches, so it takes no `else`.
                ExprKind::Let { lhs, rhs, body } => {
                    // `ProverifBindFailable` binds the value of a branch.
                    let binds_branch = |lhs: &Pat| {
                        lhs.as_binder().is_some_and(|id| {
                            let name = render_local(id);
                            name.starts_with("hax__cond") || name.starts_with("hax__scrut")
                        })
                    };
                    // A nested binding of a branch value warns on its own.
                    let nested =
                        matches!(&*rhs.kind, ExprKind::Let { lhs, .. } if binds_branch(lhs));
                    if binds_branch(lhs)
                        && !nested
                        && let Some(reason) = self.opaque_branch_reason(rhs)
                    {
                        return docs![
                            warning_marker(&reason),
                            hardline!(),
                            "let ",
                            lhs,
                            " = ",
                            self.paren_unless_atomic(rhs),
                            " in",
                            hardline!(),
                            body
                        ];
                    }
                    if self.is_trivial_binder(lhs) {
                        docs![
                            "let ",
                            lhs,
                            " = ",
                            self.paren_unless_atomic(rhs),
                            " in",
                            hardline!(),
                            body
                        ]
                    } else {
                        docs![
                            "let ",
                            lhs,
                            " = ",
                            self.paren_unless_atomic(rhs),
                            " in ",
                            self.paren_unless_atomic(body),
                            " else bitstring_err()"
                        ]
                    }
                }

                // ===== application fallback =====
                ExprKind::App { head, args, .. } => {
                    // A `reduc` field accessor reduces only on its constructor,
                    // so applying one to a value that has none can never
                    // reduce: the projection, and every caller of the letfun
                    // holding it, are dead.
                    if let (ExprKind::GlobalId(g), [arg]) = (&*head.kind, &args[..])
                        && let Some(reason) = self
                            .analysis
                            .dead_projection_reason(&self.render_id(g), arg)
                    {
                        return self.todo_document(
                            &format!(
                                "field projection `{}` {reason}; it can never reduce",
                                self.render_id(g)
                            ),
                            None,
                        );
                    }
                    // Render a `GlobalId` head as the bare name, not as a value.
                    let head_doc = match &*head.kind {
                        ExprKind::GlobalId(g) => docs![g],
                        _ => docs![head],
                    };
                    if args.is_empty() {
                        docs![head_doc, "()"]
                    } else {
                        docs![head_doc, paren_list!(args.iter().map(|a| docs![a]))]
                    }
                }

                ExprKind::Literal(literal) => docs![literal],
                // A nullary item that kept its value is a `letfun NAME() = …`,
                // and ProVerif has no unapplied letfuns, so a reference to one
                // is the zero-argument application `NAME()`.
                ExprKind::GlobalId(g)
                    if self.analysis.nullary_letfuns.contains(&self.render_id(g)) =>
                {
                    docs![g, "()"]
                }
                ExprKind::GlobalId(g) => docs![g],
                ExprKind::LocalId(local_id) => docs![local_id],
                ExprKind::Ascription { e, ty: _ } => docs![e],
                // ProVerif is first-order and has no array literals. Model a
                // fixed-size array `[a, b, c]` as a cons-list over the opaque
                // constructors `array_nil` / `array_cons` (declared in
                // `primitives.pvl`), i.e. `array_cons(a, array_cons(b,
                // array_cons(c, array_nil())))`. This is length-independent and
                // keeps everything within the uniform-bitstring encoding.
                // (Repeat arrays `[e; n]` never reach here — they are lowered
                // upstream to `rust_primitives::hax::repeat(e, n)`.)
                ExprKind::Array(elements) => elements
                    .iter()
                    .rev()
                    .fold(docs![&names::array_nil, "()"], |rest, elem| {
                        docs![&names::array_cons, "(", elem, ", ", rest, ")"]
                    }),
                ExprKind::Borrow { .. } => unreachable_by_invariant!(Drop_references),
                ExprKind::AddressOf { .. } => unreachable_by_invariant!(Reject_raw_or_mut_pointer),
                ExprKind::Assign { .. } => unreachable_by_invariant!(Local_mutation),
                // ProVerif has no loops. A `for`-loop over a collection —
                // which reaches the printer (post-`ReconstructForLoops` and
                // `LocalMutation`) as `LoopKind::ForLoop` with an explicit
                // functional `state` — is unrolled to a fixed `LOOP_BOUND`
                // depth by destructuring the iterator Seq (`array_cons` /
                // `array_nil` cons-list) one element at a time. All other
                // loop shapes (while, unconditional) reaching here without a
                // matching no-state arm below are still unsupported.
                ExprKind::Loop {
                    body,
                    kind,
                    state,
                    control_flow,
                    ..
                } => match (&**kind, control_flow) {
                    (_, Some(ControlFlowKind::BreakOrReturn)) => self
                        .todo_document("Loops containing `return` not supported in ProVerif", None),
                    (LoopKind::ForLoop { pat, iterator }, _) => self.unroll_for_loop(
                        pat,
                        iterator,
                        body,
                        state.as_ref(),
                        control_flow.is_some(),
                    ),
                    _ => self.todo_document("Loops not supported in ProVerif", None),
                },
                ExprKind::Break { .. } | ExprKind::Continue { .. } | ExprKind::Return { .. } => {
                    self.todo_document("Early-exit control flow not supported in ProVerif", None)
                }
                ExprKind::Closure { .. } => {
                    self.todo_document("Closures not supported in ProVerif", None)
                }
                ExprKind::Block { .. } => unreachable_by_invariant!(Drop_blocks),
                ExprKind::Quote { contents } => docs![contents],
                ExprKind::Resugared(_) => {
                    self.todo_document("Unsupported resugared expression", None)
                }
                ExprKind::Error(e) => docs![e],
            }
        }

        fn arm(&self, _arm: &Arm) -> DocBuilder<A> {
            // Arms are emitted via `render_arm_chain` (called from `expr`'s
            // `Match` branch); the bare default is unused.
            nil!()
        }

        fn param(&self, param: &Param) -> DocBuilder<A> {
            // A ProVerif `letfun` header takes plain `name: ty` binders.
            let Some(var) = param.pat.as_binder() else {
                unreachable_by_invariant!(Proverif_name_wildcards)
            };
            docs![var, ": ", &param.ty]
        }

        fn variant(&self, variant: &Variant) -> DocBuilder<A> {
            // Used only when emitted directly; the actual struct/enum
            // constructor declarations come out of `fn item`.
            let args = comma_sep!(variant.arguments.iter().map(|(_, ty, _)| docs![ty]));
            docs![&variant.name, paren_doc!(args)]
        }

        fn item(&self, item: &Item) -> DocBuilder<A> {
            let is_erased = item
                .meta
                .hax_attributes()
                .any(|a| matches!(a, AttrPayload::Erased));

            // Leading Rust doc comments → ProVerif `(* … *)`, but only for
            // items that actually emit something: an erased type / trait /
            // use / alias renders nothing, and a dangling comment would
            // visually attach to the *next* item.
            let renders_output = match item.kind() {
                ItemKind::Fn { .. } | ItemKind::Quote { .. } | ItemKind::Impl { .. } => true,
                ItemKind::Type { .. } => !is_erased,
                _ => false,
            };
            // Source-provenance comment for the items that carry a real def
            // span and a single name (Fn / Type); Impl methods get theirs in
            // `impl_item`.
            let src = if renders_output {
                match item.kind() {
                    ItemKind::Fn { name, .. } | ItemKind::Type { name, .. } => {
                        self.src_comment(item.meta.span, name)
                    }
                    _ => nil!(),
                }
            } else {
                nil!()
            };
            let leading_docs = if renders_output {
                docs![src, self.doc_comments(&item.meta.attributes)]
            } else {
                nil!()
            };

            let rendered = match item.kind() {
                ItemKind::Fn {
                    name, body, params, ..
                } => self.fn_decl(name, params, body, &item.meta),

                // Emit only the `[data]` constructors and the per-field
                // `reduc` accessors.
                ItemKind::Type { .. } if is_erased => return nil!(),
                ItemKind::Type { variants, .. } => {
                    intersperse!(variants.iter().map(|v| self.fun_and_reduc(v)), hardline!())
                }

                ItemKind::Quote { quote, .. } => docs![quote],

                ItemKind::TyAlias { .. } => nil!(),
                ItemKind::Trait { .. } => nil!(),
                ItemKind::Impl { items, .. } => {
                    // Render an impl block by flattening its items into
                    // top-level letfuns / consts. Specialize gives each impl
                    // item a unique flattened GlobalId
                    // (`<self_ty>__<trait>__<method>` for trait impls,
                    // `<self_ty>__<method>` for inherent impls), so we can
                    // emit them as ordinary items at the file level.
                    //
                    // ProVerif needs every symbol declared before use, but Rust
                    // methods within an impl may reference each other (or an
                    // associated const) in any source order. Topologically sort
                    // the flattened items so each is emitted after the siblings
                    // it depends on.
                    let ordered =
                        topo_sort(items, |ii| vec![self.render_id(&ii.ident)], &self.analysis);
                    intersperse!(ordered.into_iter().map(|i| docs![i]), hardline!())
                }
                ItemKind::Alias { .. } => nil!(),
                ItemKind::Use { .. } | ItemKind::RustModule => nil!(),
                ItemKind::NotImplementedYet => nil!(),
                ItemKind::Resugared(_) => nil!(),
                ItemKind::Error(e) => docs![e],
            };
            // Annotate the rendered declaration (Fn / Type) with its source span
            // for the source map. Impl items are annotated individually in
            // `impl_item`; other kinds render nothing to map.
            let rendered = match item.kind() {
                ItemKind::Fn { .. } | ItemKind::Type { .. } if renders_output => {
                    Self::annotate_with_span(rendered, item.meta.span)
                }
                _ => rendered,
            };
            docs![leading_docs, rendered]
        }

        fn metadata(&self, _metadata: &Metadata) -> DocBuilder<A> {
            nil!()
        }

        fn attribute(&self, attribute: &Attribute) -> DocBuilder<A> {
            match &attribute.kind {
                AttributeKind::DocComment { body, .. } => {
                    docs!["(* ", body.clone(), " *)", hardline!()]
                }
                AttributeKind::Tool { .. } | AttributeKind::Hax(_) => nil!(),
            }
        }

        fn quote(&self, quote: &Quote) -> DocBuilder<A> {
            concat![&quote.0]
        }

        fn quote_content(&self, quote_content: &QuoteContent) -> DocBuilder<A> {
            match quote_content {
                QuoteContent::Verbatim(s) => {
                    intersperse!(s.lines().map(|x| x.to_string()), hardline!())
                }
                QuoteContent::Expr(e) => docs![e],
                QuoteContent::Pattern(p) => docs![p],
                QuoteContent::Ty(t) => docs![t],
            }
        }

        fn spanned_ty(&self, spanned_ty: &SpannedTy) -> DocBuilder<A> {
            docs![&spanned_ty.ty]
        }

        fn generics(&self, _generics: &Generics) -> DocBuilder<A> {
            nil!()
        }

        fn generic_param(&self, _generic_param: &GenericParam) -> DocBuilder<A> {
            nil!()
        }

        fn generic_constraint(&self, _gc: &GenericConstraint) -> DocBuilder<A> {
            nil!()
        }

        fn generic_value(&self, generic_value: &GenericValue) -> DocBuilder<A> {
            match generic_value {
                GenericValue::Ty(ty) => docs![ty],
                GenericValue::Expr(expr) => docs![expr],
                GenericValue::Lifetime => unreachable_by_invariant!(Drop_references),
            }
        }

        fn lhs(&self, _lhs: &Lhs) -> DocBuilder<A> {
            unreachable_by_invariant!(Local_mutation)
        }

        fn safety_kind(&self, _safety_kind: &SafetyKind) -> DocBuilder<A> {
            nil!()
        }

        fn region(&self, _region: &Region) -> DocBuilder<A> {
            unreachable_by_invariant!(Drop_references)
        }

        fn borrow_kind(&self, _borrow_kind: &BorrowKind) -> DocBuilder<A> {
            unreachable_by_invariant!(Drop_references)
        }

        fn binding_mode(&self, _binding_mode: &BindingMode) -> DocBuilder<A> {
            nil!()
        }

        fn guard(&self, _guard: &Guard) -> DocBuilder<A> {
            unreachable_by_invariant!(Proverif_bind_failable)
        }

        fn dyn_trait_goal(&self, _: &DynTraitGoal) -> DocBuilder<A> {
            unreachable_by_invariant!(Reject_dyn)
        }

        fn impl_expr(&self, _impl_expr: &ImplExpr) -> DocBuilder<A> {
            nil!()
        }

        fn impl_ident(&self, _impl_ident: &ImplIdent) -> DocBuilder<A> {
            nil!()
        }

        fn trait_goal(&self, _trait_goal: &TraitGoal) -> DocBuilder<A> {
            nil!()
        }

        fn projection_predicate(&self, _: &ProjectionPredicate) -> DocBuilder<A> {
            nil!()
        }

        fn impl_item(&self, impl_item: &ImplItem) -> DocBuilder<A> {
            // Each impl item is a flat top-level declaration: `Specialize` gives
            // it a unique flattened name.
            // Leading Rust doc comments on the method/const → ProVerif comments
            // (associated types emit nothing, so skip them to avoid a dangling
            // comment).
            let leading_docs = if matches!(&impl_item.kind, ImplItemKind::Type { .. }) {
                nil!()
            } else {
                docs![
                    self.src_comment(impl_item.meta.span, &impl_item.ident),
                    self.doc_comments(&impl_item.meta.attributes)
                ]
            };
            let rendered = match &impl_item.kind {
                // Associated types are bitstring; nothing to declare.
                ImplItemKind::Type { .. } => nil!(),
                ImplItemKind::Fn { body, params } => {
                    self.fn_decl(&impl_item.ident, params, body, &impl_item.meta)
                }
                ImplItemKind::Resugared(_) => {
                    unreachable!("the ProVerif backend has no impl item resugaring")
                }
                ImplItemKind::Error(err) => docs![err],
            };
            docs![
                leading_docs,
                Self::annotate_with_span(rendered, impl_item.meta.span)
            ]
        }

        fn error_node(&self, error_node: &ErrorNode) -> DocBuilder<A> {
            // An earlier phase already reported why this node is an error.
            docs!["bitstring_err() ", error_marker(&error_details(error_node))]
        }
    }
};

/// Whether `expr` renders to a *self-delimiting* ProVerif term — one
/// that needs no surrounding parentheses when it appears as the RHS of
/// a `let`, as a branch of the `if`-rewrite, or as a match-arm body.
///
/// Atomic forms are variables, literals, and (possibly nested through
/// the identity passthroughs `into`/`clone`/…) function- and
/// constructor-applications `f(...)` / `C(...)` / `bitstring_err()`,
/// all of which are bounded by their own parentheses. Compound forms
/// (`let … in …`, the `if`-rewrite, match-chains, loop unrollings,
/// verbatim quotes) are NOT atomic: they end in a position where a
/// following `else`/`in` could rebind, so they keep the parens that
/// isolate them. Deliberately conservative — when unsure, returns
/// `false` (keeps the parens), so dropping them is always sound.
pub(crate) fn expr_is_atomic(expr: &Expr) -> bool {
    match &*expr.kind {
        ExprKind::LocalId(_) | ExprKind::GlobalId(_) | ExprKind::Literal(_) => true,
        ExprKind::Ascription { e, .. } => expr_is_atomic(e),
        // Identity passthroughs render their single argument verbatim,
        // so atomicity propagates through them (see `expr`).
        ExprKind::App { head, args, .. } if is_passthrough(head) => {
            args.first().map(expr_is_atomic).unwrap_or(true)
        }
        // `&&` and `||` render as a `let` chain.
        ExprKind::App { head, .. }
            if matches!(&*head.kind, ExprKind::GlobalId(g)
                if *g == names::logical_op_and || *g == names::logical_op_or) =>
        {
            false
        }
        // Any other application renders `f(args)` / `f()` — and the
        // `never_to_any` special case as `bitstring_err()`.
        ExprKind::App { .. } => true,
        ExprKind::Construct { .. } => true,
        // Cons-list `array_cons(...)` form.
        ExprKind::Array(_) => true,
        _ => false,
    }
}

/// Whether `head` is an identity on ProVerif terms, such as `Into::into`:
/// every value is a `bitstring`.
fn is_passthrough(head: &Expr) -> bool {
    matches!(&*head.kind, ExprKind::GlobalId(g)
        if [names::into, names::clone, names::unsize, names::deref, names::cast_op].contains(g))
}

/// The ProVerif backend.
pub struct ProVerifBackend {
    /// The `cargo hax into proverif` options.
    pub options: ProVerifOptions,
}

impl Backend for ProVerifBackend {
    type Printer = ProVerifPrinter;

    /// Single-file output (`lib.pvl`).
    fn module_path(&self, _module: &Module) -> Utf8PathBuf {
        Utf8PathBuf::from("lib.pvl")
    }

    /// The phase pipeline.
    fn phases(&self) -> Vec<PhaseKind> {
        use crate::phase::{PhaseKind::*, legacy::LegacyOCamlPhase::*};
        let result_model = if self.options.result_errors {
            vec![ProverifRejectTryConversions, ProverifRejectUnknownErrors]
        } else {
            vec![ProverifOkOnlyResults]
        };
        [
            vec![
                // Hides the bodies of the assumed items before any phase looks at
                // them.
                ProverifAssumeItems(self.options.assume_items.clone()),
                ProverifMarkMutReplaceBody,
                RejectUnsafe.into(),
                RejectRawOrMutPointer.into(),
                TransformHaxLibInline.into(),
                Specialize.into(),
                SimplifyQuestionMarks.into(),
                AndMutDefsite.into(),
                ReconstructForLoops.into(),
                DirectAndMut.into(),
                RejectArbitraryLhs.into(),
                DropBlocks.into(),
                DropReferences.into(),
                TrivializeAssignLhs.into(),
                HoistSideEffects.into(),
                // Hoists or-patterns to the top of each arm, where the printer
                // splits them into one arm per alternative.
                HoistDisjunctivePatterns.into(),
                SimplifyMatchReturn.into(),
                // Must precede the control-flow phases: `DropReturnBreakContinue`
                // needs the loop state it threads into `Loop`/`Break`/`Continue`.
                LocalMutation.into(),
                RewriteControlFlow.into(),
                DropReturnBreakContinue.into(),
                RejectContinue.into(),
                RejectDyn.into(),
                ReorderFields.into(),
                SortItemsNamespaceWise.into(),
                ProverifResolveTraitCalls(self.options.keep_trait_calls.clone()),
                ProverifInline,
                ProverifCombinatorsToLoops,
                ProverifExpandStructUpdate,
                ProverifFlattenBindingSubpatterns,
                ProverifRejectUnsupported,
                FilterUnprintableItems,
            ],
            result_model,
            vec![
                ProverifSimplifyBindings,
                ProverifRejectHigherOrder,
                ProverifBindFailable,
                ProverifNameWildcards,
            ],
        ]
        .concat()
    }

    /// Collapse every module into a single bag of items so the printer emits
    /// one `lib.pvl` file.
    fn items_to_module(&self, items: Vec<Item>) -> Vec<Module> {
        if items.is_empty() {
            return vec![];
        }
        let module_ident = items[0].ident.mod_only_closest_parent();
        vec![Module {
            ident: module_ident,
            items,
            meta: Metadata {
                span: Span::dummy(),
                attributes: vec![],
            },
        }]
    }

    fn modules_to_files(&self, modules: Vec<Module>, printer: Self::Printer) -> Vec<File> {
        // `items_to_module` puts every item in one module.
        let Ok([module]) = <[Module; 1]>::try_from(modules) else {
            return vec![];
        };
        let path = self.module_path(&module).to_string();
        let krate = module.ident.krate().to_string();
        let header = header(self.options.result_errors, &krate);
        let analysis = Rc::new(Analysis::new(&module.items));
        // Two Rust items that flatten to one ProVerif name would be merged by
        // the model.
        let collisions = name_collisions(&module.items);
        for collision in &collisions {
            <ProVerifPrinter as PrettyAst<Span>>::emit_diagnostic(
                &printer,
                hax_types::diagnostics::Kind::Unimplemented {
                    issue_id: None,
                    details: Some(collision.clone()),
                },
            );
        }
        let mut printer = ProVerifPrinter {
            analysis: analysis.clone(),
            ..printer
        };
        let (contents, positions) = printer.print(module);
        // Undefined externals go to a separate `missingdecl.pvl`, so the
        // unsound stubs stay visible.
        let missingdecl = self.format_external_decls(&analysis, &contents);
        // Anything the model misses arms the `hax_extraction_error` marker,
        // so the model cannot be mistaken for a complete one. See
        // [`ERROR_BANNER`].
        let errors: Vec<&str> = collisions
            .iter()
            .map(String::as_str)
            .chain(error_markers(&contents))
            .collect();
        let mut banner = if errors.is_empty() {
            String::new()
        } else {
            let listed: String = errors.iter().map(|e| format!("(*  - {e} *)\n")).collect();
            format!("{ERROR_BANNER}{listed}{}", error_banner_end(&krate))
        };
        let mut warnings: Vec<(&str, usize)> = Vec::new();
        for warning in markers(&contents, "(* HAX WARNING: ") {
            match warnings.iter_mut().find(|(w, _)| *w == warning) {
                Some((_, count)) => *count += 1,
                None => warnings.push((warning, 1)),
            }
        }
        if !warnings.is_empty() {
            banner.push_str(WARNING_BANNER);
            for (warning, count) in warnings {
                banner.push_str(&format!("(*  - {count} × {warning} *)\n"));
            }
            banner.push_str(
                "(*****************************************************************)\n\n",
            );
        }
        // The body is preceded by the header and the banner.
        let header_lines = header.matches('\n').count() + banner.matches('\n').count();
        let sourcemap = positions.to_v3(&path, header_lines);
        vec![
            File {
                contents: format!("{header}{banner}{contents}"),
                sourcemap,
                path,
            },
            File {
                path: "missingdecl.pvl".to_string(),
                contents: missingdecl,
                sourcemap: None,
            },
        ]
    }
}

impl ProVerifBackend {
    /// Emit the auto-declared-externals block for the file.
    ///
    /// A name in `referenced` needs a declaration unless `primitives.pvl`,
    /// the Result library of the chosen mode, or the `rendered` body (including
    /// its verbatim quotes) already declares it.
    fn format_external_decls(&self, analysis: &Analysis, rendered: &str) -> String {
        let mut defined: HashSet<String> = HashSet::new();
        for s in [
            "construct_fail",
            "empty",
            "bitstring_default",
            "bitstring_err",
            "Some",
            "None",
            "Option_err",
            "True",
            "False",
            "bool_default",
            "bool_err",
        ] {
            defined.insert(s.to_string());
        }
        for name in primitives_pvl_names() {
            defined.insert(name);
        }
        let result_pvl = if self.options.result_errors {
            RESULT_ERRORS_PVL
        } else {
            RESULT_PVL
        };
        for name in scan_declared_names(result_pvl) {
            defined.insert(name);
        }
        for name in scan_declared_names(rendered) {
            defined.insert(name);
        }

        let mut decls: Vec<String> = Vec::new();
        let mut keys: Vec<String> = analysis
            .references
            .keys()
            .filter(|k| !defined.contains(k.as_str()))
            .cloned()
            .collect();
        keys.sort();
        for name in keys {
            let info = &analysis.references[&name];
            // Record which emitted items refer to this external, so a reader can
            // locate the call sites a hand-written definition has to satisfy.
            if let Some(items) = analysis.reference_sources.get(&name) {
                decls.push(format!("(* referenced by: {} *)", items.join(", ")));
            }
            // Record the result shape a hand-written or imported definition must
            // produce, taken from how call sites destructure this symbol.
            if let Some(shapes) = analysis.result_shapes.get(&name) {
                decls.push(format!(
                    "(* result destructured as: {} *)",
                    shapes.join("; ")
                ));
            }
            let args = std::iter::repeat_n("bitstring", info.arity)
                .collect::<Vec<_>>()
                .join(", ");
            if info.is_constructor {
                // A Rust constructor, which a pattern can destructure: only a
                // `[data]` function may appear in a ProVerif pattern.
                decls.push(format!("fun {name}({args}): bitstring [data]."));
            } else if info.arity == 0 && !info.applied {
                decls.push(format!("const {name}: bitstring."));
            } else {
                // Not `[data]`: the symbol may be a one-way function, whose
                // inputs the attacker must not recover.
                decls.push(format!("fun {name}({args}): bitstring."));
            }
        }
        let body = if decls.is_empty() {
            "(* (none — every referenced symbol is defined in lib.pvl, *)\n\
             (* primitives.pvl, or a -lib'd dependency.)               *)"
                .to_string()
        } else {
            decls.join("\n")
        };
        format!(
            "(*****************************************************************)\n\
             (* missingdecl.pvl — UNDEFINED externals referenced by lib.pvl   *)\n\
             (*                                                               *)\n\
             (* Each symbol below is referenced by lib.pvl but defined by     *)\n\
             (* none of the hax libraries, so it is declared abstract here.   *)\n\
             (* Define it — by hand or by extracting the crate that owns it — *)\n\
             (* or deliberately leave it abstract, as an ideal-functionality  *)\n\
             (* boundary. Only Rust constructors are declared `[data]`: any   *)\n\
             (* other function may be one-way.                                *)\n\
             (*****************************************************************)\n\
             {body}\n"
        )
    }
}

/// Name of the opaque constant standing for a literal, or `None` for booleans,
/// which are the declared `True()`/`False()` constructors. Distinct literals get
/// distinct names, so they stay distinguishable in symbolic reasoning.
///
/// Integers are constants too, since ProVerif's unary `nat` cannot hold large
/// literals such as `u64::MAX`: `nat_0` to `nat_16` are declared in
/// `primitives.pvl`, larger ones in `missingdecl.pvl`.
fn lit_const_name(lit: &Literal) -> Option<String> {
    let sign = |negative: bool| if negative { "neg_" } else { "" };
    Some(match lit {
        Literal::Int {
            value, negative, ..
        } => format!("nat_{}{value}", sign(*negative)),
        Literal::String(s) => format!("string_lit__{}", sanitize_string_literal(s.as_ref())),
        Literal::Char(c) => format!("char_lit__{}", sanitize_string_literal(&c.to_string())),
        Literal::Float {
            value, negative, ..
        } => format!(
            "float_lit__{}{}",
            sign(*negative),
            sanitize_string_literal(value.as_ref())
        ),
        Literal::Bool(_) => return None,
    })
}

/// Turn a string / char / float-literal value into a valid ProVerif
/// identifier suffix. Replaces every non-`[A-Za-z0-9]` byte, `_` included,
/// with `_xHH` (two-hex), so the mapping is injective (distinct literals →
/// distinct identifiers): every `_` in the output starts an escape. Empty
/// input yields `_empty`, which no escape produces.
fn sanitize_string_literal(s: &str) -> String {
    if s.is_empty() {
        return "_empty".to_string();
    }
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() {
            out.push(b as char);
        } else {
            out.push_str(&format!("_x{b:02x}"));
        }
    }
    out
}

/// Parse `PRIMITIVES_PVL` once and return every declared name.
pub(crate) fn primitives_pvl_names() -> Vec<String> {
    static NAMES: OnceLock<Vec<String>> = OnceLock::new();
    NAMES
        .get_or_init(|| scan_declared_names(PRIMITIVES_PVL))
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The printer escapes a keyword path segment, so a library name with an
    /// unescaped one can never be referenced.
    #[test]
    fn library_names_escape_keywords() {
        for lib in [PRIMITIVES_PVL, RESULT_PVL, RESULT_ERRORS_PVL] {
            for name in scan_declared_names(lib) {
                for segment in name.split("__") {
                    assert!(
                        !ProVerifPrinter::is_reserved_keyword(segment),
                        "`{name}` has the unescaped keyword segment `{segment}`"
                    );
                }
            }
        }
    }
}
