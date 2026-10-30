//! @fls: fls_r66jhict6rlq, fls_rzw12sagm585, fls_6utorag4adlv, fls_elsfqsiqor1y
//! @title: Conditionally applied attributes with cfg_attr
//! @lean(tier): supported

#[cfg_attr(all(), allow(dead_code), inline)]
pub fn configured(x: u32) -> u32 {
    x + 1
}

#[cfg_attr(any(), deprecated)]
pub fn not_configured(x: u32) -> u32 {
    x + 2
}

#[hax_test::equiv]
pub fn attributed_functions_compute() -> bool {
    configured(1) == 2 && not_configured(1) == 3
}
