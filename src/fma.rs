extern "rust-intrinsic" {
    fn fmaf32(a: f32, b: f32, c: f32) -> f32;
    fn fmaf64(a: f64, b: f64, c: f64) -> f64;
}

pub trait Fma {
    fn fma(a: Self, b: Self, c: Self) -> Self;
}

impl Fma for f64 {
    #[inline]
    fn fma(a: f64, b: f64, c: f64) -> f64 {
        unsafe { fmaf64(a, b, c) }
    }
}

impl Fma for f32 {
    #[inline]
    fn fma(a: f32, b: f32, c: f32) -> f32 {
        unsafe { fmaf32(a, b, c) }
    }
}
