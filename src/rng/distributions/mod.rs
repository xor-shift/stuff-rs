pub mod normal;
pub mod uniform;
pub mod sphere;

pub use normal::*;
pub use uniform::*;

use super::*;

#[derive(Eq, PartialEq)]
pub struct CanonicalDistribution {}

impl RandomNumberDistribution for CanonicalDistribution {
    type ResultType = f64;
    type ParamType = ();

    fn min(&self) -> Self::ResultType { 0.0_f64 }
    fn max(&self) -> Self::ResultType { 1.0_f64.next_down() }

    fn reset(&mut self) {}
    fn get_param(&self) -> Self::ParamType { () }
    fn set_param(&mut self, _param: &Self::ParamType) {}

    fn generate<G: UniformRandomBitGenerator>(&mut self, generator: &mut G) -> Self::ResultType {
        self.generate_param(generator, &())
    }

    fn generate_param<G: UniformRandomBitGenerator>(&mut self, generator: &mut G, _param: &Self::ParamType) -> Self::ResultType {
        f64::generate_canonical(generator).get()
    }
}

mod detail {}
