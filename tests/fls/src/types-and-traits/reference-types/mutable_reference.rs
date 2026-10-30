//! @fls: fls_GUZuiST7ucib, fls_vaas9kns4zo6, fls_n6ffcms5pr0r
//! @title: Mutation through a mutable reference
//! @lean(tier): supported

pub fn add_to(target: &mut u32, n: u32) {
    *target += n;
}

#[hax_test::equiv]
pub fn mutation_is_visible_to_the_owner() -> bool {
    let mut x = 1;
    add_to(&mut x, 5);
    add_to(&mut x, 5);
    x == 11
}
