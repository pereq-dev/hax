//! @fls: fls_7Ck4LMQMeQCv, fls_99ea30a5mulj, fls_bbf54ukld7j9, fls_bqmwlona6l5w, fls_d9avvfi548t7, fls_njndlx2rps2k, fls_o3fc1z2mn8zc, fls_r610t5vsi7bx, fls_tjksia7prao1, fls_u7jk4j8gkho, fls_ubwj8uj6sbgt
//! @title: Return expressions with and without an operand
//! @lean(tier): supported

pub fn first_negative(xs: &[i32]) -> i32 {
    let mut i = 0;
    while i < xs.len() {
        if xs[i] < 0 {
            return xs[i];
        }
        i += 1;
    }
    0
}

pub fn reset_if_large(x: &mut u32) {
    if *x < 100 {
        return;
    }
    *x = 0;
}

#[hax_test::equiv]
pub fn early_returns() -> bool {
    let mut a = 500;
    let mut b = 5;
    reset_if_large(&mut a);
    reset_if_large(&mut b);
    first_negative(&[3, -2, -8]) == -2 && first_negative(&[1]) == 0 && a == 0 && b == 5
}
