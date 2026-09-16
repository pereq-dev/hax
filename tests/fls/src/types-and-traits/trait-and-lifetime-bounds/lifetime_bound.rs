//! @fls: fls_grby8tmmd8sb, fls_1Sm2Yq1Ow76f, fls_5kj8bmvb8xfc, fls_tx4uspewnk7w, fls_J9DEsd06Ttu9, fls_IfHRxSasGAih, fls_K8nPGP5xbLb7, fls_Uw6y5ZlaK6RY
//! @title: Outlives bounds on type and lifetime parameters
//! @lean(tier): supported

pub struct Holder<'a, T: 'a> {
    pub inner: &'a T,
}

pub fn longer<'a, 'b: 'a>(x: &'a u32, y: &'b u32) -> &'a u32 {
    if x > y { x } else { y }
}

pub fn read<'a, T: 'a + Copy>(h: Holder<'a, T>) -> T {
    *h.inner
}

#[hax_test::equiv]
pub fn bounds_do_not_change_values() -> bool {
    let a = 3;
    let b = 5;
    *longer(&a, &b) == 5 && read(Holder { inner: &a }) == 3
}
