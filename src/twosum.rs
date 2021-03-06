use traits::FloatEFT;
#[cfg(any(feature = "use-fma", feature = "doc"))]
use fma::{fma, Fma};

#[inline]
pub fn fasttwosum<T: FloatEFT>(x: T, y: T) -> (T, T) {
    let sum = x.clone() + y.clone();
    (sum.clone(), y - (sum - x))
}

#[inline]
pub fn twosum<T: FloatEFT>(x: T, y: T) -> (T, T) {
    let sum = x.clone() + y.clone();
    let tmp = sum.clone() - x.clone();
    (sum.clone(), (x - (sum.clone() - tmp.clone())) + (y - tmp))
}

#[inline]
pub fn safetwosum_branch<T: FloatEFT>(x: T, y: T) -> (T, T) {
    if x.clone().abs() > y.clone().abs() {
        fasttwosum(x, y)
    } else {
        fasttwosum(y, x)
    }
}

#[inline]
pub fn safetwosum_straight<T: FloatEFT>(x: T, y: T) -> (T, T) {
    let s = x.clone() + y.clone();
    let (xx, yy) = (x.clone() / T::radix(), y.clone() / T::radix()); // if uls(x)==eta, xx=eta
    let err_uf = (x - xx.clone() * T::radix()) + (y - yy.clone() * T::radix()); // all operations are exact. 0 <= |err_uf| <= 2eta
    let (ss, ee) = twosum(xx, yy); // this does not overflow if |x|, |y| < inf
    // 2 * (ss + ee) + err_uf = x + y
    let err_h = T::radix() * ss - s.clone(); // exact
    // err_h + 2. * ee + err_uf = x + y - s
    (s, (T::radix() * ee + err_h) + err_uf)
}

#[cfg(any(feature = "use-fma", feature = "doc"))]
#[inline]
pub fn safetwosum_fma<T: FloatEFT + Fma>(x: T, y: T) -> (T, T) {
    let s = x.clone() + y.clone();
    let (xx, yy) = (x.clone() / T::radix(), y.clone() / T::radix()); // if uls(x)==eta, xx=eta
    let err_uf = fma(-T::radix(), xx.clone(), x) + fma(-T::radix(), yy.clone(), y);
    let (ss, ee) = twosum(xx, yy); // this does not overflow if |x|, |y| < inf
    // 2 * (ss + ee) + err_uf = x + y
    let err_h = fma(T::radix(), ss, -s.clone()); // exact
    // err_h + 2. * ee + err_uf = x + y - s
    (s, fma(T::radix(), ee, err_h) + err_uf)
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use std::f64;
    use self::rand::Rng;

    use super::*;

    #[test]
    fn normal() {
        let mut rng = rand::thread_rng();
        for _ in 0..100000000 {
            let (l, r) = (rng.gen::<f64>(), rng.gen::<f64>());
            let (a1, b1) = safetwosum_branch(l, r);
            let (a2, b2) = safetwosum_straight(l, r);
            assert!(((a1 == a2) && (b1 == b2)) || ((a1 == a2) && a1.is_infinite()));

            #[cfg(feature = "use-fma")]
            {
                let (a2, b2) = safetwosum_fma(l, r);
                assert!(((a1 == a2) && (b1 == b2)) || ((a1 == a2) && a1.is_infinite()));
            }
        }
    }

    #[test]
    fn large() {
        let mut rng = rand::thread_rng();
        for _ in 0..10000000 {
            let (l, r) =
                (rng.gen_range::<f64>(2f64.powi(1020), f64::MAX) * rng.choose(&[1., -1.]).unwrap(),
                 rng.gen_range::<f64>(2f64.powi(1020), f64::MAX) * rng.choose(&[1., -1.]).unwrap());
            let (a1, b1) = safetwosum_branch(l, r);
            let (a2, b2) = safetwosum_straight(l, r);
            assert!(((a1 == a2) && (b1 == b2)) || ((a1 == a2) && a1.is_infinite()));

            #[cfg(feature = "use-fma")]
            {
                let (a2, b2) = safetwosum_fma(l, r);
                assert!(((a1 == a2) && (b1 == b2)) || ((a1 == a2) && a1.is_infinite()));
            }
        }
    }

    #[test]
    fn subnormal() {
        let mut rng = rand::thread_rng();
        for _ in 0..10000000 {
            let (l, r) = ((rng.gen_range::<i64>(-0x1F_FFFF_FFFF_FFFF, 0x20_0000_0000_0000) as
                           f64) * 2f64.powi(-1022) * 2f64.powi(-52),
                          rng.gen::<f64>());
            let (a1, b1) = safetwosum_branch(l, r);
            let (a2, b2) = safetwosum_straight(l, r);
            assert!((a1 == a2) && (b1 == b2));

            #[cfg(feature = "use-fma")]
            {
                let (a2, b2) = safetwosum_fma(l, r);
                assert!((a1 == a2) && (b1 == b2));
            }
        }
    }

    #[test]
    fn corner_case() {
        let res1 = safetwosum_straight(3.5630624444874539e+307, -1.7976931348623157e+308);
        assert!(!(res1.1 as f64).is_nan());

        #[cfg(feature = "use-fma")]
        {
            let res2 = safetwosum_fma(3.5630624444874539e+307, -1.7976931348623157e+308);
            assert!(res1 == res2);
        }
    }
}