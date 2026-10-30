//! @fls: fls_2dg7wl68z7ar, fls_4dv7x9nh2h4e, fls_4sh2yrslszvb, fls_5w964phrru82, fls_81nnizrxgrsm, fls_8dba4o5qg8js, fls_8wjdichfxp0y, fls_Lm5WqZ2dJvYp, fls_Mv2YpG7sLcHw, fls_Nc6JvS9rFaLm, fls_Nt6QmC3sLpXz, fls_Qr3TcH8vNpKs, fls_Zm9VxF5pQwLd, fls_d9gerg12hm2d, fls_ei4pbeksd1v8, fls_g551l8r8yh6d, fls_g6xyz0beps3o, fls_jwxykea99psw, fls_l45i24ikfavm, fls_lrdrtedyz28i, fls_mvi9z1x836qu, fls_oj8dg28xw5yp, fls_pgulnjeoxwtj, fls_y44jzkbv74bv, fls_yv11febo0kyb, fls_zJQ4LecT1HYd, fls_Bx8KsV4nQdRt
//! @title: Match expressions selecting arms in declarative order, including or-pattern arms
//! @lean(tier): supported

pub enum Token {
    Digit(u8),
    Letter(char),
    Space,
    Other,
}

pub fn weight(t: Token) -> u32 {
    match t {
        Token::Digit(0) => 0,
        Token::Digit(d) => d as u32,
        Token::Letter(_) | Token::Space => 100,
        Token::Other => 1000,
    }
}

pub fn first_match(x: u32) -> u32 {
    match x {
        0..=5 => 1,
        3..=9 => 2,
        _ => 3,
    }
}

#[hax_test::equiv]
pub fn selected_arms() -> bool {
    weight(Token::Digit(0)) == 0
        && weight(Token::Digit(7)) == 7
        && weight(Token::Letter('a')) == 100
        && weight(Token::Space) == 100
        && weight(Token::Other) == 1000
        && first_match(4) == 1
        && first_match(8) == 2
        && first_match(12) == 3
}
