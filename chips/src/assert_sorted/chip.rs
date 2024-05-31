use crate::sub_chip::SubAirWithInteractions;

use super::columns::AssertedSortedCols;
use afs_stark_backend::interaction::{Chip, Interaction};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

use super::AssertedSortedChip;

impl<F: PrimeField64, const MAX: u32> Chip<F> for AssertedSortedChip<MAX> {
    fn sends(&self) -> Vec<Interaction<F>> {
        let num_cols =
            AssertedSortedCols::<F>::get_width(self.limb_bits(), self.decomp(), self.key_vec_len());
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_numbered = AssertedSortedCols::<F>::cols_numbered(
            &all_cols,
            self.limb_bits(),
            self.decomp(),
            self.key_vec_len(),
        );

        let mut interactions: Vec<Interaction<F>> = vec![];

        let num_limbs = (self.limb_bits() + self.decomp() - 1) / self.decomp();
        let num_keys = self.key_vec_len();

        // we will range check the decomposed limbs of the key
        for i in 0..num_keys {
            // add 1 to account for the shifted last sublimb
            for j in 0..(num_limbs + 1) {
                interactions.push(Interaction {
                    fields: vec![VirtualPairCol::single_main(cols_numbered.keys_decomp[i][j])],
                    count: VirtualPairCol::constant(F::one()),
                    argument_index: self.bus_index(),
                });
            }
        }

        // append the interactions from the subchip
        let mut less_than_interactions: Vec<Interaction<F>> =
            SubAirWithInteractions::<F>::sends(&self.less_than_chip, cols_numbered.less_than_cols);
        interactions.append(&mut less_than_interactions);

        interactions
    }
}
