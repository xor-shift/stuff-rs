use std::marker::PhantomData;

use crate::{FloatConstants, FloatingPoint, ZeroAndOne};

use super::*;

pub trait NDSampler<T: FloatingPoint, const N: usize> {
    fn sample<Gen: UniformRandomBitGenerator>(&mut self, gen: &mut Gen) -> ([T; N], T);
}

pub struct UniformSphereSampler<T: FloatingPoint + GenerateCanonical<T>, const N: usize> {
    data: PhantomData<[T; N]>,
}

impl<T: FloatingPoint + GenerateCanonical<T>, const N: usize> UniformSphereSampler<T, N> {
    pub fn new() -> Self { Self { data: Default::default() } }

    #[allow(dead_code)]
    fn strategy_normalised_normal<Gen: UniformRandomBitGenerator>(gen: &mut Gen) -> [T; N] {
        let mut dist = NormalDistribution::<T>::new();

        let mut ret = [<T as ZeroAndOne>::zero(); N];
        let mut length = <T as ZeroAndOne>::zero();

        for i in 0..N {
            let t = dist.generate(gen);
            length = length + t * t;
            ret[i] = t;
        }

        length = length.sqrt();

        for i in 0..N {
            ret[i] = ret[i] / length;
        }

        ret
    }
}

impl<T: FloatingPoint + GenerateCanonical<T>, const N: usize> NDSampler<T, N> for UniformSphereSampler<T, N> {
    fn sample<Gen: UniformRandomBitGenerator>(&mut self, gen: &mut Gen) -> ([T; N], T) {
        let sample = Self::strategy_normalised_normal(gen);

        let pdf = match N {
            1 /* 1/2 */  => T::from_scalar(0.5f64).unwrap(),
            2 /* 1/2π */ => <T as FloatConstants>::FRAC_1_PI * T::from_scalar(0.5f64).unwrap(),
            3 /* 1/4π */ => <T as FloatConstants>::FRAC_1_PI * T::from_scalar(0.25f64).unwrap(),
            _ => todo!("what do you need more than 3 dimensions for anyway"),
        };

        (sample, pdf)
    }
}

pub struct UniformBallSampler<T: FloatingPoint + GenerateCanonical<T>, const N: usize> {
    data: PhantomData<[T; N]>,
}

impl<T: FloatingPoint + GenerateCanonical<T>, const N: usize> UniformBallSampler<T, N> {
    pub fn new() -> Self { Self { data: Default::default() } }

    #[allow(dead_code)]
    fn strategy_rejection<Gen: UniformRandomBitGenerator>(gen: &mut Gen) -> [T; N] {
        loop {
            let mut ret = [<T as ZeroAndOne>::zero(); N];
            let mut length = <T as ZeroAndOne>::zero();

            for i in 0..N {
                let t = T::generate_canonical(gen).get() * T::from_scalar(2).unwrap() - <T as ZeroAndOne>::one();
                ret[i] = t;
                length = length + t * t;
            }

            length = length.sqrt();

            if length < <T as ZeroAndOne>::one() {
                break ret;
            }
        }
    }

    #[allow(dead_code)]
    fn strategy_sphere_with_radius<Gen: UniformRandomBitGenerator>(gen: &mut Gen) -> [T; N] {
        let mut res = UniformSphereSampler::new().sample(gen).0;
        let radius = T::generate_canonical(gen).sqrt();

        for i in 0..N {
            res[i] = res[i] * radius.get();
        }

        res
    }
}

impl<T: FloatingPoint + GenerateCanonical<T>, const N: usize> NDSampler<T, N> for UniformBallSampler<T, N> {
    fn sample<Gen: UniformRandomBitGenerator>(&mut self, gen: &mut Gen) -> ([T; N], T) {
        let pdf = match N {
            1 /* 1/2 */  => T::from_scalar(0.5f64).unwrap(),
            2 /* 1/π */  => <T as FloatConstants>::FRAC_1_PI,
            3 /* 3/4π */ => <T as FloatConstants>::FRAC_1_PI * T::from_scalar(3f64 / 4f64).unwrap(),
            _ => todo!("why do you need more than 3 dimensions for anyway"),
        };

        (Self::strategy_sphere_with_radius(gen), pdf)
    }
}

pub struct CosineWeightedHemisphereSampler<T: FloatingPoint + GenerateCanonical<T>> {
    data: PhantomData<T>,
}

impl<T: FloatingPoint + GenerateCanonical<T>> CosineWeightedHemisphereSampler<T> {
    pub fn new() -> Self { Self { data: Default::default() } }
}

impl<T: FloatingPoint + GenerateCanonical<T>> NDSampler<T, 3> for CosineWeightedHemisphereSampler<T> {
    fn sample<Gen: UniformRandomBitGenerator>(&mut self, gen: &mut Gen) -> ([T; 3], T) {
        let cosθ = T::generate_canonical(gen).sqrt().get();
        let sinθ = (<T as ZeroAndOne>::one() - cosθ * cosθ).sqrt();
        let probability = cosθ * <T as FloatConstants>::FRAC_1_PI;

        let φ = T::from_scalar(2).unwrap() * <T as FloatConstants>::PI * T::generate_canonical(gen).get();

        let sinφ = φ.sin();
        let cosφ = φ.cos();

        ([cosφ * sinθ, sinφ * sinθ, cosθ], probability)
    }
}

pub struct PowerCosineWeightedHemisphereSampler<T: FloatingPoint + GenerateCanonical<T>> {
    alpha: T,
}

impl<T: FloatingPoint + GenerateCanonical<T>> PowerCosineWeightedHemisphereSampler<T> {
    pub fn new(alpha: T) -> Self { Self { alpha } }
}

impl<T: FloatingPoint + GenerateCanonical<T>> NDSampler<T, 3> for PowerCosineWeightedHemisphereSampler<T> {
    fn sample<Gen: UniformRandomBitGenerator>(&mut self, gen: &mut Gen) -> ([T; 3], T) {
        let one = <T as ZeroAndOne>::one();

        let cosθ = T::generate_canonical(gen).get().powf(one / (self.alpha + one));
        let sinθ = (one - cosθ * cosθ).sqrt();
        let probability = (self.alpha + one) * cosθ.powf(self.alpha) * T::from_scalar(0.5f64).unwrap() * <T as FloatConstants>::FRAC_1_PI;

        let φ = T::from_scalar(2).unwrap() * <T as FloatConstants>::PI * T::generate_canonical(gen).get();

        let sinφ = φ.sin();
        let cosφ = φ.cos();

        ([cosφ * sinθ, sinφ * sinθ, cosθ], probability)
    }
}
