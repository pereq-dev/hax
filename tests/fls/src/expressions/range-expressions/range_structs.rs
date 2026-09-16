//! @fls: fls_5a1uivj19kob, fls_bi82rusji8g0, fls_ehseim1p479z, fls_exa2ufugnpgc, fls_f648uuxxh4vk, fls_jqy0p155btca, fls_k611yoc8hk0n, fls_ke2fpgodq84u, fls_m0slikrulnvd, fls_m6n0gvg3ct1b, fls_msyv4oyk5zp9, fls_ppustuqdji7b, fls_x67xo25n0qlz, fls_xaumwogwbv3g, fls_yvh5cdgzevni, fls_zb6jk6qykun6
//! @title: Range expressions constructing Range, RangeFrom, RangeTo, and RangeFull
//! @lean(tier): supported

use core::ops::{Range, RangeFrom, RangeFull, RangeTo};

pub fn bounds(lo: u32, hi: u32) -> [u32; 4] {
    let r: Range<u32> = lo..hi;
    let from: RangeFrom<u32> = lo..;
    let to: RangeTo<u32> = ..hi;
    [r.start, r.end, from.start, to.end]
}

pub fn full() -> RangeFull {
    ..
}

#[hax_test::equiv]
pub fn range_values() -> bool {
    let RangeFull = full();
    bounds(2, 5) == [2, 5, 2, 5]
}
