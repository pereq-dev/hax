//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! A call to a method of an excluded impl is an opaque symbol, with a warning:
//! not the identity that `primitives.pvl` gives std traits.
#![allow(dead_code)]

struct Counter(u8);

#[hax_lib::exclude]
impl Clone for Counter {
    fn clone(&self) -> Self {
        Counter(self.0 ^ 1)
    }
}

fn copy(c: &Counter) -> Counter {
    c.clone()
}

struct Raw(u8);
struct Key(u8);

#[hax_lib::exclude]
impl From<Raw> for Key {
    fn from(r: Raw) -> Self {
        Key(r.0 ^ 1)
    }
}

fn to_key(r: Raw) -> Key {
    r.into()
}
