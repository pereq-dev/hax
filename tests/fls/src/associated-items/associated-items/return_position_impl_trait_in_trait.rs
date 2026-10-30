//! @fls: fls_WnsVATJvUdza, fls_Cu8FWrisrqz1, fls_b6nns7oqvdpm
//! @title: An impl Trait return type in a trait function
//! @lean(tier): supported
//! @lean(reason): aeneas fails on the `impl Trait` return type of a trait function (`Unimplemented` in `translate_fwd_ty`); the functions section of the input-language reference.

pub trait Doubler {
    fn doubled(&self) -> impl PartialEq<u32>;
}

pub struct N(pub u32);

impl Doubler for N {
    fn doubled(&self) -> impl PartialEq<u32> {
        self.0 * 2
    }
}

#[hax_test::equiv]
pub fn opaque_return_in_trait() -> bool {
    N(4).doubled() == 8
}
