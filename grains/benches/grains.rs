#![feature(test)]

extern crate test;

use test::Bencher;

fn square1(s: u32) -> u64 {
    if s < 1 || 64 < s {
        panic!("Square must be between 1 and 64")
    }
    2u64.pow(s - 1)
}

#[bench]
fn bench_total1(b: &mut Bencher) {
    b.iter(|| {
        for s in 1..=64 {
            square1(test::black_box(s));
        }
    });
}

fn square2(s: u32) -> u64 {
    if s < 1 || 64 < s {
        panic!("Square must be between 1 and 64")
    }
    1u64 << (s - 1) as u64 // equals to 2u64.pow(s - 1)
}

#[bench]
fn bench_total2(b: &mut Bencher) {
    b.iter(|| {
        for s in 1..=64 {
            square2(test::black_box(s));
        }
    });
}
