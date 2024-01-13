use crate::*;
use color::*;
use ray::*;
use stuff::smallvec::Vector;
use stuff::FloatingPoint;
use stuff::ZeroAndOne;

pub struct Sphere<T: FloatingPoint> {
    pub center: Vector<T, 3>,
    pub radius: T,
    pub albedo: Color<T>,
    pub emittance: Color<T>,
    pub material: ReflectanceType,
    pub index_of_refraction: T,
}

enum QuadraticSolution<T> {
    None,
    SingleRoot(T),
    TwoRoots(T, T),
}

#[allow(non_snake_case)]
fn solve_quadratic<T: FloatingPoint>(a: T, b: T, c: T) -> QuadraticSolution<T> {
    let two = T::as_from(2);

    let Δ = b * b - T::as_from(4) * a * c;

    // println!("{}", Δ);

    if Δ < <T as ZeroAndOne>::zero() {
        QuadraticSolution::None
    } else if Δ == <T as ZeroAndOne>::zero() {
        QuadraticSolution::SingleRoot(-b / (two * a))
    } else {
        let sqrt_Δ = Δ.sqrt();
        QuadraticSolution::TwoRoots((-b + sqrt_Δ) / (two * a), (-b - sqrt_Δ) / (two * a))
    }
}

impl<T: FloatingPoint> Intersectable<T> for Sphere<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>> {
        let direction = ray.origin - self.center;

        let a = <T as ZeroAndOne>::one();
        let b = T::as_from(2) * direction.dot(ray.direction);
        let c = direction.dot(direction) - (self.radius * self.radius);

        let threshold = T::as_from(0.00001);

        let t = match solve_quadratic(a, b, c) {
            QuadraticSolution::None => None,
            QuadraticSolution::SingleRoot(t) => {
                if t > threshold {
                    Some(t)
                } else {
                    None
                }
            }
            QuadraticSolution::TwoRoots(t_0, t_1) => match (t_0 > threshold, t_1 > threshold) {
                (false, false) => None,
                (true, false) => Some(t_0),
                (false, true) => Some(t_1),
                (true, true) => Some(t_0.min(t_1)),
            },
        };

        t.map(|t| {
            let pos = ray.origin + ray.direction * t;
            Intersection {
                distance: t,
                position: pos,
                normal: (pos - self.center) / self.radius,
                albedo: self.albedo,
                emittance: self.emittance,
                material: self.material,
                index_of_refraction: self.index_of_refraction,
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sphere_intersection() {
        let sphere = Sphere {
            center: Vector([1., 1., 5.]),
            radius: 1.5,
            albedo: Color::new(0., 0., 0.),
            emittance: Color::new(0., 0., 0.),
            material: ReflectanceType::Diffuse,
            index_of_refraction: 1.,
        };

        assert!(sphere
            .intersect(&Ray {
                origin: Vector([-1., -1., 0.]),
                direction: Vector([0., 0., 1.]).normalized(),
            })
            .is_none());

        assert!(sphere
            .intersect(&Ray {
                origin: Vector([1., 1., 0.]),
                direction: Vector([0., 0., 1.]).normalized(),
            })
            .is_some());

        assert!(sphere
            .intersect(&Ray {
                origin: Vector([3., 3., 0.]),
                direction: Vector([0., 0., 1.]).normalized(),
            })
            .is_none());
    }
}
