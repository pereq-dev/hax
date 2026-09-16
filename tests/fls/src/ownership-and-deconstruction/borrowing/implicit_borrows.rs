//! @fls: fls_c02flohk54pc, fls_j9kof0px3l7s, fls_hyl4bdjbuzbw, fls_pu19i4sj6yg0, fls_zepwytjwy049, fls_MT69AGRXH9pS, fls_bjf3futso849, fls_g4i0jb27iryr, fls_jv18y618j2s3, fls_kky9ufexrvaw, fls_nordokzfy36d, fls_o5oq4jfswr4q, fls_yfmy4v5zlgw9, fls_kup2ou22nwyl, fls_8q5ly4x104ai, fls_Z2n7EJiLY5CT, fls_ibi9mqGeHNLp, fls_f9we73i8vwq3, fls_w5bjgaov8w60, fls_7Kzu0qXiBUCb, fls_VDHBP4oAoqT5
//! @title: Explicit borrows and the implicit borrows of method calls, indexing, field access, and comparison
//! @lean(tier): supported

pub struct Counter {
    pub n: u32,
    pub items: [u32; 2],
}

impl Counter {
    pub fn bump(&mut self) {
        self.n += 1;
    }
    pub fn get(&self) -> u32 {
        self.n
    }
}

pub fn exercise() -> u32 {
    let mut c = Counter {
        n: 0,
        items: [4, 5],
    };
    c.bump();
    c.bump();
    c.items[1] += 1;
    c.n += c.items[0];
    let shared = &c;
    let equal = shared.n == c.get();
    let exclusive = &mut c;
    exclusive.n *= 2;
    if equal { c.n + c.items[1] } else { 0 }
}

#[hax_test::equiv]
pub fn borrowed_values() -> bool {
    exercise() == 18
}
