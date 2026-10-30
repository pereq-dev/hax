//! @fls: fls_a6zlvyxpgsew, fls_3aKZB0ILIkZw, fls_Xo1ODwOyX7Vm, fls_KgH6c5cC4S0G, fls_iT9WCNfUZQnC, fls_69hqMjvNno9u
//! @title: impl Trait in return position capturing a type parameter
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

pub struct Pair<T>(pub T, pub T);

impl<T: Weight> Weight for Pair<T> {
    fn weight(&self) -> u32 {
        self.0.weight() + self.1.weight()
    }
}

pub fn doubled<T: Weight + Clone>(x: T) -> impl Weight {
    Pair(x.clone(), x)
}

impl Clone for Stone {
    fn clone(&self) -> Self {
        Stone(self.0)
    }
}

#[hax_test::equiv]
pub fn returned_opaque_value() -> bool {
    doubled(Stone(4)).weight() == 8
}
