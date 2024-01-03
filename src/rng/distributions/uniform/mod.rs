use crate::{rng::*, FloatingPoint};

pub trait GenerateCanonical<T: FloatingPoint> {
    fn generate_canonical<G: UniformRandomBitGenerator>(generator: &mut G) -> Canonical<T>;
}

impl GenerateCanonical<f32> for f32 {
    fn generate_canonical<G: UniformRandomBitGenerator>(generator: &mut G) -> Canonical<f32> {
        let entropy = util::fill_with_entropy::<u32, _>(generator, 24);
        debug_assert!(entropy <= 0xFFFF_FF_u32);

        return Canonical::new((entropy as f32) * util::quick_exp2_f32(-24)).unwrap();
    }
}

impl GenerateCanonical<f64> for f64 {
    fn generate_canonical<G: UniformRandomBitGenerator>(generator: &mut G) -> Canonical<f64> {
        let entropy = util::fill_with_entropy::<u64, _>(generator, 48);
        debug_assert!(entropy <= 0xFFFF_FFFF_FFFF_u64);

        return Canonical::new((entropy as f64) * util::quick_exp2_f64(-48)).unwrap();
    }
}
