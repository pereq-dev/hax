//! @fls: fls_hp3f4r3399kt
//! @title: An item shadowing a prelude name
//! @lean(tier): supported

pub fn drop(x: u32) -> u32 {
    x + 1
}

#[hax_test::equiv]
pub fn item_shadows_prelude() -> bool {
    drop(1) == 2
}
