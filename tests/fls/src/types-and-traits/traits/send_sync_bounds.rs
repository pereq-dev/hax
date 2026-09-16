//! @fls: fls_YynbrIceKmsJ, fls_RLFIzQeAPhG6, fls_avSxO0LEka2x
//! @title: Send and Sync bounds satisfied by a plain struct
//! @lean(tier): supported

pub struct Pair {
    pub a: u32,
    pub b: u32,
}

pub fn through<T: Send + Sync>(p: T) -> T {
    p
}

#[hax_test::equiv]
pub fn bounds_are_transparent() -> bool {
    let p = through(Pair { a: 1, b: 2 });
    p.a + p.b == 3
}
