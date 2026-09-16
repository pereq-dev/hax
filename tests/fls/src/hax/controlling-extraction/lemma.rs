//! @fls: hax_lemma
//! @title: A lemma stating a property of a function as a proof obligation
//! @lean(tier): supported

pub fn double(x: u32) -> u32 {
    x.wrapping_add(x)
}

#[hax_lib::lemma]
pub fn double_is_even(x: u32) -> Proof<{ double(x) % 2 == 0 }> {}

#[hax_test::equiv]
pub fn stated_property_holds_on_an_instance() -> bool {
    double(21) == 42 && double(21) % 2 == 0
}
