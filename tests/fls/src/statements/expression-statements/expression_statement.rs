//! @fls: fls_xmdj8uj7ixoe, fls_gzzmudc1hl6s, fls_kc99n8qrszxh, fls_r8poocwqaglf, fls_88e6s3erk8tj, fls_2p9xnt519nbw, fls_4q90jb39apwr, fls_xqtztcu8ibwq
//! @title: Expression statements with and without a semicolon
//! @lean(tier): supported

pub fn bump(counter: &mut u32) -> u32 {
    *counter += 1;
    *counter
}

pub fn run() -> u32 {
    let mut c = 0;
    bump(&mut c);
    if c > 0 {
        bump(&mut c);
    }
    [1u32, 2][0];
    {
        bump(&mut c);
    }
    c
}

#[hax_test::equiv]
pub fn results_are_ignored() -> bool {
    run() == 3
}
