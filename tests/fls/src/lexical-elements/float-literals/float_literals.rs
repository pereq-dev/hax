//! @fls: fls_s76un78zyd0j, fls_fqpqnku27v99, fls_rzi7oeqokd6e, fls_2ru1zyrykd37, fls_21mhnhplzam7, fls_cbs7j9pjpusw, fls_b9w7teaw1f8f, fls_eawxng4ndhv0, fls_drqh80k0sfkb, fls_yuhza1muo7o, fls_4sxt1ct7fyen, fls_wa72rssp0jnt
//! @title: Suffixed and unsuffixed float literals
//! @lean(tier): supported
//! @lean(reason): aeneas rejects the float literals (`Improperly typed constant value`); the arithmetic section of the input-language reference.

pub fn suffixed() -> (f32, f64) {
    (1.5f32, 2.5e-1_f64)
}

pub fn from_parameter(x: f32) -> f32 {
    x * 2.0
}

// Without a constraint the literals are `f64`: in `f32` the sum rounds to
// exactly `0.3`.
pub fn defaulted_is_f64() -> bool {
    0.1 + 0.2 != 0.3
}

#[hax_test::equiv]
pub fn float_values() -> bool {
    let (a, b) = suffixed();
    a == 1.5 && b == 0.25 && from_parameter(1.25) == 2.5 && defaulted_is_f64()
}
