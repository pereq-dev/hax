//! @fls: fls_11glzggtbgb3, fls_3AQUOBo7akXu, fls_b7i26954j1hc, fls_cawdkgvvd1x6, fls_gmpq15g77o20, fls_jx3ryre0xs88, fls_ljvj1f9fv085, fls_oxxk3snd7ya0, fls_pu0n9hakkym2, fls_y7bj7y6davlh, fls_tsn6SUUG9LvW, fls_c40C6rg6rGv6
//! @title: Method calls on a struct by value, by reference, and by mutable reference
//! @lean(tier): supported

pub struct Counter {
    pub n: u32,
}

impl Counter {
    pub fn get(&self) -> u32 {
        self.n
    }

    pub fn add(&mut self, k: u32) -> u32 {
        self.n += k;
        self.n
    }

    pub fn into_inner(self) -> u32 {
        self.n
    }
}

#[hax_test::equiv]
pub fn method_receivers() -> bool {
    let mut c = Counter { n: 1 };
    let after = c.add(2);
    c.get() == 3 && after == 3 && c.into_inner() == 3
}
