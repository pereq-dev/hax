//! @fls: fls_JWx8vQwl19Fu, fls_SziFAQsio0ab, fls_jbl9xyynjo0g
//! @title: The cfg predicates true and false and a configuration option
//! @lean(tier): supported

#[cfg(true)]
pub const KEPT: u32 = 1;

#[cfg(false)]
pub const KEPT: u32 = 100;

#[cfg(any(unix, not(unix)))]
pub const OPTION: u32 = 10;

#[cfg(not(any(unix, not(unix))))]
pub const OPTION: u32 = 1000;

#[hax_test::equiv]
pub fn configured_items() -> bool {
    KEPT + OPTION == 11
}
