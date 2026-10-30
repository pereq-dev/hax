//! @fail(tc): proverif(2)
//! @fail(tc): fstar(2), legacy-lean(1)
#![feature(test)]
//@ edition: 2021
//@ compile-flags: --test

extern crate test;

#[bench]
fn my_bench(_b: &mut test::Bencher) {}
