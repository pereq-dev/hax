#![allow(non_camel_case_types)]

use crate::option::Option;

/// See [`std::primitive::bool`]
#[cfg_attr(hax_backend_fstar, hax_lib::exclude)]
pub struct bool;

impl bool {
    /// See [`std::primitive::bool::then_some`]
    pub fn then_some<T>(b: core::primitive::bool, t: T) -> Option<T> {
        if b { Option::Some(t) } else { Option::None }
    }
    /// See [`std::primitive::bool::then`]
    // opaque for F*, which loses the closure's `Output` type and so cannot type the body.
    #[cfg_attr(hax_backend_fstar, hax_lib::opaque)]
    pub fn then<T, F: FnOnce() -> T>(b: core::primitive::bool, f: F) -> Option<T> {
        if b { Option::Some(f()) } else { Option::None }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::Inject;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_then_some(b in any::<bool>(), t in any::<u8>()) {
            prop_assert_eq!(super::bool::then_some(b, t), b.then_some(t).inject());
        }

        #[test]
        fn test_then(b in any::<bool>(), t in any::<u8>()) {
            prop_assert_eq!(super::bool::then(b, || t), b.then(|| t).inject());
        }
    }
}
