//! @fls: fls_30yny2xb9b6b, fls_yqflrq9s6p6n
//! @title: Arithmetic on f32 and f64 through the float model
//! @lean(tier): supported
//! @lean(reason): aeneas rejects the float literals (`Improperly typed constant value`); the arithmetic section of the input-language reference.

pub fn average(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

pub fn halve(x: f32) -> f32 {
    x / 2.0
}

#[hax_test::equiv]
pub fn average_of_two() -> bool {
    average(1.0, 3.0) == 2.0
}

#[hax_test::equiv]
pub fn halve_four() -> bool {
    halve(4.0) == 2.0
}
