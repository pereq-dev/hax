//! @fls: fls_e5x92q2rq8a0, fls_x96a0xzcyrko, fls_qi5kyvj1e8th, fls_grtiwf7q8jah, fls_tbe2qq7whq10, fls_my93neopj9x0, fls_vqIZaEl4EKu5
//! @title: Built-in derive macros invoked through the derive attribute
//! @lean(tier): supported

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Answer {
    Yes,
    No(u8),
}

#[hax_test::equiv]
pub fn derived_implementations_extract() -> bool {
    let a = Answer::No(3);
    let b = a;
    a == b && a != Answer::Yes && b.clone() == Answer::No(3)
}
