//! @fls: fls_e7ucq87s806d, fls_4nkzidytpi6, fls_9m0gszdle0qb, fls_wzuwapjqtyyy, fls_l2xkdjeydqtx, fls_bync24y6gp93, fls_ag249y74jg6c, fls_gfvm70iqu1l4, fls_fmn33zhorkf
//! @title: A type with a Drop implementation used and dropped like any other value
//! @lean(tier): supported

pub struct Guard {
    pub value: u32,
}

impl Drop for Guard {
    fn drop(&mut self) {
        self.value = 0;
    }
}

pub fn read_before_drop() -> u32 {
    let g = Guard { value: 5 };
    let copy = g.value;
    let _uninitialized: Guard;
    copy + 1
}

#[hax_test::equiv]
pub fn destructor_is_transparent() -> bool {
    read_before_drop() == 6
}
