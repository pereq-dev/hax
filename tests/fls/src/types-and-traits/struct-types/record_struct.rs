//! @fls: fls_g1azfj548136, fls_0yaYKnFrJkhG
//! @title: Record struct definition, construction, and field access
//! @lean(tier): supported

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn manhattan(p: &Point) -> i32 {
    p.x.abs() + p.y.abs()
}

#[hax_test::equiv]
pub fn construct_and_read_fields() -> bool {
    let p = Point { x: -3, y: 4 };
    manhattan(&p) == 7
}
