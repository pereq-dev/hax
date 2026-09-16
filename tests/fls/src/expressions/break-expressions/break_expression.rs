//! @fls: fls_1wdybpfldj7q, fls_2nmadhe3ismj, fls_32fwis9pxh77, fls_54d5uydc87td, fls_56szfyilc06, fls_60imbzwg3e2x, fls_7frvr2nm2mcj, fls_8yore99adr22, fls_AurifMM8RpDp, fls_WFAW1PG1YXdM, fls_XpFLrL78QMe1, fls_bgd7d5q69q0g, fls_d7l1y2qbe8br, fls_dnnq1zym8ii0, fls_ghxns2nggffj, fls_i5ko1t2wbgxe, fls_jnpx8mx1oa7n, fls_l0c05wa9q97w, fls_l2kp8mw6bjj0, fls_yb8jv4mkmki0
//! @title: Break expressions with and without an operand in loops and named blocks
//! @lean(tier): supported

pub fn find(xs: &[u32], target: u32) -> Option<usize> {
    let mut i = 0;
    let found = loop {
        if i == xs.len() {
            break None;
        }
        if xs[i] == target {
            break Some(i);
        }
        i += 1;
    };
    found
}

pub fn until_ten(step: u32) -> u32 {
    let mut n = 0;
    while n < 100 {
        n += step;
        if n >= 10 {
            break;
        }
    }
    n
}

pub fn block_value(x: u32) -> u32 {
    'outer: {
        if x > 5 {
            break 'outer 1;
        }
        0
    }
}

#[hax_test::equiv]
pub fn break_results() -> bool {
    find(&[3, 5, 7], 5) == Some(1)
        && find(&[3], 9) == None
        && until_ten(4) == 12
        && block_value(9) == 1
        && block_value(1) == 0
}
