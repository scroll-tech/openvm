use crate::less_than::LessThanChip;

use afs_stark_backend::interaction::Interaction;
use columns::AssertSortedCols;
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

#[cfg(test)]
pub mod tests;

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
 *
 * The AssertSortedChip contains a LessThanChip subchip, which is used to constrain
 * that the rows are sorted lexicographically.
 */
#[derive(Default)]
pub struct AssertSortedChip<const MAX: u32> {
    less_than_chip: LessThanChip<MAX>,
}

impl<const MAX: u32> AssertSortedChip<MAX> {
    pub fn new(
        bus_index: usize,
        limb_bits: usize,
        decomp: usize,
        key_vec_len: usize,
        keys: Vec<Vec<u32>>,
    ) -> Self {
        Self {
            less_than_chip: LessThanChip::<MAX>::new(
                bus_index,
                limb_bits,
                decomp,
                key_vec_len,
                keys,
            ),
        }
    }

    pub fn bus_index(&self) -> usize {
        self.less_than_chip.bus_index()
    }

    pub fn limb_bits(&self) -> usize {
        self.less_than_chip.limb_bits()
    }

    pub fn decomp(&self) -> usize {
        self.less_than_chip.decomp()
    }

    pub fn key_vec_len(&self) -> usize {
        self.less_than_chip.key_vec_len()
    }

    pub fn keys(&self) -> Vec<Vec<u32>> {
        self.less_than_chip.keys().clone()
    }

    pub fn sends_custom<F: PrimeField64>(
        &self,
        cols: &AssertSortedCols<usize>,
    ) -> Vec<Interaction<F>> {
        // num_limbs is the number of sublimbs per limb of key, not including the
        // shifted last sublimb
        let num_limbs = (self.limb_bits() + self.decomp() - 1) / self.decomp();
        let num_keys = self.key_vec_len();

        let mut interactions = vec![];

        // we will range check the decomposed limbs of the key
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
