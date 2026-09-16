//! @fls: fls_1crq0mexo5r1, fls_pedy2pqrvnx7, fls_sl47k9oj5p7t, fls_cfoy86mkmqa4, fls_ag6m4mvpturw, fls_rnppz6y5z8pi, fls_u59rilepu8z9, fls_4faltss0xbn4, fls_fqt5w3qsykca, fls_h3y8r4298s53, fls_am5h8r887bz5
//! @title: Path patterns naming a constant, a unit enum variant, and a unit struct
//! @lean(tier): supported

pub const ZERO: u32 = 0;

pub enum Light {
    Red,
    Green,
}

#[derive(PartialEq, Eq)]
pub struct Token;

pub const TOKEN: Token = Token;

pub fn is_zero(n: u32) -> bool {
    match n {
        ZERO => true,
        _ => false,
    }
}

pub fn go(l: Light) -> bool {
    match l {
        Light::Green => true,
        Light::Red => false,
    }
}

pub fn token_matches(u: Token) -> bool {
    match u {
        TOKEN => true,
    }
}

pub fn unit_matches(u: Token) -> bool {
    match u {
        Token => true,
    }
}

pub enum Only {
    It,
}

pub fn only_matches(o: Only) -> bool {
    match o {
        Only::It => true,
    }
}

#[hax_test::equiv]
pub fn paths_match_their_values() -> bool {
    is_zero(0)
        && !is_zero(1)
        && go(Light::Green)
        && !go(Light::Red)
        && token_matches(Token)
        && unit_matches(Token)
        && only_matches(Only::It)
}
