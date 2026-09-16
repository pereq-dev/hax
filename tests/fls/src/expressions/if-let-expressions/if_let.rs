//! @fls: fls_4vyrufo4qdeg, fls_8fg2ufaxjkv5, fls_dsrjup2umr9, fls_ijo73wtz1sy, fls_nhngr8y850dt, fls_okVOYzTT6fBK, fls_qeho5iqiy59, fls_qfnwwvzxsl3
//! @title: If let expressions with and without an else branch
//! @lean(tier): supported

pub enum Shape {
    Circle(u32),
    Square(u32),
}

pub fn radius_or_zero(s: &Shape) -> u32 {
    if let Shape::Circle(r) = s { *r } else { 0 }
}

pub fn side_sum(shapes: &[Shape]) -> u32 {
    let mut acc = 0;
    let mut i = 0;
    while i < shapes.len() {
        if let Shape::Square(side) = &shapes[i] {
            acc += *side;
        }
        i += 1;
    }
    acc
}

#[hax_test::equiv]
pub fn matched_branches() -> bool {
    let shapes = [Shape::Circle(3), Shape::Square(2), Shape::Square(5)];
    radius_or_zero(&shapes[0]) == 3 && radius_or_zero(&shapes[1]) == 0 && side_sum(&shapes) == 7
}
