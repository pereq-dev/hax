//! @fls: fls_j1jc83erljo0, fls_23eapx3ckymf, fls_rnty1c8l5495, fls_w44hav7mw3ao, fls_dw1nq4r9ghhd, fls_oq4xn8guos8f, fls_ljavs0w61z3j, fls_cdaf8viwmdfe, fls_3jspk8obv7sd, fls_ikzjsq8heyk6, fls_snpxxcqhtjfv, fls_6v06zvi1ctub, fls_wushtmw9qt3y, fls_xscdaxvs4wx4, fls_nz5stwcc41gk, fls_76prdp6k1fga, fls_76u274l4kew8, fls_lakpily1zwfl, fls_1tftbd91yfpd, fls_338rmbazl67o, fls_8qxwwf4trnl, fls_8z1sgtvchhhw, fls_d9w3dn2yn7mo, fls_lrr7gg8tian, fls_nsh2vwx8oiw, fls_t89sw6az99z7
//! @title: A declarative macro with several rules invoked in expression, item, and type position
//! @lean(tier): supported

macro_rules! answer_to_life {
    () => {
        42u32
    };
    ($x:expr) => {
        $x + 42
    };
}

macro_rules! make_item {
    ($name:ident, $value:expr) => {
        pub fn $name() -> u32 {
            $value
        }
    };
}

macro_rules! pair_type {
    ($t:ty) => {
        ($t, $t)
    };
}

make_item!(forty_three, answer_to_life!(1));

pub fn combine() -> pair_type!(u32) {
    let a = answer_to_life!();
    let b: u32;
    b = forty_three();
    (a, b)
}

#[hax_test::equiv]
pub fn expanded_macros() -> bool {
    combine() == (42, 43)
}
