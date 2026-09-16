//! @fls: fls_drfzu02bo7oe, fls_wnhci8phdu4m, fls_ch2lvm50olqd, fls_caufcwkpz689, fls_4lg92b9yima6, fls_xn1au0blioa3, fls_jmcjboopvytb
//! @title: Variables initialized at declaration, by a later assignment, and uninitialized by a move
//! @lean(tier): supported

pub struct Token(pub u32);

pub fn init(flag: bool) -> u32 {
    let a = 1;
    let b;
    if flag {
        b = 10;
    } else {
        b = 20;
    }
    let c = Token(100);
    let moved = c;
    let c = Token(1000);
    a + b + moved.0 + c.0
}

#[hax_test::equiv]
pub fn initialized_values() -> bool {
    init(true) == 1111 && init(false) == 1121
}
