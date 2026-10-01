//! @off: fstar, lean, coq, ssprove
//!
//! Generics are erased to `bitstring`, and impl blocks render as flat letfuns.

// A trivial generic identity function.
fn id<T>(x: T) -> T {
    x
}

// A struct with a user-written Clone impl.
struct Token {
    bytes: u8,
}

trait Cloneable {
    fn clone_it(&self) -> Self;
}

impl Cloneable for Token {
    fn clone_it(&self) -> Token {
        Token { bytes: self.bytes }
    }
}

// Renders as a direct call to the `Token` impl's `clone_it` letfun.
fn duplicate(t: Token) -> Token {
    t.clone_it()
}

// An inherent impl with a single method, to exercise the inherent path.
impl Token {
    fn fresh(b: u8) -> Token {
        Token { bytes: b }
    }
}

fn make_token() -> Token {
    Token::fresh(42)
}

// Calls the single, type-erased `id`.
fn pass_through(t: Token) -> Token {
    id::<Token>(t)
}
