//! @fls: fls_doi4z6u55bi7, fls_74hp208pto22
//! @title: Read and write of a mutable static
//! @lean(tier): unsupported
//! @lean(reason): The input-language reference places mutable global state outside the value model.

pub static mut COUNTER: u32 = 0;

pub fn bump() -> u32 {
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}

#[hax_test::equiv]
pub fn bump_twice() -> bool {
    bump();
    bump() >= 2
}
