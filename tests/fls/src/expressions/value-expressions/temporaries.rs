//! @fls: fls_7q4hrt6yfr9b, fls_8uhfwqurbyqf, fls_pB6xlp4uAg37
//! @title: Value expressions evaluated as temporaries in a place context
//! @lean(tier): supported

pub fn square_plus_one(x: u32) -> u32 {
    let r: &u32 = &(x * x);
    *r + 1
}

pub fn first_of_temporary() -> u32 {
    [3u32, 4][0]
}

#[hax_test::equiv]
pub fn temporaries_hold_values() -> bool {
    square_plus_one(3) == 10 && first_of_temporary() == 3
}
