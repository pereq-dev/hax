//! @fls: fls_87jnkimc15gi, fls_owdlsaaygtho
//! @title: A safe function with the extern "C" qualifier
//! @lean(tier): supported

pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[hax_test::equiv]
pub fn abi_is_erased() -> bool {
    add(2, 3) == 5
}
