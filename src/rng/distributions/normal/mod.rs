use crate::{
    rng::{RandomNumberDistribution, UniformRandomBitGenerator},
    FloatingPoint, FromScalar,
};

use super::GenerateCanonical;

mod detail {
    use crate::rng::*;
    use crate::*;

    #[allow(dead_code)]
    pub fn box_müller<T: FloatingPoint>(samples: [Canonical<T>; 2]) -> Option<[NormallyDistributedFloat<T>; 2]> {
        let zero = T::from_scalar(0).unwrap();
        // let one = T::from_scalar(1).unwrap();
        let two = T::from_scalar(2).unwrap();

        if samples[0] == zero || samples[1] == zero {
            None
        } else {
            let r_2 = -two * samples[0].ln();
            let r = r_2.sqrt();
            let theta = two * T::PI * samples[1].get();

            Some([(r * theta.cos()).into(), (r * theta.sin()).into()])
        }
    }

    // #[allow(dead_code)]
    pub fn box_müller_marsaglia_polar<T: FloatingPoint>(samples: [Canonical<T>; 2]) -> Option<[NormallyDistributedFloat<T>; 2]> {
        let zero = T::from_scalar(0).unwrap();
        let one = T::from_scalar(1).unwrap();
        let two = T::from_scalar(2).unwrap();

        let x = samples[0] * two - one;
        let y = samples[1] * two - one;
        let s = x * x + y * y;

        if s >= one || s == zero {
            return None;
        }

        let r_2 = s.ln() * -two / s;
        let r = r_2.sqrt();

        let z_0 = x * r;
        let z_1 = y * r;

        return Some([z_0.into(), z_1.into()]);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        pub fn test_box_müller() {
            assert_eq!(box_müller([Canonical::<f64>::new(0.0_f64).unwrap(), Canonical::<f64>::new(0.0_f64).unwrap()]), None);
            assert_eq!(box_müller_marsaglia_polar([Canonical::<f64>::new(0.0_f64).unwrap(), Canonical::<f64>::new(0.0_f64).unwrap()]), None);
            assert_eq!(box_müller([Canonical::<f64>::new(0.5_f64).unwrap(), Canonical::<f64>::new(0.0_f64).unwrap()]), None);
            assert_eq!(box_müller_marsaglia_polar([Canonical::<f64>::new(0.5_f64).unwrap(), Canonical::<f64>::new(0.0_f64).unwrap()]), None);
            assert_eq!(box_müller([Canonical::<f64>::new(0.0_f64).unwrap(), Canonical::<f64>::new(0.5_f64).unwrap()]), None);
            assert_eq!(box_müller_marsaglia_polar([Canonical::<f64>::new(0.0_f64).unwrap(), Canonical::<f64>::new(0.5_f64).unwrap()]), None);
            assert_ne!(box_müller([Canonical::<f64>::new(0.5_f64).unwrap(), Canonical::<f64>::new(0.5_f64).unwrap()]), None);
            assert_eq!(box_müller_marsaglia_polar([Canonical::<f64>::new(0.5_f64).unwrap(), Canonical::<f64>::new(0.5_f64).unwrap()]), None);
            assert_ne!(box_müller_marsaglia_polar([Canonical::<f64>::new(0.2_f64).unwrap(), Canonical::<f64>::new(0.2_f64).unwrap()]), None);
        }
    }
}

pub struct NormalDistribution<T: FloatingPoint + GenerateCanonical<T>> {
    cache: Option<T>,
}

impl<T: FloatingPoint + GenerateCanonical<T>> NormalDistribution<T> {
    pub fn new() -> Self { Self { cache: None } }
}

#[derive(Copy, Clone, PartialEq)]
pub struct NormalDistributionParams<T: FloatingPoint> {
    pub mean: T,
    pub stdev: T,
}

impl<T: FloatingPoint> Default for NormalDistributionParams<T> {
    fn default() -> Self {
        NormalDistributionParams::<T> {
            mean: FromScalar::from_scalar(0).unwrap(),
            stdev: FromScalar::from_scalar(1).unwrap(),
        }
    }
}

impl<T: FloatingPoint + GenerateCanonical<T>> RandomNumberDistribution for NormalDistribution<T> {
    type ParamType = NormalDistributionParams<T>;
    type ResultType = T;

    fn min(&self) -> Self::ResultType { todo!() }
    fn max(&self) -> Self::ResultType { todo!() }

    fn reset(&mut self) {}

    fn get_param(&self) -> Self::ParamType { Default::default() }
    fn set_param(&mut self, _param: &Self::ParamType) { todo!() }

    fn generate<G: UniformRandomBitGenerator>(&mut self, generator: &mut G) -> Self::ResultType { self.generate_param(generator, &Default::default()) }

    fn generate_param<G: UniformRandomBitGenerator>(&mut self, generator: &mut G, param: &Self::ParamType) -> Self::ResultType {
        if let Some(cached_value) = self.cache {
            self.cache = None;

            return cached_value * param.stdev + param.mean;
        }

        let normal_samples = loop {
            let samples = [T::generate_canonical(generator), T::generate_canonical(generator)];

            if let Some(normal_samples) = detail::box_müller_marsaglia_polar(samples) {
                break normal_samples;
            }
        };

        //self.cache = Some(normal_samples[0].get());

        return normal_samples[1].get();
    }
}

impl<T: FloatingPoint + GenerateCanonical<T>> PartialEq for NormalDistribution<T> {
    fn eq(&self, other: &Self) -> bool { self.get_param() == other.get_param() }
}
