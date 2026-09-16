//! @fls: fls_95eooah0vcqx, fls_wUwJEGzjCUAT, fls_BN5nUuvq9AwY, fls_axMVssU7an8p, fls_iAnCoxISuxxb, fls_ouOm2TQKNsP3, fls_yaYedRNaxhTe
//! @title: A type parameter instantiated with a type containing a borrow
//! @lean(tier): supported
//! @lean(reason): The instantiation of `Holder<T>` with `&mut u32` extracts without the backward function of the borrow and fails to elaborate (`expected a product type, got Holder U32`); the functions section of the input-language reference.

pub struct Holder<T> {
    pub inner: T,
}

pub fn wrap<T>(x: T) -> Holder<T> {
    Holder { inner: x }
}

pub fn through(x: &mut u32) {
    let h = wrap(x);
    *h.inner += 1;
}

#[hax_test::equiv]
pub fn borrowed_argument() -> bool {
    let mut v = 1;
    through(&mut v);
    let shared = wrap(&v);
    *shared.inner == 2
}
