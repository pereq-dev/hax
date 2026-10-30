//! @fail(extraction): proverif(HAX0001)
//! @fail(tc): legacy-lean(1)
#![allow(dead_code)]
use hax_lib::*;

#[hax_lib::requires(x > int!(0) && x < int!(16))]
fn math_integers(x: Int) -> u8 {
    let _: Int = 3usize.lift();
    let _neg_dec = int!(-340282366920938463463374607431768211455000);
    let _pos_dec = int!(340282366920938463463374607431768211455000);
    let _neg_hex = int!(-0x3E7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFC18);
    let _pos_hex = int!(0x3E7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFC18);
    let _neg_octal = int!(-0o7637777777777777777777777777777777777777776030);
    let _pos_octal = int!(0o7637777777777777777777777777777777777777776030);
    let _neg_bin = int!(-0b111110011111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111110000011000);
    let _pos_bin = int!(0b111110011111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111110000011000);
    let _ = int!(-340282366920938463463374607431768211455000)
        > int!(340282366920938463463374607431768211455000);
    let _ = x < x;
    let _ = x >= x;
    let _ = x <= x;
    let _ = x != x;
    let _ = x == x;
    let _ = x + x;
    let _ = x - x;
    let _ = x * x;
    let _ = x / x;
    let _: i16 = x.to_i16();
    let _: i32 = x.to_i32();
    let _: i64 = x.to_i64();
    let _: i128 = x.to_i128();
    let _: isize = x.to_isize();
    let _: u16 = x.to_u16();
    let _: u32 = x.to_u32();
    let _: u64 = x.to_u64();
    let _: u128 = x.to_u128();
    let _: usize = x.to_usize();
    (x + x * x).to_u8()
}

pub fn panic_with_msg() {
    panic!("with msg")
}

#[derive(PartialEq, Eq)]
struct Foo {
    field: u8,
}

const CONSTANT: Foo = Foo { field: 3 };

fn numeric() {
    let _: usize = 123;
    let _: isize = -42;
    let _: isize = 42;
    let _: i32 = -42;
    let _: u128 = 22222222222222222222;
}

/// @fail(extraction): ssprove(HAX0001)
pub fn patterns() {
    match 1u8 {
        2 => (),
        _ => (),
    };
    match ("hello", (123, ["a", "b"])) {
        ("hello", (123, _todo)) => (),
        _ => (),
    };
    match (Foo { field: 4 }) {
        CONSTANT => (), // Note [CONSTANT] is not a free variable here, we're really matching against the *value* of CONSTANT
        _ => (),
    };
}

fn casts(x8: u8, x16: u16, x32: u32, x64: u64, xs: usize) {
    let _: u64 = x8 as u64 + x16 as u64 + x32 as u64 + x64 as u64 + xs as u64;
    let _: u32 = x8 as u32 + x16 as u32 + x32 as u32 + x64 as u32 + xs as u32;
    let _: u16 = x8 as u16 + x16 as u16 + x32 as u16 + x64 as u16 + xs as u16;
    let _: u8 = x8 as u8 + x16 as u8 + x32 as u8 + x64 as u8 + xs as u8;
    let _: i64 = x8 as i64 + x16 as i64 + x32 as i64 + x64 as i64 + xs as i64;
    let _: i32 = x8 as i32 + x16 as i32 + x32 as i32 + x64 as i32 + xs as i32;
    let _: i16 = x8 as i16 + x16 as i16 + x32 as i16 + x64 as i16 + xs as i16;
    let _: i8 = x8 as i8 + x16 as i8 + x32 as i8 + x64 as i8 + xs as i8;
}

pub fn empty_array() {
    let _: &[u8] = &[];
}

/// https://github.com/pereq-dev/hax/issues/500
fn fn_pointer_cast() {
    let f: fn(&u32) -> &u32 = |x| x;
}

/// @fail(extraction): ssprove(HAX0001)
const null: char = '\0';

fn strings() {
    let _: &str = "hello";
    let _: &str = "hello\"world";
    let _: &str = "it's";
    let _: &str = "back\\slash";
    let _: &str = "line\nbreak";
    let _: &str = "carriage\rreturn";
    let _: &str = "tab\there";
    let _: &str = "null\x00byte";
    let _: &str = "bell\x07char";
    let _: &str = "\x1b[0m";
    let _: &str = "🦀";
}
