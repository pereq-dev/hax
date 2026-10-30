//! @fls: fls_gbdd37seqoab, fls_wQTFwl88VujQ
//! @title: Enum with unit, tuple, and record variants
//! @lean(tier): supported

pub enum Shape {
    Empty,
    Circle(u32),
    Rect { w: u32, h: u32 },
}

pub fn area(s: &Shape) -> u32 {
    match s {
        Shape::Empty => 0,
        Shape::Circle(r) => 3 * *r * *r,
        Shape::Rect { w, h } => *w * *h,
    }
}

#[hax_test::equiv]
pub fn areas() -> bool {
    area(&Shape::Empty) == 0
        && area(&Shape::Circle(2)) == 12
        && area(&Shape::Rect { w: 3, h: 4 }) == 12
}
