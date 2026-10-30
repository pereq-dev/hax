//! @fls: fls_a6zlvyxpgsew
//! @title: impl Trait in argument position
//! @lean(tier): supported

pub trait Weight {
    fn weight(&self) -> u32;
}

pub struct Stone(pub u32);

impl Weight for Stone {
    fn weight(&self) -> u32 {
        self.0
    }
}

pub fn total(a: impl Weight, b: impl Weight) -> u32 {
    a.weight() + b.weight()
}

#[hax_test::equiv]
pub fn sum_of_weights() -> bool {
    total(Stone(2), Stone(3)) == 5
}
