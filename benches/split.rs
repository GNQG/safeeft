#![feature(test)]

extern crate test;
extern crate safeeft;
extern crate rand;

use safeeft::{split, safesplit_branch, safesplit_straight};
use rand::Rng;


#[bench]
fn bench_split(b: &mut test::Bencher) {
    let mut rng = rand::thread_rng();
    let mut a = [0.; 10000];
    for f in &mut a[..] {
        *f = rng.gen();
    }

    b.iter(|| for f in a.into_iter() {
               test::black_box(split(*f));
           })
}

#[bench]
fn bench_safesplit_branch(b: &mut test::Bencher) {
    let mut rng = rand::thread_rng();
    let mut a = [0.; 10000];
    for f in &mut a[..] {
        *f = rng.gen();
    }

    b.iter(|| for f in a.into_iter() {
               test::black_box(safesplit_branch(*f));
           })
}

#[bench]
fn bench_safesplit_straight(b: &mut test::Bencher) {
    let mut rng = rand::thread_rng();
    let mut a = [0.; 10000];
    for f in &mut a[..] {
        *f = rng.gen();
    }

    b.iter(|| for f in a.into_iter() {
               test::black_box(safesplit_straight(*f));
           })
}
