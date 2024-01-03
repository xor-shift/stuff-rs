use crate::rng::engines::generic::GenericLinearGenerator;

use super::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xoshiro256Permuter;

impl Permuter<u64, 4> for Xoshiro256Permuter {
    fn permute(mut state: [u64; 4]) -> [u64; 4] {
        let t = state[1] << 17;

        state[2] ^= state[0];
        state[3] ^= state[1];
        state[1] ^= state[2];
        state[0] ^= state[3];

        state[2] ^= t;

        state[3] = state[3].rotate_left(45);

        return state;
    }
}

impl PolyDiscarder<u64, 4> for Xoshiro256Permuter {
    fn polynomials() -> &'static [(usize, [u64; 4])] {
        &[
            (192, [0x76e15d3efefdcbbf_u64, 0xc5004e441c522fb3_u64, 0x77710069854ee241_u64, 0x39109bb02acbe635_u64]),
            (128, [0x180ec6d33cfd0aba_u64, 0xd5a61266f0c9392c_u64, 0xa9582618e03fc9aa_u64, 0x39abdc4529b1661c_u64]),
        ]
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xoshiro256PScrambler;

impl Scrambler<u64, 4> for Xoshiro256PScrambler {
    fn scramble(state: &[u64; 4]) -> u64 {
        state[0].wrapping_add(state[3])
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xoshiro256PPScrambler;

impl Scrambler<u64, 4> for Xoshiro256PPScrambler {
    fn scramble(state: &[u64; 4]) -> u64 {
        state[0].wrapping_add(state[3]).rotate_left(23).wrapping_add(state[0])
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xoshiro256SSScrambler;

impl Scrambler<u64, 4> for Xoshiro256SSScrambler {
    fn scramble(state: &[u64; 4]) -> u64 {
        state[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9)
    }
}

pub type Xoshiro256P = GenericLinearGenerator<u64, 4, Xoshiro256Permuter, Xoshiro256PScrambler, Xoshiro256Permuter>;
pub type Xoshiro256PP = GenericLinearGenerator<u64, 4, Xoshiro256Permuter, Xoshiro256PPScrambler, Xoshiro256Permuter>;
pub type Xoshiro256SS = GenericLinearGenerator<u64, 4, Xoshiro256Permuter, Xoshiro256SSScrambler, Xoshiro256Permuter>;

#[cfg(test)]
mod tests {
    use super::super::test_tables;
    use crate::rng::*;
    use super::*;

    #[test]
    fn test_xoshiro256p_consistency() {
        let mut generator = Xoshiro256P::new();
        test_tables::TABLE_XOSHIRO256P_SEED_0.test(&mut generator);
        test_tables::TABLE_XOSHIRO256P_SEED_0.test(&mut generator);
        test_tables::TABLE_XOSHIRO256P_SEED_0.test(&mut generator);
    }

    #[test]
    fn test_xoshiro256pp_consistency() {
        let mut generator = Xoshiro256PP::new();
        test_tables::TABLE_XOSHIRO256PP_SEED_0.test(&mut generator);
        test_tables::TABLE_XOSHIRO256PP_SEED_0.test(&mut generator);
        test_tables::TABLE_XOSHIRO256PP_SEED_0.test(&mut generator);
    }

    #[test]
    fn test_xoshiro256ss_consistency() {
        let mut generator = Xoshiro256SS::new();
        test_tables::TABLE_XOSHIRO256SS_SEED_0.test(&mut generator);
        test_tables::TABLE_XOSHIRO256SS_SEED_0.test(&mut generator);
        test_tables::TABLE_XOSHIRO256SS_SEED_0.test(&mut generator);
    }

    #[test]
    fn test_xoshiro256p_jump_consistency() {
        let mut generator = Xoshiro256P::new();
        let discard_128 = |engine: &mut Xoshiro256P| engine.discard(128);
        let discard_192 = |engine: &mut Xoshiro256P| engine.discard(192);

        test_tables::JUMP_TABLE_XOSHIRO256P_128.test(&mut generator, discard_128);
        test_tables::JUMP_TABLE_XOSHIRO256P_192.test(&mut generator, discard_192);
        test_tables::JUMP_TABLE_XOSHIRO256P_128.test(&mut generator, discard_128);
        test_tables::JUMP_TABLE_XOSHIRO256P_192.test(&mut generator, discard_192);
        test_tables::JUMP_TABLE_XOSHIRO256P_128.test(&mut generator, discard_128);
        test_tables::JUMP_TABLE_XOSHIRO256P_192.test(&mut generator, discard_192);
    }

    #[test]
    fn test_xoshiro256pp_jump_consistency() {
        let mut generator = Xoshiro256PP::new();
        let discard_128 = |engine: &mut Xoshiro256PP| engine.discard(128);
        let discard_192 = |engine: &mut Xoshiro256PP| engine.discard(192);

        test_tables::JUMP_TABLE_XOSHIRO256PP_128.test(&mut generator, discard_128);
        test_tables::JUMP_TABLE_XOSHIRO256PP_192.test(&mut generator, discard_192);
        test_tables::JUMP_TABLE_XOSHIRO256PP_128.test(&mut generator, discard_128);
        test_tables::JUMP_TABLE_XOSHIRO256PP_192.test(&mut generator, discard_192);
        test_tables::JUMP_TABLE_XOSHIRO256PP_128.test(&mut generator, discard_128);
        test_tables::JUMP_TABLE_XOSHIRO256PP_192.test(&mut generator, discard_192);
    }

    #[test]
    fn test_xoshiro256ss_jump_consistency() {
        let mut generator = Xoshiro256SS::new();
        let discard_128 = |engine: &mut Xoshiro256SS| engine.discard(128);
        let discard_192 = |engine: &mut Xoshiro256SS| engine.discard(192);

        test_tables::JUMP_TABLE_XOSHIRO256SS_128.test(&mut generator, discard_128);
        test_tables::JUMP_TABLE_XOSHIRO256SS_192.test(&mut generator, discard_192);
        test_tables::JUMP_TABLE_XOSHIRO256SS_128.test(&mut generator, discard_128);
        test_tables::JUMP_TABLE_XOSHIRO256SS_192.test(&mut generator, discard_192);
        test_tables::JUMP_TABLE_XOSHIRO256SS_128.test(&mut generator, discard_128);
        test_tables::JUMP_TABLE_XOSHIRO256SS_192.test(&mut generator, discard_192);
    }
}
