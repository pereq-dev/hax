use crate::prelude::*;
use colored::Colorize;

pub mod message;
pub mod report;

#[derive_group(Serializers)]
#[derive(Debug, Clone, JsonSchema, Eq, PartialEq, Hash)]
pub struct Diagnostics {
    pub kind: Kind,
    pub span: Vec<hax_frontend_exporter::Span>,
    pub context: String,
    pub owner_id: Option<hax_frontend_exporter::DefId>,
}

impl std::fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            Kind::Unimplemented { issue_id:_, details } => write!(
                f,
                "something is not implemented yet.\n{}",
                match details {
                    Some(details) => format!("{}", details),
                    _ => "".to_string(),
                },
            ),
            Kind::UnsupportedMacro { id } => write!(
                f,
                "The unexpanded macro {} is not supported by this backend.\nPlease verify the argument you passed to the {} (or {}) option.",
                id.bold(),
                "--inline-macro-call".bold(), "-i".bold()
            ),
            Kind::UnsafeBlock => write!(f, "Unsafe blocks are not allowed."),
            Kind::AssertionFailure {details} => write!(
                f,
                "Fatal error: something we considered as impossible occurred! {}\nDetails: {}",
                "Please report this by submitting an issue on GitHub!".bold(),
                details
            ),
            Kind::UnallowedMutRef => write!(
                f,
                "The mutation of this {} is not allowed here.",
                "&mut".bold()
            ),
            Kind::ExpectedMutRef => write!(
                f,
                "At this position, Hax was expecting an expression of the shape `&mut _`.\nHax forbids `f(x)` (where `f` expects a mutable reference as input) when `x` is not a {}{} or when it is a dereference expression.

{}",
                "place expression".bold(),
                "[1]".bright_black(),
                "[1]: https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions"
            ),
            Kind::ClosureMutatesParentBindings {bindings} => write!(
                f,
                "The bindings {:?} cannot be mutated here: they don't belong to the closure scope, and this is not allowed.",
                bindings
            ),
            Kind::ArbitraryLHS => write!(f, "Assignation of an arbitrary left-hand side is not supported.\n`lhs = e` is fine only when `lhs` is a combination of local identifiers, field accessors and index accessors."),

            Kind::AttributeRejected {reason} => write!(f, "Here, this attribute cannot be used: {reason}."),

            Kind::NonTrivialAndMutFnInput => write!(f, "The support in hax of function with one or more inputs of type `&mut _` is limited.\nOnly trivial patterns are allowed there: `fn f(x: &mut (T, U)) ...` is allowed while `f((x, y): &mut (T, U))` is rejected."),

            Kind::FStarParseError { fstar_snippet, details: _ } => write!(f, "The following code snippet could not be parsed as valid F*:\n```\n{fstar_snippet}\n```"),

            Kind::ExplicitRejection { reason , .. } => write!(f, "Explicit rejection by a phase in the Hax engine:\n{}", reason),

            _ => write!(f, "{:?}", self.kind),
        }?;
        write!(f, "\n\n")?;
        if let Some(issue) = self.kind.issue_number() {
            write!(
                f,
                "This is discussed in issue https://github.com/pereq-dev/hax/issues/{issue}.\nPlease upvote or comment this issue if you see this error message.\n"
            )?;
        }
        write!(
            f,
            "{}",
            format!(
                "Note: the error was labeled with context `{}`.\n",
                self.context
            )
            .bright_black()
        )?;
        Ok(())
    }
}

impl Kind {
    fn issue_number(&self) -> Option<u32> {
        match self {
            Kind::UnsafeBlock => None,
            Kind::ExplicitRejection { issue_id, .. } | Kind::Unimplemented { issue_id, .. } => {
                issue_id.clone()
            }
            Kind::AssertionFailure { .. } => None,
            Kind::UnallowedMutRef => Some(420),
            Kind::UnsupportedMacro { .. } => None,
            Kind::ErrorParsingMacroInvocation { .. } => None,
            Kind::ClosureMutatesParentBindings { .. } => Some(1060),
            Kind::ArbitraryLHS => None,
            Kind::UnsupportedTupleSize { .. } => None,
            Kind::ExpectedMutRef => Some(420),
            Kind::NonTrivialAndMutFnInput => Some(1405),
            Kind::AttributeRejected { .. } => None,
            Kind::FStarParseError { .. } => None,
            Kind::OcamlEngineErrorPayload { .. } => None,
        }
    }
}

#[derive_group(Serializers)]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, JsonSchema)]
#[repr(u16)]
pub enum Kind {
    /// Unsafe code is not supported
    UnsafeBlock = 0,

    /// A feature is not currently implemented
    Unimplemented {
        /// Issue on the GitHub repository
        issue_id: Option<u32>,
        details: Option<String>,
    } = 1,

    /// Unknown error
    // This is useful when doing sanity checks (i.e. one can yield
    // this error kind for cases that should never happen)
    AssertionFailure {
        details: String,
    } = 2,

    /// Unallowed mutable reference
    UnallowedMutRef = 3,

    /// Unsupported macro invokation
    UnsupportedMacro {
        id: String,
    } = 4,

    /// Error parsing a macro invocation to a macro treated specifcially by a backend
    ErrorParsingMacroInvocation {
        macro_id: String,
        details: String,
    } = 5,

    /// Mutation of bindings living outside a closure scope are not supported
    ClosureMutatesParentBindings {
        bindings: Vec<String>,
    } = 6,

    /// Assignation of an arbitrary left-hand side is not supported. `lhs = e` is fine only when `lhs` is a combination of local identifiers, field accessors and index accessors.
    ArbitraryLHS = 7,

    /// A phase explicitely rejected this chunk of code
    ExplicitRejection {
        reason: String,
        issue_id: Option<u32>,
    } = 8,

    /// A backend doesn't support a tuple size
    UnsupportedTupleSize {
        tuple_size: u32,
        reason: String,
    } = 9,

    ExpectedMutRef = 10,

    /// &mut inputs should be trivial patterns
    NonTrivialAndMutFnInput = 11,

    /// An hax attribute (from `hax-lib-macros`) was rejected
    AttributeRejected {
        reason: String,
    } = 12,

    /// A snippet of F* code could not be parsed
    FStarParseError {
        fstar_snippet: String,
        details: String,
    } = 13,

    /// Internal encoding
    OcamlEngineErrorPayload(String) = 9999,
}

impl Kind {
    // https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting
    pub fn discriminant(&self) -> u16 {
        unsafe { *(self as *const Self as *const u16) }
    }

    pub fn code(&self) -> String {
        format!("HAX{:0>4}", self.discriminant())
    }
}
