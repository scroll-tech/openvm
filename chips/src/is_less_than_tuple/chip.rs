use crate::{is_less_than::columns::IsLessThanCols, sub_chip::SubAirWithInteractions};
use afs_stark_backend::interaction::{Chip, Interaction};
use p3_field::PrimeField64;

use super::{columns::IsLessThanTupleCols, IsLessThanTupleChip};

impl<F: PrimeField64> Chip<F> for IsLessThanTupleChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        let num_cols = IsLessThanTupleCols::<F>::get_width(
            self.air.limb_bits().clone(),
            *self.air.decomp(),
            self.air.tuple_len(),
        );
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_numbered = IsLessThanTupleCols::<usize>::from_slice(
            &all_cols,
            self.air.limb_bits().clone(),
            *self.air.decomp(),
            self.air.tuple_len(),
        );

        SubAirWithInteractions::sends(self, cols_numbered)
    }
}

impl<F: PrimeField64> SubAirWithInteractions<F> for IsLessThanTupleChip {
    fn sends(&self, col_indices: IsLessThanTupleCols<usize>) -> Vec<Interaction<F>> {
        // num_limbs is the number of limbs, not including the last shifted limb
        let mut interactions = vec![];

        for i in 0..self.air.tuple_len() {
            let mut is_less_than_cols = vec![
                col_indices.io.x[i],
                col_indices.io.y[i],
                col_indices.aux.less_than[i],
                col_indices.aux.lower_bits[i],
            ];

            is_less_than_cols.extend_from_slice(&col_indices.aux.lower_bits_decomp[i]);

            let is_less_than_cols = IsLessThanCols::<usize>::from_slice(
                &is_less_than_cols,
                self.air.limb_bits().clone()[i],
                *self.air.decomp(),
            );

            let curr_interactions =
                SubAirWithInteractions::<F>::sends(&self.air.is_lt_airs[i], is_less_than_cols);
            interactions.extend(curr_interactions);
        }

        interactions
    }
}
