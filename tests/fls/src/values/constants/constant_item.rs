//! @fls: fls_5o5iu4j8in4l, fls_ndmfqxjpvsqy, fls_ndobth7s92if, fls_xezt9hl069h4, fls_buyaqara7am4
//! @title: Constant items substituted by their values
//! @lean(tier): supported

pub const LIMIT: u32 = 3 * 4;
pub const NAMES: [u8; 2] = [1, 2];

pub fn twice_limit() -> u32 {
    LIMIT + LIMIT
}

#[hax_test::equiv]
pub fn substituted_values() -> bool {
    twice_limit() == 24 && NAMES[1] == 2
}
