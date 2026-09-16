//! @fls: fls_ibrmiwfypldh, fls_jfde2vg6mtww, fls_t17h5h6a6v4c, fls_w0nb0mphho7b, fls_8dcldbvu7lav, fls_eeocxst9vafn, fls_47khd5ljsxeq, fls_dowxbphqvk3n, fls_mt94jvoot9dx
//! @title: Immutable statics read through paths, including one inside a generic function
//! @lean(tier): supported

pub static SCALE: u32 = 10;
pub static TABLE: [u8; 3] = [1, 2, 3];

pub fn scaled<T>(_marker: T, x: u32) -> u32 {
    static OFFSET: u32 = 1;
    x * SCALE + OFFSET
}

pub fn read_table() -> u32 {
    let r = &TABLE;
    r[0] as u32 + TABLE[2] as u32
}

#[hax_test::equiv]
pub fn static_values() -> bool {
    scaled((), 2) == 21 && scaled(true, 0) == 1 && read_table() == 4
}
