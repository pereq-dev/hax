//! Equivalence tests for `alloc::collections::VecDeque`.

use rust_lean_test_macro::rust_lean_test;
use std::collections::VecDeque;

fn deque() -> VecDeque<u8> {
    let mut d = VecDeque::new();
    d.push_back(1);
    d.push_back(2);
    d.push_back(3);
    d
}

#[rust_lean_test]
pub fn test_vec_deque_iter() -> bool {
    let d = deque();
    let mut total = 0u8;
    for x in d.iter() {
        total = total * 10 + *x;
    }
    total == 123
}

#[rust_lean_test]
pub fn test_vec_deque_remove() -> bool {
    let mut d = deque();
    d.remove(1) == Some(2) && d.len() == 2 && d[1] == 3
}

#[rust_lean_test]
pub fn test_vec_deque_remove_out_of_range() -> bool {
    let mut d = deque();
    d.remove(3).is_none() && d.len() == 3
}
