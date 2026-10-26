//! @fail(tc): legacy-lean(1)
//! For loops over inclusive integer ranges with `break`.

/// @fail(extraction): coq(HAX0001), ssprove(HAX0001), proverif(HAX0008)
fn with_break(n: u8) -> u64 {
    let mut count = 0u64;
    for i in 0..=n {
        if i == 3 {
            break;
        }
        count += 1;
    }
    count
}
