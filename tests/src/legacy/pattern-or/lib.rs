//! @fail(extraction): proverif(HAX0001, HAX0001)
#![allow(dead_code)]

pub enum E {
    A,
    B,
}

pub fn bar(x: E) {
    match x {
        E::A | E::B => (),
    }
}
pub fn nested(x: Option<i32>) -> i32 {
    match x {
        Some(1 | 2) => 1,
        Some(x) => x,
        None => 0,
    }
}

pub fn deep(x: (i32, Option<i32>)) -> i32 {
    match x {
        (1 | 2, Some(3 | 4)) => 0,
        (x, _) => x,
    }
}

pub fn equivalent(x: (i32, Option<i32>)) -> i32 {
    match x {
        (1, Some(3)) | (1, Some(4)) | (2, Some(3)) | (2, Some(4)) => 0,
        (x, _) => x,
    }
}

pub fn deep_capture(x: Result<(i32, i32), (i32, i32)>) -> i32 {
    match x {
        Ok((1 | 2, x)) | Err((3 | 4, x)) => x,
        Ok((x, _)) | Err((x, _)) => x,
    }
}
