//! @fls: fls_fuBYWRrgxlbQ, fls_URl9CeIVsiWs, fls_JhmQpUoExiNZ, fls_MipY2emZFF6d, fls_YPesUZqYHVUX
//! @title: A type alias for a boxed trait object with the elided 'static bound
//! @lean(tier): supported
//! @lean(reason): aeneas fails on the trait object value (`Assertion failed: new value doesn't have the same type as its destination`); the traits section of the input-language reference.

pub trait Tag {
    fn tag(&self) -> u8;
}

pub struct Unit;

impl Tag for Unit {
    fn tag(&self) -> u8 {
        1
    }
}

pub type Tagged = Box<dyn Tag>;

pub fn boxed() -> Tagged {
    Box::new(Unit)
}

#[hax_test::equiv]
pub fn constructed_trait_object() -> bool {
    let t: Tagged = boxed();
    t.tag() == 1
}
