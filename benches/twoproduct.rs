#![feature(test)]
#![cfg_attr(feature = "use-fma", feature(cfg_target_feature))]

extern crate test;
extern crate safeeft;
extern crate rand;

use safeeft::{twoproduct, safetwoproduct_branch, safetwoproduct_straight};
#[cfg(feature = "use-fma")]
use safeeft::safetwoproduct_fma;
use rand::Rng;

fn gen_f64(rng: &mut rand::ThreadRng) -> f64 {
    (rng.next_f64()+1.) * 2f64.powi(rng.gen_range(-25, 25))
        * ((rng.gen_range::<i16>(0, 2) * 2 - 1) as f64)
}

#[bench]
fn bench_twoproduct(b: &mut test::Bencher) {
    let mut rng = rand::thread_rng();
    let mut l = [0.; 10000];
    let mut r = [0.; 10000];
    for i in 0..10000 {
        l[i] = gen_f64(&mut rng);
        r[i] = gen_f64(&mut rng);
    }

    b.iter(|| for (f1, f2) in l.into_iter().zip(r.into_iter()) {
               test::black_box(twoproduct(*f1, *f2));
           })
}

#[bench]
fn bench_safetwoproduct_branch(b: &mut test::Bencher) {
    let mut rng = rand::thread_rng();
    let mut l = [0.; 10000];
    let mut r = [0.; 10000];
    for i in 0..10000 {
        l[i] = gen_f64(&mut rng);
        r[i] = gen_f64(&mut rng);
    }

    b.iter(|| for (f1, f2) in l.into_iter().zip(r.into_iter()) {
               test::black_box(safetwoproduct_branch(*f1, *f2));
           })
}

#[bench]
fn bench_safetwoproduct_straight(b: &mut test::Bencher) {
    let mut rng = rand::thread_rng();
    let mut l = [0.; 10000];
    let mut r = [0.; 10000];
    for i in 0..10000 {
        l[i] = gen_f64(&mut rng);
        r[i] = gen_f64(&mut rng);
    }

    b.iter(|| for (f1, f2) in l.into_iter().zip(r.into_iter()) {
               test::black_box(safetwoproduct_straight(*f1, *f2));
           })
}

#[cfg(feature = "use-fma")]
#[bench]
fn bench_safetwoproduct_fma(b: &mut test::Bencher) {
    let mut rng = rand::thread_rng();
    let mut l = [0.; 10000];
    let mut r = [0.; 10000];
    for i in 0..10000 {
        l[i] = gen_f64(&mut rng);
        r[i] = gen_f64(&mut rng);
    }

    b.iter(|| for (f1, f2) in l.into_iter().zip(r.into_iter()) {
               test::black_box(safetwoproduct_fma(*f1, *f2));
           })
}
