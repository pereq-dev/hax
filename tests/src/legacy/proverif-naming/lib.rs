//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! Names the backend generates never capture or merge the user's.

fn g(x: u8) -> u8 {
    x
}

// The second binding of `x` must not be renamed to the user's `x_1`.
fn rebinding(a: u8, b: u8) -> u8 {
    let x = a;
    let x_1 = b;
    let x = g(x);
    g(x_1) + x
}

// Distinct string literals get distinct constants.
fn literals() -> (&'static str, &'static str, &'static str, &'static str) {
    ("a b", "a_x20b", "", "empty")
}

// The body of a `pv_extern` refers to the parameters under their printed
// names.
#[hax_lib::pv_extern]
fn external(_rng: u8, new: u8) -> u8 {
    new
}

// A struct update whose base is a call projects each field from one
// evaluation of that call.
struct Triple {
    a: u8,
    b: u8,
    c: u8,
}

fn make_triple(x: u8) -> Triple {
    Triple { a: x, b: x, c: x }
}

fn update(x: u8) -> Triple {
    Triple {
        a: 0,
        ..make_triple(x)
    }
}

// Without an `else`, an `if` is unit when its condition does not hold.
fn if_without_else(c: bool, x: u8) -> u8 {
    if c {
        g(x);
    }
    x
}

// `_x` and `u_x` render alike, but stay distinct locals: this returns `a`.
fn underscore_prefix(a: u8, b: u8) -> u8 {
    let u_x = a;
    let _x = b;
    u_x
}

// A local named like a literal constant does not capture the literal.
fn literal_name(nat_3: u8) -> u8 {
    nat_3 + 3
}

struct Point {
    x: u8,
    y: u8,
}

// User locals named like the binders phases generate are not captured.
fn generated_names(hax__struct_base: u8, p: Point, v: Vec<u8>) -> (Point, Vec<u8>) {
    let q = Point { x: hax__struct_base, ..p };
    let __map_acc = hax__struct_base;
    let w = v.iter().map(|y| *y + __map_acc).collect();
    (q, w)
}
