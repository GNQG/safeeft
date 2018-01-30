#![feature(test)]
#![cfg_attr(feature = "use-fma", feature(cfg_target_feature))]

extern crate test;
extern crate safeeft;
extern crate rand;

use safeeft::{twosum, safetwosum_branch, safetwosum_straight};
#[cfg(feature = "use-fma")]
use safeeft::safetwosum_fma;
use rand::Rng;

fn gen_f64(rng: &mut rand::ThreadRng) -> f64 {
    (rng.next_f64()+1.) * 2f64.powi(rng.gen_range(-25, 25))
        * ((rng.gen_range::<i16>(0, 2) * 2 - 1) as f64)
}

#[bench]
fn bench_twosum(b: &mut test::Bencher) {
    let mut rng = rand::thread_rng();
    let mut l = [0.; 10000];
    let mut r = [0.; 10000];
    for i in 0..10000 {
        l[i] = gen_f64(&mut rng);
        r[i] = gen_f64(&mut rng);
    }

    b.iter(|| for (f1, f2) in l.into_iter().zip(r.into_iter()) {
               test::black_box(twosum(*f1, *f2));
           })
}

#[bench]
fn bench_safetwosum_branch(b: &mut test::Bencher) {
    let mut rng = rand::thread_rng();
    let mut l = [0.; 10000];
    let mut r = [0.; 10000];
    for i in 0..10000 {
        l[i] = gen_f64(&mut rng);
        r[i] = gen_f64(&mut rng);
    }

    b.iter(|| for (f1, f2) in l.into_iter().zip(r.into_iter()) {
               test::black_box(safetwosum_branch(*f1, *f2));
           })
}

#[bench]
fn bench_safetwosum_straight(b: &mut test::Bencher) {
    let mut rng = rand::thread_rng();
    let mut l = [0.; 10000];
    let mut r = [0.; 10000];
    for i in 0..10000 {
        l[i] = gen_f64(&mut rng);
        r[i] = gen_f64(&mut rng);
    }

    b.iter(|| for (f1, f2) in l.into_iter().zip(r.into_iter()) {
               test::black_box(safetwosum_straight(*f1, *f2));
           })
}

#[cfg(feature = "use-fma")]
#[bench]
fn bench_safetwosum_fma(b: &mut test::Bencher) {
    let mut rng = rand::thread_rng();
    let mut l = [0.; 10000];
    let mut r = [0.; 10000];
    for i in 0..10000 {
        l[i] = gen_f64(&mut rng);
        r[i] = gen_f64(&mut rng);
    }

    b.iter(|| for (f1, f2) in l.into_iter().zip(r.into_iter()) {
               test::black_box(safetwosum_fma(*f1, *f2));
           })
}
