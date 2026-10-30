//! @fls: fls_42ijvuqqqlvh, fls_X9kdEAPTqsAe, fls_i68oxj659hc1, fls_ehamppbq4gmg, fls_fouu0z3jwoad
//! @title: Index expressions on a struct through Index and IndexMut
//! @lean(tier): supported

use core::ops::{Index, IndexMut};

pub struct Pair {
    pub a: u32,
    pub b: u32,
}

impl Index<bool> for Pair {
    type Output = u32;
    fn index(&self, second: bool) -> &u32 {
        if second { &self.b } else { &self.a }
    }
}

impl IndexMut<bool> for Pair {
    fn index_mut(&mut self, second: bool) -> &mut u32 {
        if second { &mut self.b } else { &mut self.a }
    }
}

#[hax_test::equiv]
pub fn overloaded_index() -> bool {
    let mut p = Pair { a: 1, b: 2 };
    p[true] += 5;
    p[false] == 1 && p[true] == 7
}
