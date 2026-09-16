//! @fls: fls_1of56vl2ewq0, fls_7b0fvwrmz0mh, fls_7xh2iphiteam
//! @title: The non_exhaustive attribute on an enum, a variant, and a struct
//! @lean(tier): supported

#[non_exhaustive]
pub enum Status {
    Active,
    #[non_exhaustive]
    Failed {
        code: u32,
    },
}

#[non_exhaustive]
pub struct Config {
    pub level: u32,
}

pub fn describe(s: &Status) -> u32 {
    match s {
        Status::Active => 1,
        Status::Failed { code, .. } => *code,
    }
}

#[hax_test::equiv]
pub fn attributed_types_behave_as_plain() -> bool {
    let c = Config { level: 3 };
    describe(&Status::Active) == 1 && describe(&Status::Failed { code: 7 }) == 7 && c.level == 3
}
