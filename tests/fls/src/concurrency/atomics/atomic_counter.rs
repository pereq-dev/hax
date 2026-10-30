//! @fls: fls_3pjla9s93mhd, fls_wn4ynaio8u47, fls_q7mn6pdd8bix, fls_4urmnh4mfehl
//! @title: An atomic counter incremented through a shared reference
//! @lean(tier): unsupported
//! @lean(reason): The value-model boundary of the input-language reference: atomics need a memory model of observable interleavings that the sequential value semantics does not provide. charon and aeneas accept the atomic operations as opaque calls, so only the missing model stops the extraction at `lake build` (`Unknown identifier core.sync.atomic.AtomicU32Align4U32.fetch_add`).

use core::sync::atomic::{AtomicU32, Ordering};

pub fn bump(counter: &AtomicU32) -> u32 {
    counter.fetch_add(1, Ordering::SeqCst) + 1
}

#[hax_test::equiv]
pub fn counted() -> bool {
    let c = AtomicU32::new(0);
    bump(&c);
    bump(&c) == 2 && c.load(Ordering::SeqCst) == 2
}
