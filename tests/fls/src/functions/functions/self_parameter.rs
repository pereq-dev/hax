//! @fls: fls_ymeo93t4mz4, fls_PGtp39f6gJwU, fls_Ogziu8S01qPQ, fls_35aSvBxBnIzm, fls_yZ2yIXxmy2ri
//! @title: Self parameters by value, by shared reference, and by mutable reference
//! @lean(tier): supported

pub struct Acc(pub u32);

impl Acc {
    pub fn get(&self) -> u32 {
        self.0
    }

    pub fn add(&mut self, k: u32) {
        self.0 += k;
    }

    pub fn finish(self) -> u32 {
        self.0 * 10
    }
}

#[hax_test::equiv]
pub fn self_kinds() -> bool {
    let mut a = Acc(1);
    a.add(2);
    a.get() == 3 && a.finish() == 30
}
