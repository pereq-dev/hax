//! @fls: hax_loop_invariant
//! @title: A loop invariant stated inside a for loop
//! @lean(tier): supported
//! @lean(reason): The Lean backend has no definition for `hax_lib::_internal_loop_invariant`, which `loop_invariant!` expands to (`Unknown identifier hax_lib._internal_loop_invariant`), so the extraction fails to elaborate; the contracts section of the input-language reference.

#[hax_lib::requires(n < 1000)]
pub fn triangle(n: u32) -> u32 {
    let mut acc = 0;
    for i in 0..n {
        hax_lib::loop_invariant!(|i: u32| 2 * acc == i * i - i);
        acc += i;
    }
    acc
}

#[hax_test::equiv]
pub fn invariant_holds() -> bool {
    triangle(5) == 10
}
