//! @fls: fls_8kqo952gjhaf, fls_pfhmcafsjyf7, fls_ljocmnaz2m49, fls_jb6krd90tjmc
//! @title: A call of an unsafe function without unsafe operations, inside an unsafe block
//! @lean(tier): supported

pub unsafe fn double(x: u32) -> u32 {
    x * 2
}

pub fn twice(x: u32) -> u32 {
    unsafe { double(x) }
}

#[hax_test::equiv]
pub fn called() -> bool {
    twice(4) == 8
}
