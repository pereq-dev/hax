//! @fls: fls_lyw4t098sxrj, fls_2bkcn83smy2y, fls_90hQvSh7Bfyg, fls_FILuR3pfwjw3, fls_v3a6y2ze44v2, fls_ldr7tsuqw34s, fls_sxo1jb25pl8a, fls_WAA4WmohGu6T, fls_wRmvtgQkFA6w, fls_ce73bg0BqV1X, fls_jlNKxkuhsvX4, fls_q0KFfNALMslq, fls_ar03D5rxjzy0, fls_5dlnffim6fso, fls_9rhflreuubhq, fls_s86dgrdpl1w4, fls_IPYvldMqduf4, fls_MOXId37fcNPY, fls_2UyFcB6Our1v, fls_irdKqoYzBM0M, fls_gAWsqibl4GLq, fls_yY58pFpkig9o
//! @title: Simple, renaming, glob, and nesting use imports
//! @lean(tier): supported

pub mod shapes {
    pub fn square(x: u32) -> u32 {
        x * x
    }
    pub fn cube(x: u32) -> u32 {
        x * x * x
    }
    pub mod colors {
        pub const RED: u32 = 1;
        pub const BLUE: u32 = 2;
    }
    pub enum Kind {
        Round,
        Flat,
    }
}

pub mod user {
    use super::shapes::Kind::{self, Flat, Round};
    use super::shapes::colors::*;
    use super::shapes::{cube, square as sq};

    pub fn compute(k: Kind) -> u32 {
        let base = match k {
            Round => RED,
            Flat => BLUE,
        };
        sq(base) + cube(base)
    }
}

#[hax_test::equiv]
pub fn imported_names() -> bool {
    user::compute(shapes::Kind::Round) == 2 && user::compute(shapes::Kind::Flat) == 12
}
