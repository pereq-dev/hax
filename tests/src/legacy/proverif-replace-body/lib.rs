//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! `proverif::replace_body` swaps a function's body for verbatim ProVerif.

struct Counter(u8);

impl Counter {
    // The replacement is the whole result.
    #[hax_lib::proverif::replace_body("nat_0")]
    fn zero(&self) -> u8 {
        self.0
    }

    // The replacement gives the result, and the model keeps `self`
    // unchanged, with a warning.
    #[hax_lib::proverif::replace_body("self")]
    fn bump(&mut self) {
        self.0 += 1;
    }
}

fn use_bump(mut c: Counter) -> Counter {
    c.bump();
    c
}

// `model` is declared by the text placed before `declares`, which comes after
// its user in the source: the text is still emitted first.
#[hax_lib::proverif::replace_body("model(x)")]
fn uses(x: u8) -> u8 {
    x
}

#[hax_lib::proverif::before("letfun model(x: bitstring) = x.")]
fn declares() {}

// A replaced item's text declares it, so `missingdecl.pvl` does not.
#[hax_lib::proverif::replace(
    "letfun new_tests__legacy__proverif_replace_body__lib__replaced(x: bitstring) = x."
)]
fn replaced(x: u8) -> u8 {
    x ^ 1
}

fn uses_replaced(x: u8) -> u8 {
    replaced(x)
}

// The original body of a replaced function is not referenced.
#[hax_lib::proverif::replace_body("x")]
fn replaced_body(x: u8) -> u8 {
    x.rotate_left(3)
}

// A replaced body is not emitted, so its closures are not rejected.
#[hax_lib::pv_constructor]
fn pack(x: u8) -> [u8; 1] {
    [x]
}

#[hax_lib::pv_inverse_of(pack)]
fn unpack(bytes: [u8; 1]) -> Option<u8> {
    let first = |b: [u8; 1]| b[0];
    Some(first(bytes))
}
