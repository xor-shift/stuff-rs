pub mod bit;
pub mod checked;
pub mod wrapping;

pub use bit::*;
pub use checked::*;
pub use wrapping::*;

macro_rules! builtin_trait_impl {
    ($type_name:ty, $arg_type_name:ty, $trait_name:ident, $function_name:ident) => {
        impl $trait_name<$type_name> for $type_name {
            type Output = $type_name;

            fn $function_name(self, other: $arg_type_name) -> Self::Output {
                return self.$function_name(other);
            }
        }
    };
}

macro_rules! builtin_trait_impl_checked {
    ($type_name:ty, $arg_type_name:ty, $trait_name:ident, $function_name:ident) => {
        impl $trait_name<$type_name> for $type_name {
            type Output = $type_name;

            fn $function_name(self, other: $arg_type_name) -> Option<Self::Output> {
                return self.$function_name(other);
            }
        }
    };
}

macro_rules! impl_all_builtins {
    ($type_name:ty) => {
        builtin_trait_impl!($type_name, $type_name, WrappingAdd, wrapping_add);
        builtin_trait_impl!($type_name, $type_name, WrappingSub, wrapping_sub);
        builtin_trait_impl!($type_name, $type_name, WrappingMul, wrapping_mul);
        builtin_trait_impl!($type_name, $type_name, WrappingDiv, wrapping_div);
        builtin_trait_impl!($type_name, u32, WrappingShr, wrapping_shr);
        builtin_trait_impl!($type_name, u32, WrappingShl, wrapping_shl);
        builtin_trait_impl_checked!($type_name, $type_name, CheckedAdd, checked_add);
        builtin_trait_impl_checked!($type_name, $type_name, CheckedSub, checked_sub);
        builtin_trait_impl_checked!($type_name, $type_name, CheckedMul, checked_mul);
        builtin_trait_impl_checked!($type_name, $type_name, CheckedDiv, checked_div);
        builtin_trait_impl_checked!($type_name, u32, CheckedShr, checked_shr);
        builtin_trait_impl_checked!($type_name, u32, CheckedShl, checked_shl);
    };
}

impl_all_builtins!(u8);
impl_all_builtins!(i8);
impl_all_builtins!(u16);
impl_all_builtins!(i16);
impl_all_builtins!(u32);
impl_all_builtins!(i32);
impl_all_builtins!(u64);
impl_all_builtins!(i64);
impl_all_builtins!(u128);
impl_all_builtins!(i128);
impl_all_builtins!(usize);
impl_all_builtins!(isize);
