//! @fls: fls_xrjp7xw9jutz, fls_l96kyix5xsof, fls_Rp73YEE3aFdI, fls_m0zxktz168e0, fls_y1MUhnKCxK6T, fls_tncxxsyutppf, fls_XsxeOd32JI8x, fls_tvsadfy9uibu, fls_ui0i3rpt5v5u, fls_87o6n9et9jio, fls_gzyx9lfi5pvd, fls_q8wl7pidx2za, fls_p4potvq7x532
//! @title: Conditional compilation with cfg predicates
//! @lean(tier): supported

#[cfg(all())]
pub fn kept() -> u32 {
    1
}

#[cfg(any())]
pub fn kept() -> u32 {
    100
}

#[cfg(not(any()))]
pub const FLAG: u32 = 10;

#[cfg(not(all()))]
pub const FLAG: u32 = 1000;

pub fn total() -> u32 {
    let mut t = kept() + FLAG;
    #[cfg(all(not(any()), any(all(), any())))]
    {
        t += 100;
    }
    t
}

#[hax_test::equiv]
pub fn configured_items() -> bool {
    total() == 111
}
