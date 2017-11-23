use std::f64;

#[inline]
pub fn split(a: f64) -> (f64, f64) {
    let tmp = a * (2f64.powi(27) + 1.);
    let x = tmp - (tmp - a);
    (x, a - x)
}

#[inline]
pub fn safesplit_branch(a: f64) -> (f64, f64) {
    // unsafe when usp(a) >= 2^997 (a >= 0x1.FFFFFF8000000p+1022) <-- ?????
    // theoritically split(DOUBLE_MAX) == (succ(DOUBLE_MAX)==2^1023, some negative float)
    if a > 1. {
        let t = split(a * 2f64.powi(-50));
        (t.0 * 2f64.powi(50), t.1 * 2f64.powi(50))
    } else {
        split(a)
    }
}

#[inline]
pub fn safesplit_straight(a: f64) -> (f64, f64, f64) {
    // Returns a_high, a_low, a_err which satisfy a == 2 * a_high + 2 * a_low + a_err.
    // 2 * a_high may overflow, so to get a, you should write a_high + (a_high + (2.*a_low + a_err))
    let aa = a * 0.5;
    let err = a - aa * 2.; // if usp(a) == 2^-1074, err == 2^-1074, else 0.

    let step = (((aa + f64::MIN_POSITIVE) - aa) * 2f64.powi(1000)) * 2f64.powi(200) +
               2f64.powi(-52);
    let split_shift = split(aa * step);

    (split_shift.0 / step, split_shift.1 / step, err)
}

#[cfg(test)]
mod tests {
    extern crate num_traits;
    extern crate rand;

    use std::f64;
    use self::rand::Rng;
    use self::num_traits::Float;

    use super::*;

    #[allow(dead_code)]
    fn is_split(a: f64, b: f64) -> bool {
        let (a_mant, a_expo, _) = a.integer_decode();
        let (b_mant, b_expo, _) = b.integer_decode();

        ((a_expo - b_expo >= 27) && (((a_mant | b_mant) & 0x7FFFFFFu64) == 0)) |
        ((a_expo - b_expo == 26) && ((a_mant & 0x7FFFFFFu64) == 0) && (a_mant % 2 == 0) &&
         (b_mant == 0x10_0000_0000_0000)) |
        ((a_expo == b_expo) && (b_expo == -1022) &&
         (b_mant <= 2u64.pow(a_mant.trailing_zeros() - 1)))
    }

    #[test]
    fn normal() {
        let mut rng = rand::thread_rng();
        for _ in 0..10000000 {
            let fl = rng.gen_range(2f64.powi(-1022 + 27), 1e+308) *
                     (1. - 2. * (rng.gen_range(0, 1) as f64));

            let s1 = split(fl);
            if s1.0.is_nan() {
                continue;
            }
            assert!(s1.0 + s1.1 == fl);

            let s2 = safesplit_straight(fl);

            let (s2h_prop, s2l_prop) = (s2.0.integer_decode(), s2.1.integer_decode());
            assert!((s2h_prop.0 & 0x7FFFFFF) == 0);
            assert!((s2l_prop.0 & 0x7FFFFFF) == 0);
            assert!(s2.2.abs() <= f64::MIN_POSITIVE * 2f64.powi(-52));
            assert!(s2.0.abs() * 2f64.powi(-26) >= s2.1.abs());

            assert_eq!(s1.0, s2.0 * 2.);
            assert_eq!(s1.1, s2.1 * 2.);
            //assert_eq!(s1.0 + s1.1, s2.0 * 2. + s2.1 * 2. + s2.2);
        }
    }

    #[test]
    fn large() {
        let mut rng = rand::thread_rng();
        for _ in 0..10000000 {
            let fl = (rng.gen_range::<i64>(-0x1F_FFFF_FFFF_FFFF, 0x20_0000_0000_0000) as f64) *
                     2f64.powi(1022 - 52);

            let s = safesplit_straight(fl);

            if s.0.is_nan() {
                unreachable!()
            }

            let (sh_prop, sl_prop) = (s.0.integer_decode(), s.1.integer_decode());
            assert!((sh_prop.0 & 0x7FFFFFF) == 0);
            assert!((sl_prop.0 & 0x7FFFFFF) == 0);

            assert_eq!(((s.2 + s.1 * 2.) + s.0) + s.0, fl);
            assert!(s.0.abs() * 2f64.powi(-26) >= s.1.abs());
        }

        let s = safesplit_straight(f64::MAX);

        let (sh_prop, sl_prop) = (s.0.integer_decode(), s.1.integer_decode());
        assert!((sh_prop.0 & 0x7FFFFFF) == 0);
        assert!((sl_prop.0 & 0x7FFFFFF) == 0);

        assert_eq!(((s.2 + s.1 * 2.) + s.0) + s.0, f64::MAX);
        assert!(s.0.abs() * 2f64.powi(-26) >= s.1.abs());
    }

    #[test]
    fn subnormal() {
        let mut rng = rand::thread_rng();
        for _ in 0..10000000 {
            let fl = ((rng.gen_range::<i64>(-0xFFFF_FFFF_FFFF, 0x1_0000_0000_0000) as f64) *
                      2f64.powi(-1022)) * 2f64.powi(-52);
            let s = safesplit_straight(fl);
            assert_eq!(((s.2 + s.1 * 2.) + s.0) + s.0, fl);
            assert!(s.0.abs() * 2f64.powi(-26) >= s.1.abs());
        }
    }
}
