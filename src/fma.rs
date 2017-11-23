extern crate simd;

use self::simd::x86::sse2::f64x2;

#[inline]
pub fn fma(a: f64, b: f64, c: f64) -> f64 {
    f64x2::new(a, 0.)
        .fma(f64x2::new(b, 0.), f64x2::new(c, 0.))
        .extract(0)
}