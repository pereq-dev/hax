//! @fls: fls_a0u9nnaf6drz, fls_9stc6nul6vq9, fls_pf1v89h7pjhh, fls_h6b1fuw4nvi1
//! @title: Source code removed by conditional compilation
//! @lean(tier): supported

#[cfg(any())]
pub fn value() -> u32 {
    1
}

#[cfg(all())]
pub fn value() -> u32 {
    2
}

#[cfg_attr(any(), deprecated)]
pub fn plain() -> u32 {
    3
}

#[hax_test::equiv]
pub fn compiled_items() -> bool {
    value() + plain() == 5
}
