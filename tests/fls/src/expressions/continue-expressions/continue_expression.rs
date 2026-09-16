//! @fls: fls_767gv7zhqamh, fls_d0bmw8xiw5nk, fls_gvuesa5ekeif, fls_r5ke7b9n7k3s, fls_vmyuuptfnwek
//! @title: A continue expression restarting the innermost loop
//! @lean(tier): supported

pub fn sum_odd(limit: u32) -> u32 {
    let mut i = 0;
    let mut acc = 0;
    while i < limit {
        i += 1;
        if i % 2 == 0 {
            continue;
        }
        acc += i;
    }
    acc
}

#[hax_test::equiv]
pub fn skipped_iterations() -> bool {
    sum_odd(6) == 9
}
