use crate::rng::engines::{Permuter, PolyDiscarder, Scrambler};
use crate::rng::engines::generic::GenericLinearGenerator;

macro_rules! easy_permuter {
    ($name:ident, $a:literal, $b:literal, $c:literal) => {
        impl Permuter<u64, 2> for $name {
            fn permute(state: [u64; 2]) -> [u64; 2] { detail::permute_xoroshiro($a, $b, $c, state) }
        }
    };
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xoroshiro128PScrambler;

impl Scrambler<u64, 2> for Xoroshiro128PScrambler {
    fn scramble(state: &[u64; 2]) -> u64 {
        state[0].wrapping_add(state[1])
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xoroshiro128PPScrambler;

impl Scrambler<u64, 2> for Xoroshiro128PPScrambler {
    fn scramble(state: &[u64; 2]) -> u64 {
        state[0].wrapping_add(state[1]).rotate_left(17).wrapping_add(state[0])
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xoroshiro128SSScrambler;

mod detail {
    pub fn permute_xoroshiro(a: u32, b: u32, c: u32, mut state: [u64; 2]) -> [u64; 2] {
        let s_0 = state[0];
        let s_1 = state[0] ^ state[1];

        state[0] = s_0.rotate_left(a) ^ s_1 ^ (s_1.checked_shl(b).unwrap());
        state[1] = s_1.rotate_left(c);

        state
    }
}

impl Scrambler<u64, 2> for Xoroshiro128SSScrambler {
    fn scramble(state: &[u64; 2]) -> u64 {
        state[0].wrapping_mul(5).rotate_left(7).wrapping_mul(9)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xoroshiro128PSPermuter;

easy_permuter!(Xoroshiro128PSPermuter, 24, 16, 37);

impl PolyDiscarder<u64, 2> for Xoroshiro128PSPermuter {
    fn polynomials() -> &'static [(usize, [u64; 2])] {
        &[
            (96, [0xd2a98b26625eee7b_u64, 0xdddf9b1090aa7ac1_u64]),
            (64, [0xdf900294d8f554a5_u64, 0x170865df4b3201fc_u64]),
        ]
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xoroshiro128PPPermuter;

easy_permuter!(Xoroshiro128PPPermuter, 49, 21, 28);

impl PolyDiscarder<u64, 2> for Xoroshiro128PPPermuter {
    fn polynomials() -> &'static [(usize, [u64; 2])] {
        &[
            (96, [0x360fd5f2cf8d5d99_u64, 0x9c6e6877736c46e3_u64]),
            (64, [0x2bd7a6a6e99c2ddc_u64, 0x0992ccaf6a6fca05_u64]),
        ]
    }
}

pub type Xoroshiro128P = GenericLinearGenerator<u64, 2, Xoroshiro128PSPermuter, Xoroshiro128PScrambler, Xoroshiro128PSPermuter>;
pub type Xoroshiro128SS = GenericLinearGenerator<u64, 2, Xoroshiro128PSPermuter, Xoroshiro128SSScrambler, Xoroshiro128PSPermuter>;
pub type Xoroshiro128PP = GenericLinearGenerator<u64, 2, Xoroshiro128PPPermuter, Xoroshiro128PPScrambler, Xoroshiro128PPPermuter>;

#[cfg(test)]
mod tests {
    use super::super::test_tables;
    use crate::rng::*;
    use super::*;

    #[test]
    fn test_xoroshiro128p_consistency() {
        let mut generator = Xoroshiro128P::new();
        test_tables::TABLE_XOROSHIRO128P_SEED_0.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128P_SEED_1.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128P_SEED_2.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128P_SEED_0.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128P_SEED_1.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128P_SEED_2.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128P_SEED_0.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128P_SEED_1.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128P_SEED_2.test(&mut generator);
    }

    #[test]
    fn test_xoroshiro128pp_consistency() {
        let mut generator = Xoroshiro128PP::new();
        test_tables::TABLE_XOROSHIRO128PP_SEED_0.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128PP_SEED_1.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128PP_SEED_2.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128PP_SEED_0.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128PP_SEED_1.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128PP_SEED_2.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128PP_SEED_0.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128PP_SEED_1.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128PP_SEED_2.test(&mut generator);
    }

    #[test]
    fn test_xoroshiro128ss_consistency() {
        let mut generator = Xoroshiro128SS::new();
        test_tables::TABLE_XOROSHIRO128SS_SEED_0.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128SS_SEED_1.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128SS_SEED_2.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128SS_SEED_0.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128SS_SEED_1.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128SS_SEED_2.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128SS_SEED_0.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128SS_SEED_1.test(&mut generator);
        test_tables::TABLE_XOROSHIRO128SS_SEED_2.test(&mut generator);
    }

    #[test]
    fn test_xoroshiro128p_jump_consistency() {
        let mut generator = Xoroshiro128P::new();
        let discard_64 = |engine: &mut Xoroshiro128P| engine.discard(64);
        let discard_96 = |engine: &mut Xoroshiro128P| engine.discard(96);

        test_tables::JUMP_TABLE_XOROSHIRO128P_64.test(&mut generator, discard_64);
        test_tables::JUMP_TABLE_XOROSHIRO128P_96.test(&mut generator, discard_96);
        test_tables::JUMP_TABLE_XOROSHIRO128P_64.test(&mut generator, discard_64);
        test_tables::JUMP_TABLE_XOROSHIRO128P_96.test(&mut generator, discard_96);
        test_tables::JUMP_TABLE_XOROSHIRO128P_64.test(&mut generator, discard_64);
        test_tables::JUMP_TABLE_XOROSHIRO128P_96.test(&mut generator, discard_96);
    }

    #[test]
    fn test_xoroshiro128pp_jump_consistency() {
        let mut generator = Xoroshiro128PP::new();
        let discard_64 = |engine: &mut Xoroshiro128PP| engine.discard(64);
        let discard_96 = |engine: &mut Xoroshiro128PP| engine.discard(96);

        test_tables::JUMP_TABLE_XOROSHIRO128PP_64.test(&mut generator, discard_64);
        test_tables::JUMP_TABLE_XOROSHIRO128PP_96.test(&mut generator, discard_96);
        test_tables::JUMP_TABLE_XOROSHIRO128PP_64.test(&mut generator, discard_64);
        test_tables::JUMP_TABLE_XOROSHIRO128PP_96.test(&mut generator, discard_96);
        test_tables::JUMP_TABLE_XOROSHIRO128PP_64.test(&mut generator, discard_64);
        test_tables::JUMP_TABLE_XOROSHIRO128PP_96.test(&mut generator, discard_96);
    }

    #[test]
    fn test_xoroshiro128ss_jump_consistency() {
        let mut generator = Xoroshiro128SS::new();
        let discard_64 = |engine: &mut Xoroshiro128SS| engine.discard(64);
        let discard_96 = |engine: &mut Xoroshiro128SS| engine.discard(96);

        test_tables::JUMP_TABLE_XOROSHIRO128SS_64.test(&mut generator, discard_64);
        test_tables::JUMP_TABLE_XOROSHIRO128SS_96.test(&mut generator, discard_96);
        test_tables::JUMP_TABLE_XOROSHIRO128SS_64.test(&mut generator, discard_64);
        test_tables::JUMP_TABLE_XOROSHIRO128SS_96.test(&mut generator, discard_96);
        test_tables::JUMP_TABLE_XOROSHIRO128SS_64.test(&mut generator, discard_64);
        test_tables::JUMP_TABLE_XOROSHIRO128SS_96.test(&mut generator, discard_96);
    }
}
