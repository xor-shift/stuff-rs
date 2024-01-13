use std::num::NonZeroUsize;

use super::{distributions::NormalDistribution, RandomNumberEngine};

struct BinBuilder {
    start: f64,
    end: f64,
    // bin_size: f64,

    bins: Vec<usize>,
    underflows: usize,
    overflows: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bin {
    Underflow,
    Regular(usize),
    Overflow,
}

impl BinBuilder {
    pub fn new_range(start: f64, end: f64, bin_count: NonZeroUsize) -> Self {
        // let value_range = end - start;

        Self {
            start,
            end,
            // bin_size: value_range / (bin_count.get() as f64),
            bins: vec![0usize; bin_count.get()],
            underflows: 0,
            overflows: 0,
        }
    }

    fn decide_bin(&self, val: f64) -> Bin {
        let canonicalised = (val - self.start) / (self.end - self.start);
        let canoical_steps_per_bin = 1f64 / (self.bins.len() as f64);
        let steps_needed = canonicalised / canoical_steps_per_bin;
        let steps_needed = steps_needed.floor();

        return if steps_needed < 0f64 {
            Bin::Underflow
        } else if steps_needed >= (self.bins.len() as f64) {
            Bin::Overflow
        } else {
            Bin::Regular(steps_needed as usize)
        };
    }

    pub fn insert(&mut self, val: f64) -> (Bin, usize) {
        let bin = self.decide_bin(val);
        let target = match bin {
            Bin::Underflow => &mut self.underflows,
            Bin::Overflow => &mut self.overflows,
            Bin::Regular(idx) => &mut self.bins[idx],
        };

        *target += 1;

        (bin, *target)
    }

    pub fn get(&self, bin: Bin) -> usize {
        match bin {
            Bin::Underflow => self.underflows,
            Bin::Overflow => self.overflows,
            Bin::Regular(idx) => self.bins[idx],
        }
    }

    #[allow(dead_code)]
    pub fn get_for(&self, val: f64) -> usize { self.get(self.decide_bin(val)) }

    #[allow(dead_code)]
    pub fn print(&self, height: usize) {
        // let max_digits = self.bins.len().ilog10() + 1;

        let max_bin_height = *self.bins.iter().max().unwrap();

        for (bin_idx, count) in self.bins.as_slice().into_iter().enumerate() {
            let norm_height = *count as f64 / max_bin_height as f64;
            let cur_height = norm_height * height as f64;

            let foo = std::iter::repeat('#').take(cur_height as usize).chain(std::iter::repeat(' ').take(height - cur_height as usize)).collect::<String>();

            println!("{0:04}: {1:}", bin_idx, foo);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &usize)> { self.bins.iter().enumerate() }
}

#[test]
fn test_bin_builder() {
    // -2.0 to -1.2
    // -1.2 to  0.6
    //  0.6 to  2.4
    //  2.4 to  3.2
    //  3.2 to  4.0
    let mut bin_builder = BinBuilder::new_range(-2f64, 2f64, 5.try_into().unwrap());

    assert_eq!(bin_builder.insert(-1.1999f64), (Bin::Regular(1), 1));
    assert_eq!(bin_builder.insert(-1.1999f64), (Bin::Regular(1), 2));
    assert_eq!(bin_builder.insert(-1.2001f64), (Bin::Regular(0), 1));
    assert_eq!(bin_builder.insert(-1.2001f64), (Bin::Regular(0), 2));
    assert_eq!(bin_builder.insert(-1.9999f64), (Bin::Regular(0), 3));
    assert_eq!(bin_builder.insert(-2.0001f64), (Bin::Underflow, 1));

    assert_eq!(bin_builder.insert(2.0001f64), (Bin::Overflow, 1));
    assert_eq!(bin_builder.insert(1.9999f64), (Bin::Regular(4), 1));
    assert_eq!(bin_builder.insert(1.9998f64), (Bin::Regular(4), 2));
}

fn average_square_error<Discrete, Continuous>(range: Discrete, cont_fn: Continuous) -> f64
where
    Discrete: Iterator<Item = (f64, f64)>,
    Continuous: Fn(f64) -> f64,
{
    let (sample_count, sum_of_errors) = range
        .map(|(x, y)| {
            let expected_y = cont_fn(x);
            (expected_y - y).powi(2)
        })
        .enumerate()
        .fold((0, 0f64), |acc, v| (v.0, acc.1 + v.1));

    return sum_of_errors / (sample_count as f64);
}

fn test_bins<G: FnMut() -> f64, Continuous: Fn(f64) -> f64>(generate_fn: &mut G, reference_fn: Continuous, num_iters: usize, num_bins: usize, range_start: f64, range_end: f64) {
    let mut bins = BinBuilder::new_range(range_start, range_end, num_bins.try_into().unwrap());

    for _ in 0..num_iters {
        bins.insert(generate_fn());
    }

    let max_bin_height = bins.iter().fold(0usize, |a, b| std::cmp::max(a, *b.1));
    let normalisation_multiplier = 1f64 / max_bin_height as f64;

    let samples = std::iter::repeat(std::iter::once(())) //
        .take(num_bins)
        .enumerate()
        .map(|(idx, _)| {
            let x_param = (2 * idx + 1) as f64 / (2 * num_bins) as f64;
            let x = x_param * (range_end - range_start) + range_start;

            (x, bins.get(Bin::Regular(idx)) as f64 * normalisation_multiplier)
        });

    // bins.print(24);

    let mean_square_error = average_square_error(samples, reference_fn);

    assert!(mean_square_error < 0.05f64);
}

#[test]
fn test_canonical_bins() {
    let mut generator = crate::rng::engines::Xoroshiro128P::new();
    generator.seed_from_result(0xDEADBEEF_CAFEBABE_u64);

    use crate::rng::distributions::GenerateCanonical;

    let mut generate_fn = move || -> f64 { <f64 as GenerateCanonical<_>>::generate_canonical(&mut generator).get() };

    test_bins(&mut generate_fn, |_| 1f64, 16384, 32, 0f64, 1f64);
}

#[test]
fn test_normal_bins() {
    use crate::rng::RandomNumberDistribution;

    let mut generator = crate::rng::engines::Xoroshiro128P::new();
    generator.seed_from_result(0xDEADBEEF_CAFEBABE_u64);

    let mut distribution = NormalDistribution::<f64>::new();

    let mut generate_fn = move || -> f64 { distribution.generate(&mut generator) };

    let norm_dist_pdf = |x: f64| (1f64 / (2f64 * std::f64::consts::PI).sqrt()) * std::f64::consts::E.powf(-0.5f64 * x.powi(2)) / 0.398942280401f64;

    test_bins(&mut generate_fn, norm_dist_pdf, 16384, 32, -2f64, 2f64);
}
