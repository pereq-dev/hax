//! @fls: fls_kvqzmt7my5dh, fls_pe5kh8y8u664, fls_mrjhpiq5refe, fls_2dmeukyjqz9y, fls_2xq8852gihn9, fls_jajvvwoy3399, fls_1r0vm6rg13o9
//! @title: A parenthesized pattern inside a reference pattern
//! @lean(tier): supported

pub fn small_or_large(x: &u32) -> u8 {
    match x {
        &(0..=9) => 0,
        &(10 | 20) => 1,
        _ => 2,
    }
}

#[hax_test::equiv]
pub fn parentheses_group_the_subpattern() -> bool {
    small_or_large(&4) == 0 && small_or_large(&20) == 1 && small_or_large(&15) == 2
}
