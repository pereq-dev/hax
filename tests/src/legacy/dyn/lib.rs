//! @fail(tc): legacy-lean(1)
//! @fail(extraction): legacy-lean(HAX0001, HAX0001)
//! @fail(tc): fstar(72)

#![allow(dead_code)]

pub trait Printable<S> {
    fn stringify(&self) -> S;
}

impl Printable<String> for i32 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

/// @fail(extraction): ssprove(HAX0008), coq(HAX0008)
/// @fail(extraction): proverif(HAX0008)
pub fn print(a: Box<dyn Printable<String>>) {
    println!("{}", a.stringify());
}
