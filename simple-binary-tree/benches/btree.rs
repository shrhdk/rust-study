#![feature(test)]

extern crate test;

use test::Bencher;

use rand::seq::SliceRandom;

use simple_binary_tree::BTreeSet;

const N: usize = 1000;

fn random_numbers(size: usize) -> Vec<usize> {
    let mut v: Vec<usize> = (0..size).collect();
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
    v
}

#[bench]
fn bench_btree_with_random_numbers(b: &mut Bencher) {
    b.iter(|| {
        let mut set = BTreeSet::new();
        for n in random_numbers(N) {
            set.insert(n);
        }
        for n in 0..N {
            assert!(set.contains(&n));
        }
    });
}

#[bench]
fn bench_btree_with_sequential_numbers(b: &mut Bencher) {
    b.iter(|| {
        let mut set = BTreeSet::new();
        for n in 0..N {
            set.insert(n);
        }
        for n in 0..N {
            assert!(set.contains(&n));
        }
    });
}
