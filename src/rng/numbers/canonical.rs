use crate::*;
use crate::rng::*;
use crate::rng::distributions::GenerateCanonical;

/*
pub trait CanonicalFloat
    where
        Self::UnderlyingType: FloatingPoint + GenerateCanonical,
        Self: Sized + Copy + Clone + PartialEq
        + TryFrom<Self::UnderlyingType> + PartialEq<Self::UnderlyingType>
        + core::ops::Add<Self::UnderlyingType, Output=Self::UnderlyingType>
        + core::ops::Sub<Self::UnderlyingType, Output=Self::UnderlyingType>
        + core::ops::Mul<Self::UnderlyingType, Output=Self::UnderlyingType>
        + core::ops::Div<Self::UnderlyingType, Output=Self::UnderlyingType>
        + 'static
{
    type UnderlyingType: FloatingPoint + GenerateCanonical;

    fn floor(self) -> Self;
    // return 0
    fn ceil(self) -> Self::UnderlyingType;
    fn round(self) -> Self::UnderlyingType;
    fn trunc(self) -> Self;
    // return 0
    fn fract(self) -> Self { return self; }
    fn abs(self) -> Self { return self; }

    fn copysign(self, sign: Self::UnderlyingType) -> Self::UnderlyingType;

    fn signum(self) -> Self::UnderlyingType;

    fn mul_add(self, _a: Self::UnderlyingType, _b: Self::UnderlyingType) -> Self::UnderlyingType { todo!() }
    fn div_euclid(self, _rhs: Self::UnderlyingType) -> Self::UnderlyingType { todo!() }
    fn rem_euclid(self, _rhs: Self::UnderlyingType) -> Self::UnderlyingType { todo!() }
    fn powi(self, _n: i32) -> Self::UnderlyingType { todo!() }
    fn powf(self, _n: Self::UnderlyingType) -> Self::UnderlyingType { todo!() }

    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
    fn exp(self) -> Self::UnderlyingType;
    fn exp2(self) -> Self::UnderlyingType;
    fn ln(self) -> Self::UnderlyingType;
    fn log(self, base: Self::UnderlyingType) -> Self::UnderlyingType;
    fn log2(self) -> Self::UnderlyingType;
    fn log10(self) -> Self::UnderlyingType;

    // trig functions don't make a lot of sense for this kind of a number
}

macro_rules! make_canonical {
    ($name:ident, $underlying:ty) => {
        #[derive(Debug, PartialEq, Copy, Clone)]
        pub struct $name(pub $underlying);

        impl TryFrom<$underlying> for $name {
            type Error = ();

            fn try_from(value: $underlying) -> Result<Self, Self::Error> {
                if value < (0 as $underlying) || value > (1 as $underlying) {
                    Err(())
                } else {
                    Ok($name(value))
                }
            }
        }

        impl PartialEq<$underlying> for $name {
            fn eq(&self, other: &$underlying) -> bool { self.0 == *other }
            fn ne(&self, other: &$underlying) -> bool { self.0 != *other }
        }

        macro_rules! impl_non_canonical_oper {
            ($fn_name:ident) => {
                fn $fn_name(self) -> Self::UnderlyingType { Self::UnderlyingType::$fn_name(self.0) }
            };
        }

        macro_rules! impl_canonical_oper {
            ($fn_name:ident) => {
                fn $fn_name(self) -> Self {
                    if let Ok(ret) = Self::UnderlyingType::$fn_name(self.0).try_into() {
                        ret
                    } else {
                        panic!("operation on canonical float that should not produce non-canonical float produced non-canonical float")
                    }
                }
            };
        }

        impl CanonicalFloat for $name {
            type UnderlyingType = $underlying;

            impl_canonical_oper!(floor);
            impl_non_canonical_oper!(ceil);
            impl_non_canonical_oper!(round);
            // impl_non_canonical_oper, round_ties_even);
            impl_canonical_oper!(trunc);
            impl_canonical_oper!(fract);
            impl_canonical_oper!(abs);

            fn copysign(self, sign: Self::UnderlyingType) -> Self::UnderlyingType { self.0.copysign(sign) }

            impl_non_canonical_oper!(signum);

            impl_canonical_oper!(sqrt);
            impl_canonical_oper!(cbrt);
            impl_non_canonical_oper!(exp);
            impl_non_canonical_oper!(exp2);
            impl_non_canonical_oper!(ln);
            fn log(self, base: Self::UnderlyingType) -> Self::UnderlyingType { self.0.log(base) }
            impl_non_canonical_oper!(log2);
            impl_non_canonical_oper!(log10);
        }

        macro_rules! impl_operation {
            ($canon_type:ty, $op_impl:path, $op_name:ident) => {
                impl $op_impl for $canon_type {
                    type Output = $underlying;
                    fn $op_name(self, rhs: $underlying) -> Self::Output { <$underlying as $op_impl>::$op_name(self.0, rhs) }
                }
            };
        }

        impl_operation!($name, core::ops::Add<$underlying>, add);
        impl_operation!($name, core::ops::Sub<$underlying>, sub);
        impl_operation!($name, core::ops::Mul<$underlying>, mul);
        impl_operation!($name, core::ops::Div<$underlying>, div);
    };
}

make_canonical!(CanonicalF64, f64);
make_canonical!(CanonicalF32, f32);
*/

