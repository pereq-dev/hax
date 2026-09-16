//! @fls: fls_5ifai8nkp5ek, fls_cjywisyiyti6, fls_gvanx4874ycy
//! @title: Path expressions resolving to constants, functions, statics, and constructors
//! @lean(tier): supported

pub const LIMIT: u32 = 10;
pub static NAME_LEN: usize = 4;

pub struct Wrapper(pub u32);

pub enum Shape {
    Square(u32),
    Dot,
}

pub fn twice(x: u32) -> u32 {
    x * 2
}

pub fn build() -> (u32, Wrapper, Shape, Shape, usize) {
    let f = twice;
    (f(LIMIT), Wrapper(1), Shape::Square(2), Shape::Dot, NAME_LEN)
}

#[hax_test::equiv]
pub fn resolved_entities() -> bool {
    let (a, w, s, d, n) = build();
    let side = match s {
        Shape::Square(k) => k,
        Shape::Dot => 0,
    };
    let dot = match d {
        Shape::Dot => true,
        _ => false,
    };
    a == 20 && w.0 == 1 && side == 2 && dot && n == 4 && self::path_expressions_self::twice(1) == 2
}

pub mod path_expressions_self {
    pub use super::twice;
}
