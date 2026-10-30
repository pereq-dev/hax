//! @fls: fls_snpxxcqhtjfv, fls_6v06zvi1ctub
//! @title: Built-in macros invoked in expression and statement position
//! @lean(tier): supported

pub fn classify(x: Option<u32>) -> u32 {
    let is_small = matches!(x, Some(0..=9));
    let never: bool = cfg!(any());
    debug_assert_eq!(is_small, matches!(x, Some(v) if v < 10));
    if never {
        100
    } else if is_small {
        1
    } else {
        0
    }
}

#[hax_test::equiv]
pub fn builtin_expansions() -> bool {
    classify(Some(4)) == 1 && classify(Some(40)) == 0 && classify(None) == 0
}
