//! @fail(extraction): proverif(HAX0001)
//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! Trait calls resolve to the impl whose trait arguments and Self type match,
//! not just the head of the Self type.

struct Key(u8);

trait Make<T> {
    fn make(t: T) -> Self;
}

impl Make<u8> for Key {
    fn make(t: u8) -> Key {
        Key(t)
    }
}

impl Make<bool> for Key {
    fn make(t: bool) -> Key {
        Key(if t { 1 } else { 0 })
    }
}

fn from_u8() -> Key {
    Key::make(3u8)
}

fn from_bool() -> Key {
    Key::make(true)
}

struct Wrapper<T>(T);

trait Describe {
    fn describe(&self) -> u8;
}

impl Describe for Wrapper<u8> {
    fn describe(&self) -> u8 {
        1
    }
}

impl Describe for Wrapper<bool> {
    fn describe(&self) -> u8 {
        2
    }
}

fn describe_u8(w: Wrapper<u8>) -> u8 {
    w.describe()
}

fn describe_bool(w: Wrapper<bool>) -> u8 {
    w.describe()
}

// A generic impl matches every instance.
struct Boxed<T>(T);

impl<T> Describe for Boxed<T> {
    fn describe(&self) -> u8 {
        3
    }
}

fn describe_boxed(b: Boxed<u16>) -> u8 {
    b.describe()
}

struct W(u8);

impl PartialEq for W {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

// `!=` is the negation of the user's `eq`.
fn ne_custom(a: W, b: W) -> bool {
    a != b
}

trait WithDefault {
    fn m(&self) -> u8 {
        0
    }
}

struct A;

impl WithDefault for &A {
    fn m(&self) -> u8 {
        1
    }
}

impl WithDefault for A {}

// The `&A` and `A` impls look alike once references are dropped: rejected.
fn call_default(a: A) -> u8 {
    a.m()
}

struct Celsius(u8);
struct Fahrenheit(u8);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 + 32)
    }
}

// `.into()` calls the user's `From` impl rather than being the identity.
fn into_user_from(c: Celsius) -> Fahrenheit {
    c.into()
}

fn from_user_from(c: Celsius) -> Fahrenheit {
    Fahrenheit::from(c)
}

// With no `From` impl in the crate, `.into()` stays the std conversion.
fn into_std(x: u8) -> u32 {
    x.into()
}

struct Countdown(u8);

impl Iterator for Countdown {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        None
    }
}

// `count` is a default method of a std trait: an undefined symbol of the trait,
// with a warning.
fn count_down(c: Countdown) -> usize {
    c.count()
}

struct W2(u8);

impl PartialEq<u8> for W2 {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialEq<u8> for &W2 {
    fn eq(&self, other: &u8) -> bool {
        self.0 != *other
    }
}

// The `W2` and `&W2` impls of a std trait look alike once references are
// dropped: the call stays the trait's, with a warning.
fn ambiguous_std(w: W2) -> bool {
    w == 3
}
