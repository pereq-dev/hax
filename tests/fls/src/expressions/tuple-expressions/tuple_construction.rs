//! @fls: fls_581y6jq1eyn8, fls_87rp1hfwvjel, fls_k2aznqo9j49p, fls_ljz3sxmfzflm, fls_waf55yd3mpsq
//! @title: Tuple expressions with initializers evaluated left to right
//! @lean(tier): supported

pub fn next(counter: &mut u32) -> u32 {
    *counter += 1;
    *counter
}

pub fn ordered() -> (u32, u32) {
    let mut c = 0;
    (next(&mut c), next(&mut c))
}

#[hax_test::equiv]
pub fn tuple_values() -> bool {
    let t = (1u8, true);
    ordered() == (1, 2) && t.0 == 1 && t.1
}
