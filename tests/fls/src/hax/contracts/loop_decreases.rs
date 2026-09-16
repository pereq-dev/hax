//! @fls: hax_loop_decreases
//! @title: A termination measure for a while loop stated with loop_decreases
//! @lean(tier): supported
//! @lean(reason): The Lean backend has no definition for `hax_lib::_internal_loop_decreases`, which `loop_decreases!` expands to (`Unknown identifier hax_lib._internal_loop_decreases`), so the extraction fails to elaborate; the contracts section of the input-language reference.

pub fn count_down(mut n: u32) -> u32 {
    let mut steps = 0;
    while n > 0 {
        hax_lib::loop_decreases!(n);
        n -= 1;
        steps += 1;
    }
    steps
}

#[hax_test::equiv]
pub fn loop_terminates() -> bool {
    count_down(5) == 5
}
