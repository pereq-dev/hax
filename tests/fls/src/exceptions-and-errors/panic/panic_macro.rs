//! @fls: fls_eg0orgibg98m, fls_a554v4n0khye, fls_i9njhpte5l0t, fls_n6q7bksyn1m
//! @title: A panic on the failing branch of a checked computation
//! @lean(tier): supported

pub fn must_be_positive(x: i32) -> i32 {
    if x <= 0 {
        panic!("not positive");
    }
    x
}

#[hax_test::equiv]
pub fn non_panicking_path() -> bool {
    must_be_positive(3) == 3
}
