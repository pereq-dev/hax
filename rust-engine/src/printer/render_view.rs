//! Tools for rendering Rust paths into strings.
//!
//! This module takes a typed [`View`] (a list of [`PathSegment`]s) and turns it
//! into either:
//! - a structured [`Rendered`] (with `module` vs. `path` parts), or
//! - a single flat `String`.
//!
//! The [`RenderView`] trait allows for customization.

use crate::{
    ast::identifiers::global_id::{
        ReservedSuffix,
        view::{PathSegment, PathSegmentPayload, UnnamedPathSegmentPayload, View},
    },
    symbol::Symbol,
};

use std::collections::HashSet;
use std::sync::OnceLock;

/// A helper trait to render a [`View`] (a typed list of path segments) into
/// strings.
///
/// Rendering is split into two parts:
/// - module path: the crate + module prefix,
/// - relative path: the remaining (non-module) segments, and both may contain
///   hierarchical sub-segments (e.g. `Foo::MyVariant::field`).
///
/// Implementors can:
/// - override how unnamed segments (e.g. `impl`, `anon const`) are displayed,
/// - override how each segment is rendered,
/// - customize the separator (defaults to `"::"`),
/// - render to either a structured [`Rendered`] or a single flat `String`.
///
/// # Terminology
///
/// A path segment can be:
/// - named: carries a `Symbol` that can be printed as-is,
/// - unnamed: carries an [`UnnamedPathSegmentPayload`] (like `Impl`, `Closure`,
///   …), which must be turned into a `Symbol` first (see
///   [`RenderView::render_unnamed_path_segment_payload`]).
///
/// # Hierarchical segments
///
/// Some segments are actually small trees (e.g., field → constructor → type).
/// [`RenderView::render_path_segment`] returns all display atoms for such a
/// segment, so callers can flatten or join as needed.
pub trait RenderView: Sized {
    /// List of reserved keywords that will be escaped when rendering
    fn reserved_keywords() -> &'static HashSet<String> {
        static SET: OnceLock<HashSet<String>> = OnceLock::new();
        SET.get_or_init(|| [].into_iter().collect())
    }

    /// Check if a string is a reserved keyword that needs escaping
    fn is_reserved_keyword(id: &str) -> bool {
        let reserved = Self::reserved_keywords();
        reserved.contains(id)
    }

    /// Check if a string needs escaping
    fn should_escape(id: &str) -> bool {
        Self::is_reserved_keyword(id)
    }

    /// Escape a string if it needs escaping according to `Self::should_escape`
    fn escape(id: &str) -> String {
        // See https://github.com/pereq-dev/hax/issues/1866
        let id = id.replace([' ', '<', '>'], "_");
        if id.is_empty() {
            "_ERROR_EMPTY_ID_".to_string()
        } else if Self::should_escape(id.trim_start_matches("_")) {
            format!("_{id}")
        } else {
            id
        }
    }

    /// Converts an unnamed path segment payload into a printable [`Symbol`].
    ///
    /// Unnamed segments include `impl`, `anon const`, `inline const`, `foreign mod`,
    /// `global_asm`, `use`, `opaque`, and `closure`. By default, these map to
    /// their capitalized identifier (e.g., `Impl`, `AnonConst`, …).
    ///
    /// Override this method to customize how unnamed items appear in output.
    fn render_unnamed_path_segment_payload(&self, unnamed: UnnamedPathSegmentPayload) -> Symbol {
        default::render_unnamed_path_segment_payload(self, unnamed)
    }

    /// Converts a full [`PathSegmentPayload`] (named or unnamed) into a printable [`Symbol`].
    ///
    /// Named payloads return their `Symbol` unchanged. Unnamed payloads are delegated to
    /// [`render_unnamed_path_segment_payload`].
    fn render_path_segment_payload(&self, payload: PathSegmentPayload) -> Symbol {
        match payload {
            PathSegmentPayload::Named(symbol) => symbol,
            PathSegmentPayload::Unnamed(unnamed) => {
                self.render_unnamed_path_segment_payload(unnamed)
            }
        }
    }

    /// Renders a single [`PathSegment`] into a vector of display atoms.
    ///
    /// Most segments render to a single atom (e.g., `"Foo"`). Hierarchical segments
    /// (like a field) render to multiple atoms representing their parent chain
    /// (e.g., `["Foo", "MyVariant", "my_field"]`). Disambiguators (see
    /// [`PathSegment::disambiguator`]) are suffixed as `_N` when `N > 0`.
    ///
    /// The resulting atoms are suitable for joining with [`separator`](Self::separator),
    /// or for further grouping into module vs. relative path.
    fn render_path_segment(&self, seg: &PathSegment) -> Vec<String> {
        default::render_path_segment(self, seg)
    }

    /// Renders the optional suffix
    fn render_suffix(&self, suffix: &ReservedSuffix) -> String {
        default::render_suffix(suffix)
    }

    /// Renders just the module path (crate + modules) of a [`View`], as a list of atoms.
    ///
    /// This is a convenience wrapper around [`render`](Self::render) that returns only
    /// the `module` component.
    fn module(&self, view: &View) -> Vec<String> {
        self.render(view).module
    }

    /// Allows backends to adjust a module path before rendering, e.g., to shorten it according
    /// to currenly open namespaces.
    fn relativize_module_path<'a>(&self, module_path: &'a [PathSegment]) -> &'a [PathSegment] {
        module_path
    }

    /// Renders a [`View`] into a structured [`Rendered`] value,
    /// splitting output into `module` and `path` parts.
    ///
    /// Internally, this uses [`View::split_at_module`] to separate module segments
    /// from the remaining non-module segments, rendering each with
    /// [`render_path_segment`].
    fn render(&self, view: &View) -> Rendered {
        let (module_path, relative_path) = view.split_at_module();
        let module_path = self.relativize_module_path(module_path);
        let path_segment = |seg| self.render_path_segment(seg);
        let mut path: Vec<String> = relative_path.iter().flat_map(path_segment).collect();
        if let Some(last) = path.last_mut()
            && let Some(suffix) = view.suffix()
        {
            last.push_str(&self.render_suffix(suffix));
        }
        Rendered {
            module: module_path.iter().flat_map(path_segment).collect(),
            path,
        }
    }

    /// Returns the string used to join rendered atoms (defaults to `"::"`).
    ///
    /// Override to customize separators (e.g., `"."`).
    fn separator(&self) -> &str {
        "::"
    }

    /// Lazy render a view as an iterator of strings.
    ///
    /// This chains `rendered.module` and `rendered.path` in order.
    fn rendered_to_strings(&self, rendered: Rendered) -> impl Iterator<Item = String> {
        rendered.module.into_iter().chain(rendered.path)
    }

    /// Joins the atoms contained in a [`Rendered`] into a single string using
    /// [`separator`](Self::separator).
    ///
    /// This concatenates `rendered.module` and `rendered.path` in order, inserting
    /// the separator between atoms.
    fn rendered_to_string(&self, rendered: Rendered) -> String {
        self.rendered_to_strings(rendered)
            .collect::<Vec<_>>()
            .join(self.separator())
    }

    /// Convenience: renders a [`View`] straight to a single `String`.
    fn render_string(&self, view: &View) -> String {
        self.rendered_to_string(self.render(view))
    }

    /// Convenience: renders a [`View`] straight to a iterator of `String`s.
    fn render_strings(&self, view: &View) -> impl Iterator<Item = String> {
        self.rendered_to_strings(self.render(view))
    }
}

