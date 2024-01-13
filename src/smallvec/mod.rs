mod matrix;
mod vector;

pub use matrix::*;
pub use vector::*;

use crate::FloatingPoint;

pub trait Number //
    : Sized + Copy + Clone
    + crate::ZeroAndOne
    + PartialOrd
    + std::ops::Neg<Output = Self>
    + std::ops::Add<Self, Output = Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Div<Self, Output = Self> {
        fn abs(self) -> Self {
            if self < Self::zero() { -self } else { self }
        }

        fn min(self, other: Self) -> Self {
            if self < other { self } else { other }
        }

        fn max(self, other: Self) -> Self {
            if other < self { self } else { other }
        }

        fn clamp(self, lo: Self, hi: Self) -> Self { self.min(hi).max(lo) }
    }

impl<T: FloatingPoint> Number for T {}

impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
