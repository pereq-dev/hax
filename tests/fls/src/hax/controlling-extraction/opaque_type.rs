//! @fls: hax_opaque_type
//! @title: A type whose definition is hidden from the extraction with opaque_type
//! @lean(tier): supported
//! @lean(reason): The opaque type and the opaque functions over it extract as axioms, so the guard cannot evaluate (`failed to compile definition, consider marking it as 'noncomputable'`); the specification surface of the input-language reference pairs opaque items with a hand-written model.

#[allow(deprecated)]
#[hax_lib::opaque_type]
pub struct Handle {
    raw: u32,
}

#[hax_lib::opaque]
pub fn make(raw: u32) -> Handle {
    Handle { raw }
}

#[hax_lib::opaque]
pub fn read(h: &Handle) -> u32 {
    h.raw
}

#[hax_test::equiv]
pub fn opaque_type_value() -> bool {
    read(&make(7)) == 7
}
