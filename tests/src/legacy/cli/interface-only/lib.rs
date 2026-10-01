//! @fail(extraction): proverif(HAX0001, HAX0001)
//! @fail(tc): legacy-lean(1)
//! @fail(extraction): legacy-lean(HAX0001, HAX0001, HAX0001)
//! @fail(tc): fstar(47)

#![allow(dead_code)]

/// This item contains unsafe blocks and raw references, two features
/// not supported by hax. Thanks to the `-i` flag and the `+:`
/// modifier, `f` is still extractable as an interface.
///
/// Expressions within type are still extracted, as well as pre- and
/// post-conditions.
#[hax_lib::requires(x < 254)]
#[hax_lib::ensures(|r| r[0] > x)]
/// @fail(extraction): ssprove(HAX0008, HAX0008, HAX0008, HAX0008), coq(HAX0008, HAX0008, HAX0008, HAX0008), fstar(HAX0008, HAX0008, HAX0008, HAX0008)
/// @fail(extraction): legacy-lean(HAX0008, HAX0008, HAX0008, HAX0008)
/// @fail(extraction): proverif(HAX0008, HAX0008, HAX0008, HAX0008)
fn f(x: u8) -> [u8; 4] {
    let y = x as *const i8;

    unsafe {
        println!("{}", *y);
    }

    [x + 1, x, x, x]
}

/// This struct contains a field which uses raw pointers, which are
/// not supported by hax. This item cannot be extracted at all: we
/// need to exclude it with `-i '-*::Foo'`.
/// @fail(extraction): fstar(HAX0008), coq(HAX0008), ssprove(HAX0008)
/// @fail(extraction): legacy-lean(HAX0008)
/// @fail(extraction): proverif(HAX0008)
struct Foo {
    unsupported_field: *const u8,
}

struct Bar;

/// Non-inherent implementations are extracted, their bodies are not
/// dropped. This might be a bit surprising: see
/// https://github.com/pereq-dev/hax/issues/616.
impl From<()> for Bar {
    fn from((): ()) -> Self {
        Bar
    }
}

/// If you need to drop the body of a method, please hoist it:
impl From<u8> for Bar {
    fn from(x: u8) -> Self {
        fn from(_: u8) -> Bar {
            Bar
        }
        from(x)
    }
}

pub struct Holder<T> {
    pub(crate) value: Vec<T>,
}

impl<T> From<()> for Holder<T> {
    fn from((): ()) -> Self {
        Holder { value: Vec::new() }
    }
}

pub struct Param<const SIZE: usize> {
    pub(crate) value: [u8; SIZE],
}

impl<const SIZE: usize> From<()> for Param<SIZE> {
    fn from((): ()) -> Self {
        Param { value: [0; SIZE] }
    }
}

fn f_generic<const X: usize, U>(_x: U) -> Param<X> {
    Param { value: [0; X] }
}

trait T {
    type Assoc;
    fn d();
}

/// Impls with associated types are not erased
impl T for u8 {
    type Assoc = u8;
    fn d() {}
}
trait T2 {
    fn d();
}

/// Items can be forced to be transparent
#[hax_lib::transparent]
#[hax_lib::attributes]
impl T2 for u8 {
    #[hax_lib::requires(false)]
    fn d() {}
}

#[hax_lib::requires(b.len() >= n)]
#[hax_lib::ensures(|out| out <= n)]
fn padlen(b: &[u8], n: usize) -> usize {
    if n > 0 && b[n - 1] == 0 {
        1 + padlen(b, n - 1)
    } else {
        0
    }
}
