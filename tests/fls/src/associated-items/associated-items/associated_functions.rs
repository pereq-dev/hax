//! @fls: fls_ckzd25qd213t, fls_bnTcCbDvdp94, fls_N3cdn4lCZ2Bf, fls_lj7492aq7fzo, fls_b6nns7oqvdpm, fls_qb5qpfe0uwk, fls_amWtS80fPtza, fls_oy92gzxgc273, fls_WXnCWfJGoQx3, fls_OaszUw4IFobz, fls_Wd2FZRomB5yn, fls_lcEyToYIlcmf
//! @title: Associated functions and methods in traits, trait implementations, and inherent implementations
//! @lean(tier): supported

pub trait Shape {
    fn unit() -> Self;
    fn area(&self) -> u32;
    fn grow(&mut self);
}

pub struct Square(pub u32);

impl Shape for Square {
    fn unit() -> Self {
        Square(1)
    }
    fn area(&self) -> u32 {
        self.0 * self.0
    }
    fn grow(&mut self) {
        self.0 += 1;
    }
}

impl Square {
    pub fn side(self) -> u32 {
        self.0
    }
}

#[hax_test::equiv]
pub fn associated_calls() -> bool {
    let mut s = Square::unit();
    s.grow();
    s.area() == 4 && s.side() == 2
}