/// Default rendering helpers used by [`RenderView`]'s blanket implementations.
///
/// You can call these directly when composing your own renderer, or override the
/// trait methods to change behavior selectively.
pub mod default {
    use super::*;

    /// Default mapping of unnamed payloads to printable symbols.
    pub fn render_unnamed_path_segment_payload<V: RenderView + Sized>(
        _render_view: &V,
        unnamed: UnnamedPathSegmentPayload,
    ) -> Symbol {
        Symbol::new(match unnamed {
            UnnamedPathSegmentPayload::Impl => "Impl",
            UnnamedPathSegmentPayload::AnonConst => "AnonConst",
            UnnamedPathSegmentPayload::InlineConst => "InlineConst",
            UnnamedPathSegmentPayload::Foreign => "Foreign",
            UnnamedPathSegmentPayload::GlobalAsm => "GlobalAsm",
            UnnamedPathSegmentPayload::Use => "Use",
            UnnamedPathSegmentPayload::Opaque => "Opaque",
            UnnamedPathSegmentPayload::Closure => "Closure",
        })
    }

    /// Default rendering of a single [`PathSegment`] into display atoms.
    ///
    /// This walks the segment's parent chain (see [`PathSegment::parents`]) and
    /// produces an atom for each level using
    /// [`RenderView::render_path_segment_payload`]. If a level has a disambiguator
    /// `> 0`, it is appended as `_<n>` (e.g., `Foo_2`).
    pub fn render_path_segment<V: RenderView + Sized>(
        render_view: &V,
        seg: &PathSegment,
    ) -> Vec<String> {
        let mut strings: Vec<String> = seg
            .parents()
            .map(|seg| {
                let id = render_view.render_path_segment_payload(seg.payload());
                let d = seg.disambiguator();
                if d > 0 {
                    format!("{id}_{d}")
                } else {
                    format!("{id}")
                }
            })
            .map(|str| V::escape(&str))
            .collect();
        strings.reverse();
        strings
    }

    /// Default suffix rendering
    pub fn render_suffix(suffix: &ReservedSuffix) -> String {
        match suffix {
            ReservedSuffix::Pre => "_pre",
            ReservedSuffix::Post => "_post",
            ReservedSuffix::Cast => "_cast_to_repr",
        }
        .to_owned()
    }
}

/// The structured result of rendering a [`View`].
///
/// - `module`: atoms for the crate + modules prefix (may be empty for local/anonymous contexts),
/// - `path`:   atoms for the remaining segments (item, constructors, fields, etc.).
///
/// Join with [`RenderView::rendered_to_string`] to obtain a single string.
pub struct Rendered {
    /// Crate + module atoms (e.g., `["my_crate", "a", "b"]`).
    pub module: Vec<String>,
    /// Non-module atoms (e.g., `["Foo::f", "MyEnum::MyVariant::my_field"]`).
    pub path: Vec<String>,
}
