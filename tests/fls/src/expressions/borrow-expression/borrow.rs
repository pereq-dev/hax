//! @fls: fls_2jd0mgw4zja4, fls_350qejoq9i23, fls_50j167r4v61b, fls_5b2x5ri2w54r, fls_8cvmee9bzs40, fls_agl09ia869rk, fls_chr03xll75d, fls_nnqfkl228hjx, fls_r7ix8webgqlm, fls_ya77l2zgtilp, fls_ursc5ynymoy
//! @title: Immutable and mutable borrow expressions
//! @lean(tier): supported

pub fn read_twice(x: &u32) -> u32 {
    *x + *x
}

pub fn set(x: &mut u32, v: u32) {
    *x = v;
}

#[hax_test::equiv]
pub fn borrows() -> bool {
    let mut answer = 42;
    let ref_answer = &answer;
    let doubled = read_twice(ref_answer);
    let mut_answer = &mut answer;
    set(mut_answer, 7);
    doubled == 84 && answer == 7
}
