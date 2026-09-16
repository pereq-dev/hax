//! @fls: hax_concretization
//! @title: The Concretization trait lowering an abstract value to a machine integer
//! @lean(tier): supported
//! @lean(reason): The Lean model of `hax_lib::Int` has no `Concretization` instance for `u16` (`Unknown constant hax_lib.int.Int.Insts.Hax_libAbstractionConcretizationU16.concretize`), so the extraction fails to elaborate; the refinement section of the input-language reference.

// The hax-lib API under test exists only in hax mode.
#![cfg(hax)]

use hax_lib::{Abstraction, Concretization, int};

pub fn round_trip(x: u16) -> u16 {
    let abstract_value = x.lift() + int!(1);
    let back: u16 = abstract_value.concretize();
    back
}

#[hax_test::equiv]
pub fn concretized_value() -> bool {
    round_trip(41) == 42
}
