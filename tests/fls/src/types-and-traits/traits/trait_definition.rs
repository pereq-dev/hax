//! @fls: fls_tani6lesan9u, fls_PiAR1B26SoZV, fls_Y28596CVBzDG, fls_AdbbUZZgMEsQ
//! @title: A trait with a method using Self, implemented by a local type
//! @lean(tier): supported

pub trait Scale {
    fn scale(self, k: u32) -> Self;
}

pub struct Meters(pub u32);

impl Scale for Meters {
    fn scale(self, k: u32) -> Self {
        Meters(self.0 * k)
    }
}

#[hax_test::equiv]
pub fn scaled() -> bool {
    Meters(3).scale(4).0 == 12
}
