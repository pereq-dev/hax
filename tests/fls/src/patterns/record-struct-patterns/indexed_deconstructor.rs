//! @fls: fls_g6dytd6aq62d, fls_mnh35ehva8tx, fls_p2rjnlbvifaa, fls_8bi8q3usubby, fls_f0u0j4q90lpl, fls_9y1gbv47z23o, fls_D5tAGzrjXFTu
//! @title: Record struct patterns with indexed deconstructors on tuple structs and unit variants
//! @lean(tier): supported

pub struct Pair(pub u32, pub u32);

pub enum Marker {
    Here,
    There(u8),
}

pub fn second(p: Pair) -> u32 {
    let Pair { 1: b, .. } = p;
    b
}

pub fn is_here(m: Marker) -> bool {
    match m {
        Marker::Here {} => true,
        Marker::There { 0: _ } => false,
    }
}

#[hax_test::equiv]
pub fn indexed_fields() -> bool {
    second(Pair(1, 2)) == 2 && is_here(Marker::Here) && !is_here(Marker::There(1))
}
