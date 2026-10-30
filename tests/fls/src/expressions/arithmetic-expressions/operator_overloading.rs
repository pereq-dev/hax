//! @fls: fls_asibqpe3z95h, fls_cayhj5hcuhcg, fls_b9g0r9vc4rou, fls_blyr18iao20n, fls_ryzhdpxgm7ii
//! @title: Arithmetic operators on a struct through the core::ops traits
//! @lean(tier): supported

use core::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct V2 {
    pub x: i32,
    pub y: i32,
}

impl Add for V2 {
    type Output = V2;
    fn add(self, o: V2) -> V2 {
        V2 {
            x: self.x + o.x,
            y: self.y + o.y,
        }
    }
}

impl Sub for V2 {
    type Output = V2;
    fn sub(self, o: V2) -> V2 {
        V2 {
            x: self.x - o.x,
            y: self.y - o.y,
        }
    }
}

impl Mul<i32> for V2 {
    type Output = V2;
    fn mul(self, k: i32) -> V2 {
        V2 {
            x: self.x * k,
            y: self.y * k,
        }
    }
}

#[hax_test::equiv]
pub fn overloaded_operators() -> bool {
    let a = V2 { x: 1, y: 2 };
    let b = V2 { x: 3, y: 5 };
    (a + b) * 2 - a == V2 { x: 7, y: 12 }
}
