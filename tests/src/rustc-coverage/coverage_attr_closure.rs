//! @fail(extraction): proverif(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001)
//! @fail(tc): legacy-lean(1)
#![feature(coverage_attribute, stmt_expr_attributes)]
#![allow(dead_code)]

/// @fail(extraction): ssprove(HAX0001)
static GLOBAL_CLOSURE_ON: fn(&str) = #[coverage(on)]
|input: &str| {
    println!("{input}");
};
/// @fail(extraction): ssprove(HAX0001)
static GLOBAL_CLOSURE_OFF: fn(&str) = #[coverage(off)]
|input: &str| {
    println!("{input}");
};

/// @fail(extraction): ssprove(HAX0001)
#[coverage(on)]
fn contains_closures_on() {
    let _local_closure_on = #[coverage(on)]
    |input: &str| {
        println!("{input}");
    };
    let _local_closure_off = #[coverage(off)]
    |input: &str| {
        println!("{input}");
    };
}

/// @fail(extraction): ssprove(HAX0001)
#[coverage(off)]
fn contains_closures_off() {
    let _local_closure_on = #[coverage(on)]
    |input: &str| {
        println!("{input}");
    };
    let _local_closure_off = #[coverage(off)]
    |input: &str| {
        println!("{input}");
    };
}

#[coverage(off)]
fn main() {
    contains_closures_on();
    contains_closures_off();
}
