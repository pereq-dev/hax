//! @fls: fls_sgrvona1nb6h, fls_88b9bmhra55f, fls_eWac7zOda3lh
//! @title: Dynamic dispatch through a boxed trait object
//! @lean(tier): supported
//! @lean(reason): aeneas fails on the method call through `Box<dyn Speak>` (`Assertion failed: new value doesn't have the same type as its destination`); the traits section of the input-language reference.

pub trait Speak {
    fn volume(&self) -> u32;
}

pub struct Loud;
pub struct Quiet;

impl Speak for Loud {
    fn volume(&self) -> u32 {
        10
    }
}

impl Speak for Quiet {
    fn volume(&self) -> u32 {
        1
    }
}

pub fn pick(loud: bool) -> Box<dyn Speak> {
    if loud {
        Box::new(Loud)
    } else {
        Box::new(Quiet)
    }
}

#[hax_test::equiv]
pub fn late_bound_method() -> bool {
    pick(true).volume() == 10 && pick(false).volume() == 1
}
