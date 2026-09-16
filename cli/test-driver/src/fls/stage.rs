//! Support tiers, pipeline stages, and their classification. Every backend
//! has its own scale of stages; a stage is a position on one scale.

use std::{fmt::Display, str::FromStr};

use anyhow::{Error, Result, bail};
use hax_types::cli_options::BackendName;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
/// Intended support of a construct by a backend, in the vocabulary of the
/// input-language reference.
pub enum Tier {
    /// In scope; the pipeline is expected to check the test.
    Supported,
    /// Out of scope by design: the *Unsupported* status of the input-language
    /// reference, or a use outside the scope of a *Partial* row; the pipeline is
    /// expected to reject the test.
    Unsupported,
}

impl Tier {
    pub fn as_str(self) -> &'static str {
        match self {
            Tier::Supported => "supported",
            Tier::Unsupported => "unsupported",
        }
    }
}

impl FromStr for Tier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "supported" => Ok(Tier::Supported),
            "unsupported" => Ok(Tier::Unsupported),
            _ => bail!("unknown tier `{s}`, expected `supported` or `unsupported`"),
        }
    }
}

impl Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The stages of one backend, from the earliest rejection to full success.
/// `accepted` is the index of the first stage at which the target language
/// accepts the output. An unsupported construct that the tools pass through
/// as an opaque item has no model, so its extraction fails below `accepted`,
/// and only a stage at or above it is an acceptance.
#[derive(Debug)]
pub struct StageScale {
    pub backend: BackendName,
    pub names: &'static [&'static str],
    pub accepted: usize,
}

/// The scale of the Lean backend: charon, then aeneas, then `lake build` of
/// the extraction, then `lake build` of the guard file.
pub static LEAN: StageScale = StageScale {
    backend: BackendName::Lean,
    names: &[
        "charon-rejected",
        "aeneas-rejected",
        "extracted",
        "elaborates",
        "checked",
    ],
    accepted: 3,
};

pub fn scale_of(backend: BackendName) -> Result<&'static StageScale> {
    match backend {
        BackendName::Lean => Ok(&LEAN),
        other => bail!("the FLS suite has no stage scale for the backend `{other}` yet"),
    }
}

impl StageScale {
    pub fn parse(&'static self, s: &str) -> Result<Stage> {
        match self.names.iter().position(|name| *name == s) {
            Some(index) => Ok(Stage { scale: self, index }),
            None => bail!(
                "unknown stage `{s}` for the backend `{}`, expected one of {}",
                self.backend,
                self.names.join(", ")
            ),
        }
    }

    pub fn top(&'static self) -> Stage {
        Stage {
            scale: self,
            index: self.names.len() - 1,
        }
    }

    /// The stage with the given name. A name outside the scale is a
    /// programming error.
    pub fn stage(&'static self, name: &str) -> Stage {
        self.parse(name).unwrap_or_else(|e| panic!("{e}"))
    }
}

/// How far the pipeline of a backend gets on a test.
#[derive(Clone, Copy, Debug)]
pub struct Stage {
    scale: &'static StageScale,
    index: usize,
}

impl Stage {
    pub fn as_str(self) -> &'static str {
        self.scale.names[self.index]
    }

    pub fn is_accepted(self) -> bool {
        self.index >= self.scale.accepted
    }

    pub fn is_top(self) -> bool {
        self == self.scale.top()
    }

    pub fn next(self) -> Option<Stage> {
        (!self.is_top()).then(|| Stage {
            index: self.index + 1,
            ..self
        })
    }

    pub fn prev(self) -> Option<Stage> {
        (self.index > 0).then(|| Stage {
            index: self.index - 1,
            ..self
        })
    }
}

impl PartialEq for Stage {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.scale, other.scale) && self.index == other.index
    }
}

impl Eq for Stage {}

/// What the report says about a test, derived from its tier and its stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Classification {
    Supported,
    Planned,
    Unsupported,
    UnexpectedlyAccepted,
}

impl Classification {
    pub fn as_str(self) -> &'static str {
        match self {
            Classification::Supported => "supported",
            Classification::Planned => "planned",
            Classification::Unsupported => "unsupported",
            Classification::UnexpectedlyAccepted => "unexpectedly accepted, to be triaged",
        }
    }
}

pub fn classify(tier: Tier, stage: Stage) -> Classification {
    match tier {
        Tier::Supported if stage.is_top() => Classification::Supported,
        Tier::Supported => Classification::Planned,
        Tier::Unsupported if stage.is_accepted() => Classification::UnexpectedlyAccepted,
        Tier::Unsupported => Classification::Unsupported,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static OTHER: StageScale = StageScale {
        backend: BackendName::Fstar,
        names: &["rejected", "extracted"],
        accepted: 2,
    };

    #[test]
    fn classification_rows() {
        let s = |name| LEAN.stage(name);
        assert_eq!(
            classify(Tier::Supported, s("checked")),
            Classification::Supported
        );
        assert_eq!(
            classify(Tier::Supported, s("aeneas-rejected")),
            Classification::Planned
        );
        assert_eq!(
            classify(Tier::Unsupported, s("aeneas-rejected")),
            Classification::Unsupported
        );
        assert_eq!(
            classify(Tier::Unsupported, s("charon-rejected")),
            Classification::Unsupported
        );
        assert_eq!(
            classify(Tier::Unsupported, s("extracted")),
            Classification::Unsupported
        );
        assert_eq!(
            classify(Tier::Unsupported, s("elaborates")),
            Classification::UnexpectedlyAccepted
        );
        assert_eq!(
            classify(Tier::Unsupported, s("checked")),
            Classification::UnexpectedlyAccepted
        );
        assert_eq!(
            classify(Tier::Unsupported, OTHER.stage("extracted")),
            Classification::Unsupported
        );
    }

    #[test]
    fn stages_are_ordered_within_a_scale() {
        let names = LEAN.names;
        for pair in names.windows(2) {
            assert_eq!(LEAN.stage(pair[0]).next(), Some(LEAN.stage(pair[1])));
        }
        assert_eq!(
            LEAN.stage("extracted").next(),
            Some(LEAN.stage("elaborates"))
        );
        assert_eq!(LEAN.top(), LEAN.stage("checked"));
        assert_eq!(LEAN.top().next(), None);
    }

    #[test]
    fn scales_do_not_mix() {
        assert!(LEAN.parse("rejected").is_err());
        assert!(OTHER.parse("checked").is_err());
        assert_ne!(LEAN.stage("extracted"), OTHER.stage("extracted"));
    }
}
