use crate::*;

pub trait FloatConstants {
    type FloatType: Sized + Copy + Clone;

    const PI: Self::FloatType;
    const TAU: Self::FloatType;
    const PHI: Self::FloatType;
    const EGAMMA: Self::FloatType;
    const FRAC_PI_2: Self::FloatType;
    const FRAC_PI_3: Self::FloatType;
    const FRAC_PI_4: Self::FloatType;
    const FRAC_PI_6: Self::FloatType;
    const FRAC_PI_8: Self::FloatType;
    const FRAC_1_PI: Self::FloatType;
    const FRAC_1_SQRT_PI: Self::FloatType;
    const FRAC_2_PI: Self::FloatType;
    const FRAC_2_SQRT_PI: Self::FloatType;
    const SQRT_2: Self::FloatType;
    const FRAC_1_SQRT_2: Self::FloatType;
    const SQRT_3: Self::FloatType;
    const FRAC_1_SQRT_3: Self::FloatType;
    const E: Self::FloatType;
    const LOG2_10: Self::FloatType;
    const LOG2_E: Self::FloatType;
    const LOG10_2: Self::FloatType;
    const LOG10_E: Self::FloatType;
    const LN_2: Self::FloatType;
    const LN_10: Self::FloatType;
}

macro_rules! impl_float_constants {
    ($for_type:ident) => {
        impl FloatConstants for $for_type {
            type FloatType = $for_type;

            const PI: Self::FloatType = core::$for_type::consts::PI;
            const TAU: Self::FloatType = core::$for_type::consts::TAU;
            const PHI: Self::FloatType = core::$for_type::consts::PHI;
            const EGAMMA: Self::FloatType = core::$for_type::consts::EGAMMA;
            const FRAC_PI_2: Self::FloatType = core::$for_type::consts::FRAC_PI_2;
            const FRAC_PI_3: Self::FloatType = core::$for_type::consts::FRAC_PI_3;
            const FRAC_PI_4: Self::FloatType = core::$for_type::consts::FRAC_PI_4;
            const FRAC_PI_6: Self::FloatType = core::$for_type::consts::FRAC_PI_6;
            const FRAC_PI_8: Self::FloatType = core::$for_type::consts::FRAC_PI_8;
            const FRAC_1_PI: Self::FloatType = core::$for_type::consts::FRAC_1_PI;
            const FRAC_1_SQRT_PI: Self::FloatType = core::$for_type::consts::FRAC_1_SQRT_PI;
            const FRAC_2_PI: Self::FloatType = core::$for_type::consts::FRAC_2_PI;
            const FRAC_2_SQRT_PI: Self::FloatType = core::$for_type::consts::FRAC_2_SQRT_PI;
            const SQRT_2: Self::FloatType = core::$for_type::consts::SQRT_2;
            const FRAC_1_SQRT_2: Self::FloatType = core::$for_type::consts::FRAC_1_SQRT_2;
            const SQRT_3: Self::FloatType = core::$for_type::consts::SQRT_3;
            const FRAC_1_SQRT_3: Self::FloatType = core::$for_type::consts::FRAC_1_SQRT_3;
            const E: Self::FloatType = core::$for_type::consts::E;
            const LOG2_10: Self::FloatType = core::$for_type::consts::LOG2_10;
            const LOG2_E: Self::FloatType = core::$for_type::consts::LOG2_E;
            const LOG10_2: Self::FloatType = core::$for_type::consts::LOG10_2;
            const LOG10_E: Self::FloatType = core::$for_type::consts::LOG10_E;
            const LN_2: Self::FloatType = core::$for_type::consts::LN_2;
            const LN_10: Self::FloatType = core::$for_type::consts::LN_10;
        }
    };
}

impl_float_constants!(f32);
impl_float_constants!(f64);

pub trait FloatingPoint:
    Copy
    + Clone
    + FloatConstants<FloatType = Self>
    + ZeroAndOne
    + FromScalar<u8>
    + ToScalar<u8>
    + FromScalar<i8>
    + ToScalar<i8>
    + FromScalar<u16>
    + ToScalar<u16>
    + FromScalar<i16>
    + ToScalar<i16>
    + FromScalar<u32>
    + ToScalar<u32>
    + FromScalar<i32>
    + ToScalar<i32>
    + FromScalar<u64>
    + ToScalar<u64>
    + FromScalar<i64>
    + ToScalar<i64>
    + FromScalar<u128>
    + ToScalar<u128>
    + FromScalar<i128>
    + ToScalar<i128>
    + FromScalar<usize>
    + ToScalar<usize>
    + FromScalar<isize>
    + ToScalar<isize>
    + FromScalar<f32>
    + ToScalar<f32>
    + FromScalar<f64>
    + ToScalar<f64>
    + std::ops::Add<Self, Output = Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Div<Self, Output = Self>
    + std::ops::Neg<Output = Self>
    + PartialEq
    + PartialOrd
    + 'static
{
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn copysign(self, sign: Self) -> Self;

    fn mul_add(self, a: Self, b: Self) -> Self;
    fn div_euclid(self, rhs: Self) -> Self;
    fn rem_euclid(self, rhs: Self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
    fn cbrt(self) -> Self;
    fn hypot(self, other: Self) -> Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, rhs: Self) -> Self;
    fn sin_cos(self) -> (Self, Self) { (self.sin(), self.cos()) }

    #[rustfmt::skip]
    fn min(self, other: Self) -> Self { if self < other { self } else { other } }

    #[rustfmt::skip]
    fn max(self, other: Self) -> Self { if self < other { other } else { self } }

    #[rustfmt::skip]
    fn clamp(self, min: Self, max: Self) -> Self { self.max(min).min(max) }
}

