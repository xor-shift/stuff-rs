use crate::integral::convert::FromScalar;
use crate::rng::{RandomNumberEngine, UniformRandomBitGenerator};
use std::marker::{ConstParamTy, StructuralPartialEq};

use super::*;

pub struct SplitMixParameters {
    gamma: u64,
    m0: u64,
    s0: u32,
    m1: u64,
    s1: u32,
    m2: u64,
    s2: u32,
}

const SPLITMIX32_PARAMS: SplitMixParameters = SplitMixParameters {
    gamma: 314159265_u64,
    m0: 1664525_u64,
    s0: 15_u32,
    m1: 0x5ce4e5b9_u64,
    s1: 13_u32,
    m2: 0x1331c1eb_u64,
    s2: 15_u32,
};

const SPLITMIX64_PARAMS: SplitMixParameters = SplitMixParameters {
    gamma: 0x9e3779b97f4a7c15_u64,
    m0: 1_u64,
    s0: 30_u32,
    m1: 0xbf58476d1ce4e5b9_u64,
    s1: 27_u32,
    m2: 0x94d049bb133111eb_u64,
    s2: 31_u32,
};

impl StructuralPartialEq for SplitMixParameters {}

impl Eq for SplitMixParameters {}

impl PartialEq for SplitMixParameters {
    #[rustfmt::skip]
    fn eq(&self, other: &Self) -> bool {
        true
            && self.gamma == other.gamma
            && self.m0 == other.m0
            && self.s0 == other.s0
            && self.m1 == other.m1
            && self.s1 == other.s1
            && self.m2 == other.m2
            && self.s2 == other.s2
    }
}

impl ConstParamTy for SplitMixParameters {}

#[derive(Copy, Clone)]
pub struct SplitMixEngine<T: UnsignedIntegral, const PARAMS: SplitMixParameters> {
    state: T,
}

impl<T: UnsignedIntegral, const PARAMS: SplitMixParameters> PartialEq<Self> for SplitMixEngine<T, PARAMS> {
    fn eq(&self, other: &Self) -> bool { self.state.eq(&other.state) }
}

impl<T: UnsignedIntegral, const PARAMS: SplitMixParameters> Eq for SplitMixEngine<T, PARAMS> {}

impl<T: UnsignedIntegral, const PARAMS: SplitMixParameters> SplitMixEngine<T, PARAMS> {
    pub fn new() -> Self { Self { state: T::zero() } }

    pub fn split(self) -> (Self, Self) { todo!() }
}

impl<T: UnsignedIntegral, const PARAMS: SplitMixParameters> UniformRandomBitGenerator for SplitMixEngine<T, PARAMS> {
    type ResultType = T;

    fn min() -> Self::ResultType { NumericLimits::min() }

    fn max() -> Self::ResultType { NumericLimits::max() }

    fn generate(&mut self) -> Self::ResultType {
        self.state = self.state.wrapping_add(FromScalar::from_scalar(PARAMS.gamma).unwrap());
        self.state = self.state.wrapping_mul(FromScalar::from_scalar(PARAMS.m0).unwrap());

        let mut ret = self.state;
        ret = (ret ^ (ret >> PARAMS.s0)).wrapping_mul(FromScalar::from_scalar(PARAMS.m1).unwrap());
        ret = (ret ^ (ret >> PARAMS.s1)).wrapping_mul(FromScalar::from_scalar(PARAMS.m2).unwrap());
        ret = ret ^ (ret >> PARAMS.s2);

        return ret;
    }
}

impl<T: UnsignedIntegral, const PARAMS: SplitMixParameters> RandomNumberEngine for SplitMixEngine<T, PARAMS> {
    fn discard(&mut self, _z: usize) { todo!() }

    fn reset(&mut self) { self.seed_from_result(T::zero()); }

    fn seed_from<G: UniformRandomBitGenerator<ResultType = u32>>(&mut self, _generator: &mut G) {
        //self.state =  ((generator.generate() as u64) << 32) | (generator.generate() as u64);
        todo!()
    }

    fn seed_from_result(&mut self, v: Self::ResultType) { self.state = v; }
}

pub type SplitMix32 = SplitMixEngine<u32, SPLITMIX32_PARAMS>;
pub type SplitMix64 = SplitMixEngine<u64, SPLITMIX64_PARAMS>;

#[cfg(test)]
mod tests {
    use super::super::test_tables;

    #[test]
    fn test_splitmix32() {
        use super::SplitMix32;
        let mut engine = SplitMix32::new();
        test_tables::TABLE_SPLITMIX32_SEED_0.test(&mut engine);
        test_tables::TABLE_SPLITMIX32_SEED_1.test(&mut engine);
        test_tables::TABLE_SPLITMIX32_SEED_2.test(&mut engine);
    }

    #[test]
    fn test_splitmix64() {
        use super::SplitMix64;
        let mut engine = SplitMix64::new();
        test_tables::TABLE_SPLITMIX64_SEED_0.test(&mut engine);
        test_tables::TABLE_SPLITMIX64_SEED_1.test(&mut engine);
        test_tables::TABLE_SPLITMIX64_SEED_2.test(&mut engine);
    }
}
