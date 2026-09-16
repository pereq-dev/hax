//! @fls: fls_63jqtyw0rz8c, fls_3c4g9njja5s5, fls_bsykgnbatpmi
//! @title: A closure whose captures are Copy is Copy and Clone
//! @lean(tier): supported
//! @lean(reason): The `Copy` bound on the closure type is instantiated with aeneas' builtin `Std.core.marker.Copy` where the generic expects the core model's `core.marker.Copy`; the extraction fails to elaborate (`Application type mismatch`); the functions section of the input-language reference.

pub fn twice<F: Fn(u32) -> u32 + Copy>(f: F) -> u32 {
    let g = f;
    let h = f.clone();
    g(1) + h(2)
}

#[hax_test::equiv]
pub fn copied_closure_calls() -> bool {
    let k = 10;
    twice(|x| x + k) == 23
}
