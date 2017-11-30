extern crate num_traits;
use core::ops::{Neg, Add, Sub, Mul, Div};
use core::cmp::PartialOrd;
use core::marker::Sized;
use self::num_traits::Float;

pub trait FloatEFT
    : Neg<Output = Self> + Add<Self, Output = Self> + Sub<Self, Output = Self> +
    Mul<Self, Output = Self> + Div<Self, Output = Self> +
    PartialOrd + Clone + Sized {
        fn abs(&self) -> Self;
        fn split_coef() -> Self;
        fn epsilon() -> Self;
        fn min_pos() -> Self;
        fn one() -> Self;
        fn base() -> Self;
}

impl<T: Float> FloatEFT for T {
    #[inline]
    fn abs(&self) -> Self {
        self.clone().abs()
    }
    #[inline]
    fn split_coef() -> Self {
        ((Self::one() / Self::epsilon()) * Self::base() * Self::base()).sqrt() + Self::one()
    }
    #[inline]
    fn epsilon() -> Self {
        Self::epsilon()
    }
    #[inline]
    fn min_pos() -> Self {
        Self::min_positive_value()
    }
    #[inline]
    fn one() -> Self {
        Self::one()
    }
    #[inline]
    fn base() -> Self {
        Self::one() + Self::one()
    }
}
