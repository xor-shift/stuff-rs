use std::marker::PhantomData;
use crate::rng::*;
use super::*;

use rng::util::fill_with_entropy;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GenericLinearGenerator<T: UnsignedIntegral, const N: usize, P: Permuter<T, N>, S: Scrambler<T, N>, D: Discarder<T, N>> {
    state: [T; N],

    permuter: PhantomData<P>,
    scrambler: PhantomData<S>,
    discarder: PhantomData<D>,
}

impl<T: UnsignedIntegral, const N: usize, P: Permuter<T, N>, S: Scrambler<T, N>, D: Discarder<T, N>> GenericLinearGenerator<T, N, P, S, D> {
    pub fn new() -> Self {
        Self {
            state: [T::zero(); N],
            permuter: Default::default(),
            scrambler: Default::default(),
            discarder: Default::default(),
        }
    }
}

impl<T: UnsignedIntegral, const N: usize, P: Permuter<T, N>, S: Scrambler<T, N>, D: Discarder<T, N>> UniformRandomBitGenerator for GenericLinearGenerator<T, N, P, S, D> {
    type ResultType = T;

    fn min() -> Self::ResultType {
        NumericLimits::min()
    }

    fn max() -> Self::ResultType {
        NumericLimits::max()
    }

    fn generate(&mut self) -> Self::ResultType {
        let res = S::scramble(&self.state);
        self.state = P::permute(self.state);
        return res;
    }
}

impl<T: UnsignedIntegral, const N: usize, P: Permuter<T, N>, S: Scrambler<T, N>, D: Discarder<T, N>> RandomNumberEngine for GenericLinearGenerator<T, N, P, S, D> {
    fn discard(&mut self, z: usize) {
        self.state = D::discard(self.state, z);
    }

    fn reset(&mut self) {
        self.state = [T::zero(); N];
    }

    fn seed_from<G: UniformRandomBitGenerator<ResultType=u32>>(&mut self, _generator: &mut G) {
        todo!()
    }

    fn seed_from_result(&mut self, v: Self::ResultType) {
        if T::DIGITS == 64 {
            let mut generator = SplitMix64::new();
            generator.seed_from_result(<T as ToScalar<u64>>::to_scalar(v).unwrap());

            for v in &mut self.state {
                *v = fill_with_entropy(&mut generator, T::DIGITS);
            }
        } else if T::DIGITS == 32 {
            let mut generator = SplitMix32::new();
            generator.seed_from_result(<T as ToScalar<u32>>::to_scalar(v).unwrap());

            for v in &mut self.state {
                *v = fill_with_entropy(&mut generator, T::DIGITS);
            }
        } else {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Eq, PartialEq, Copy, Clone)]
    struct TestPS;

    impl Scrambler<u64, 2> for TestPS {
        fn scramble(state: &[u64; 2]) -> u64 {
            state[0].wrapping_add(state[1])
        }
    }

    impl Permuter<u64, 2> for TestPS {
        fn permute(mut state: [u64; 2]) -> [u64; 2]{
            state[0] = state[0].wrapping_add(1);
            state[1] = state[1].wrapping_add(1);

            return state;
        }
    }

    impl PolyDiscarder<u64, 2> for TestPS {
        fn polynomials() -> &'static [(usize, [u64; 2])] {
            return &[
                (128, [0, 0]),
                (64, [0, 0]),
                (32, [0, 0]),
                (8, [0, 0]),
                (2, [0, 0]),
            ];
        }
    }

    type TestGenerator = GenericLinearGenerator<u64, 2, TestPS, TestPS, TestPS>;

    #[test]
    fn test_generic_linear_engine() {
        let mut generator = TestGenerator::new();
        assert_eq!(0, generator.generate());
        assert_eq!(2, generator.generate());

        generator.discard(123);
    }
}
