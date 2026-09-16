//! @fls: fls_ftvua2hlvr08, fls_5gl67ftc3m21, fls_nsny832ap4v1, fls_26Xgem831Nqg, fls_Im7miUSS87xs
//! @title: A slice as a view into an array
//! @lean(tier): supported

pub fn sum(xs: &[u32]) -> u32 {
    let mut total = 0;
    let mut i = 0;
    while i < xs.len() {
        total += xs[i];
        i += 1;
    }
    total
}

#[hax_test::equiv]
pub fn sum_of_slice() -> bool {
    let a = [1u32, 2, 3, 4];
    let s: &[u32] = &a[1..3];
    sum(s) == 5 && sum(&a) == 10
}
