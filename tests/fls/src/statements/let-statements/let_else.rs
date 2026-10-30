//! @fls: fls_ct7pp7jnfr86, fls_SR3dIgR5K0Kq, fls_ea9bRFZjH8Im, fls_4j9riqyf4p9, fls_7j4qlwg72ege
//! @title: Let statements with an else block for a refutable pattern
//! @lean(tier): supported

pub enum Shape {
    Circle(u32),
    Square(u32),
}

pub fn radius_or_zero(s: Option<Shape>) -> u32 {
    let Some(shape) = s else {
        return 0;
    };
    let Shape::Circle(r) = shape else {
        return 1;
    };
    r
}

#[hax_test::equiv]
pub fn diverging_else() -> bool {
    radius_or_zero(None) == 0
        && radius_or_zero(Some(Shape::Square(4))) == 1
        && radius_or_zero(Some(Shape::Circle(7))) == 7
}
