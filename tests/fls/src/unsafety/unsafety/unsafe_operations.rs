//! @fls: fls_8kqo952gjhaf, fls_ovn9czwnwxue, fls_pfhmcafsjyf7, fls_jd1inwz7ulyw, fls_ljocmnaz2m49, fls_jb6krd90tjmc
//! @title: Unsafe operations inside an unsafe block and an unsafe function
//! @lean(tier): unsupported
//! @lean(reason): The value-model boundary of the input-language reference: unsafe code steps outside the guarantees that make the functional translation sound.

pub unsafe fn read_raw(p: *const u32) -> u32 {
    unsafe { *p }
}

pub fn read(x: &u32) -> u32 {
    let p: *const u32 = x;
    unsafe { read_raw(p) }
}

#[hax_test::equiv]
pub fn value_through_unsafe_operations() -> bool {
    read(&7) == 7
}
