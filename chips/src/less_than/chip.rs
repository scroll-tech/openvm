use crate::sub_chip::SubAirWithInteractions;

use super::columns::LessThanCols;
use afs_stark_backend::interaction::{Chip, Interaction};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

use super::LessThanChip;

impl<F: PrimeField64, const MAX: u32> Chip<F> for LessThanChip<MAX> {
    fn sends(&self) -> Vec<Interaction<F>> {
        let num_cols = LessThanCols::<F>::get_width(
            *self.air.limb_bits(),
            *self.air.decomp(),
            *self.air.key_vec_len(),
        );
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_numbered = LessThanCols::<usize>::from_slice(
            &all_cols,
            *self.air.limb_bits(),
            *self.air.decomp(),
            *self.air.key_vec_len(),
        );

        SubAirWithInteractions::sends(self, cols_numbered)
    }
}

impl<F: PrimeField64, const MAX: u32> SubAirWithInteractions<F> for LessThanChip<MAX> {
    fn sends(&self, col_indices: LessThanCols<usize>) -> Vec<Interaction<F>> {
        // num_limbs is the number of sublimbs per limb of key, not including the
        // shifted last sublimb
        let num_limbs = (*self.air.limb_bits() + *self.air.decomp() - 1) / *self.air.decomp();
        let num_keys = *self.air.key_vec_len();

        let mut interactions = vec![];

        // we range check the limbs of the lower_bits so that we know each element
        // of lower_bits has at most limb_bits bits
        for i in 0..num_keys {
            for j in 0..(num_limbs + 1) {
                interactions.push(Interaction {
                    fields: vec![VirtualPairCol::single_main(
                        col_indices.aux.lower_bits_decomp[i][j],
                    )],
                    count: VirtualPairCol::constant(F::one()),
                    argument_index: *self.bus_index(),
                });
            }
        }

        interactions
    }
}
