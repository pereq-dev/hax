//! @fls: fls_dti0uu7rz81w, fls_bv87t1gvj7bz, fls_bl55g03jmayf, fls_c354oryv513p, fls_g0JEluWqBpNc, fls_r9fhuiq1ys1p
//! @title: Weak keywords as identifiers outside their context and 'static as a lifetime
//! @lean(tier): supported

pub fn as_identifiers() -> u32 {
    let union = 1;
    let safe = 2;
    let macro_rules = 3;
    union + safe + macro_rules
}

pub fn as_lifetime<T: 'static>(x: T) -> T {
    x
}

#[hax_test::equiv]
pub fn weak_keywords_depend_on_context() -> bool {
    as_identifiers() + as_lifetime(4) == 10
}
