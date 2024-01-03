pub mod distributions;
pub mod engines;
pub mod numbers;
#[cfg(test)] pub mod test;

pub use numbers::*;

pub trait UniformRandomBitGenerator {
    type ResultType: crate::integral::UnsignedIntegral;

    fn min() -> Self::ResultType;
    fn max() -> Self::ResultType;

    fn generate(&mut self) -> Self::ResultType;
}

pub trait RandomNumberEngine: UniformRandomBitGenerator + Eq + Clone {
    fn discard(&mut self, z: usize);

    fn reset(&mut self);

    fn seed_from<G: UniformRandomBitGenerator<ResultType=u32>>(&mut self, generator: &mut G);
    fn seed_from_result(&mut self, v: Self::ResultType);
}

pub trait RandomNumberDistribution: PartialEq {
    type ResultType;
    type ParamType: PartialEq;

    fn min(&self) -> Self::ResultType;
    fn max(&self) -> Self::ResultType;

    fn reset(&mut self);

    fn get_param(&self) -> Self::ParamType;
    fn set_param(&mut self, param: &Self::ParamType);

    fn generate<G: UniformRandomBitGenerator>(&mut self, generator: &mut G) -> Self::ResultType;
    fn generate_param<G: UniformRandomBitGenerator>(&mut self, generator: &mut G, param: &Self::ParamType) -> Self::ResultType;
}

mod util {
    use super::*;
    use crate::*;

    pub fn naive_bits_per_call<G: UniformRandomBitGenerator>() -> u32 {
        let generator_range = G::max() - G::min();

        if generator_range == <G::ResultType as NumericLimits>::max() {
            G::ResultType::DIGITS
        } else {
            (generator_range + ZeroAndOne::one()).bit_width() - 1u32
        }
    }

    /// Calls generator.generate() but truncates the result to naive_bits_per_call<G> bits.
    /// Regardless of G::min(), the minimum result is 0.
    pub fn naively_get_bits_from_generator<G: UniformRandomBitGenerator>(generator: &mut G) -> G::ResultType {
        let n_bits = naive_bits_per_call::<G>();

        let res = generator.generate();
        let res = res - G::min();
        let irrelevant_leading_bits = G::ResultType::DIGITS - n_bits;
        let res = (res << irrelevant_leading_bits) >> irrelevant_leading_bits;

        return res;
    }

    pub fn fill_with_entropy<T: UnsignedIntegral + FromScalar<G::ResultType>, G: UniformRandomBitGenerator>(generator: &mut G, wanted_bits: u32) -> T {
        debug_assert!(wanted_bits <= T::DIGITS);

        if naive_bits_per_call::<G>() == wanted_bits {
            return FromScalar::from_scalar(generator.generate()).unwrap();
        }

        let whole_calls = wanted_bits / naive_bits_per_call::<G>();
        let bits_after_whole_calls = wanted_bits % naive_bits_per_call::<G>();

        let mut ret = T::zero();

        for _ in 0..whole_calls {
            ret = ret << naive_bits_per_call::<G>();
            ret = ret | FromScalar::from_scalar(naively_get_bits_from_generator(generator)).unwrap();
        }

        if bits_after_whole_calls != 0 {
            debug_assert!(naive_bits_per_call::<G>() > bits_after_whole_calls);

            let excess_entropy = naively_get_bits_from_generator(generator);
            let excess_entropy = excess_entropy >> (naive_bits_per_call::<G>() - bits_after_whole_calls);

            ret = ret << bits_after_whole_calls;
            ret = ret | FromScalar::from_scalar(excess_entropy).unwrap();
        }

        return ret;
    }

    pub fn quick_exp2_f64(n: i32) -> f64 {
        assert!(n <= 1023, "quick_exp2_f64 must not be used to generate infinities");
        assert!(n >= -1022, "quick_exp2_f64 must not be used to generate subnormals");

        let exponent_bits = ((0x3ff + n) as u64) << 52;
        let result = unsafe { std::mem::transmute(exponent_bits) };

        return result;
    }

