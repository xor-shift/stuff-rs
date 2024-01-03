use stuff::{smallvec::Vector, FloatingPoint};

use crate::color::Color;

pub struct Ray<T: FloatingPoint> {
    pub origin: Vector<T, 3>,
    pub direction: Vector<T, 3>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ReflectanceType {
    Diffuse,
    PerfectMirror,
    Dielectric,
}

pub struct Intersection<T: FloatingPoint> {
    pub distance: T,
    pub position: Vector<T, 3>,
    pub normal: Vector<T, 3>,
    pub albedo: Color<T>,
    pub emittance: Color<T>,
    pub material: ReflectanceType,
    pub index_of_refraction: T,
}

impl<T: FloatingPoint> Intersection<T> {
    pub fn select_best(lhs: Option<Self>, rhs: Option<Self>) -> Option<Self> {
        match (lhs, rhs) {
            (None, None) => None,
            (Some(v), None) => Some(v),
            (None, Some(v)) => Some(v),
            (Some(lhs), Some(rhs)) => Some(if lhs.distance > rhs.distance { rhs } else { lhs }),
        }
    }
}

pub trait Intersectable<T: FloatingPoint> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>>;
}

impl<F: FloatingPoint, T: Intersectable<F>, const N: usize> Intersectable<F> for [T; N] {
    fn intersect(&self, ray: &Ray<F>) -> Option<Intersection<F>> {
        let mut intersection = None;
        for shape in self {
            intersection = Intersection::select_best(intersection, shape.intersect(ray));
        }

        intersection
    }
}

pub fn reflect<T: FloatingPoint>(wo: Vector<T, 3>, normal: Vector<T, 3>) -> Vector<T, 3> { -wo + normal * T::as_from(2) * (normal.dot(wo)) }

pub fn refract<T: FloatingPoint>(wo: Vector<T, 3>, normal: Vector<T, 3>, incident_index: T, transmittant_index: T) -> Option<(Vector<T, 3>, T)> {
    let l = -wo;

    let index_ratio = incident_index / transmittant_index;

    let cosθ_i = -l.dot(normal);
    let sin2θ_i = T::one() - cosθ_i * cosθ_i;
    let sin2θ_t = index_ratio * index_ratio * sin2θ_i;

    if sin2θ_t >= T::one() {
        return None;
    }

    let cosθ_t = (T::one() - sin2θ_t).sqrt();

    let refracted_direction = l * index_ratio + normal * (index_ratio * cosθ_i - cosθ_t);

    let r0 = ((incident_index - transmittant_index) / (incident_index + transmittant_index)).powi(2);
    let r = r0 + (T::one() - r0) * (T::one() - cosθ_i).powi(5);

    return Some((refracted_direction, T::one() - r));
}
