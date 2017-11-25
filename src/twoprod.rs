use twosum::fasttwosum;
use split::{split, safesplit_straight};
#[cfg(feature = "use-fma")]
#[cfg(target_feature = "fma")]
use fma::fma;

#[inline]
pub fn twoproduct(a: f64, b: f64) -> (f64, f64) {
    let prod = a * b;
    let ((a1, a2), (b1, b2)) = (split(a), split(b));
    (prod, a2 * b2 - (((prod - a1 * b1) - a1 * b2) - a2 * b1))
}

#[inline]
pub fn safetwoproduct_branch(a: f64, b: f64) -> (f64, f64) {
    let prod = a * b;
    let ((a1, a2), (b1, b2)) = if a.abs() >= 2f64.powi(996) {
        (split(a * 2f64.powi(-28)), split(b * 2f64.powi(28)))
    } else if b.abs() >= 2f64.powi(996) {
        (split(a * 2f64.powi(28)), split(b * 2f64.powi(-28)))
    } else {
        (split(a), split(b))
    };
    let tmp = if prod.abs() > 2f64.powi(1023) {
        ((prod * 0.5) - (a1 * 0.5) * b1) * 2.
    } else {
        prod - a1 * b1
    };
    (prod, a2 * b2 - ((tmp - a1 * b2) - a2 * b1))
}

#[inline]
pub fn safetwoproduct_straight(a: f64, b: f64) -> (f64, f64) {
    let prod = a * b;
    let ((a1, a2, a3), (b1, b2, b3)) = (safesplit_straight(a), safesplit_straight(b));
    let two_a1b1 = 2. * (a1 * b1);
    let mid = fasttwosum(prod, -two_a1b1);
    (prod,
     ((4. * a2) * b2 - ((((mid.0 - two_a1b1) + mid.1) - (4. * b2) * a1) - (4. * a2) * b1)) +
     a * b3 + b * a3)
}

#[cfg(any(all(feature = "use-fma",target_feature = "fma"), feature = "doc"))]
#[inline]
pub fn safetwoproduct_fma(a: f64, b: f64) -> (f64, f64) {
    let prod = a * b;
    (prod, fma(a, b, -prod))
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
            #[cfg(target_feature = "fma")]
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
            #[cfg(target_feature = "fma")]
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
        #[cfg(target_feature = "fma")]
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
            #[cfg(target_feature = "fma")]
            {
                let (a2, b2) = safetwoproduct_fma(l, r);
                assert!((a1 == a2) && (b1 == b2));
            }
        }
    }
}
