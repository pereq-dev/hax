//! @fls: fls_fhahcc1mz2qh, fls_fedo8zhgpla5, fls_d1kc73hpncpo, fls_30u9ij164ww3, fls_x0bmzl1315gq, fls_mpeuhov0umfa, fls_ysfgdzaiww8z, fls_7rxnxd4ybxbt, fls_l2nwz166curc, fls_gwc08xayno7q
//! @title: Reference patterns dereferencing shared and mutable references
//! @lean(tier): supported

pub fn is_zero(x: &u32) -> bool {
    match x {
        &0 => true,
        &_ => false,
    }
}

pub fn copy_out(x: &(u32, u32)) -> u32 {
    let &(a, b) = x;
    a + b
}

pub fn bump(x: &mut u32) -> u32 {
    match x {
        &mut 0 => 1,
        &mut n => n + 1,
    }
}

#[hax_test::equiv]
pub fn dereferenced_matches() -> bool {
    let mut m = 4;
    is_zero(&0) && !is_zero(&3) && copy_out(&(1, 2)) == 3 && bump(&mut m) == 5 && bump(&mut 0) == 1
}
