use crate::*;

pub mod generic;
pub mod random_device;
pub mod splitmix;
pub mod xoroshiro;
pub mod xoshiro;

#[cfg(test)]
mod test_tables;

pub use random_device::*;
pub use splitmix::*;
pub use xoroshiro::*;
pub use xoshiro::*;

pub trait Permuter<T: Integral, const N: usize>: Copy + Clone + Eq + PartialEq {
    // type StateType: Integral = T;
    // const STATE_SIZE: usize = N;

    fn permute(state: [T; N]) -> [T; N];
}

pub trait Scrambler<T: Integral, const N: usize>: Copy + Clone + Eq + PartialEq {
    // type StateType: Integral = T;
    // const STATE_SIZE: usize = N;

    fn scramble(state: &[T; N]) -> T;
}

pub trait Discarder<T: Integral, const N: usize>: Copy + Clone + Eq + PartialEq {
    fn discard(state: [T; N], z: usize) -> [T; N];
}

pub trait PolyDiscarder<T: Integral, const N: usize>: Copy + Clone + Eq + PartialEq {
    /// `polynomials().windows(2).map(|v| v[0] > v[1]).reduce(|a, b| a && b).unwrap_or(true)`
    ///
    /// must hold true.
    fn polynomials() -> &'static [(usize, [T; N])];
}

impl<T: Integral, const N: usize, P: PolyDiscarder<T, N> + Permuter<T, N>> Discarder<T, N> for P {
    fn discard(mut state: [T; N], mut z: usize) -> [T; N] {
        debug_assert!(Self::polynomials().windows(2).map(|v| v[0] > v[1]).reduce(|a, b| a && b).unwrap_or(true));
        //debug_assert!(Self::polynomials().iter().map(|v| v.0.count_ones() == 1).reduce(|a, b| a && b).unwrap_or(true));

        // this does not work for arbitrarily descending divisors
        /*let placeholder = (usize::MAX, [T::zero(); N]);
        let polys_and_counts = std::iter::once(&placeholder)
            .chain(Self::polynomials().iter())
            .map_windows(|w: &[_; 2]| {
                let to_sub = (z / w[0].0) * w[0].0;
                let left_z = z - to_sub;
                (left_z / w[1].0, w[1].1)
            });

        for (count, poly) in polys_and_counts {
            for _ in 0..count {
                state = util::jump_new(state, &poly, &P::permute);
            }
        }

        for _ in 0..z - Self::polynomials().last().map(|v| z / v.0 * v.0).unwrap_or(0) {
            state = P::permute(state);
        }

        return state;*/

        // this is terrible

        for (jump_length, poly) in Self::polynomials() {
            let jump_count = z / jump_length;
            z -= jump_count * jump_length;

            for _ in 0..jump_count {
                state = util::jump_new(state, poly, &P::permute);
            }
        }

        for _ in 0..z {
            state = P::permute(state);
        }

        return state;
    }
}

mod util {
    #[cfg(test)]
    use super::super::RandomNumberEngine;
    use crate::integral::*;

    // see stuff/libs/random/test/consistency.cpp
    #[cfg(test)]
    pub struct ConsistencyTestTable<T: Integral, const N: usize> {
        pub date: &'static str,
        pub seed: T,
        pub table: [T; N],
    }

    #[cfg(test)]
    impl<T: Integral, const N: usize> ConsistencyTestTable<T, N> {
        pub fn test<E: RandomNumberEngine<ResultType = T>>(&'static self, engine: &mut E) {
            engine.seed_from_result(self.seed);

            let generated = std::iter::repeat(()).map(move |_| engine.generate());
            for (generated, expected) in generated.zip(self.table) {
                assert_eq!(generated, expected);
            }
        }
    }

    #[cfg(test)]
    pub struct JumpConsistencyTestTable<T: Integral, const N: usize> {
        pub date: &'static str,
        pub table: [(T, T); N],
    }

    #[cfg(test)]
    impl<T: Integral, const N: usize> JumpConsistencyTestTable<T, N> {
        pub fn test<E, F>(&'static self, engine: &mut E, jumper: F)
        where
            E: RandomNumberEngine<ResultType = T>,
            F: Fn(&mut E),
        {
            for (seed, expected_after_jump) in self.table {
                engine.seed_from_result(seed);
                jumper(engine);
                let after_jump = engine.generate();
                assert_eq!(after_jump, expected_after_jump);
            }
        }
    }

    #[deprecated]
    #[allow(dead_code)]
    pub fn jump_f2_with_polynomial<T: Integral, const N: usize, F: Fn(&mut [T; N]) -> ()>(state: &mut [T; N], poly: [T; N], permuter: &F) {
        let mut res = [T::zero(); N];

        for v in poly {
            for b in 0..T::DIGITS {
                if (v & (T::one() << b)) != T::zero() {
                    for i in 0..N {
                        res[i] = res[i] ^ state[i];
                    }
                }

                permuter(state);
            }
        }

        for (state_elem, &res_elem) in state.iter_mut().zip(res.iter()) {
            *state_elem = res_elem;
        }
    }

    pub fn jump_new<T: Integral, const N: usize, F: Fn([T; N]) -> [T; N]>(mut state: [T; N], poly: &[T; N], permuter: &F) -> [T; N] {
        let mut res = [T::zero(); N];

        for &v in poly {
            for b in 0..T::DIGITS {
                if (v & (T::one() << b)) != T::zero() {
                    for i in 0..N {
                        res[i] = res[i] ^ state[i];
                    }
                }

                state = permuter(state);
            }
        }

        return res;
    }
}
