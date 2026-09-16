//! @fls: fls_djlglv2eaihl, fls_xangrq3tfze0, fls_2w0xi6rxw3uz, fls_9zitf1fvvfk8, fls_a2d8ltpgtvn6, fls_x7ct9k82fpgn, fls_r2drzo3dixe4, fls_z2kzyin8dyr7
//! @title: Safe functions with the Rust, C, and implicit C ABIs
//! @lean(tier): supported

pub extern "Rust" fn rust_abi(x: u32) -> u32 {
    x + 1
}

pub extern "C" fn c_abi(x: u32) -> u32 {
    x + 2
}

// rustfmt would spell out the ABI.
#[rustfmt::skip]
#[allow(missing_abi)]
pub extern fn implicit_c_abi(x: u32) -> u32 {
    x + 3
}

pub fn default_abi(x: u32) -> u32 {
    x + 4
}

#[hax_test::equiv]
pub fn abi_is_erased() -> bool {
    rust_abi(0) + c_abi(0) + implicit_c_abi(0) + default_abi(0) == 10
}
