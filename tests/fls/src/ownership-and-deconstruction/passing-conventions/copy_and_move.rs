//! @fls: fls_fvwx2ufeyzcs, fls_h2pgsij1rbms, fls_6ul3f6v0foma, fls_yx2knbby70fy, fls_3ztdz02efeoc, fls_vveEJn7lngT8, fls_cfqzpmnfeh8h, fls_go9gdlk5d3km, fls_459xx6febmf0, fls_3xyq50abdiv6, fls_7kcx3u8gvl1d, fls_i4hrifsb9msr, fls_7tadh1zel0fc, fls_ywt328hcieka, fls_konzgoybhfqm, fls_vk1xhvdaakh0, fls_3xk3p1unbjy5, fls_4bnbv7mqod57
//! @title: Values passed by copy and by move
//! @lean(tier): supported

pub struct Owned(pub u32);

pub struct Pair {
    pub left: Owned,
    pub right: Owned,
}

pub fn take(o: Owned) -> u32 {
    o.0
}

pub fn pass() -> u32 {
    let x: i32 = 42;
    let y = x;
    let copied = x + y;
    let owned = Owned(1);
    let moved = owned;
    let pair = Pair {
        left: Owned(10),
        right: Owned(20),
    };
    let left = pair.left;
    let temp = take(Owned(100));
    copied as u32 + take(moved) + left.0 + pair.right.0 + temp
}

#[hax_test::equiv]
pub fn passed_values() -> bool {
    pass() == 215
}
