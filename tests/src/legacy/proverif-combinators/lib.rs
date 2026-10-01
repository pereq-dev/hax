//! @fail(extraction): proverif(HAX0001)
//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! Iterator combinators rewritten into bounded loops.

// Collected elements keep their iteration order.
fn collect_in_order(v: Vec<u8>) -> Vec<u8> {
    v.iter().map(|x| *x).collect()
}

fn filter_map_in_order(v: Vec<Option<u8>>) -> Vec<u8> {
    v.into_iter().filter_map(|x| x).collect()
}

// The second search resumes after the first match, so the iterator left over
// by `find` is not the original one.
fn find_twice(v: Vec<u8>) -> (Option<u8>, Option<u8>) {
    let mut it = v.into_iter();
    let a = it.find(|x| *x > 0);
    let b = it.find(|x| *x > 0);
    (a, b)
}

// A loop with no accumulator skips the iterations past the unroll bound,
// rather than ending the trace.
fn for_each_then(v: Vec<u8>, x: u8) -> u8 {
    v.iter().for_each(|y| {
        let _ = y;
    });
    x
}

// Collecting into an `Option` is not pushing every element.
fn collect_option(v: Vec<u8>) -> Option<Vec<u8>> {
    v.into_iter().map(|x| x.checked_add(1)).collect()
}
