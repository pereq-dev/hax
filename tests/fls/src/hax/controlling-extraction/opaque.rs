//! @fls: hax_opaque
//! @title: A function whose body is hidden from the extraction with opaque
//! @lean(tier): supported
//! @lean(reason): The opaque function extracts as an axiom, so the guard cannot evaluate (`failed to compile definition, consider marking it as 'noncomputable'`); the specification surface of the input-language reference pairs `#[opaque]` with a hand-written model.

#[hax_lib::opaque]
pub fn mix(x: u32, y: u32) -> u32 {
    x.rotate_left(5) ^ y
}

pub fn use_mix(x: u32) -> u32 {
    mix(x, 1) ^ 1
}

#[hax_test::equiv]
pub fn opaque_body_is_callable() -> bool {
    use_mix(1) == 32
}
