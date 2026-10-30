//! @fls: fls_khy2e23i9o7z, fls_sWiU26n2xS3r
//! @title: Mutation through a shared reference with Cell
//! @lean(tier): unsupported
//! @lean(reason): The value-model boundary of the input-language reference: interior mutability destroys the unique-owner guarantee the value model depends on. charon and aeneas accept `Cell::new`, `Cell::get`, and `Cell::set` as opaque calls, so the extraction is stopped only by the missing model at `lake build`.

use core::cell::Cell;

pub fn bump(c: &Cell<u32>) {
    c.set(c.get() + 1);
}

#[hax_test::equiv]
pub fn mutated_through_shared_reference() -> bool {
    let c = Cell::new(1);
    bump(&c);
    bump(&c);
    c.get() == 3
}
