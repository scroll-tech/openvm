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

    pub fn sends_custom<F: PrimeField64>(
        &self,
        cols: &AssertSortedCols<usize>,
    ) -> Vec<Interaction<F>> {
        // num_limbs is the number of sublimbs per limb of key, not including the
        // shifted last sublimb
        let num_limbs = (*self.less_than_chip.air.limb_bits() + *self.less_than_chip.air.decomp()
            - 1)
            / *self.less_than_chip.air.decomp();
        let num_keys = *self.less_than_chip.air.key_vec_len();

        let mut interactions = vec![];

        // we will range check the decomposed limbs of the key
        for i in 0..num_keys {
            // add 1 to account for the shifted last sublimb
            for j in 0..(num_limbs + 1) {
                interactions.push(Interaction {
                    fields: vec![VirtualPairCol::single_main(cols.keys_decomp[i][j])],
                    count: VirtualPairCol::constant(F::one()),
                    argument_index: *self.less_than_chip.bus_index(),
                });
            }
        }

        interactions
    }
}
