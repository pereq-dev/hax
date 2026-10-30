//! @fls: fls_ks6y1syab2bp, fls_ryfcrqrkp28y, fls_ehf9r6halgh1, fls_gwuc2xffosu, fls_w369n8lmwr7g, fls_4is6h95jj3gd, fls_5lo1hs8wzz0t, fls_qgilaqy5zx7q, fls_2u99arsbnlnk, fls_dexg9g9cct30, fls_boc7juqj69hw, fls_4dr1stiw82v9, fls_h14emtt6iyk3, fls_qhdofvbso3gl
//! @title: Tuple struct patterns on tuple structs and tuple enum variants
//! @lean(tier): supported

pub struct Triple(pub u32, pub u32, pub u32);

pub enum Msg {
    Move(i32, i32),
    Quit,
}

pub fn outer_sum(t: Triple) -> u32 {
    let Triple(a, .., c) = t;
    a + c
}

pub fn dx(m: Msg) -> i32 {
    match m {
        Msg::Move(x, _) => x,
        Msg::Quit => 0,
    }
}

#[hax_test::equiv]
pub fn positional_fields() -> bool {
    outer_sum(Triple(1, 2, 3)) == 4 && dx(Msg::Move(-2, 5)) == -2 && dx(Msg::Quit) == 0
}
