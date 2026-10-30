//! @fail(extraction): proverif(HAX0001)
//! @fail(tc): fstar(2), legacy-lean(1)
#![allow(unused_assignments)]

#[rustfmt::skip]
/// @fail(extraction): fstar(HAX0001), coq(HAX0001, HAX0001), legacy-lean(HAX0001), ssprove(HAX0001)
fn main() {
    // Initialize test constants in a way that cannot be determined at compile time, to ensure
    // rustc and LLVM cannot optimize out statements (or coverage counters) downstream from
    // dependent conditions.
    let is_true = std::env::args().len() == 1;

    let mut countdown = 0;

    if
        is_true
    {
        countdown
        =
            10
        ;
    }

    loop
    {
        if
            countdown
                ==
            0
        {
            break
            ;
        }
        countdown
        -=
        1
        ;
    }
}
