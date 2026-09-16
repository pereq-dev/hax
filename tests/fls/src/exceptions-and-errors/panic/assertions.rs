//! @fls: fls_eg0orgibg98m, fls_a554v4n0khye
//! @title: The assertion macros that panic when their condition fails
//! @lean(tier): supported

pub fn clamp(x: u32) -> u32 {
    let y = if x > 10 { 10 } else { x };
    assert!(y <= 10);
    assert_eq!(y + 0, y);
    assert_ne!(y, 11);
    debug_assert!(y <= x);
    if y > 10 {
        unreachable!();
    }
    y
}

#[hax_test::equiv]
pub fn assertions_hold() -> bool {
    clamp(3) == 3 && clamp(30) == 10
}
