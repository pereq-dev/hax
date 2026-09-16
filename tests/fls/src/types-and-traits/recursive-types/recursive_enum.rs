//! @fls: fls_z22std1crl49
//! @title: A recursive enum through Box
//! @lean(tier): supported

pub enum List {
    Nil,
    Cons(u32, Box<List>),
}

pub fn sum(l: &List) -> u32 {
    match l {
        List::Nil => 0,
        List::Cons(x, rest) => *x + sum(rest),
    }
}

#[hax_test::equiv]
pub fn sum_of_three() -> bool {
    let l = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    sum(&l) == 6
}
