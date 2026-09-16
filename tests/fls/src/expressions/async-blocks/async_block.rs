//! @fls: fls_9ghp5yet75y6, fls_hhidi5ukxo, fls_ncd0wkgtldem, fls_oisws5qykedi, fls_pvnofoomgwl5, fls_tzclkasinpoq
//! @title: An async block capturing a variable
//! @lean(tier): unsupported
//! @lean(reason): The value-model boundary of the input-language reference: async compiles to a poll-driven state machine whose suspension points need an execution model.

use core::future::Future;

pub fn make(x: u32) -> impl Future<Output = u32> {
    async move { x + 1 }
}

pub fn keep() -> u32 {
    let _fut = make(1);
    2
}

#[hax_test::equiv]
pub fn future_is_created() -> bool {
    keep() == 2
}
