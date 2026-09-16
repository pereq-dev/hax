//! @fls: fls_t24iojx7yc23, fls_CSuxTkwR96j9, fls_1941wid94hlg
//! @title: A function item passed to a generic Fn bound and copied
//! @lean(tier): supported
//! @lean(reason): The `Copy` bound on the function item type is instantiated with aeneas' builtin `Std.core.marker.Copy` where the generic expects the core model's `core.marker.Copy`; the extraction fails to elaborate (`Application type mismatch`); the functions section of the input-language reference.

pub fn double(x: u32) -> u32 {
    x * 2
}

pub fn apply_twice<F: Fn(u32) -> u32 + Copy>(f: F, x: u32) -> u32 {
    let g = f;
    g(f(x))
}

#[hax_test::equiv]
pub fn function_item_through_bound() -> bool {
    apply_twice(double, 3) == 12
}
