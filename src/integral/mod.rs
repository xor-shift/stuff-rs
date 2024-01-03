pub mod convert;
pub mod float;
pub mod ops;

pub use convert::*;
pub use float::*;
pub use ops::bit::*;
pub use ops::checked::*;
pub use ops::wrapping::*;

use std::ops::*;

pub trait Integral
: Sized + Clone + Copy
+ PartialEq + Eq + PartialOrd + Ord
+ std::fmt::Debug
+ crate::numeric_limits::NumericLimits
+ ZeroAndOne

+ CheckedAdd<Output=Self> + CheckedSub<Output=Self>
+ CheckedMul<Output=Self> + CheckedDiv<Output=Self>
+ CheckedShr<Output=Self> + CheckedShl<Output=Self>
+ WrappingAdd<Output=Self> + WrappingSub<Output=Self>
+ WrappingMul<Output=Self> + WrappingDiv<Output=Self>
+ WrappingShr<Output=Self> + WrappingShl<Output=Self>

+ Add<Output=Self> + Sub<Output=Self>
+ Mul<Output=Self> + Div<Output=Self>
+ BitAnd<Output=Self> + BitOr<Output=Self> + BitXor<Output=Self>
+ Shr<u32, Output=Self> + Shl<u32, Output=Self>

+ FromScalar<u8> + ToScalar<u8>
+ FromScalar<i8> + ToScalar<i8>
+ FromScalar<u16> + ToScalar<u16>
+ FromScalar<i16> + ToScalar<i16>
+ FromScalar<u32> + ToScalar<u32>
+ FromScalar<i32> + ToScalar<i32>
+ FromScalar<u64> + ToScalar<u64>
+ FromScalar<i64> + ToScalar<i64>
+ FromScalar<u128> + ToScalar<u128>
+ FromScalar<i128> + ToScalar<i128>
+ FromScalar<usize> + ToScalar<usize>
+ FromScalar<isize> + ToScalar<isize>

+ 'static
{}

pub trait UnsignedIntegral: Integral + BitFiddlable {}

pub trait SignedIntegral: Integral {}

impl Integral for u8 {}

impl UnsignedIntegral for u8 {}

impl Integral for i8 {}

impl SignedIntegral for i8 {}

impl Integral for u16 {}

impl UnsignedIntegral for u16 {}

impl Integral for i16 {}

impl SignedIntegral for i16 {}

impl Integral for u32 {}

impl UnsignedIntegral for u32 {}

impl Integral for i32 {}

impl SignedIntegral for i32 {}

impl Integral for u64 {}

impl UnsignedIntegral for u64 {}

impl Integral for i64 {}

impl SignedIntegral for i64 {}

impl Integral for u128 {}

impl UnsignedIntegral for u128 {}

impl Integral for i128 {}

impl SignedIntegral for i128 {}

impl Integral for usize {}

impl UnsignedIntegral for usize {}

impl Integral for isize {}

impl SignedIntegral for isize {}
