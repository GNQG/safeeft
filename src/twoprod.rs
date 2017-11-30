use traits::FloatEFT;
use twosum::fasttwosum;
use split::{split, safesplit_straight};
#[cfg(any(feature = "use-fma", feature = "doc"))]
use fma::{fma, Fma};

#[inline]
pub fn twoproduct<T: FloatEFT>(a: T, b: T) -> (T, T) {
    let prod = a.clone() * b.clone();
    let ((a1, a2), (b1, b2)) = (split(a), split(b));
    (prod.clone(),
     a2.clone() * b2.clone() - (((prod - a1.clone() * b1.clone()) - a1 * b2) - a2 * b1))
}

#[inline]
pub fn safetwoproduct_branch<T: FloatEFT>(a: T, b: T) -> (T, T) {
    let prod = a.clone() * b.clone();
    let ((a1, a2), (b1, b2)) = if a.abs() >= T::one() / (T::min_pos() / T::epsilon()) {
        (split(a * T::epsilon()), split(b / T::epsilon()))
    } else if b.abs() >= (T::min_pos() / T::epsilon()) {
        (split(a / T::epsilon()), split(b * T::epsilon()))
    } else {
        (split(a), split(b))
    };
    let tmp = if prod.abs() > T::base() / T::min_pos() {
        ((prod.clone() / T::base()) - (a1.clone() / T::base()) * b1.clone()) * T::base()
    } else {
        prod.clone() - a1.clone() * b1.clone()
    };
    (prod, a2.clone() * b2.clone() - ((tmp - a1 * b2) - a2 * b1))
}

#[inline]
pub fn safetwoproduct_straight<T: FloatEFT>(a: T, b: T) -> (T, T) {
    let prod = a.clone() * b.clone();
    let ((a1, a2, a3), (b1, b2, b3)) = (safesplit_straight(a.clone()),
                                        safesplit_straight(b.clone()));
    let two_a1b1 = T::base() * (a1.clone() * b1.clone());
    let mid = fasttwosum(prod.clone(), -two_a1b1.clone());
    (prod,
     ((T::base() * T::base() * a2.clone()) * b2.clone() -
      ((((mid.0 - two_a1b1) + mid.1) - (T::base() * T::base() * b2) * a1) -
       (T::base() * T::base() * a2) * b1)) + a * b3 + b * a3)
}

#[cfg(any(feature = "use-fma", feature = "doc"))]
#[inline]
pub fn safetwoproduct_fma<T: FloatEFT + Fma>(a: T, b: T) -> (T, T) {
    let prod = a.clone() * b.clone();
    (prod.clone(), fma(a, b, -prod))
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
        for _ in 0..10000000 {
            let (l, r) = (rng.gen_range::<f64>(2f64.powi(-510), 2f64.powi(510)) *
                          rng.choose(&[1., -1.]).unwrap(),
                          rng.gen_range::<f64>(2f64.powi(-510), 2f64.powi(510)) *
                          rng.choose(&[1., -1.]).unwrap());
            let (a1, b1) = safetwoproduct_branch(l, r);
            let (a2, b2) = safetwoproduct_straight(l, r);
            assert!((a1 == a2) && (b1 == b2));

            #[cfg(feature = "use-fma")]
            {
                let (a2, b2) = safetwoproduct_fma(l, r);
                assert!((a1 == a2) && (b1 == b2));
            }
        }
    }

    #[test]
    fn extreme() {
        let mut rng = rand::thread_rng();
        for _ in 0..10000000 {
            let (l, r) = (rng.gen_range::<f64>(2f64.powi(510), f64::MAX) *
                          rng.choose(&[1., -1.]).unwrap(),
                          rng.gen_range::<f64>(f64::MIN_POSITIVE, 2f64.powi(-510)) *
                          rng.choose(&[1., -1.]).unwrap());
            let (a1, b1) = safetwoproduct_branch(l, r);
            let (a2, b2) = safetwoproduct_straight(l, r);
            assert!((a1 == a2) && (b1 == b2));

            #[cfg(feature = "use-fma")]
            {
                let (a2, b2) = safetwoproduct_fma(l, r);
                assert!((a1 == a2) && (b1 == b2));
            }
        }
    }

    #[test]
    fn corner_case() {
        let (l, r) = (6.929001713869936e+236, 2.5944475251952003e+71);
        let (a1, b1) = safetwoproduct_branch(l, r);
        let (a2, b2) = safetwoproduct_straight(l, r);
        assert!((a1 == a2) && (b1 == b2));

        #[cfg(feature = "use-fma")]
        {
            let (a2, b2) = safetwoproduct_fma(l, r);
            assert!((a1 == a2) && (b1 == b2));
        }
    }

    #[test]
    fn subnormal_without_underflow() {
        let mut rng = rand::thread_rng();
        for _ in 0..10000000 {
            let (l, r) = ((rng.gen_range::<i64>(-0x1F_FFFF_FFFF_FFFF, 0x20_0000_0000_0000) as
                           f64) * 2f64.powi(-1022) * 2f64.powi(-52),
                          rng.gen_range::<f64>(2f64.powi(53), f64::MAX) *
                          rng.choose(&[1., -1.]).unwrap());
            let (a1, b1) = safetwoproduct_branch(l, r);
            let (a2, b2) = safetwoproduct_straight(l, r);
            assert!((a1 == a2) && (b1 == b2));

            #[cfg(feature = "use-fma")]
            {
                let (a2, b2) = safetwoproduct_fma(l, r);
                assert!((a1 == a2) && (b1 == b2));
            }
        }
    }
}
