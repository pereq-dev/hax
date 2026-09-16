//! @fls: fls_pjmbmj2b35y8, fls_xtty2ino4vwc, fls_d0298bmlyuu4, fls_d8spdkjzp496, fls_5u1e0lkt0ab1, fls_vidbcv25dyud
//! @title: Derived implementations of Clone, Copy, PartialEq, Eq, PartialOrd, Ord, and Default
//! @lean(tier): supported

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct S<T> {
    pub x: T,
    pub y: u8,
}

#[hax_test::equiv]
pub fn derived_behaviour() -> bool {
    let a = S { x: 1u32, y: 2 };
    let b = a;
    let c = a.clone();
    let d: S<u32> = Default::default();
    a == b && b == c && d.x == 0 && d < a && a >= d && a.cmp(&d) == core::cmp::Ordering::Greater
}
