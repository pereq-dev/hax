//! @fls: fls_ry3an0mwb63g, fls_47x0ep8of8wr, fls_yopmjbnw8tbl, fls_a2utf0tmuhy4, fls_eIHc8Y9fBtr0
//! @title: A trait implementation for a struct and for a primitive type
//! @lean(tier): supported

pub trait Area {
    fn area(&self) -> u32;
}

pub struct Rect(pub u32, pub u32);

impl Area for Rect {
    fn area(&self) -> u32 {
        self.0 * self.1
    }
}

impl Area for u32 {
    fn area(&self) -> u32 {
        *self
    }
}

pub fn total<A: Area, B: Area>(a: &A, b: &B) -> u32 {
    a.area() + b.area()
}

#[hax_test::equiv]
pub fn implemented_trait() -> bool {
    total(&Rect(2, 3), &4u32) == 10
}
