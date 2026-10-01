//! @off: fstar, lean, coq, ssprove
//! @fail(extraction): proverif(HAX0001)
//!
//! A nullary item keeps its initializer, so a projection off a `const` whose
//! value is known reduces like any other constructor projection. Only a
//! constant whose body is *not* extracted — here `#[hax_lib::opaque]` — stays
//! an opaque atom with no constructor; a projection off that one can never
//! reduce, so it is flagged and rendered as the error sink.

struct Tag(u32);

impl Tag {
    const DEFAULT: Tag = Tag(0);
}

/// The value of `DEFAULT` survives, so `.0` reduces to `nat_0`.
fn const_projection() -> u32 {
    Tag::DEFAULT.0
}

#[hax_lib::opaque]
fn hidden() -> Tag {
    Tag(1)
}

/// `hidden()` has no body in the model, so `.0` can never reduce.
fn dead_projection() -> u32 {
    hidden().0
}

fn live_projection(t: Tag) -> u32 {
    t.0
}
