// this is terrible
// TODO: rewrite

use std::io::Read;

use crate::rng::UniformRandomBitGenerator;

pub struct RandomDevice {
    rand_file: Option<std::fs::File>,
    last_entropy: u64,
}

impl RandomDevice {
    pub fn new() -> Self { Self { rand_file: None, last_entropy: 0 } }

    fn strat_file(&mut self) -> Option<u64> {
        if let Some(rand_file) = &mut self.rand_file {
            let mut buf = [0; 8];
            if let Ok(_) = rand_file.read_exact(&mut buf) {
                Some(u64::from_le_bytes(buf))
            } else {
                self.rand_file = None;
                None
            }
        } else {
            None
        }
    }

    fn rdx_impl<const N: usize, T: Sized + Default + Copy, Fun: Fn(&mut T) -> i32>(fun: Fun) -> Option<[T; N]> {
        let mut ret = [Default::default(); N];

        for i in 0..N {
            let res = fun(&mut ret[i]);
            if res != 1 {
                return None;
            }
        }

        Some(ret)
    }

    fn strat_rdx_impl<Fun16, Fun32, Fun64>(fun_16: Fun16, fun_32: Fun32, fun_64: Fun64) -> Option<u64>
    where
        Fun16: Fn(&mut u16) -> i32,
        Fun32: Fn(&mut u32) -> i32,
        Fun64: Fn(&mut u64) -> i32,
    {
        if let Some(v) = Self::rdx_impl::<1, _, _>(fun_64) {
            return Some(v[0]);
        }

        if let Some(v) = Self::rdx_impl::<2, _, _>(fun_32) {
            return Some(((v[0] as u64) << 32) | (v[1] as u64));
        }

        if let Some(v) = Self::rdx_impl::<4, _, _>(fun_16) {
            return Some(((v[0] as u64) << 48) | ((v[1] as u64) << 32) | ((v[2] as u64) << 16) | (v[3] as u64));
        }

        None
    }

    fn strat_rdseed() -> Option<u64> {
        Self::strat_rdx_impl(
            |v| unsafe { core::arch::x86_64::_rdseed16_step(v) },
            |v| unsafe { core::arch::x86_64::_rdseed32_step(v) },
            |v| unsafe { core::arch::x86_64::_rdseed64_step(v) },
        )
    }

    fn strat_rdrand() -> Option<u64> {
        Self::strat_rdx_impl(
            |v| unsafe { core::arch::x86_64::_rdrand16_step(v) },
            |v| unsafe { core::arch::x86_64::_rdrand32_step(v) },
            |v| unsafe { core::arch::x86_64::_rdrand64_step(v) },
        )
    }

    fn strat_fallback(&mut self) -> Option<u64> { None }
}

impl UniformRandomBitGenerator for RandomDevice {
    type ResultType = u64;

    fn min() -> Self::ResultType { u64::MIN }
    fn max() -> Self::ResultType { u64::MAX }

    fn generate(&mut self) -> Self::ResultType {
        if let Some(res) = self.strat_file() {
            res
        } else if let Some(res) = Self::strat_rdseed() {
            res
        } else if let Some(res) = Self::strat_rdrand() {
            res
        } else if let Some(res) = self.strat_fallback() {
            res
        } else {
            panic!("failed to generate a random number through any of the strategies")
        }
    }
}
