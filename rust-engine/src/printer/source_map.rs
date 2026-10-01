//! Source maps: the position of every `Span`-annotated sub-document in a
//! printed fragment, and their encoding as a source-map v3.

use crate::ast::span::Span;

/// The 0-based `(line, col, span)` at which each `Span`-annotated sub-document
/// of a printed fragment begins.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SourceMap(pub Vec<(usize, usize, Span)>);

impl SourceMap {
    /// Encode as a source-map v3 for `out_file`, whose printed fragment was
    /// preceded by `line_offset` lines. Anchors whose span has no on-disk file
    /// (e.g. phase-synthesized nodes) are skipped; `None` when none is left.
    pub fn to_v3(
        &self,
        out_file: &str,
        line_offset: usize,
    ) -> Option<hax_types::engine_api::SourceMap> {
        let mut sources: Vec<String> = Vec::new();
        // (gen_line, gen_col, source index, src_line0, src_col0)
        let mut segs: Vec<(usize, usize, usize, usize, usize)> = Vec::new();
        for (line, col, span) in &self.0 {
            let Some(fs) = span.as_frontend_spans().first() else {
                continue;
            };
            let Some(path) = fs.filename.to_path() else {
                continue;
            };
            let path = path.display().to_string();
            let idx = match sources.iter().position(|p| *p == path) {
                Some(i) => i,
                None => {
                    sources.push(path);
                    sources.len() - 1
                }
            };
            // rustc lines are 1-based; source-map lines are 0-based.
            segs.push((
                line + line_offset,
                *col,
                idx,
                fs.lo.line.saturating_sub(1),
                fs.lo.col,
            ));
        }
        if segs.is_empty() {
            return None;
        }
        segs.sort_by_key(|s| (s.0, s.1));
        let max_line = segs.iter().map(|s| s.0).max().unwrap_or(0);
        let mut mappings = String::new();
        let (mut p_src, mut p_sl, mut p_sc) = (0i64, 0i64, 0i64);
        let mut segs = segs.into_iter().peekable();
        for line in 0..=max_line {
            if line > 0 {
                mappings.push(';');
            }
            // The generated column is delta-encoded within a line only.
            let mut p_gc = 0i64;
            let mut first = true;
            while let Some((_, gc, idx, sl, sc)) = segs.next_if(|s| s.0 == line) {
                if !first {
                    mappings.push(',');
                }
                first = false;
                for (value, previous) in [
                    (gc, &mut p_gc),
                    (idx, &mut p_src),
                    (sl, &mut p_sl),
                    (sc, &mut p_sc),
                ] {
                    vlq_encode(value as i64 - *previous, &mut mappings);
                    *previous = value as i64;
                }
            }
        }
        Some(hax_types::engine_api::SourceMap {
            version: 3,
            file: out_file.to_string(),
            sourceRoot: String::new(),
            sources,
            // `cargo-hax` fills `sourcesContent` from disk when it writes the
            // `<file>.map` sidecar.
            sourcesContent: Vec::new(),
            names: Vec::new(),
            mappings,
        })
    }
}

/// Append the base64 VLQ encoding of `value` to `out`.
fn vlq_encode(value: i64, out: &mut String) {
    const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    // The sign goes in the low bit.
    let mut v: u64 = (value.unsigned_abs() << 1) | u64::from(value < 0);
    loop {
        let mut digit = (v & 0b1_1111) as usize;
        v >>= 5;
        if v != 0 {
            digit |= 0b10_0000;
        }
        out.push(B64[digit] as char);
        if v == 0 {
            break;
        }
    }
}

/// A `pretty` renderer that produces the same text as
/// [`pretty::DocBuilder::pretty`], and records the position at which every
/// [`Span`]-annotated sub-document begins.
#[derive(Default)]
pub(crate) struct SpanPositionRenderer {
    /// Accumulated rendered text.
    pub(crate) out: String,
    /// Current output line (0-based).
    line: usize,
    /// Current output column (0-based, counted in `char`s).
    col: usize,
    /// Annotation stack: each entry is `(span, already_anchored)`.
    stack: Vec<(Span, bool)>,
    /// One anchor per annotated region that produced text.
    pub(crate) positions: SourceMap,
}

impl SpanPositionRenderer {
    /// Anchor every not-yet-anchored span on the stack to the current output
    /// position, so that a region maps to the position of its first character.
    fn anchor(&mut self) {
        let (line, col) = (self.line, self.col);
        for entry in self.stack.iter_mut() {
            if !entry.1 {
                self.positions.0.push((line, col, entry.0));
                entry.1 = true;
            }
        }
    }
}

impl pretty::Render for SpanPositionRenderer {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<usize, Self::Error> {
        if s.is_empty() {
            return Ok(0);
        }
        self.anchor();
        for c in s.chars() {
            if c == '\n' {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }
        self.out.push_str(s);
        Ok(s.len())
    }

    fn fail_doc(&self) -> Self::Error {}
}

impl<'a> pretty::RenderAnnotated<'a, Span> for SpanPositionRenderer {
    fn push_annotation(&mut self, span: &'a Span) -> Result<(), Self::Error> {
        self.stack.push((*span, false));
        Ok(())
    }

    fn pop_annotation(&mut self) -> Result<(), Self::Error> {
        // A region that produced no text leaves an un-anchored entry; dropping
        // it here keeps its span from being anchored to later text.
        self.stack.pop();
        Ok(())
    }
}
