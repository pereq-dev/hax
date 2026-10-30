//! @fls: fls_bsykgnbatpmi, fls_zfj4l8bigdg0, fls_bn0ueivujnqk, fls_u01kt5glbuz8
//! @title: Closures called through Fn, FnMut, and FnOnce bounds
//! @lean(tier): supported
//! @lean(reason): The calls through the `FnOnce` and `FnMut` bounds extract with a closure state of the wrong arity (`Type mismatch: call_once has type closure → Unit → RustM (U32 × closure)`), so the extraction fails to elaborate; the functions section of the input-language reference.

pub fn call_fn<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
    f(x) + f(x)
}

pub fn call_fn_mut<F: FnMut() -> u32>(mut f: F) -> u32 {
    f() + f()
}

pub fn call_fn_once<F: FnOnce() -> u32>(f: F) -> u32 {
    f()
}

#[hax_test::equiv]
pub fn fn_closure_captures_by_reference() -> bool {
    let k = 3;
    call_fn(|x| x + k, 4) == 14
}

#[hax_test::equiv]
pub fn fn_mut_closure_mutates_capture() -> bool {
    let mut counter = 0;
    let total = call_fn_mut(|| {
        counter += 1;
        counter
    });
    total == 3 && counter == 2
}

#[hax_test::equiv]
pub fn fn_once_closure_moves_capture() -> bool {
    let v = 5u32;
    call_fn_once(move || v) == 5
}
