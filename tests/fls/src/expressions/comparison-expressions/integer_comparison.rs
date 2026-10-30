//! @fls: fls_1udbc4aom6ok, fls_2ggb7a7nhrk9, fls_4vbrc31r0o60, fls_6cb4wg59wmef, fls_7n5gol6a8lod, fls_7pfsqby2saag, fls_8luq5sellcaq, fls_96mt7gx5ogo0, fls_9s4re3ujnfis, fls_asfrqemqviad, fls_c93pacid548a, fls_d62qfloqk2ub, fls_dkbjn7noq8n2, fls_ebvyhqbb921g, fls_gqy6uuowij9e, fls_htrjqxiv3avh, fls_hyy974ksbbrq, fls_irlqykpbtvd, fls_kdga59xx4nx3, fls_kezynx2xc1q7, fls_kodwkh58hmdv, fls_mab6yirx77zl, fls_or0i2cqxwl8o, fls_pso38dowbk91, fls_rfomib80bnn2, fls_ruyho6cu7rxg, fls_s6sq6p8th5nt, fls_udnhkbxpk83m, fls_udonl4c7f6pz, fls_ukm97arfzsk1, fls_w71j7i3n1kit, fls_wapl0ir7uvbp, fls_wrftg7onlkmm, fls_wytygse41vzm, fls_xmtxkit3qpw7, fls_yd4qqi39w248, fls_ydt9zvh0h5ex, fls_yxwe1o27u6ns, fls_yzuceqx6nxwa
//! @title: The six comparison operators on integers
//! @lean(tier): supported

pub fn compare(a: i32, b: i32) -> [bool; 6] {
    [a == b, a != b, a < b, a <= b, a > b, a >= b]
}

#[hax_test::equiv]
pub fn comparison_results() -> bool {
    compare(1, 2) == [false, true, true, true, false, false]
        && compare(2, 2) == [true, false, false, true, false, true]
        && compare(3, 2) == [false, true, false, false, true, true]
}
