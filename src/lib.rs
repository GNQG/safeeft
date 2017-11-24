//! Safe and branchless error-free transformation algorithms for floating point numbers.
//! 
//! ## Abstract
//! Error-free transformation (EFT) algorithms for floating point numbers are those
//! conserve mathematical equality between input and output.
//! For example, `twosum` by D. E. Knuth[1] is an EFT algorithm that conserves
//! summation of two floating point numbers with non-overlapping property:
//! `(a,b) = twosum(x,y), a+b=x+y, 0.5ulp(a) >= b`.
//!
//! ```
//! fn twosum(x: f64, y: f64) -> (f64, f64) {
//!     let sum = x + y;
//!     let tmp = sum - x;
//!     (sum, (x - (sum - tmp)) + (y - tmp))
//! ```
//!
//! But in practical, especially with IEEE 754 Std. floating point arithmetic[4], 
//! there were several cases which broke equality and M. Kashiwagi[3] fixed it using branch.
//! 
//! This crate provides safe and branchless EFT impllementations for `twosum`, `split`
//! and `twoproduct`[2]. The word "safe" means "if output is representable with
//! normal/subnormal floating point number, the algorithm is mathematically correct."
//!
//! ## Accerelation
//! With nightly compiler and x86 cpu supporting `fma`, you can accelerate some algorithms
//! using a `use-fma` feature gate:
//! 
//! `$ RUSTFLAGS='-C target-feature=+fma' cargo build --features use-fma`
//! 
//! ## References
//! 1. D. E. Knuth, "The Art of Computer Programming", vol. 2. Addison-Wesley, Reading, MA, 3rd edition, 1998.
//! 2. T. J. Dekker, "A Floating-Point Technique for Extending the Available Precision", Numer. Math. 18(3), 224-242, 1971.
//! 3. M. Kashiwagi, "Emulation of Rounded Arithmetic in Rounding to Nearest(Japanese only)", NAS2014, 2014.
//! 4. American National Standards Institute and Institute of Electrical and Electronic Engineers, "IEEE Standard for Binary Floating-Point Arithmetic", ANSI/IEEE Standard 754-2008, 2008.

#![cfg_attr(feature = "use-fma", feature(cfg_target_feature,link_llvm_intrinsics))]

mod twosum;
mod split;
mod twoprod;

#[cfg(feature = "use-fma")]
#[cfg(target_feature = "fma")]
mod fma;

pub use twosum::*;
pub use split::*;
pub use twoprod::*;
