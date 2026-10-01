use hax_adt_into::derive_group;
use schemars::JsonSchema;

#[derive_group(Serializers)]
#[derive(Debug, Clone, JsonSchema, Hash, Eq, PartialEq)]
pub enum Glob {
    One,  // *
    Many, // **
}

impl ToString for Glob {
    fn to_string(&self) -> String {
        match self {
            Self::One => "*",
            Self::Many => "**",
        }
        .to_string()
    }
}

#[derive_group(Serializers)]
#[derive(Debug, Clone, JsonSchema, Hash, Eq, PartialEq)]
pub enum NamespaceChunk {
    Glob(Glob),
    Exact(String),
}

impl ToString for NamespaceChunk {
    fn to_string(&self) -> String {
        match self {
            Self::Glob(glob) => glob.to_string(),
            Self::Exact(string) => string.to_string(),
        }
    }
}

impl std::convert::From<&str> for NamespaceChunk {
    fn from(s: &str) -> Self {
        match s {
            "*" => NamespaceChunk::Glob(Glob::One),
            "**" => NamespaceChunk::Glob(Glob::Many),
            _ => NamespaceChunk::Exact(String::from(s)),
        }
    }
}

#[derive_group(Serializers)]
#[derive(Debug, Clone, JsonSchema, Hash, Eq, PartialEq)]
pub struct Namespace {
    pub chunks: Vec<NamespaceChunk>,
}

impl ToString for Namespace {
    fn to_string(&self) -> String {
        self.chunks
            .iter()
            .map(NamespaceChunk::to_string)
            .collect::<Vec<_>>()
            .join("::")
            .to_string()
    }
}

impl std::convert::From<String> for Namespace {
    fn from(s: String) -> Self {
        Namespace {
            chunks: s
                .split("::")
                .filter(|s| !s.is_empty())
                .map(NamespaceChunk::from)
                .collect(),
        }
    }
}

impl Namespace {
    pub fn matches(&self, path: &Vec<String>) -> bool {
        let path: Vec<Option<&str>> = path.iter().map(|s| Some(s.as_str())).collect();
        self.matches_segments(&path)
    }

    /// Whether the namespace matches `path`, whose `None` segments (e.g. `impl`
    /// blocks) only a glob matches.
    pub fn matches_segments(&self, path: &[Option<&str>]) -> bool {
        fn aux(pattern: &[NamespaceChunk], path: &[Option<&str>]) -> bool {
            match (pattern, path) {
                ([], []) => true,
                ([NamespaceChunk::Exact(x), pattern @ ..], [Some(y), path @ ..]) => {
                    x == y && aux(pattern, path)
                }
                ([NamespaceChunk::Glob(Glob::One), pattern @ ..], [_, path @ ..]) => {
                    aux(pattern, path)
                }
                ([NamespaceChunk::Glob(Glob::Many), pattern @ ..], []) => aux(pattern, path),
                ([NamespaceChunk::Glob(Glob::Many), pattern_tl @ ..], [_path_hd, path_tl @ ..]) => {
                    aux(pattern_tl, path) || aux(pattern, path_tl)
                }
                _ => false,
            }
        }
        aux(self.chunks.as_slice(), path)
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    /// Whether we should evaluate and inline the value of anonymous constants (inline `const {}`
    /// blocks or advanced constant expressions as in `[T; N+1]`), or refer to them as
    /// `GlobalName`s.
    pub inline_anon_consts: bool,
    /// Options related to bounds.
    pub bounds_options: BoundsOptions,
    /// Resolve definition identifiers to their concrete impl counterpart when possible in `ItemRef::translate`.
    pub item_ref_use_concrete_impl: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct BoundsOptions {
    /// Add `T: Destruct` bounds to every type generic, so that we can build `ImplExpr`s to know
    /// what code is run on drop.
    pub resolve_destruct: bool,
    /// Prune `T: Sized` and `T: MetaSized` predicates.
    pub prune_sized: bool,
}
