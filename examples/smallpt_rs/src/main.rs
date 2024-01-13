#![feature(sync_unsafe_cell)]

mod camera;
mod color;
mod ray;

mod sphere;

use std::sync::{Arc, Mutex};

use camera::*;
use color::*;
use ray::*;

use rng::distributions::sphere::NDSampler;
use rng::distributions::GenerateCanonical;
use rng::*;
use smallvec::*;
use stuff::*;

const SPHERES: [sphere::Sphere<f64>; 8] = [
    // light
    sphere::Sphere {
        center: Vector([0., 42.499, 15.]),
        radius: 40.,
        albedo: Color::new(0., 0., 0.),
        emittance: Color(Vector::new_explode(12.)),
        material: ray::ReflectanceType::Diffuse,
        index_of_refraction: 1.,
    },
    // mirror
    sphere::Sphere {
        center: Vector([-1.75, -2.5 + 0.9, 17.5]),
        radius: 0.9,
        albedo: Color(Vector::new_explode(0.99)),
        emittance: Color(Vector::new_explode(0.)),
        material: ray::ReflectanceType::PerfectMirror,
        index_of_refraction: 1.,
    },
    // glass
    sphere::Sphere {
        center: Vector([1.75, -2.5 + 0.9 + 0.2, 16.5]),
        radius: 0.9,
        albedo: Color(Vector::new_explode(0.99)),
        emittance: Color(Vector::new_explode(0.)),
        material: ray::ReflectanceType::Dielectric,
        index_of_refraction: 1.7,
    },
    // ceiling
    sphere::Sphere {
        center: Vector([0., 5000., 5.]),
        radius: 4997.5,
        albedo: Color::new(0.75, 0.75, 0.75),
        emittance: Color::new(0., 0., 0.),
        material: ray::ReflectanceType::Diffuse,
        index_of_refraction: 1.,
    },
    // floor
    sphere::Sphere {
        center: Vector([0., -5000., 5.]),
        radius: 4997.5,
        albedo: Color::new(0.75, 0.75, 0.75),
        emittance: Color::new(0., 0., 0.),
        material: ray::ReflectanceType::Diffuse,
        index_of_refraction: 1.,
    },
    // backwall
    sphere::Sphere {
        center: Vector([0., 0., 5000.]),
        radius: 4980.,
        albedo: Color::new(0.75, 0.75, 0.75),
        emittance: Color::new(0., 0., 0.),
        material: ray::ReflectanceType::Diffuse,
        index_of_refraction: 1.,
    },
    // right wall
    sphere::Sphere {
        center: Vector([5000., 0., 0.]),
        radius: 4996.5,
        albedo: Color::new(0.25, 0.25, 0.75),
        emittance: Color::new(0., 0., 0.),
        material: ray::ReflectanceType::Diffuse,
        index_of_refraction: 1.,
    },
    // left wall
    sphere::Sphere {
        center: Vector([-5000., 0., 0.]),
        radius: 4996.5,
        albedo: Color::new(0.75, 0.25, 0.25),
        emittance: Color::new(0., 0., 0.),
        material: ray::ReflectanceType::Diffuse,
        index_of_refraction: 1.,
    },
];

#[derive(Clone)]
struct RenderConfiguration {
    camera: PinholeCamera<f64>,
    dimensions: (usize, usize),
    samples: usize,
}

fn trace_iterative<T: Intersectable<f64>, Gen: stuff::rng::UniformRandomBitGenerator>(mut ray: Ray<f64>, scene: &T, gen: &mut Gen) -> Color<f64> {
    let mut attenuation = Vector::new_explode(1.);
    let mut light = Vector::new_explode(0.);

    for depth in 0.. {
        let intersection = if let Some(intersection) = scene.intersect(&ray) {
            intersection
        } else {
            break;
        };

        let going_in = ray.direction.dot(intersection.normal) < 0.;
        let oriented_normal = if going_in { intersection.normal } else { -intersection.normal };
        let wo = -ray.direction;

        let (wi, cos_brdf_over_wi_pdf) = match intersection.material {
            ReflectanceType::Diffuse => {
                let (wi, wi_pdf) = stuff::rng::distributions::sphere::UniformSphereSampler::new().sample(gen);
                let wi = Vector(wi);
                let wi = if wi.dot(intersection.normal) >= 0. { wi } else { -wi };
                let wi_pdf = wi_pdf * 2.;
                let brdf = 0.5 / std::f64::consts::PI;
                (wi, wi.dot(intersection.normal).abs() * brdf / wi_pdf)
            }
            ReflectanceType::PerfectMirror => {
                let wi = ray::reflect(wo, intersection.normal);
                (wi, 1.) // specular. the brdf and the pdf cancel eachother out too
            }
            ReflectanceType::Dielectric => {
                let (transmittant_index, incident_index) = if going_in { (intersection.index_of_refraction, 1.) } else { (1., intersection.index_of_refraction) };

                if let Some((refraction, p_refraction)) = ray::refract(wo, oriented_normal, incident_index, transmittant_index) {
                    let generated = f64::generate_canonical(gen).get();

                    if generated < p_refraction {
                        // the brdf and the pdf are the same, they hence cancel eachother
                        (refraction, 1.)
                    } else {
                        (ray::reflect(wo, oriented_normal), 1.)
                    }
                } else {
                    (ray::reflect(wo, oriented_normal), 1.)
                }
            }
        };

        ray = Ray {
            origin: intersection.position + wi * 0.000001,
            direction: wi,
        };

        light = light + intersection.emittance.0 * attenuation;

        let cur_attenuation = intersection.albedo.0 * cos_brdf_over_wi_pdf;
        attenuation = cur_attenuation * attenuation;

        if depth > 4 && attenuation.length() < 0.2 {
            let p = attenuation[0].max(attenuation[1]).max(attenuation[2]);
            if p == 0. {
                break;
            }

            if f64::generate_canonical(gen).get() < p {
                attenuation = attenuation / (1. - p);
            } else {
                break;
            }
        }
    }

    Color(light)
}

