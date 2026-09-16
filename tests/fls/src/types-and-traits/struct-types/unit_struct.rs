//! @fls: fls_g1azfj548136
//! @title: Unit struct as a value without fields
//! @lean(tier): supported

pub struct Marker;

pub fn tag(_m: Marker) -> u8 {
    1
}

#[hax_test::equiv]
pub fn unit_struct_value() -> bool {
    tag(Marker) == 1
}
