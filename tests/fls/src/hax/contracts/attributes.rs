//! @fls: hax_attributes
//! @title: Contracts on the methods of an impl block enabled with attributes
//! @lean(tier): supported

pub struct Counter {
    pub n: u32,
}

#[hax_lib::attributes]
impl Counter {
    #[hax_lib::requires(self.n < 1000)]
    #[hax_lib::ensures(|result| result == self.n + 1)]
    pub fn next(&self) -> u32 {
        self.n + 1
    }
}

#[hax_test::equiv]
pub fn method_contract() -> bool {
    Counter { n: 4 }.next() == 5
}
