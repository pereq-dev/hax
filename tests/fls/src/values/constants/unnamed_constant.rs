//! @fls: fls_3mhj0kkupwuz, fls_5o5iu4j8in4l
//! @title: An unnamed constant holding a compile-time assertion
//! @lean(tier): supported

pub const LIMIT: u32 = 12;
const _: () = assert!(LIMIT == 12);

#[hax_test::equiv]
pub fn named_constant_still_usable() -> bool {
    LIMIT == 12
}
