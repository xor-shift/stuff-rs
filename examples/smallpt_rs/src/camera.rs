use crate::*;
use ray::*;

use stuff::{FloatingPoint, ZeroAndOne};

pub trait Camera<T: FloatingPoint> {
    fn generate_ray<Gen: stuff::rng::UniformRandomBitGenerator>(&self, screen_coords: (usize, usize), gen: &mut Gen) -> (Ray<T>, T);
}

#[derive(Clone)]
pub struct PinholeCamera<T: FloatingPoint> {
    origin: Vector<T, 3>,
    screen_dims: (usize, usize),
    fov: T,
}

impl<T: FloatingPoint + GenerateCanonical<T>> PinholeCamera<T> {
    pub const fn new(origin: Vector<T, 3>, screen_dims: (usize, usize), fov: T) -> Self { Self { origin, screen_dims, fov } }
}

impl<T: FloatingPoint + GenerateCanonical<T>> Camera<T> for PinholeCamera<T> {
    fn generate_ray<Gen: stuff::rng::UniformRandomBitGenerator>(&self, screen_coords: (usize, usize), gen: &mut Gen) -> (Ray<T>, T) {
        let one = <T as ZeroAndOne>::one();
        let two = T::as_from(2);

        let half_θ = self.fov / two;
        //let d = screen_coords.0 as f64 * (half_θ.atan()) / 2f64 ;
        let d = (one / (two * half_θ.sin())) * ((T::as_from(self.screen_dims.0) * (two - T::as_from(self.screen_dims.0))).abs()).sqrt();
        let offset = (T::generate_canonical(gen) * two - one, T::generate_canonical(gen) * two - one);
        let direction = Vector([
            T::as_from(screen_coords.0) - (T::as_from(self.screen_dims.0) / two) + offset.0, //
            T::as_from(self.screen_dims.1 - screen_coords.1 - 1) - (T::as_from(self.screen_dims.1) / two) + offset.1,
            d,
        ])
        .normalized();

        (Ray { origin: self.origin, direction }, one)
    }
}
