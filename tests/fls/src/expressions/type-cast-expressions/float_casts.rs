//! @fls: fls_ltioqbhl14g0, fls_4s69s9pcvbn7, fls_2etd73f8jg2n, fls_4fd5vkh0jt4, fls_4xldaoj5ac6t, fls_50714cvaqkfv, fls_cx86k8yfjhht, fls_g3xbmp8zx1yh, fls_gzmdwibl5s4w, fls_hcc5odh52bk7, fls_mjqvjt7v8a25, fls_o2ep4b6t287z, fls_vfofk2aagsj5
//! @title: Casts between integers and floating point types
//! @lean(tier): supported
//! @lean(reason): aeneas rejects the float literals (`Improperly typed constant value`); the arithmetic section of the input-language reference.

pub fn to_float(x: i32) -> f64 {
    x as f64
}

pub fn to_int(x: f64) -> i32 {
    x as i32
}

pub fn widen(x: f32) -> f64 {
    x as f64
}

#[hax_test::equiv]
pub fn float_conversions() -> bool {
    to_float(3) == 3.0
        && to_int(-2.7) == -2
        && to_int(1e10) == i32::MAX
        && to_int(f64::NAN) == 0
        && widen(0.5) == 0.5
}
