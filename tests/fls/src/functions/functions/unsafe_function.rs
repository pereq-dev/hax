//! @fls: fls_7ucwmzqtittv, fls_5hn8fkf7rcvz
//! @title: An unsafe function called from an unsafe block
//! @lean(tier): supported

pub unsafe fn halve_nonzero(x: u32) -> u32 {
    x / 2
}

pub fn halve(x: u32) -> u32 {
    if x == 0 {
        0
    } else {
        unsafe { halve_nonzero(x) }
    }
}

#[hax_test::equiv]
pub fn halved() -> bool {
    halve(8) == 4 && halve(0) == 0
}
