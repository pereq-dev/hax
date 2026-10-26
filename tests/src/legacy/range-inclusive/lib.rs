//! For loops over inclusive integer ranges.

/// `end + 1` does not fit in a `u8` here.
/// @fail(extraction): proverif(HAX0008)
fn full_u8_range() -> u64 {
    let mut count = 0u64;
    for i in 0u8..=255u8 {
        count += 1;
    }
    count
}

#[hax_lib::requires(n < 255)]
#[hax_lib::ensures(|res| n < res)]
/// @fail(extraction): proverif(HAX0008)
fn count_up_to(n: u8) -> u8 {
    let mut count = 0u8;
    for i in 0..=n {
        hax_lib::loop_invariant!(|i: u8| count == i);
        count += 1;
    }
    count
}

/// @fail(extraction): proverif(HAX0008)
fn empty_range() -> u64 {
    let mut count = 0u64;
    for i in 5u8..=3u8 {
        count += 1;
    }
    count
}

/// @fail(extraction): proverif(HAX0008)
fn single_element() -> u64 {
    let mut count = 0u64;
    for i in 3u8..=3u8 {
        count += 1;
    }
    count
}

/// @fail(extraction): proverif(HAX0008)
fn with_return(n: u8) -> u8 {
    for i in 0..=n {
        if i == 7 {
            return i;
        }
    }
    0
}
