//! @fls: fls_uVTpA7gbLCYX, fls_2DZAP6JJjJ9h, fls_emcNJzl2tHSA, fls_hBFlaUrrhqZ6, fls_5RcnETrW6f9m, fls_jMeWhn4sNTPF, fls_HpWSAfaTA1Dz
//! @title: Structural equality of scalars, tuples, arrays, references, and derived types
//! @lean(tier): supported

#[derive(PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[hax_test::equiv]
pub fn scalars_and_compounds() -> bool {
    let a = [1u8, 2, 3];
    let b = [1u8, 2, 3];
    true == true && 'x' == 'x' && 3u16 == 3 && (1, false) == (1, false) && a == b
}

#[hax_test::equiv]
pub fn references_and_slices() -> bool {
    let a = [1u8, 2, 3];
    let b = [1u8, 2, 3];
    &a == &b && a[..2] == b[..2]
}

#[hax_test::equiv]
pub fn derived_equality() -> bool {
    Point { x: 1, y: 2 } == Point { x: 1, y: 2 } && Point { x: 1, y: 2 } != Point { x: 2, y: 1 }
}
