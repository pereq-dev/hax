//! @fls: fls_g1azfj548136
//! @title: Tuple struct definition, construction, and field access
//! @lean(tier): supported

pub struct Point(pub i32, pub i32);

#[hax_test::equiv]
pub fn construct_and_project() -> bool {
    let p = Point(1, 2);
    p.0 + p.1 == 3
}
