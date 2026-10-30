//! @fls: fls_5y6ae0xqux57, fls_x564isbhobym, fls_l3iwn56n1uz8
//! @title: Associated constants in a trait with a default, an implementation, and an inherent implementation
//! @lean(tier): supported

pub trait Limits {
    const MAX: u32;
    const MIN: u32 = 0;
}

pub struct Small;

impl Limits for Small {
    const MAX: u32 = 9;
}

impl Small {
    pub const SIZE: u32 = 2;
}

pub fn span<T: Limits>() -> u32 {
    T::MAX - T::MIN
}

#[hax_test::equiv]
pub fn constant_values() -> bool {
    span::<Small>() == 9 && Small::SIZE == 2 && <Small as Limits>::MIN == 0
}
