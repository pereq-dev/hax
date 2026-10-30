//! @fls: fls_fx7b3qv3ghca, fls_r8nqxry2dlww, fls_9vjijqi9w8wn
//! @title: Fixed-size array type with a constant size operand
//! @lean(tier): supported

pub const LEN: usize = 4;

pub fn squares() -> [u32; LEN] {
    let mut a: [u32; LEN] = [0; LEN];
    let mut i = 0;
    while i < LEN {
        a[i] = (i as u32) * (i as u32);
        i += 1;
    }
    a
}

#[hax_test::equiv]
pub fn array_of_squares() -> bool {
    squares() == [0, 1, 4, 9]
}
