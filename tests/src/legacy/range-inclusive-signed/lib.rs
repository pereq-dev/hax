//! @fail(tc): legacy-lean(1)
//! For loops over inclusive ranges of signed integers.

/// @fail(extraction): proverif(HAX0008)
fn signed_range() -> i32 {
    let mut sum = 0i32;
    for i in -3i32..=3i32 {
        sum += i;
    }
    sum
}
