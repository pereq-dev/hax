//! @fls: fls_udn9lyf3m0z6, fls_yvkdcs4pmxjf, fls_n570za6a9nqd, fls_3h5vr7xk2rrt, fls_tms5r9f5ogcb, fls_bysv5r7iuf5j, fls_3BGncWvMumEt, fls_uf0sg25awre6, fls_o7cqfdnr253y, fls_ap85svxyuhvg
//! @title: Borrows of literal constant expressions promoted to 'static
//! @lean(tier): supported

pub fn defaults() -> &'static [u32] {
    &[1, 2, 3]
}

pub fn answer() -> &'static u32 {
    &42
}

#[hax_test::equiv]
pub fn promoted_values() -> bool {
    defaults()[2] == 3 && *answer() == 42
}
