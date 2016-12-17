#![cfg(feature = "bench")]
#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn bench_me_harder(b: &mut Bencher) {
}

