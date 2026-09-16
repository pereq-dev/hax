//! @fls: fls_ei4pbeksd1v8, fls_hs1rr54hu18w, fls_Uv7QsP2nKcDx, fls_Dp8aJr2LqYwS, fls_Wm4QvN7xTcRb, fls_Rt5XcG8nMbPw, fls_Ks9FdL3pVxQa, fls_sbtx1l6n2tp2, fls_Xd4LqN9vTrPm, fls_Hw6CkR3mVzQn, fls_gfD9XwVj5hab, fls_Gc5RpT8nYvQm, fls_Nw2KxL7qVmRs, fls_Rh9TsD4vQpLm, fls_g6xyz0beps3o, fls_Mv2YpG7sLcHw, fls_Bx8KsV4nQdRt
//! @title: Match arm guards filtering matched arms
//! @lean(tier): supported

pub fn classify(x: Option<u32>) -> u32 {
    match x {
        Some(0) => 0,
        Some(n) if n % 2 == 0 => 2,
        Some(n) if n % 3 == 0 => 3,
        Some(_) => 1,
        None => 9,
    }
}

#[hax_test::equiv]
pub fn guarded_arms() -> bool {
    classify(Some(0)) == 0
        && classify(Some(4)) == 2
        && classify(Some(9)) == 3
        && classify(Some(7)) == 1
        && classify(None) == 9
}
