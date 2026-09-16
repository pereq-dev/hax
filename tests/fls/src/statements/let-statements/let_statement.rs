//! @fls: fls_ct7pp7jnfr86, fls_SR3dIgR5K0Kq, fls_1prqh1trybwz, fls_djkm8r2iuu6u, fls_ppj9gvhp8wcj, fls_iB25BeFys0j8, fls_r38TXWKQPjxv, fls_6QSdwF4pzjoe, fls_zObyLdya4DYc, fls_m8a7gesa4oim, fls_oaxnre7m9s10, fls_t5bjwluyv8za, fls_4j9riqyf4p9, fls_t53g5hlabqw1, fls_7j4qlwg72ege
//! @title: Let statements with and without type ascriptions and initializers
//! @lean(tier): supported

pub fn compute(flag: bool) -> u32 {
    let ascribed: u32 = 4;
    let inferred = ascribed * 2;
    let (a, b): (u32, u32) = (inferred, 1);
    let later;
    if flag {
        later = a;
    } else {
        later = b;
    }
    later
}

#[hax_test::equiv]
pub fn bound_values() -> bool {
    compute(true) == 8 && compute(false) == 1
}
