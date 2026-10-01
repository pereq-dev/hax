//! @fail(tc): proverif(2)
//! @fail(tc): fstar(2), legacy-lean(1)
//@ edition: 2021

#[macro_export]
macro_rules! macro_that_defines_a_function {
    (fn $name:ident () $body:tt) => {
        fn $name () -> () $body
    }
}

// Non-executable comment.
