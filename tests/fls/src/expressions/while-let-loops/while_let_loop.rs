//! @fls: fls_bC60ZSC9yUOI, fls_fmdlyp9r9zl7, fls_gTfSLePwHTES, fls_pTq4LIGIoAtN, fls_pacf1uavh1qt, fls_z2ht5iaat5ag
//! @title: While let loops consuming an Option-producing state
//! @lean(tier): supported

pub fn next(state: &mut u32) -> Option<u32> {
    if *state == 0 {
        None
    } else {
        *state -= 1;
        Some(*state)
    }
}

pub fn drain(mut state: u32) -> u32 {
    let mut total = 0;
    while let Some(v) = next(&mut state) {
        total += v;
    }
    total
}

#[hax_test::equiv]
pub fn loop_until_none() -> bool {
    drain(4) == 6 && drain(0) == 0
}
