//! @fls: fls_dCIIVXGhXDlO
//! @title: A type parameter substituted at two different types
//! @lean(tier): supported

pub fn first<T>(pair: (T, T)) -> T {
    pair.0
}

#[hax_test::equiv]
pub fn substituted_twice() -> bool {
    first((1u8, 2u8)) == 1 && first((true, false))
}
