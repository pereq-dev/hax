//! @fls: fls_wt81sbsecmu0, fls_ckcnkbb6y3cq, fls_ze0u9gfylmhn
//! @title: A value moved from one owner to another
//! @lean(tier): supported

pub struct Token(pub u32);

pub fn consume(t: Token) -> u32 {
    t.0
}

#[hax_test::equiv]
pub fn ownership_transfers() -> bool {
    let first = Token(5);
    let second = first;
    consume(second) == 5
}
