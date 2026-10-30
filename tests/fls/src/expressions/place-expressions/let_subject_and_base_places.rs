//! @fls: fls_5yXuTLQOQ3cc, fls_9r7dopqf1nzl, fls_JBfZuFDQg3mU
//! @title: Place expressions as an if let subject and as a base initializer
//! @lean(tier): supported

#[derive(Clone, Copy)]
pub struct Config {
    pub limit: Option<u32>,
    pub scale: u32,
}

pub fn read(c: &Config) -> u32 {
    let mut total = 0;
    if let Some(l) = c.limit {
        total += l;
    }
    total * c.scale
}

#[hax_test::equiv]
pub fn places_are_read() -> bool {
    let base = Config {
        limit: Some(4),
        scale: 1,
    };
    let updated = Config { scale: 2, ..base };
    read(&base) == 4 && read(&updated) == 8
}