#[allow(dead_code)]
fn kernel_preview<CameraType: Camera<f64>, Gen: stuff::rng::UniformRandomBitGenerator>(pixel_cooridnates: (usize, usize), gen: &mut Gen, camera: &CameraType, samples: usize) -> Color<f64> {
    let res = (0..samples)
        .map(|_| {
            let (ray, _ray_pdf) = camera.generate_ray(pixel_cooridnates, gen);
            if let Some(intersection) = SPHERES.intersect(&ray) {
                intersection.albedo.0 * intersection.normal.dot(ray.direction).abs()
            } else {
                Vector::new_explode(0.)
            }
        })
        .fold(Vector::new_explode(0.), |acc, v| acc + v)
        / samples as f64;

    color::Color(res)
}

#[allow(dead_code)]
fn kernel<CameraType: Camera<f64>, Gen: stuff::rng::UniformRandomBitGenerator>(pixel_cooridnates: (usize, usize), gen: &mut Gen, camera: &CameraType, samples: usize) -> Color<f64> {
    let res = (0..samples)
        .map(|_| {
            let (ray, ray_pdf) = camera.generate_ray(pixel_cooridnates, gen);
            trace_iterative(ray, &SPHERES, gen).0 / ray_pdf
        })
        .fold(Vector::new_explode(0.), |acc, v| acc + v)
        / samples as f64;

    color::Color(res)
}

#[allow(dead_code)]
fn single_threaded(config: &RenderConfiguration) {
    let mut rd = stuff::rng::engines::RandomDevice::new();
    let mut generator = stuff::rng::engines::Xoshiro256PP::new();
    generator.seed_from_result(rd.generate());

    let pixels = (0..config.dimensions.1) //
        .map(|y| (0..config.dimensions.0).map(move |x| (x, y)))
        .flatten()
        .map(|pixel_coords| kernel(pixel_coords, &mut generator, &config.camera, config.samples));

    let image = stuff::qoi::Image::from_pixels_it(config.dimensions.0 as u32, config.dimensions.1 as u32, pixels.map(|v| v.to_qoi_color()));
    let mut file = std::fs::File::create("./out.qoi").expect("failed to open out.qoi");
    image.encode_to_writer(&mut file).expect("failed to write to out.qoi");
}

#[allow(dead_code)]
fn multi_threaded(config: RenderConfiguration) {
    struct Product {
        for_row: usize,
        pixels: Vec<Color<f64>>,
    }

    let (sender, receiver) = std::sync::mpsc::channel();
    let row = Arc::new(Mutex::new(0usize));

    let mut workers = Vec::new();

    let mut rd = stuff::rng::engines::RandomDevice::new();
    let mut base_generator = stuff::rng::engines::Xoshiro256PP::new();
    base_generator.seed_from_result(rd.generate());

    let config = Arc::new(config);

    for _ in 0..std::thread::available_parallelism().unwrap().get() {
        let row = row.clone();
        let sender = sender.clone();
        let config = config.clone();

        base_generator.discard(192);
        let mut generator = base_generator.clone();

        workers.push(std::thread::spawn(move || loop {
            let row = {
                let mut row = row.lock().unwrap();
                *row += 1;
                *row - 1
            };

            if row >= config.dimensions.1 {
                break;
            }

            let pixels = (0..config.dimensions.0).map(|x| kernel((x, row), &mut generator, &config.camera, config.samples)).collect();

            sender.send(Product { for_row: row, pixels }).unwrap();
        }));
    }

    drop(sender);

    let mut image = stuff::qoi::Image::new(config.dimensions.0 as u32, config.dimensions.1 as u32);
    while let Ok(product) = receiver.recv() {
        println!("processing row {}", product.for_row);

        for (x, pix) in product.pixels.into_iter().enumerate() {
            *image.pixel_mut(x, product.for_row) = pix.to_qoi_color();
        }
    }

    workers.into_iter().for_each(|v| v.join().unwrap());

    let mut file = std::fs::File::create("./out.qoi").expect("failed to open out.qoi");
    image.encode_to_writer(&mut file).expect("failed to write to out.qoi");
}

pub fn main() {
    let dimensions = (64 * 4, 36 * 4);

    let config = RenderConfiguration {
        camera: PinholeCamera::new(Vector([0., 0.5, -1.]), dimensions, 45. * std::f64::consts::PI / 180.),
        dimensions: dimensions,
        samples: 128,
    };

    multi_threaded(config);
}
