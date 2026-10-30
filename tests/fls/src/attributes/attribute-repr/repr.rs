//! @fls: fls_is2esjz1sy36, fls_wbdtpntjr95w, fls_p1ugiol1e5v5
//! @title: The repr attribute with C, primitive, and transparent representations
//! @lean(tier): supported

#[repr(C)]
pub struct Pair {
    pub a: u8,
    pub b: u32,
}

#[repr(u16)]
pub enum Code {
    Ok = 200,
    NotFound = 404,
}

#[repr(transparent)]
pub struct Wrapper(pub u32);

#[hax_test::equiv]
pub fn representation_is_transparent() -> bool {
    let p = Pair { a: 1, b: 2 };
    p.a as u32 + p.b == 3 && Code::NotFound as u16 - Code::Ok as u16 == 204 && Wrapper(9).0 == 9
}
