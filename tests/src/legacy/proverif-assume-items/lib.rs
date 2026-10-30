//! @off: fstar, lean, coq, ssprove, legacy-lean
//! @cli: into proverif --assume-items +**::assumed_parse --assume-items +**::assumed_method --assume-items +**::ASSUMED_CONST --assume-items +**::assumed_cycle
//!
//! `--assume-items` drops the bodies of the selected functions.

fn assumed_parse(x: u8) -> u8 {
    x + 1
}

fn kept(x: u8) -> u8 {
    assumed_parse(x)
}

struct S;

impl S {
    fn assumed_method(&self, x: u8) -> u8 {
        x
    }

    const ASSUMED_CONST: u8 = 7;
}

// Assuming `assumed_cycle` cuts the recursion, so `in_cycle` keeps its body.
fn assumed_cycle(x: u8) -> u8 {
    in_cycle(x)
}

fn in_cycle(x: u8) -> u8 {
    if x == 0 { 0 } else { assumed_cycle(x - 1) }
}