    pub fn quick_exp2_f32(n: i32) -> f32 {
        assert!(n <= 127, "quick_exp2_f32 must not be used to generate infinities");
        assert!(n >= -126, "quick_exp2_f32 must not be used to generate subnormals");

        let exponent_bits = ((0x7f + n) as u32) << 23;
        let result = unsafe { std::mem::transmute(exponent_bits) };

        return result;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        struct DummyGenerator0;

        impl UniformRandomBitGenerator for DummyGenerator0 {
            type ResultType = u32;
            fn min() -> Self::ResultType { 1 }
            fn max() -> Self::ResultType { u32::MAX }
            fn generate(&mut self) -> Self::ResultType { u32::MAX }
        }

        struct DummyGenerator1;

        impl UniformRandomBitGenerator for DummyGenerator1 {
            type ResultType = u128;
            fn min() -> Self::ResultType { 1 }
            fn max() -> Self::ResultType { 2 }
            fn generate(&mut self) -> Self::ResultType { 2 }
        }

        struct DummyGenerator2;

        impl UniformRandomBitGenerator for DummyGenerator2 {
            type ResultType = u64;
            fn min() -> Self::ResultType { 64 }
            fn max() -> Self::ResultType { 127 }
            fn generate(&mut self) -> Self::ResultType { 127 }
        }

        use super::engines::*;

        #[test]
        fn test_naive_bits_per_call() {
            assert_eq!(naive_bits_per_call::<SplitMix32>(), 32u32);
            assert_eq!(naive_bits_per_call::<SplitMix64>(), 64u32);
            assert_eq!(naive_bits_per_call::<Xoroshiro128P>(), 64u32);
            assert_eq!(naive_bits_per_call::<Xoroshiro128PP>(), 64u32);
            assert_eq!(naive_bits_per_call::<Xoroshiro128SS>(), 64u32);
            assert_eq!(naive_bits_per_call::<DummyGenerator0>(), 31u32);
            assert_eq!(naive_bits_per_call::<DummyGenerator1>(), 1u32);
            assert_eq!(naive_bits_per_call::<DummyGenerator2>(), 6u32);
        }

        #[test]
        fn test_generate_entropy() {
            let mut generator = DummyGenerator2 {};
            let res = fill_with_entropy::<u32, _>(&mut generator, 7);
            assert_eq!(res, 127u32);
        }

        #[test]
        fn test_exp2_f32() {
            assert_eq!(quick_exp2_f32(-126), 1.17549435082e-38_f32);
            assert_eq!(quick_exp2_f32(-4), 0.0625_f32);
            assert_eq!(quick_exp2_f32(-3), 0.125_f32);
            assert_eq!(quick_exp2_f32(-2), 0.25_f32);
            assert_eq!(quick_exp2_f32(-1), 0.5_f32);
            assert_eq!(quick_exp2_f32(0), 1_f32);
            assert_eq!(quick_exp2_f32(1), 2_f32);
            assert_eq!(quick_exp2_f32(2), 4_f32);
            assert_eq!(quick_exp2_f32(3), 8_f32);
            assert_eq!(quick_exp2_f32(4), 16_f32);
            assert_eq!(quick_exp2_f32(127), 1.7014118346e+38_f32);
        }

        #[test]
        fn test_exp2_f64() {
            assert_eq!(quick_exp2_f64(-1022), 2.2250738585072014e-308_f64);
            assert_eq!(quick_exp2_f64(-4), 0.0625_f64);
            assert_eq!(quick_exp2_f64(-3), 0.125_f64);
            assert_eq!(quick_exp2_f64(-2), 0.25_f64);
            assert_eq!(quick_exp2_f64(-1), 0.5_f64);
            assert_eq!(quick_exp2_f64(0), 1_f64);
            assert_eq!(quick_exp2_f64(1), 2_f64);
            assert_eq!(quick_exp2_f64(2), 4_f64);
            assert_eq!(quick_exp2_f64(3), 8_f64);
            assert_eq!(quick_exp2_f64(4), 16_f64);
            assert_eq!(quick_exp2_f64(1023), 8.98846567431158e+307_f64);
        }
    }
}
