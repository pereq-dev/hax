//! @off: fstar, lean, coq, ssprove, legacy-lean
//! @cli: into proverif --keep-trait-calls **::Codec::encode
//!
//! `--keep-trait-calls` keeps a call of a trait item as it is, for a
//! hand-written library to model, rather than resolving it to the impl.

trait Codec {
    fn encode(&self) -> u8;
    fn tag(&self) -> u8;
}

struct A(u8);

impl Codec for A {
    fn encode(&self) -> u8 {
        self.0
    }
    fn tag(&self) -> u8 {
        1
    }
}

// `encode` stays `Codec::encode`; `tag` resolves to the impl.
fn use_codec(a: A) -> (u8, u8) {
    (a.encode(), a.tag())
}

// A library defines the kept call: no warning on a branch on it.
fn branch_on_kept(a: A) -> u8 {
    if a.encode() == 0 { 1 } else { 0 }
}
