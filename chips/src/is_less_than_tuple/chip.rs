use crate::{
    is_less_than::columns::{IsLessThanAuxCols, IsLessThanCols, IsLessThanIOCols},
    sub_chip::SubAirWithInteractions,
};
use afs_stark_backend::interaction::{Chip, Interaction};
use p3_field::PrimeField64;

use super::{columns::IsLessThanTupleCols, IsLessThanTupleAir, IsLessThanTupleChip};

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

        SubAirWithInteractions::sends(&self.air, cols_numbered)
    }
}

impl<F: PrimeField64> SubAirWithInteractions<F> for IsLessThanTupleAir {
    fn sends(&self, col_indices: IsLessThanTupleCols<usize>) -> Vec<Interaction<F>> {
        // num_limbs is the number of limbs, not including the last shifted limb
        let mut interactions = vec![];

        for i in 0..self.tuple_len() {
            let is_less_than_cols = IsLessThanCols {
                io: IsLessThanIOCols {
                    x: col_indices.io.x[i],
                    y: col_indices.io.y[i],
                    less_than: col_indices.aux.less_than[i],
                },
                aux: IsLessThanAuxCols {
                    lower: col_indices.aux.less_than_cols[i].lower,
                    lower_decomp: col_indices.aux.less_than_cols[i].lower_decomp.clone(),
                },
            };

            let curr_interactions =
                SubAirWithInteractions::<F>::sends(&self.is_lt_airs[i], is_less_than_cols);
            interactions.extend(curr_interactions);
        }

        interactions
    }
}
