//! @fls: fls_66m2hwkju0vv, fls_ib8p7dfwddx2, fls_ucFupTeCyylb
//! @title: Constant expressions in discriminants, static initializers, and const blocks
//! @lean(tier): supported

pub enum Level {
    Low = 1 + 2,
    High = 2 * 4,
}

pub static LIMIT: u32 = 3 * 5;

#[hax_test::equiv]
pub fn contexts_evaluate() -> bool {
    let block = const { 4 * 4 };
    Level::Low as u8 == 3 && Level::High as u8 == 8 && LIMIT == 15 && block == 16
}
