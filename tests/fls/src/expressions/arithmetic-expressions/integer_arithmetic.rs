//! @fls: fls_2ude3wrxji2p, fls_3de9ulyzuoa, fls_43knkymqpj7t, fls_5nsa9zefz9cv, fls_5rdrkvspw57z, fls_62gpbubfj30w, fls_9x2i1zlsm364, fls_Et5gp1I7VqBX, fls_FxLnXeGT2n9u, fls_Kdr6fLrRj0Du, fls_NcLf4o1dpniS, fls_Q9dhNiICGIfr, fls_Vy0DyZqfy7Iv, fls_aalxhbvu8kdi, fls_ad9tc6ki8vcq, fls_albbLSTYtmyq, fls_asibqpe3z95h, fls_b94ojbfukhvd, fls_bndpd66973ev, fls_bveocgaagk1n, fls_dstca76y08ge, fls_g28igfbnwfe0, fls_gld1u9fnsj6d, fls_ittf4yggk7do, fls_izmfimd4yg27, fls_jjmc1xgny77, fls_kN0HnldvDXSg, fls_kf41bphvlse3, fls_kpbxcdaflb06, fls_kr8Opj3c7uvb, fls_lr2a21v5en59, fls_ryzhdpxgm7ii, fls_thcumw8n8xbw, fls_thyq4h55mx55, fls_u3jwnrqun5kl, fls_u3pstd6xe43y, fls_v8vekngd27sz, fls_vk17mfv47wk9, fls_ylqm6wucq2sw, fls_zLroZh43MOtN
//! @title: Addition, subtraction, multiplication, division, and remainder on integers
//! @lean(tier): supported

pub fn ops(a: u32, b: u32) -> [u32; 5] {
    [a + b, a - b, a * b, a / b, a % b]
}

pub fn signed_div(a: i32, b: i32) -> [i32; 2] {
    [a / b, a % b]
}

#[hax_test::equiv]
pub fn unsigned_results() -> bool {
    ops(17, 5) == [22, 12, 85, 3, 2]
}

#[hax_test::equiv]
pub fn signed_results_truncate_towards_zero() -> bool {
    signed_div(-7, 2) == [-3, -1] && signed_div(7, -2) == [-3, 1]
}
