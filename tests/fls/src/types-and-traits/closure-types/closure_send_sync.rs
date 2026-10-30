//! @fls: fls_2nuhy0ujgq18, fls_5jh07heok8sy
//! @title: A closure whose captures are Send and Sync satisfies Send and Sync bounds
//! @lean(tier): supported

pub fn apply_shared<F: Fn(u32) -> u32 + Send + Sync>(f: F, x: u32) -> u32 {
    f(x)
}

#[hax_test::equiv]
pub fn closure_is_send_and_sync() -> bool {
    let offset = 3;
    apply_shared(|x| x + offset, 4) == 7
}
