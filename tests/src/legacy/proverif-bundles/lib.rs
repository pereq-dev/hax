//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! Items of mutually dependent modules, which hax bundles together, keep their
//! meaning: record literals follow the declaration order, and `.into()` calls
//! the `From` impl.
#![allow(dead_code)]

mod a {
    pub struct R2 {
        pub x: u8,
        pub y: u8,
    }

    pub struct Key(pub u8);

    // `from` calls into `b`, which calls back into `a`: the two modules are
    // bundled.
    impl From<super::b::Raw> for Key {
        fn from(r: super::b::Raw) -> Self {
            Key(super::b::helper(r.0))
        }
    }

    pub fn k(x: u8) -> u8 {
        x ^ 1
    }

    pub fn make(x: u8, y: u8) -> R2 {
        super::b::build(x, y)
    }

    pub fn first(r: R2) -> u8 {
        r.x
    }
}

mod b {
    pub struct Raw(pub u8);

    pub fn helper(x: u8) -> u8 {
        super::a::k(x)
    }

    // `R2 { y, x }` is `R2(x, y)`.
    pub fn build(x: u8, y: u8) -> super::a::R2 {
        super::a::R2 { y, x }
    }

    pub fn to_key(r: Raw) -> super::a::Key {
        r.into()
    }
}

mod c {
    // Outside the bundle, but using its items.
    pub fn build(x: u8, y: u8) -> super::a::R2 {
        super::a::R2 { y, x }
    }

    pub fn to_key(r: super::b::Raw) -> super::a::Key {
        r.into()
    }
}
