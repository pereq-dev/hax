//! @fls: fls_8cz4rdrklaj4, fls_yyhebj4qyk34, fls_tw8u0cc5867l, fls_vp2ov6ykueue, fls_5uf74nvdm64o, fls_a5prbmuruma4
//! @title: Associated types declared with a bound and initialized in an implementation
//! @lean(tier): supported

pub trait Container {
    type Item: Copy;
    fn first(&self) -> Self::Item;
}

pub struct Pair(pub u32, pub u32);

impl Container for Pair {
    type Item = u32;
    fn first(&self) -> u32 {
        self.0
    }
}

pub fn twice<C: Container>(c: &C) -> (C::Item, C::Item) {
    (c.first(), c.first())
}

#[hax_test::equiv]
pub fn projected_values() -> bool {
    twice(&Pair(4, 5)) == (4, 4)
}
