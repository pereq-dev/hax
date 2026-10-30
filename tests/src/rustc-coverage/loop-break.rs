//! @fail(extraction): proverif(HAX0001)
//@ edition: 2021

/// @fail(extraction): legacy-lean(HAX0001), fstar(HAX0001), coq(HAX0001, HAX0001), ssprove(HAX0001)
fn main() {
    loop {
        if core::hint::black_box(true) {
            break;
        }
    }
}

// This test is a lightly-modified version of `tests/mir-opt/coverage/instrument_coverage.rs`.
// If this test needs to be blessed, then the mir-opt version probably needs to
// be blessed too!
