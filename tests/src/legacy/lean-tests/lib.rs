//! @fail(extraction): proverif(HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001, HAX0001)
//! @fail(tc): legacy-lean(1)
//! @off: ssprove, fstar, coq
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod array;
pub mod associated_types;
pub mod binops;
pub mod casts;
pub mod comments;
pub mod constants;
pub mod enums;
pub mod floats;
pub mod ite;
pub mod loops;
pub mod matching;
pub mod monadic;
pub mod nested_control_flow;
pub mod opaque;
pub mod recursion;
pub mod specs;
pub mod structs;
pub mod traits;
pub mod types;

const FORTYTWO: usize = 42;
const MINUS_FORTYTWO: isize = -42;

fn returns42() -> usize {
    FORTYTWO
}

fn add_two_numbers(x: usize, y: usize) -> usize {
    x + y
}

fn letBinding(x: usize, y: usize) -> usize {
    let useless = ();
    let result1 = x + y;
    let result2 = result1 + 2;
    result2 + 1
}

fn closure() -> i32 {
    let x = 41;
    let f1 = |y| y + x;
    let f2 = |y, z| y + x + z;
    let res1 = f1(1);
    let res2 = f2(2, 3);
    res1 + res2
}

#[hax_lib::legacy_lean::before("example : Nat := 42")]
fn test_before_verbatime_single_line(x: u8) -> u8 {
    42
}

#[hax_lib::legacy_lean::before(
    "
def multiline : Unit := ()

"
)]
fn test_before_verbatim_multi_line(x: u8) -> u8 {
    32
}

const NULL_CHAR: char = '\0';

/// Test string literals with escape sequences
fn string_escapes() {
    let _empty = "";
    let _plain = "hello world";
    let _with_quotes = "she said \"hello\"";
    let _with_single_quote = "it's fine";
    let _with_backslash = "path\\to\\file";
    let _with_newline = "line1\nline2";
    let _with_tab = "col1\tcol2";
    let _with_carriage_return = "before\rafter";
    let _mixed = "say \"hello\"\nand\t'goodbye'\\end";
    let _carriage_return = "carriage\rreturn";
    let _control_chars = "null\x00byte bell\x07char font\x1b[0mreset";
}
