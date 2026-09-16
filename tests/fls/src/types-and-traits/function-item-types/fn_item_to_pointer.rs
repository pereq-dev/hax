//! @fls: fls_e9x4f7qxvvjv, fls_t24iojx7yc23
//! @title: A function item coerced to a function pointer
//! @lean(tier): supported
//! @lean(reason): aeneas rejects the function pointer type (`Arrow types are not supported yet`) and the coercion to it (`Invalid input for unop: cast`); the functions section of the input-language reference.

pub fn increment(x: u32) -> u32 {
    x + 1
}

pub fn apply(f: fn(u32) -> u32, x: u32) -> u32 {
    f(x)
}

#[hax_test::equiv]
pub fn function_item_as_pointer() -> bool {
    let f: fn(u32) -> u32 = increment;
    apply(f, 1) == 2 && apply(increment, 2) == 3
}
