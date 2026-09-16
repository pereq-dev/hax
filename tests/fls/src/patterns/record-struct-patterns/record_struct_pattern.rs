//! @fls: fls_vjdkpr3zml51, fls_6o3x101wo478, fls_p4OplpUvS04l, fls_pre3YwAv01FE, fls_MK83WE0iDqNf, fls_r8rat3qmc4hy, fls_hUX723DmLg2a, fls_g6dytd6aq62d, fls_46u4ddj0yf93, fls_qu3dvfdq6oy7, fls_9wfizujx0szd, fls_7t0be1w2hq3c, fls_8364ueejn5y3, fls_jTh9Hur0qsIb, fls_m91ith3rjy79, fls_brhtaaxt1s3s, fls_zRCiKnhQebyp, fls_evuhau2rwm8i, fls_bde1hpvrosui, fls_447s4hc07ozn, fls_vfdb1i5l41yk, fls_yfk52fr7trw3, fls_6sdcykdrpe5d, fls_r307spfk6cs9
//! @title: Record struct patterns with named and shorthand deconstructors
//! @lean(tier): supported

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub enum Shape {
    Circle { radius: u32 },
    Rect { w: u32, h: u32 },
}

pub fn swap(p: Point) -> Point {
    let Point { x: a, y: b } = p;
    Point { x: b, y: a }
}

pub fn area(s: Shape) -> u32 {
    match s {
        Shape::Circle { radius } => 3 * radius * radius,
        Shape::Rect { w, .. } if w == 0 => 0,
        Shape::Rect { mut w, h } => {
            w += 1;
            w * h
        }
    }
}

#[hax_test::equiv]
pub fn deconstructed_fields() -> bool {
    let s = swap(Point { x: 1, y: 2 });
    s.x == 2
        && s.y == 1
        && area(Shape::Circle { radius: 2 }) == 12
        && area(Shape::Rect { w: 0, h: 5 }) == 0
        && area(Shape::Rect { w: 2, h: 5 }) == 15
}
