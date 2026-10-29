use anodized::spec;

// Specs on functions

#[spec(requires: x > 0)]
fn f1(x: u8) {}


/// Reference for comparison
#[hax_lib::requires(x > 0)]
fn f2(x: u8) {}

#[spec(ensures: |output| output == x)]
fn f3(x: u8) -> u8 {
    x
}
#[spec(ensures: |res| res == x)]
fn f4(x: u8) -> u8 {
    x
}

/// Reference for comparison
#[hax_lib::ensures(|res| res == x)]
fn f5(x: u8) -> u8 {
    x
}
