//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! A symbol named in a verbatim ProVerif quote is bare there, even a function
//! that takes arguments.

#![allow(dead_code)]

#[hax_lib::proverif::replace("fun ${encrypt}(bitstring, bitstring): bitstring.")]
fn encrypt(key: u8, message: u8) -> u8 {
    key ^ message
}

fn send(key: u8, message: u8) -> u8 {
    encrypt(key, message)
}
