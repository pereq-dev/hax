//! @fls: fls_CYtxPjK3zq2T, fls_I9JaKZelMiby, fls_vucd1u38sq7i, fls_kyr81mi01me2, fls_mjg7yrq66hh0, fls_ydowwijzirmm
//! @title: A subtrait calling a method of its supertrait
//! @lean(tier): supported

pub trait Shape {
    fn area(&self) -> u32;
}

pub trait Circle: Shape {
    fn radius(&self) -> u32;

    fn area_over_radius(&self) -> u32 {
        self.area() / self.radius()
    }
}

pub struct Disc(pub u32);

impl Shape for Disc {
    fn area(&self) -> u32 {
        3 * self.0 * self.0
    }
}

impl Circle for Disc {
    fn radius(&self) -> u32 {
        self.0
    }
}

pub fn describe<C: Circle>(c: &C) -> u32 {
    c.area() + c.radius()
}

#[hax_test::equiv]
pub fn supertrait_method_through_subtrait() -> bool {
    let d = Disc(2);
    describe(&d) == 14 && d.area_over_radius() == 6
}
