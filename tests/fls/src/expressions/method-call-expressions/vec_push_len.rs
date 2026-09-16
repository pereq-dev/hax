//! @fls: fls_b7i26954j1hc, fls_y7bj7y6davlh
//! @title: Method calls on a heap-allocated collection through its model
//! @lean(tier): supported

pub fn push_three() -> Vec<u8> {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v
}

#[hax_test::equiv]
pub fn len_after_push() -> bool {
    push_three().len() == 3
}
