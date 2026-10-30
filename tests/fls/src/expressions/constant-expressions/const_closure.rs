//! @fls: fls_8nyu6phm1nji
//! @title: A closure without captures in a constant expression
//! @lean(tier): supported
//! @lean(reason): aeneas rejects the function pointer type of the constant (`Arrow types are not supported yet`) and the coercion of the closure to it (`Invalid input for unop: cast`); the functions section of the input-language reference.

pub const ADD_ONE: fn(u32) -> u32 = |x| x + 1;

#[hax_test::equiv]
pub fn closure_constant_evaluates() -> bool {
    ADD_ONE(1) == 2
}
