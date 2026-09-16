//! @fls: fls_8fU3h8PLasqA, fls_QMpI8K43K2yU, fls_R48VKcEIbfXC, fls_oFIRXBPXu6Zv
//! @title: Arithmetic overflow observed through wrapping and checked operations
//! @lean(tier): supported

pub fn wrapping(x: u8) -> u8 {
    x.wrapping_add(250)
}

pub fn checked(x: u8) -> Option<u8> {
    x.checked_add(250)
}

#[hax_test::equiv]
pub fn overflow_behaviour() -> bool {
    wrapping(10) == 4 && checked(10) == None && checked(5) == Some(255)
}
