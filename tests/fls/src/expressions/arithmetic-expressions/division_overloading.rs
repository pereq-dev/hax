//! @fls: fls_qd6ggdgq2hob, fls_k7lmxvpkxtub
//! @title: Division and remainder operators on a struct through the core::ops traits
//! @lean(tier): supported

use core::ops::{Div, Rem};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct V2 {
    pub x: i32,
    pub y: i32,
}

impl Div<i32> for V2 {
    type Output = V2;
    fn div(self, k: i32) -> V2 {
        V2 {
            x: self.x / k,
            y: self.y / k,
        }
    }
}

impl Rem<i32> for V2 {
    type Output = V2;
    fn rem(self, k: i32) -> V2 {
        V2 {
            x: self.x % k,
            y: self.y % k,
        }
    }
}

#[hax_test::equiv]
pub fn overloaded_operators() -> bool {
    let v = V2 { x: 7, y: 12 };
    v / 2 == V2 { x: 3, y: 6 } && v % 5 == V2 { x: 2, y: 2 }
}
