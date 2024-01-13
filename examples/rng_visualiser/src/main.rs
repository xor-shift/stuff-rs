#![feature(const_float_bits_conv)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_for)]
#![feature(const_maybe_uninit_as_mut_ptr)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod colormap;

use stuff::rng::*;

#[derive(Clone, Copy)]
struct VisualisationConfiguration {
    pub image_dims: (u32, u32),
    pub vis_x_range: (f64, f64),
    pub vis_y_range: (f64, f64),
}

fn visualise<Fun: FnMut() -> (f64, f64)>(mut fun: Fun, config: &VisualisationConfiguration, samples: usize) -> stuff::qoi::Image {
    let bin_it = |val: f64, bin_count: u32, range: (f64, f64)| -> u32 {
        let t = (val - range.0) / (range.1 - range.0);

        if t < 0f64 {
            0
        } else if t >= 1f64 {
            bin_count + 1
        } else {
            (t * bin_count as f64).trunc() as u32 + 1
        }
    };

    let bins = (config.image_dims.0 - 2, config.image_dims.1 - 2);

    let mut generate_bin = move || {
        let sample = fun();

        let bin_x = bin_it(sample.0, bins.0, config.vis_x_range);
        let bin_y = bin_it(sample.1, bins.1, config.vis_y_range);

        (bin_x, bin_y)
    };

    let mut bins = vec![0usize; config.image_dims.0 as usize * config.image_dims.1 as usize];

    for _ in 0..samples {
        let bin = generate_bin();
        bins[bin.1 as usize * config.image_dims.0 as usize + bin.0 as usize] += 1;
    }

    let (min, max) = (*bins.iter().min().unwrap(), *bins.iter().max().unwrap());

    let image = stuff::qoi::Image::from_pixels_it(
        config.image_dims.0,
        config.image_dims.1,
        bins.iter().map(|&v| {
            let color = colormap::map_to_color((v - min) as f64 / (max - min) as f64, &colormap::COLORMAP_TURBO);
            stuff::qoi::Color::from_rgba_bytes([
                (color.0 * 255f64).round().clamp(0f64, 255f64) as u8,
                (color.1 * 255f64).round().clamp(0f64, 255f64) as u8,
                (color.2 * 255f64).round().clamp(0f64, 255f64) as u8,
                255,
            ])
        }),
    );

    image
}

fn quick_visualise_2d<Sampler: stuff::rng::distributions::sphere::NDSampler<f64, 2>>(random_seed: u64, sampler_name: &str, sampler: &mut Sampler) {
    let mut generator = stuff::rng::engines::Xoshiro256PP::new();
    generator.seed_from_result(random_seed);

    let image = visualise(
        || (sampler.sample(&mut generator).0).into(),
        &VisualisationConfiguration {
            image_dims: (512, 512),
            vis_x_range: (-1.5f64, 1.5f64),
            vis_y_range: (-1.5f64, 1.5f64),
        },
        8192,
    );

    let mut file = std::fs::File::create(format!("out/d2_xy_{}.qoi", sampler_name)).unwrap();
    image.encode_to_writer(&mut file).unwrap();
}

fn quick_visualise_3d<Sampler: stuff::rng::distributions::sphere::NDSampler<f64, 3>>(random_seed: u64, sampler_name: &str, sampler: &mut Sampler) {
    let mut generator = stuff::rng::engines::Xoshiro256PP::new();
    generator.seed_from_result(random_seed);

    let config = VisualisationConfiguration {
        image_dims: (512, 512),
        vis_x_range: (-1.5f64, 1.5f64),
        vis_y_range: (-1.5f64, 1.5f64),
    };

    let mut sample_2d = |indices: [usize; 2]| {
        let res = sampler.sample(&mut generator).0;
        [res[indices[0]], res[indices[1]]]
    };

    let image_xy = visualise(|| sample_2d([0, 1]).into(), &config, 8192);
    let image_yz = visualise(|| sample_2d([1, 2]).into(), &config, 8192);
    let image_xz = visualise(|| sample_2d([0, 2]).into(), &config, 8192);

    image_xy.encode_to_writer(&mut std::fs::File::create(format!("out/d3_xy_{}.qoi", sampler_name)).unwrap()).unwrap();
    image_yz.encode_to_writer(&mut std::fs::File::create(format!("out/d3_yz_{}.qoi", sampler_name)).unwrap()).unwrap();
    image_xz.encode_to_writer(&mut std::fs::File::create(format!("out/d3_xz_{}.qoi", sampler_name)).unwrap()).unwrap();
}

pub fn main() {
    let mut rd = stuff::rng::engines::RandomDevice::new();
    println!("here, have a random number: {}", rd.generate());

    let mut generator = stuff::rng::engines::Xoshiro256PP::new();
    generator.seed_from_result(rd.generate());

    quick_visualise_2d(rd.generate(), "uniform_sphere", &mut stuff::rng::distributions::sphere::UniformSphereSampler::<f64, 2>::new());
    quick_visualise_2d(rd.generate(), "uniform_ball", &mut stuff::rng::distributions::sphere::UniformBallSampler::<f64, 2>::new());
    quick_visualise_3d(rd.generate(), "uniform_sphere", &mut stuff::rng::distributions::sphere::UniformSphereSampler::<f64, 3>::new());
    quick_visualise_3d(rd.generate(), "cos_hemisphere", &mut stuff::rng::distributions::sphere::CosineWeightedHemisphereSampler::<f64>::new());
    quick_visualise_3d(rd.generate(), "cos_pow0.5_hemisphere", &mut stuff::rng::distributions::sphere::PowerCosineWeightedHemisphereSampler::<f64>::new(0.564));
    quick_visualise_3d(rd.generate(), "cos_pow1_hemisphere", &mut stuff::rng::distributions::sphere::PowerCosineWeightedHemisphereSampler::<f64>::new(1f64));
    quick_visualise_3d(rd.generate(), "cos_pow1.5_hemisphere", &mut stuff::rng::distributions::sphere::PowerCosineWeightedHemisphereSampler::<f64>::new(1.5f64));
    quick_visualise_3d(rd.generate(), "cos_pow2_hemisphere", &mut stuff::rng::distributions::sphere::PowerCosineWeightedHemisphereSampler::<f64>::new(2f64));
    quick_visualise_3d(rd.generate(), "cos_pow4_hemisphere", &mut stuff::rng::distributions::sphere::PowerCosineWeightedHemisphereSampler::<f64>::new(4f64));
    quick_visualise_3d(rd.generate(), "cos_pow8_hemisphere", &mut stuff::rng::distributions::sphere::PowerCosineWeightedHemisphereSampler::<f64>::new(8f64));
    quick_visualise_3d(rd.generate(), "uniform_ball", &mut stuff::rng::distributions::sphere::UniformBallSampler::<f64, 3>::new());
}
