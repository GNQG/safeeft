#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.fma.f64"]
    #[inline]
    fn fma_f64(a: f64, b: f64, c: f64) -> f64;
}

#[inline]
pub fn fma(a: f64, b: f64, c: f64) -> f64 {
    unsafe { fma_f64(a, b, c) }
}