#[cfg(feature = "use-fma")]
#[cfg(target_feature = "fma")]
use fma::fma;

#[inline]
pub fn fasttwosum(x: f64, y: f64) -> (f64, f64) {
    let sum = x + y;
    (sum, y - (sum - x))
}

#[inline]
pub fn twosum(x: f64, y: f64) -> (f64, f64) {
    let sum = x + y;
    let tmp = sum - x;
    (sum, (x - (sum - tmp)) + (y - tmp))
}

#[inline]
pub fn safetwosum_branch(x: f64, y: f64) -> (f64, f64) {
    if x.abs() > y.abs() {
        fasttwosum(x, y)
    } else {
        fasttwosum(y, x)
    }
}

#[inline]
pub fn safetwosum_straight(x: f64, y: f64) -> (f64, f64) {
    let (xx, yy) = (x * 0.5, y * 0.5); // if uls(x)==eta, xx=eta
    let err_uf = (x - xx * 2.) + (y - yy * 2.); // all operations are exact. 0 <= |err_uf| <= 2eta
    let (ss, ee) = twosum(xx, yy); // this does not overflow if |x|, |y| < inf
    let (sum, err) = fasttwosum(ss * 2., err_uf); // this is exact because |ss| >= 2eta >= |err_uf| or ss==0
    (sum, ee * 2. + err) // addition is exact
}

#[cfg(any(all(feature = "use-fma",target_feature = "fma"), feature = "doc"))]
#[inline]
pub fn safetwosum_fma(x: f64, y: f64) -> (f64, f64) {
    let (xx, yy) = (x * 0.5, y * 0.5);
    let err_uf = fma(-2., xx, x) + fma(-2., yy, y);
    let (ss, ee) = twosum(xx, yy); // this does not overflow if |x|, |y| < inf
    let sum = fma(2., ss, err_uf);
    let err = err_uf - fma(-2., ss, sum);
    (sum, fma(2., ee, err))
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
            #[cfg(target_feature = "fma")]
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
            #[cfg(target_feature = "fma")]
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
            #[cfg(target_feature = "fma")]
            {
                let (a2, b2) = safetwosum_fma(l, r);
                assert!((a1 == a2) && (b1 == b2));
            }
        }
    }

    #[test]
    fn corner_case() {
        let res1 = safetwosum_straight(3.5630624444874539e+307, -1.7976931348623157e+308);
        assert!(!res1.1.is_nan());

        #[cfg(feature = "use-fma")]
        #[cfg(target_feature = "fma")]
        {
            let res2 = safetwosum_fma(3.5630624444874539e+307, -1.7976931348623157e+308);
            assert!(res1 == res2);
        }
    }
}