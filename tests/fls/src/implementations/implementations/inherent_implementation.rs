//! @fls: fls_ivxpoxggy7s6, fls_v0n0bna40dqr, fls_yopmjbnw8tbl, fls_Mcpdzzcw43M7, fls_eIHc8Y9fBtr0, fls_o62i75sjzp9y, fls_yuyesijndu9n
//! @title: An inherent implementation using Self as an alias for the implementing type
//! @lean(tier): supported

pub struct Circle {
    pub radius: u32,
}

impl Circle {
    pub fn new(radius: u32) -> Self {
        Self { radius }
    }

    pub fn diameter(&self) -> u32 {
        self.radius * 2
    }
}

#[hax_test::equiv]
pub fn inherent_methods() -> bool {
    Circle::new(3).diameter() == 6
}
