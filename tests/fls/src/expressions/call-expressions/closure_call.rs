//! @fls: fls_fvgfx17ossd9, fls_4t6imtiw6kzt, fls_9bbewx1l7h5h, fls_cu2ubdm3tfwb, fls_s3q3sej1hgho, fls_ixebnlcccmit, fls_9lt4wh9ql5ae, fls_nb0eqky2akzt, fls_7p6zrjbpj0kl
//! @title: Call expressions invoking closures through Fn, FnMut, and FnOnce
//! @lean(tier): supported

pub fn run() -> u32 {
    let k = 2;
    let by_ref = |x: u32| x * k;
    let mut total = 0;
    let mut by_mut = |x: u32| total += x;
    let owned = [1u32, 2, 3];
    let by_once = move || owned[2];
    by_mut(by_ref(3));
    by_mut(1);
    total + by_once()
}

#[hax_test::equiv]
pub fn closure_calls() -> bool {
    run() == 10
}
