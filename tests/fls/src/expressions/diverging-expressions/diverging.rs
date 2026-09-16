//! @fls: fls_4wDpAHhnWZAB, fls_7HA5UThwjbQj, fls_cmBVodJMjZi7, fls_oth9vFtcb9l4, fls_pdDr8Lk1GQ0T, fls_xqxdHziqgWf5, fls_xsOgdiIzysP1
//! @title: Return, break, and panic as diverging expressions in value position
//! @lean(tier): supported

pub fn first_even(xs: &[u32]) -> u32 {
    let mut i = 0;
    let found = loop {
        if i >= xs.len() {
            return 0;
        }
        let x = xs[i];
        if x % 2 == 0 {
            break x;
        }
        i += 1;
    };
    found
}

pub fn checked_div(a: u32, b: u32) -> u32 {
    let d: u32 = if b == 0 {
        panic!("division by zero")
    } else {
        b
    };
    a / d
}

#[hax_test::equiv]
pub fn diverging_arms_are_unified() -> bool {
    first_even(&[1, 3, 4]) == 4 && first_even(&[1, 3]) == 0 && checked_div(9, 3) == 3
}
