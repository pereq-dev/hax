//! @fls: fls_v2wrytr3t04h, fls_g1iYVw7upBnH, fls_8gpvNJfVlyaD, fls_KcI6yK0P8Onn
//! @title: Calling function pointers with and without a return type
//! @lean(tier): supported
//! @lean(reason): aeneas rejects a function pointer type with a reference parameter (`We don't support arrow types with locally quantified regions`); the functions section of the input-language reference.

pub fn negate(x: i32) -> i32 {
    -x
}

pub fn reset(x: &mut i32) {
    *x = 0;
}

pub fn run(f: fn(i32) -> i32, g: fn(&mut i32), x: i32) -> i32 {
    let mut y = f(x);
    let z = y;
    g(&mut y);
    z + y
}

#[hax_test::equiv]
pub fn pointers_called() -> bool {
    run(negate, reset, 4) == -4
}
