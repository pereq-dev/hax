//! @fls: fls_vsk4vhnuiyyz, fls_ckeitwiv326r, fls_ebangxc36t74, fls_gwu4cn4ziabe, fls_ko1x0gp9e7y3
//! @title: Error handling with Result and Option values
//! @lean(tier): supported

pub fn checked_div(a: u32, b: u32) -> Result<u32, u8> {
    if b == 0 { Err(1) } else { Ok(a / b) }
}

pub fn first(xs: &[u32]) -> Option<u32> {
    if xs.is_empty() { None } else { Some(xs[0]) }
}

#[hax_test::equiv]
pub fn tiered_error_handling() -> bool {
    let ok = match checked_div(6, 3) {
        Ok(v) => v,
        Err(_) => 0,
    };
    let err = checked_div(1, 0).is_err();
    ok == 2 && err && first(&[4]) == Some(4) && first(&[]) == None
}
