//! @fls: fls_B2eKGWaJhFKD, fls_pfa81kv2mru8
//! @title: Sign negation of a floating point value
//! @lean(tier): supported
//! @lean(reason): aeneas rejects the float literals (`Improperly typed constant value`); the arithmetic section of the input-language reference.

pub fn negate(x: f32) -> f32 {
    -x
}

#[hax_test::equiv]
pub fn negated_float() -> bool {
    negate(1.5) == -1.5
}
