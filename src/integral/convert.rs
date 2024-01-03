use crate::numeric_limits::NumericLimits;
use crate::Integral;

pub trait ToScalar<T> {
    fn to_scalar(self) -> Option<T>;
    fn as_to(self) -> T;
}

pub trait FromScalar<T>
where
    Self: Sized,
{
    fn from_scalar(v: T) -> Option<Self>;
    fn as_from(v: T) -> Self;
}

pub trait ZeroAndOne {
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! impl_to {
    ($type_from:ty as $type_to:ty) => {
        impl ToScalar<$type_to> for $type_from {
            #[inline]
            fn to_scalar(self) -> Option<$type_to> {
                const DIGITS_TO: u32 = <$type_to as NumericLimits>::DIGITS;
                const DIGITS_FROM: u32 = <$type_from as NumericLimits>::DIGITS;

                if DIGITS_TO >= DIGITS_FROM {
                    Some(self.try_into().unwrap())
                } else {
                    // const SIGNED_TO: bool = <$type_to as NumericLimits>::IS_SIGNED;
                    const SIGNED_FROM: bool = <$type_from as NumericLimits>::IS_SIGNED;

                    let cond = if SIGNED_FROM {
                        <$type_to>::MAX as $type_from >= self
                        && <$type_to>::MIN as $type_from <= self
                    } else {
                        <$type_to>::MAX as $type_from >= self
                    };

                    if cond { Some(self.try_into().unwrap()) } else { None }
                }
            }

            fn as_to(self) -> $type_to { self as $type_to }
        }
    };

    ($type_from:ty) => {
        impl ZeroAndOne for $type_from {
            #[inline]
            fn zero() -> $type_from { 0 as $type_from }

            #[inline]
            fn one() -> $type_from { 1 as $type_from }
        }

        impl_to!($type_from => u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
    };

    { $type_first:ty $(, $types_rest:ty)* } => {
        impl_to!($type_first);
        impl_to!{$($types_rest),*}
    };

    ($type_from:ty =>) => {};

    ($type_from:ty => $type_to:ty $(, $rest_of_the_types: ty)*) => {
        impl_to!($type_from as $type_to);
        impl_to!($type_from => $($rest_of_the_types),*);
    };
}

impl_to! {u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize}

macro_rules! impl_from {
    ($type_to:ty) => {
        impl<T: Integral> FromScalar<T> for $type_to {
            fn from_scalar(v: T) -> Option<$type_to> { <T as ToScalar<$type_to>>::to_scalar(v) }

            fn as_from(v: T) -> $type_to { <T as ToScalar<$type_to>>::as_to(v) }
        }
    };
}

impl_from!(u8);
impl_from!(u16);
impl_from!(u32);
impl_from!(u64);
impl_from!(u128);
impl_from!(usize);
impl_from!(i8);
impl_from!(i16);
impl_from!(i32);
impl_from!(i64);
impl_from!(i128);
impl_from!(isize);

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_scalar() {
        use super::ToScalar;
        assert_eq!(<u8 as ToScalar<u8>>::to_scalar(1u8), Some(1u8));
        assert_eq!(<u8 as ToScalar<i8>>::to_scalar(1u8), Some(1i8));
        assert_eq!(<u8 as ToScalar<i8>>::to_scalar(127u8), Some(127i8));
        assert_eq!(<u8 as ToScalar<i8>>::to_scalar(128u8), None);
        assert_eq!(<u8 as ToScalar<u16>>::to_scalar(255u8), Some(255u16));
        assert_eq!(<u16 as ToScalar<u8>>::to_scalar(255u16), Some(255u8));
        assert_eq!(<u16 as ToScalar<u8>>::to_scalar(256u16), None);
    }

    #[test]
    fn test_from_scalar() {
        use super::FromScalar;
        assert_eq!(u8::from_scalar(1u8), Some(1u8));
        assert_eq!(u8::from_scalar(1i8), Some(1u8));
        assert_eq!(u8::from_scalar(127i8), Some(127u8));
        assert_eq!(i8::from_scalar(128u8), None);
        assert_eq!(u8::from_scalar(255u16), Some(255u8));
        assert_eq!(u16::from_scalar(255u8), Some(255u16));
        assert_eq!(u8::from_scalar(256u16), None);
    }
}
