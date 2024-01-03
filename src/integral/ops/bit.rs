// https://english.stackexchange.com/questions/55193/when-to-drop-the-e-when-ending-in-able
// "The only situation that comes to mind where an -e- is absolutely required before -able is when it modifies the pronunciation of a consonant, typically g or c"

pub trait BitFiddlable: crate::NumericLimits + Sized {
    fn swap_bytes(self) -> Self;

    fn has_single_bit(self) -> bool { Self::count_ones(self) == 1u32 }
    fn bit_ceil(self) -> Self { todo!() }
    fn bit_floor(self) -> Self { todo!() }
    fn bit_width(self) -> u32 { Self::DIGITS - self.leading_zeros() }

    fn leading_zeros(self) -> u32;
    fn leading_ones(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn trailing_ones(self) -> u32;
    fn count_ones(self) -> u32;
}

macro_rules! impl_bit_fiddlable {
    ($type_name:ty) => {
        impl BitFiddlable for $type_name {
            fn swap_bytes(self) -> Self { <$type_name>::swap_bytes(self) }

            fn leading_zeros(self) -> u32 { <$type_name>::leading_zeros(self) }
            fn leading_ones(self) -> u32 { <$type_name>::leading_ones(self) }
            fn trailing_zeros(self) -> u32 { <$type_name>::trailing_zeros(self) }
            fn trailing_ones(self) -> u32 { <$type_name>::trailing_ones(self) }
            fn count_ones(self) -> u32 { <$type_name>::count_ones(self) }
        }
    };
}

impl_bit_fiddlable!(u8);
impl_bit_fiddlable!(u16);
impl_bit_fiddlable!(u32);
impl_bit_fiddlable!(u64);
impl_bit_fiddlable!(u128);
impl_bit_fiddlable!(usize);
