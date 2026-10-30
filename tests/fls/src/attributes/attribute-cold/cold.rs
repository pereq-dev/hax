//! @fls: fls_8zdexi5lgm2f, fls_wle815gb9ai2
//! @title: The cold attribute on a rarely called function
//! @lean(tier): supported

#[cold]
pub fn fallback() -> u32 {
    0
}

pub fn pick(x: Option<u32>) -> u32 {
    match x {
        Some(v) => v,
        None => fallback(),
    }
}

#[hax_test::equiv]
pub fn cold_path_value() -> bool {
    pick(None) == 0 && pick(Some(3)) == 3
}
