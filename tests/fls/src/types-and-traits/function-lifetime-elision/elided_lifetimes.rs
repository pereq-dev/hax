//! @fls: fls_lAdIRCFFlydD, fls_qtjc7334wzhj, fls_vcmmkp9uruhr, fls_9wtuclhm7yz5, fls_JmP6O9zj8fkV, fls_YmUQ8ZiQuycp, fls_5ZAQ9p7jQuc2, fls_sIMN6Sd8xUZz, fls_dIyisjNIx9dC
//! @title: Function signatures with implicitly and explicitly elided lifetimes
//! @lean(tier): supported

pub struct Wrapper<'a>(pub &'a u32);

pub fn implicit(x: &u32) -> &u32 {
    x
}

pub fn explicit(w: Wrapper<'_>) -> &'_ u32 {
    w.0
}

impl Wrapper<'_> {
    pub fn get(&self) -> &u32 {
        self.0
    }
}

#[hax_test::equiv]
pub fn elision_is_transparent() -> bool {
    let x = 6;
    let w = Wrapper(&x);
    *implicit(&x) == 6 && *w.get() == 6 && *explicit(w) == 6
}
