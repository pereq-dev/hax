//! @fls: fls_72bpdsxxbgeq, fls_7e7tka4f2f1a, fls_8ibfqxtnahzx, fls_9cc0ml2sru6x, fls_9ifaterm8nop, fls_aeh5pzpcjveq, fls_d68ddlse4zp, fls_f6wktzofzdn1, fls_fyend8kkpqq4, fls_g73vguanbs1x, fls_gw49nukfveib, fls_jjf3sz9ddfhy, fls_y9bc691kkh6v
//! @title: Dereferencing references and a Box through the Deref traits
//! @lean(tier): supported

pub fn read(r: &u32) -> u32 {
    *r
}

pub fn write(r: &mut u32) {
    *r = *r + 1;
}

pub fn unbox(b: Box<u32>) -> u32 {
    *b
}

pub fn write_box(mut b: Box<u32>) -> u32 {
    *b += 10;
    *b
}

#[hax_test::equiv]
pub fn dereferenced_values() -> bool {
    let mut answer = 42;
    write(&mut answer);
    read(&answer) == 43 && unbox(Box::new(4)) == 4 && write_box(Box::new(1)) == 11
}
