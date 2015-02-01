#![deny(warnings)]
#![feature(test)]

extern crate shove;
extern crate test;

use test::Bencher;

#[bench]
fn foo(_bencher: &mut Bencher) {
    3 + 7;
}
