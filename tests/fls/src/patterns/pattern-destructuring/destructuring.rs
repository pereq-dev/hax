//! @fls: fls_UfvoShNTxVD2, fls_YuQxJCjneaW5, fls_nrTZ4mn6IJCl, fls_wAMHIDvLG0LM
//! @title: Destructuring enums, structs, and tuples with underscore and rest patterns
//! @lean(tier): supported

pub enum Token {
    Number(u32, bool),
    Word(u8, u8, u8),
    End,
}

pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub fn first_of_token(t: Token) -> u32 {
    match t {
        Token::Number(n, _) => n,
        Token::Word(first, ..) => first as u32,
        Token::End => 0,
    }
}

pub fn width(r: Rect) -> u32 {
    let Rect { w, x: _, .. } = r;
    w
}

pub fn middle(t: (u32, u32, u32, u32)) -> u32 {
    let (_, m, ..) = t;
    m
}

#[hax_test::equiv]
pub fn destructured_values() -> bool {
    first_of_token(Token::Number(7, true)) == 7
        && first_of_token(Token::Word(3, 4, 5)) == 3
        && first_of_token(Token::End) == 0
        && width(Rect {
            x: 1,
            y: 2,
            w: 3,
            h: 4,
        }) == 3
        && middle((1, 2, 3, 4)) == 2
}
