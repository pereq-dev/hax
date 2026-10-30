//! @fls: fls_3jeootwe6ucu, fls_bsykgnbatpmi
//! @title: A non-capturing closure coerced to a function pointer
//! @lean(tier): supported
//! @lean(reason): aeneas rejects the function pointer type (`Arrow types are not supported yet`) and the coercion to it (`Invalid input for unop: cast`); the functions section of the input-language reference.

pub fn apply(f: fn(u32) -> u32, x: u32) -> u32 {
    f(x)
}

#[hax_test::equiv]
pub fn closure_as_fn_pointer() -> bool {
    apply(|x| x * 3, 4) == 12
}
