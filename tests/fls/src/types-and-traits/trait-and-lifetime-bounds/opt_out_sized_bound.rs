//! @fls: fls_AzuZmR9DXSQh
//! @title: A ?Sized opt-out bound allows a slice type argument
//! @lean(tier): supported

pub trait Count {
    fn count(&self) -> usize;
}

impl Count for [u8] {
    fn count(&self) -> usize {
        self.len()
    }
}

impl Count for u8 {
    fn count(&self) -> usize {
        1
    }
}

pub fn count_of<T: ?Sized + Count>(x: &T) -> usize {
    x.count()
}

#[hax_test::equiv]
pub fn sized_and_unsized_arguments() -> bool {
    let s: &[u8] = &[7, 8];
    count_of(s) == 2 && count_of(&7u8) == 1
}
