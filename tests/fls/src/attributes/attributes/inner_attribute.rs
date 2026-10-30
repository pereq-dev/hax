//! @fls: fls_rnzxj1t0hehl, fls_9TMRVlQwAdTB, fls_8o6vmzbw1b1j, fls_yd0ehw5csaur
//! @title: An inner attribute on a module and an outer attribute on a function
//! @lean(tier): supported

pub mod quiet {
    #![allow(dead_code)]

    fn unused() {}

    pub fn used() -> u32 {
        7
    }
}

#[allow(clippy::needless_return)]
pub fn value() -> u32 {
    return quiet::used();
}

#[hax_test::equiv]
pub fn attributed_items_behave_as_plain() -> bool {
    value() == 7
}
