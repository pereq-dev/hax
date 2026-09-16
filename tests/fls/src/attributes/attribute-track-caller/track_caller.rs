//! @fls: fls_w1pxtzp7acty, fls_y1e258p4rby5, fls_zch43jpetmdu, fls_gxxbf6eag3et
//! @title: The track_caller attribute on a function and on a trait function
//! @lean(tier): supported

pub trait Checked {
    #[track_caller]
    fn checked_get(&self, i: usize) -> u32;
}

pub struct Table(pub [u32; 2]);

impl Checked for Table {
    fn checked_get(&self, i: usize) -> u32 {
        self.0[i]
    }
}

#[track_caller]
pub fn nonzero(x: u32) -> u32 {
    if x == 0 {
        panic!("zero");
    }
    x
}

#[hax_test::equiv]
pub fn tracked_functions_compute() -> bool {
    nonzero(4) == 4 && Table([5, 6]).checked_get(1) == 6
}
