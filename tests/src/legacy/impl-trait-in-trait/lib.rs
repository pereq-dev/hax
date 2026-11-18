//! @off: coq, ssprove, proverif
//! Return-position `impl Trait` in trait methods (#1965).

#![allow(dead_code)]

pub struct Foo(u8);

pub trait Streamer {
    fn stream(&self) -> impl Clone;
}

impl Streamer for Foo {
    fn stream(&self) -> impl Clone {
        self.0
    }
}

/// @fail(extraction): fstar(HAX0001), legacy-lean(HAX0001)
pub trait GenStreamer {
    fn gstream<const N: usize>(&self) -> impl Clone;
}
