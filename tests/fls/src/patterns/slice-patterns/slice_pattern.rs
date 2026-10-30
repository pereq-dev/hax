//! @fls: fls_qqiu594hki8g, fls_ndor56nou676, fls_h6x9xlxi7y5n, fls_jbmxu7y5fnm6, fls_9yuobz1jsehf, fls_hzyv4ofu0ny, fls_kwQyiSoyAwZ8, fls_twhwiy213ibf, fls_ei7y4ul6n6hu, fls_ad2jud5h1rfp, fls_pc97m47p34wq, fls_19iygu12s315
//! @title: Slice patterns over arrays
//! @lean(tier): supported

pub fn sum_of_three(a: [u32; 3]) -> u32 {
    let [x, y, z] = a;
    x + y + z
}

pub fn swap_ends(a: [u32; 3]) -> [u32; 3] {
    match a {
        [x, y, z] => [z, y, x],
    }
}

#[hax_test::equiv]
pub fn array_matches() -> bool {
    sum_of_three([1, 2, 3]) == 6 && swap_ends([1, 2, 3]) == [3, 2, 1]
}
