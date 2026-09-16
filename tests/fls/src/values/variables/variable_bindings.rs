//! @fls: fls_hl5tnd9yy252, fls_vgi0gh5zmoiu, fls_81dlbula47nu, fls_3p0sb9ppmg3w, fls_g8etd5lsgn9j, fls_awpw61yofckz
//! @title: Variables declared without an initializer and assigned on every path
//! @lean(tier): supported

pub fn pick(flag: bool) -> u32 {
    let chosen: u32;
    if flag {
        chosen = 1;
    } else {
        chosen = 2;
    }
    let temporary_sum = (chosen + 1) * 2;
    temporary_sum
}

#[hax_test::equiv]
pub fn initialized_before_use() -> bool {
    pick(true) == 4 && pick(false) == 6
}
