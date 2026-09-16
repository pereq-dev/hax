//! @fls: fls_ho4kem1slcxg, fls_7le2vcdbtxbq, fls_x3alg07yd7hx, fls_4hulwazdu20i, fls_ptocwx5p25lj, fls_ygam5nisv98c, fls_1nxknwjdp0am, fls_e5a5z5yht26l, fls_ZjJ7y9r6QQMW, fls_pybv4krsvktv, fls_mbdS0xiNlj92
//! @title: Field, method, and call resolution through references and Box
//! @lean(tier): supported

pub struct Point {
    pub x: u32,
}

impl Point {
    pub fn get(&self) -> u32 {
        self.x
    }
}

pub fn through_references(p: &&Point) -> u32 {
    p.x + p.get()
}

pub fn through_box(b: Box<Point>) -> u32 {
    b.x + b.get()
}

pub fn call_through_reference<F: Fn(u32) -> u32>(f: &F, x: u32) -> u32 {
    f(x)
}

#[hax_test::equiv]
pub fn resolved_through_dereference() -> bool {
    let p = Point { x: 3 };
    through_references(&&p) == 6
        && through_box(Box::new(Point { x: 4 })) == 8
        && call_through_reference(&|v| v + 1, 1) == 2
}
