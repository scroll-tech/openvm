use crate::sub_chip::SubAirWithInteractions;

use super::columns::AssertSortedCols;
use afs_stark_backend::interaction::{Chip, Interaction};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

use super::AssertSortedChip;

impl<F: PrimeField64> Chip<F> for AssertSortedChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        let num_cols = AssertSortedCols::<F>::get_width(
            self.air.limb_bits().clone(),
            *self.air.decomp(),
            *self.air.key_vec_len(),
        );
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_numbered = AssertSortedCols::<usize>::from_slice(
            &all_cols,
            self.air.limb_bits().clone(),
            *self.air.decomp(),
            *self.air.key_vec_len(),
        );

        let mut interactions: Vec<Interaction<F>> = vec![];

        // we will range check the decomposed limbs of the key
        for i in 0..*self.air.key_vec_len() {
            let num_limbs = (self.air.limb_bits()[i] + *self.air.decomp() - 1) / *self.air.decomp();
            // add 1 to account for the shifted last sublimb
            for j in 0..(num_limbs + 1) {
                interactions.push(Interaction {
                    fields: vec![VirtualPairCol::single_main(cols_numbered.keys_decomp[i][j])],
                    count: VirtualPairCol::constant(F::one()),
                    argument_index: self.range_checker.bus_index(),
                });
            }
        }

        let subchip_interactions = SubAirWithInteractions::<F>::sends(
            &self.is_less_than_tuple_chip.air,
            cols_numbered.is_less_than_tuple_cols,
        );

        interactions.extend(subchip_interactions);

        interactions
    }
}
