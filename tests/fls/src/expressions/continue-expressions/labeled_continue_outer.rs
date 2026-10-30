//! @fls: fls_767gv7zhqamh, fls_ckm6i9c3s6j8, fls_gm74eo754rq9
//! @title: A labeled continue restarting an outer loop
//! @lean(tier): supported
//! @lean(reason): aeneas rejects a `continue` to an outer loop (`Continue to outer loops are not supported yet`); the loops section of the input-language reference.

pub fn count_pairs_without_equal(n: u32) -> u32 {
    let mut count = 0;
    let mut i = 0;
    'game_loop: while i < n {
        i += 1;
        let mut j = 0;
        while j < n {
            j += 1;
            if i == j {
                continue 'game_loop;
            }
            count += 1;
        }
    }
    count
}

#[hax_test::equiv]
pub fn outer_restart() -> bool {
    count_pairs_without_equal(3) == 3
}