macro_rules! impl_lossless_from_conversions {
    ($for_type:ty: ) => {};

    ($for_type:ty: $from_type:ty $(, $rest_of_the_types:ty)*) => {
        impl FromScalar<$from_type> for $for_type {
            fn from_scalar(v: $from_type) -> Option<Self> { Some(From::from(v)) }
            fn as_from(v: $from_type) -> Self { v as _ }
        }
        impl_lossless_from_conversions!($for_type: $($rest_of_the_types),*);
    }
}

macro_rules! impl_as_from_conversions {
    ($for_type:ty: ) => {};

    ($for_type:ty: $from_type:ty $(, $rest_of_the_types:ty)*) => {
        impl FromScalar<$from_type> for $for_type {
            fn from_scalar(v: $from_type) -> Option<Self> { Some(v as _) }
            fn as_from(v: $from_type) -> Self { v as _ }
        }
        impl_as_from_conversions!($for_type: $($rest_of_the_types),*);
    }
}

macro_rules! impl_as_to_conversions {
    ($for_type:ty: ) => {};

    ($for_type:ty: $to_type:ty $(, $rest_of_the_types: ty)*) => {
        impl ToScalar<$to_type> for $for_type {
            fn to_scalar(self) -> Option<$to_type> { Some(self as _) }
            fn as_to(self) -> $to_type { self as _ }
        }
        impl_as_to_conversions!($for_type: $($rest_of_the_types),*);
    }
}

macro_rules! impl_float_passthrough {
    ($for_type:ty) => {
        impl FloatingPoint for $for_type {
            fn floor(self) -> Self { Self::floor(self) }
            fn ceil(self) -> Self { Self::ceil(self) }
            fn round(self) -> Self { Self::round(self) }
            fn trunc(self) -> Self { Self::trunc(self) }
            fn fract(self) -> Self { Self::fract(self) }
            fn abs(self) -> Self { Self::abs(self) }
            fn signum(self) -> Self { Self::signum(self) }
            fn copysign(self, sign: Self) -> Self { Self::copysign(self, sign) }

            fn mul_add(self, a: Self, b: Self) -> Self { Self::mul_add(self, a, b) }
            fn div_euclid(self, rhs: Self) -> Self { Self::div_euclid(self, rhs) }
            fn rem_euclid(self, rhs: Self) -> Self { Self::rem_euclid(self, rhs) }
            fn powi(self, n: i32) -> Self { Self::powi(self, n) }
            fn powf(self, n: Self) -> Self { Self::powf(self, n) }
            fn sqrt(self) -> Self { Self::sqrt(self) }
            fn exp(self) -> Self { Self::exp(self) }
            fn exp2(self) -> Self { Self::exp2(self) }
            fn ln(self) -> Self { Self::ln(self) }
            fn log(self, base: Self) -> Self { Self::log(self, base) }
            fn log2(self) -> Self { Self::log2(self) }
            fn log10(self) -> Self { Self::log10(self) }
            fn cbrt(self) -> Self { Self::cbrt(self) }
            fn hypot(self, other: Self) -> Self { Self::hypot(self, other) }

            fn sin(self) -> Self { Self::sin(self) }
            fn cos(self) -> Self { Self::cos(self) }
            fn tan(self) -> Self { Self::tan(self) }
            fn asin(self) -> Self { Self::asin(self) }
            fn acos(self) -> Self { Self::acos(self) }
            fn atan(self) -> Self { Self::atan(self) }
            fn atan2(self, rhs: Self) -> Self { Self::atan2(self, rhs) }
        }
    };
}

impl_lossless_from_conversions!(f32: u8, i8, u16, i16);
impl_as_from_conversions!(f32: u32, i32, u64, i64, u128, i128, usize, isize);
impl_as_to_conversions!(f32: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

impl_lossless_from_conversions!(f64: u8, i8, u16, i16, u32, i32);
impl_as_from_conversions!(f64: u64, i64, u128, i128, usize, isize);
impl_as_to_conversions!(f64: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

impl FromScalar<f32> for f32 {
    fn from_scalar(v: f32) -> Option<Self> { Some(v) }
    fn as_from(v: f32) -> Self { v as _ }
}

impl FromScalar<f64> for f32 {
    fn from_scalar(v: f64) -> Option<Self> { Some(v as _) }
    fn as_from(v: f64) -> Self { v as _ }
}

impl ToScalar<f32> for f32 {
    fn to_scalar(self) -> Option<f32> { Some(self) }
    fn as_to(self) -> f32 { self as _ }
}

impl ToScalar<f64> for f32 {
    fn to_scalar(self) -> Option<f64> { Some(From::from(self)) }
    fn as_to(self) -> f64 { self as _ }
}

impl ZeroAndOne for f32 {
    fn zero() -> Self { 0f32 }
    fn one() -> Self { 1f32 }
}

impl_float_passthrough!(f32);

impl FromScalar<f32> for f64 {
    fn from_scalar(v: f32) -> Option<Self> { Some(From::from(v)) }
    fn as_from(v: f32) -> Self { v as _ }
}

impl FromScalar<f64> for f64 {
    fn from_scalar(v: f64) -> Option<Self> { Some(v) }
    fn as_from(v: f64) -> Self { v as _ }
}

impl ToScalar<f32> for f64 {
    fn to_scalar(self) -> Option<f32> { Some(self as _) }
    fn as_to(self) -> f32 { self as _ }
}

impl ToScalar<f64> for f64 {
    fn to_scalar(self) -> Option<f64> { Some(self) }
    fn as_to(self) -> f64 { self as _ }
}

impl ZeroAndOne for f64 {
    fn zero() -> Self { 0f64 }
    fn one() -> Self { 1f64 }
}

impl_float_passthrough!(f64);
