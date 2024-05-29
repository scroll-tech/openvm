use crate::range_gate::RangeCheckerGateChip;

use afs_stark_backend::interaction::Interaction;
use columns::SortedLimbsCols;
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

/**
 * This Chip constrains that consecutive rows are sorted lexicographically.
 *
 * Each row consists of a key decomposed into limbs, and the chip constrains
 * each limb has at most limb_bits bits, where limb_bits is at most 31. It
 * does this by interacting with a RangeCheckerGateChip. Because the range checker
 * gate can take MAX up to 2^20, we further decompose each limb into sublimbs
 * of size decomp bits.
 */
#[derive(Default)]
pub struct SortedLimbsChip<const MAX: u32> {
    bus_index: usize,
    limb_bits: usize,
    decomp: usize,
    key_vec_len: usize,
    keys: Vec<Vec<u32>>,

    pub range_checker_gate: RangeCheckerGateChip<MAX>,
}

impl<const MAX: u32> SortedLimbsChip<MAX> {
    pub fn new(
        bus_index: usize,
        limb_bits: usize,
        decomp: usize,
        key_vec_len: usize,
        keys: Vec<Vec<u32>>,
    ) -> Self {
        Self {
            bus_index,
            limb_bits,
            decomp,
            key_vec_len,
            keys,
            range_checker_gate: RangeCheckerGateChip::<MAX>::new(bus_index),
        }
    }

    pub fn bus_index(&self) -> usize {
        self.bus_index
    }

    pub fn limb_bits(&self) -> usize {
        self.limb_bits
    }

    pub fn decomp(&self) -> usize {
        self.decomp
    }

    pub fn key_vec_len(&self) -> usize {
        self.key_vec_len
    }

    pub fn keys(&self) -> Vec<Vec<u32>> {
        self.keys.clone()
    }

    pub fn sends_custom<F: PrimeField64>(
        &self,
        cols: SortedLimbsCols<usize>,
    ) -> Vec<Interaction<F>> {
        // num_limbs is the number of sublimbs per limb of key, not including the
        // shifted last sublimb
        let num_limbs = (self.limb_bits() + self.decomp() - 1) / self.decomp();
        let num_keys = self.key_vec_len();

        let mut interactions = vec![];

        for i in 0..num_keys {
            // add 1 to account for the shifted last sublimb
            for j in 0..(num_limbs + 1) {
                interactions.push(Interaction {
                    fields: vec![VirtualPairCol::single_main(cols.keys_decomp[i][j])],
                    count: VirtualPairCol::constant(F::one()),
                    argument_index: self.bus_index(),
                });
            }
        }

        interactions
    }
}
