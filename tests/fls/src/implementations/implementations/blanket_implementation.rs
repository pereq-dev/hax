//! @fls: fls_ry3an0mwb63g, fls_yopmjbnw8tbl
//! @title: A blanket trait implementation for every type with a bound
//! @lean(tier): supported

pub trait Size {
    fn size(&self) -> u32;
}

pub trait Describe {
    fn describe(&self) -> u32;
}

impl<T: Size> Describe for T {
    fn describe(&self) -> u32 {
        self.size() + 100
    }
}

pub struct Byte;

impl Size for Byte {
    fn size(&self) -> u32 {
        1
    }
}

#[hax_test::equiv]
pub fn blanket_method() -> bool {
    Byte.describe() == 101
}
