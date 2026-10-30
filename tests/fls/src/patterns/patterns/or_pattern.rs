//! @fls: fls_imegtsi224ts, fls_6xx34zr069bj, fls_cma5t8waon0x, fls_8xzjb0yzftkd, fls_6bwTtGKb7ba7, fls_VQMmveZUfNTn, fls_mp6i4blzexnu, fls_JJ1fJa1SsaWh, fls_zv73CR8rplIa, fls_tlwr4u7bjhh5, fls_tZJgZDWVChJV, fls_wh201rmh6u6d, fls_vstdqifqipbh
//! @title: Or-patterns at the top level and nested in a match arm
//! @lean(tier): supported

pub fn classify(n: u32) -> u8 {
    match n {
        0 | 1 => 0,
        2 | 3 | 5 | 7 => 1,
        _ => 2,
    }
}

pub fn either_small(p: (u32, u32)) -> bool {
    match p {
        (0 | 1, _) | (_, 0 | 1) => true,
        _ => false,
    }
}

pub fn bound_in_both(x: Option<Result<u32, u32>>) -> u32 {
    match x {
        Some(Ok(v) | Err(v)) => v,
        None => 0,
    }
}

#[hax_test::equiv]
pub fn alternatives_match() -> bool {
    classify(1) == 0 && classify(5) == 1 && classify(4) == 2
}

#[hax_test::equiv]
pub fn nested_alternatives() -> bool {
    either_small((9, 1)) && either_small((0, 9)) && !either_small((4, 4))
}

#[hax_test::equiv]
pub fn same_binding_in_alternatives() -> bool {
    bound_in_both(Some(Ok(3))) == 3 && bound_in_both(Some(Err(4))) == 4 && bound_in_both(None) == 0
}
