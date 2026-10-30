//! @fls: fls_odd1hj3y1mgu, fls_qypjjpcf8uwq
//! @title: Inline modules nested three levels deep
//! @lean(tier): supported

pub mod geometry {
    pub mod shapes {
        pub mod square {
            pub fn area(side: u32) -> u32 {
                side * side
            }
        }
        pub fn unit_area() -> u32 {
            square::area(1)
        }
    }
    pub fn total(side: u32) -> u32 {
        shapes::square::area(side) + shapes::unit_area()
    }
}

#[hax_test::equiv]
pub fn items_in_nested_modules() -> bool {
    geometry::total(3) == 10
}
