//! @off: fstar, lean, coq, ssprove, legacy-lean
//!
//! Every integer type's byte conversions and `MAX`, whose `core::num` impl
//! numbers `primitives.pvl` names.
fn bytes_u8(x: u8) -> (u8, u8, u8) {
    (u8::from_be_bytes(x.to_be_bytes()), u8::from_le_bytes(x.to_le_bytes()), u8::MAX)
}
fn bytes_u16(x: u16) -> (u16, u16, u16) {
    (u16::from_be_bytes(x.to_be_bytes()), u16::from_le_bytes(x.to_le_bytes()), u16::MAX)
}
fn bytes_u32(x: u32) -> (u32, u32, u32) {
    (u32::from_be_bytes(x.to_be_bytes()), u32::from_le_bytes(x.to_le_bytes()), u32::MAX)
}
fn bytes_u64(x: u64) -> (u64, u64, u64) {
    (u64::from_be_bytes(x.to_be_bytes()), u64::from_le_bytes(x.to_le_bytes()), u64::MAX)
}
fn bytes_u128(x: u128) -> (u128, u128, u128) {
    (u128::from_be_bytes(x.to_be_bytes()), u128::from_le_bytes(x.to_le_bytes()), u128::MAX)
}
fn bytes_usize(x: usize) -> (usize, usize, usize) {
    (usize::from_be_bytes(x.to_be_bytes()), usize::from_le_bytes(x.to_le_bytes()), usize::MAX)
}
fn bytes_i8(x: i8) -> (i8, i8, i8) {
    (i8::from_be_bytes(x.to_be_bytes()), i8::from_le_bytes(x.to_le_bytes()), i8::MAX)
}
fn bytes_i16(x: i16) -> (i16, i16, i16) {
    (i16::from_be_bytes(x.to_be_bytes()), i16::from_le_bytes(x.to_le_bytes()), i16::MAX)
}
fn bytes_i32(x: i32) -> (i32, i32, i32) {
    (i32::from_be_bytes(x.to_be_bytes()), i32::from_le_bytes(x.to_le_bytes()), i32::MAX)
}
fn bytes_i64(x: i64) -> (i64, i64, i64) {
    (i64::from_be_bytes(x.to_be_bytes()), i64::from_le_bytes(x.to_le_bytes()), i64::MAX)
}
fn bytes_i128(x: i128) -> (i128, i128, i128) {
    (i128::from_be_bytes(x.to_be_bytes()), i128::from_le_bytes(x.to_le_bytes()), i128::MAX)
}
fn bytes_isize(x: isize) -> (isize, isize, isize) {
    (isize::from_be_bytes(x.to_be_bytes()), isize::from_le_bytes(x.to_le_bytes()), isize::MAX)
}

fn min_i8() -> i8 {
    i8::MIN
}
