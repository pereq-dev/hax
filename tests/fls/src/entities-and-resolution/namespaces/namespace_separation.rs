//! @fls: fls_1d4jm61qnt4l, fls_avsua7bho205, fls_ckptn88o6lla, fls_u1533bngb0yv, fls_8o3izim4zf8t, fls_ufp3btk8pet5, fls_aivi97hhfxy2, fls_ttr6v8ca4av0, fls_qmf7lk6h96sv
//! @title: The same name in the type and value namespaces
//! @lean(tier): supported
//! @lean(reason): aeneas maps the type namespace and the value namespace to one Lean namespace, so a function and a struct of the same name clash (`Name clash detected: the following identifiers are bound to the same name "Point"`); the Rust subset of the input-language reference.

#[allow(non_snake_case)]
pub fn Point(x: u32) -> u32 {
    x + 1
}

pub struct Point {
    pub x: u32,
}

pub enum Shape {
    Dot(u32),
}

pub const LIMIT: u32 = 3;

#[hax_test::equiv]
pub fn names_do_not_clash() -> bool {
    let p = Point { x: 1 };
    let Shape::Dot(d) = Shape::Dot(2);
    let limit = LIMIT * 3;
    Point(p.x) + d + limit == 13
}
