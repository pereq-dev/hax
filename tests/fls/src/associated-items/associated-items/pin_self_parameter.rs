//! @fls: fls_oy92gzxgc273, fls_WXnCWfJGoQx3, fls_IKSPR7ZQMErU
//! @title: A method whose self parameter is a pinned mutable reference
//! @lean(tier): supported
//! @lean(reason): The Lean core model has neither `Pin::new` nor the `Deref` instance of `Pin` (`Unknown constant CoreModels.core.pin.Pin.new`); the model-trust section of the input-language reference.

use core::pin::Pin;

pub struct Square(pub u32);

impl Square {
    pub fn pinned_side(self: Pin<&mut Self>) -> u32 {
        self.0
    }
}

#[hax_test::equiv]
pub fn pinned_receiver() -> bool {
    let mut s = Square(2);
    Pin::new(&mut s).pinned_side() == 2
}
