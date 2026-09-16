//! @fls: fls_h5994su1yft3, fls_v8atmrwz6wzk, fls_7nd5tixyqir8, fls_w2dzqq54fjhb, fls_ufmd38hi9t9y, fls_5allcjkjnon2, fls_3bibysz95ktn, fls_fxq19dqtmifj, fls_drhpcwoblcux, fls_v86qrsqcs3nd, fls_dd49lb2k3erc, fls_t6ef5x4x5poi, fls_kqtgjgn1hqrj, fls_ws15ilzf8n6z, fls_ni4mgq3mouek, fls_6c9ax4qsr1gy, fls_sqcgvpr4egtx, fls_9ys0itbp4okd, fls_b46gbyid15zx, fls_f8ag276ecbze, fls_twwjcrcfirdi, fls_wovu7330vdrq, fls_7xopdco6iy74, fls_nb5cb6en2p5w, fls_gd28wfcfs2pv, fls_67a7p57nzbul, fls_cq0qunw51m94, fls_o1e4tnh7v3db, fls_6vnv3ygisjr, fls_s6m9abmmtc9i, fls_s19vu65z96y5, fls_4x27kfiodb8, fls_4rhjpdu4zfqj
//! @title: The logical operations and equality on bool
//! @lean(tier): supported

pub fn not(a: bool) -> bool {
    !a
}

pub fn and(a: bool, b: bool) -> bool {
    a & b
}

pub fn or(a: bool, b: bool) -> bool {
    a | b
}

pub fn xor(a: bool, b: bool) -> bool {
    a ^ b
}

pub fn eq(a: bool, b: bool) -> bool {
    a == b
}

#[hax_test::equiv]
pub fn not_table() -> bool {
    !not(true) && not(false)
}

#[hax_test::equiv]
pub fn and_table() -> bool {
    and(true, true) && !and(true, false) && !and(false, true) && !and(false, false)
}

#[hax_test::equiv]
pub fn or_table() -> bool {
    or(true, true) && or(true, false) && or(false, true) && !or(false, false)
}

#[hax_test::equiv]
pub fn xor_table() -> bool {
    !xor(true, true) && xor(true, false) && xor(false, true) && !xor(false, false)
}

#[hax_test::equiv]
pub fn eq_table() -> bool {
    eq(true, true) && !eq(true, false) && !eq(false, true) && eq(false, false)
}

#[hax_test::equiv]
pub fn ne_is_not_eq() -> bool {
    (true != false) == !eq(true, false) && (true != true) == !eq(true, true)
}
