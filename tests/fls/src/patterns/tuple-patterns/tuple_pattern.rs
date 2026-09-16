//! @fls: fls_e2manugp4e0b, fls_yhcaz6v49ub2, fls_xk8udu4k61kj, fls_8r81vtv5hnrd, fls_6WCm0Ra8NQl4, fls_a3qvQjyilORx, fls_5bXqIaKiFcLg, fls_soHCAVfGlv5f, fls_iiKvYs61959S, fls_KmIHFxlBYelZ, fls_cC6ohNuiltfL, fls_w4xypnrnhycb, fls_vnx1bpval595, fls_dzf32f40y7fr, fls_krl32txvxxkz, fls_drb114dtvlpt
//! @title: Tuple patterns in let statements and match arms
//! @lean(tier): supported

pub fn swap(pair: (u32, bool)) -> (bool, u32) {
    let (n, flag) = pair;
    (flag, n)
}

pub fn quadrant(p: (i32, i32)) -> u8 {
    match p {
        (0, 0) => 0,
        (x, y) if x > 0 && y > 0 => 1,
        (_, y) if y > 0 => 2,
        (.., _) => 3,
    }
}

#[hax_test::equiv]
pub fn tuple_fields_by_position() -> bool {
    swap((3, true)) == (true, 3)
        && quadrant((0, 0)) == 0
        && quadrant((1, 1)) == 1
        && quadrant((-1, 1)) == 2
        && quadrant((1, -1)) == 3
}