#[derive(Clone, Copy)]
pub struct Canonical<T: FloatingPoint>(T);

impl<T: FloatingPoint> PartialEq<T> for Canonical<T> {
    fn eq(&self, other: &T) -> bool { self.0 == *other }
    fn ne(&self, other: &T) -> bool { self.0 != *other }
}

macro_rules! impl_non_canonical_oper {
    ($fn_name:ident) => {
        pub fn $fn_name(self) -> T { T::$fn_name(self.0) }
    };
}

macro_rules! impl_canonical_oper {
    ($fn_name:ident) => {
        pub fn $fn_name(self) -> Self {
            if let Some(ret) = Self::new(T::$fn_name(self.0)) {
                ret
            } else {
                panic!("operation on canonical float that should not produce non-canonical float produced non-canonical float")
            }
        }
    };
}

impl<T: FloatingPoint> Canonical<T> {
    type UnderlyingType = T;

    pub fn new(value: T) -> Option<Self> {
        let zero = FromScalar::from_scalar(0).unwrap();
        let one = FromScalar::from_scalar(1).unwrap();

        if value < zero || value > one {
            None
        } else {
            Some(Canonical(value))
        }
    }

    pub fn get(self) -> T { self.0 }

    impl_canonical_oper!(floor);
    impl_non_canonical_oper!(ceil);
    impl_non_canonical_oper!(round);
    // impl_non_canonical_oper, round_ties_even);
    impl_canonical_oper!(trunc);
    impl_canonical_oper!(fract);
    impl_canonical_oper!(abs);

    fn copysign(self, sign: T) -> T{ self.0.copysign(sign) }

    impl_non_canonical_oper!(signum);

    impl_canonical_oper!(sqrt);
    impl_canonical_oper!(cbrt);
    impl_non_canonical_oper!(exp);
    impl_non_canonical_oper!(exp2);
    impl_non_canonical_oper!(ln);
    fn log(self, base: T) -> T{ self.0.log(base) }
    impl_non_canonical_oper!(log2);
    impl_non_canonical_oper!(log10);
}

macro_rules! impl_operation {
    ($op_impl:ident, $op_name:ident) => {
        impl<T: FloatingPoint> core::ops::$op_impl<T> for Canonical<T> {
            type Output = T;
            fn $op_name(self, rhs: T) -> Self::Output { T::$op_name(self.0, rhs) }
        }
    };
}

impl_operation!(Add, add);
impl_operation!(Sub, sub);
impl_operation!(Mul, mul);
impl_operation!(Div, div);
