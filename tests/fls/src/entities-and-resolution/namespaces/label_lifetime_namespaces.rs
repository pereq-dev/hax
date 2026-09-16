//! @fls: fls_9e3xeza853wx, fls_w625tk3ogdui
//! @title: A variable sharing its name with a lifetime parameter and with a label
//! @lean(tier): supported

pub fn larger<'a>(a: &'a u32, b: &'a u32) -> &'a u32 {
    if *a >= *b { a } else { b }
}

pub fn successor(a: u32) -> u32 {
    'a: loop {
        break 'a a + 1;
    }
}

#[hax_test::equiv]
pub fn names_do_not_clash() -> bool {
    let a = 3;
    *larger(&a, &2) + successor(a) == 7
}
