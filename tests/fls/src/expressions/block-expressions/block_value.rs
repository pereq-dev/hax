//! @fls: fls_13b5n127rj92, fls_1hzup0sf8l7l, fls_9nmssjseq3jt, fls_a3ulnvyc1ut, fls_elcl73psruxw, fls_kKZPKvJ902cw, fls_ltEygvWDtHXE, fls_nf65p0l0v0gr, fls_nzdpw59plr2g, fls_ob76y2ymdd27, fls_tn3hj7k2lliu, fls_u0avbm147nyh, fls_u4gj2lnkq9ub
//! @title: Block expressions with statements, a tail expression, and the unit value
//! @lean(tier): supported

pub fn with_tail(x: u32) -> u32 {
    let y = {
        let a = x + 1;
        let b = a * 2;
        b - 1
    };
    y
}

pub fn without_tail(x: &mut u32) -> () {
    let unit: () = {
        *x += 1;
    };
    unit
}

#[hax_test::equiv]
pub fn block_values() -> bool {
    let mut n = 1;
    without_tail(&mut n);
    with_tail(3) == 7 && n == 2
}
