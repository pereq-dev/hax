//! @fls: fls_4z91ymz3ciup, fls_57t368kema7h, fls_939cugbxju5e, fls_9s4znhi0u3ys, fls_ccqomsereni2, fls_e5b9n910z1cp, fls_i31rodt42m0z, fls_ij8rebvupb85, fls_lmxz5768v5d8, fls_lwyq3vyc91rn, fls_ph7fsphbpbv4, fls_qed1pps827dv, fls_qo05owpmtag0, fls_sjwd8o5mknjo, fls_sm2hx8sh4agb, fls_uib1ml41mfrn, fls_vsxsbqps64o, fls_w7x9wy6t0qcp, fls_y3p6rtm7ek3l, fls_yjx1t3x6qpfg, fls_z3gj1v6g605r
//! @title: Struct expressions with named, shorthand, indexed, and base initializers
//! @lean(tier): supported

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

pub struct Pair(pub u32, pub u32);

pub enum Shape {
    Rect { w: u32, h: u32 },
    Unit,
}

pub fn build(width: u32) -> Config {
    let base = Config {
        width: 0,
        height: 2,
        depth: 3,
    };
    Config { width, ..base }
}

pub fn area(s: Shape) -> u32 {
    match s {
        Shape::Rect { w, h } => w * h,
        Shape::Unit => 1,
    }
}

#[hax_test::equiv]
pub fn constructed_values() -> bool {
    let c = build(7);
    let p = Pair { 1: 9, 0: 8 };
    let unit = Shape::Unit {};
    c.width == 7
        && c.height == 2
        && c.depth == 3
        && p.0 == 8
        && p.1 == 9
        && area(Shape::Rect { w: 2, h: 3 }) == 6
        && area(unit) == 1
}
