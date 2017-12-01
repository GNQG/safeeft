extern crate num_traits;
extern crate float_traits;
use core::clone::Clone;
use self::num_traits::One;
use self::float_traits::{IEEE754Float, BinaryFloat};

pub trait FloatEFT: IEEE754Float + Clone {
    #[inline]
    fn split_coef() -> Self {
        let int_one = <Self as BinaryFloat>::Expo::one();
        let int_two = <Self as BinaryFloat>::Expo::one() + <Self as BinaryFloat>::Expo::one();
        Self::two_powi((Self::bits() + int_one) / int_two) + Self::one()
    }
}

impl<T: IEEE754Float + Clone> FloatEFT for T {}
