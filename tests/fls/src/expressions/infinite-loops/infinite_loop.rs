//! @fls: fls_99imks9hj3kp, fls_b314wjbv0zwe, fls_lp15ilkul2uv, fls_p11qw6mtxlda, fls_pg3r6nyl865, fls_q3qpcf2fz7h, fls_w4tj5gofwih1, fls_wf11yp1jwf53, fls_BjZjuiFnPtFd, fls_aw6qczl4zpko, fls_eg93m93gvwal, fls_phpoq9ho8f1v, fls_y1d8kd1bdlmx, fls_tx5u743391h7
//! @title: Loop expressions terminated by break, with and without a value
//! @lean(tier): supported

pub fn first_square_above(limit: u32) -> u32 {
    let mut i = 0;
    loop {
        i += 1;
        if i * i > limit {
            break i * i;
        }
    }
}

pub fn count_down(mut n: u32) -> u32 {
    let mut steps = 0;
    'counting: loop {
        if n == 0 {
            break 'counting;
        }
        n -= 1;
        steps += 1;
    }
    steps
}

#[hax_test::equiv]
pub fn loop_results() -> bool {
    first_square_above(10) == 16 && count_down(4) == 4
}
