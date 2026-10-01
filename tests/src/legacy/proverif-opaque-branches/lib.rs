//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! Branches on values the model cannot always evaluate carry a warning, which
//! the banner lists.
#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::VecDeque;

// `len() > 64` only reduces when both sides are small literals.
fn long_message(m: &[u8]) -> u8 {
    if m.len() > 64 { 1 } else { 0 }
}

// `cmp` is not defined by the model, so no arm is taken.
fn compare(a: u8, b: u8) -> u8 {
    match a.cmp(&b) {
        Ordering::Less => 0,
        Ordering::Equal => 1,
        Ordering::Greater => 2,
    }
}

// `VecDeque::contains` is not defined either.
fn seen(q: &VecDeque<u8>, x: u8) -> bool {
    if q.contains(&x) { true } else { false }
}

// A branch the model evaluates carries no warning.
fn is_zero(x: u8) -> u8 {
    if x == 0 { 1 } else { 0 }
}

// A symbol declared only by verbatim text is defined: no warning.
#[hax_lib::exclude]
fn parse(x: u8) -> bool {
    x == 0
}

#[hax_lib::proverif::before(
    "fun new_tests__legacy__proverif_opaque_branches__lib__parse(bitstring): bitstring [data]."
)]
fn uses_parse(x: u8) -> u8 {
    if parse(x) { 1 } else { 0 }
}

// A symbol `pv_inverse_of` declares, through an embedded name, is defined: no
// warning.
#[hax_lib::pv_constructor]
fn encode(x: u8) -> u8 {
    x
}

#[hax_lib::pv_inverse_of(encode)]
fn decode(x: u8) -> Option<u8> {
    Some(x)
}

fn round_trip(x: u8) -> u8 {
    match decode(encode(x)) {
        Some(y) => y,
        None => 0,
    }
}
