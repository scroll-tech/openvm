use std::borrow::Borrow;

use afs_stark_backend::air_builders::PartitionedAirBuilder;
use p3_air::{Air, AirBuilder, AirBuilderWithPublicValues, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use crate::MAX_COMMITMENT_LEN;

use super::PageNodeReadChip;

impl<F: Field> BaseAir<F> for PageNodeReadChip {
    fn width(&self) -> usize {
        self.get_width()
    }
}

impl<AB: PartitionedAirBuilder + AirBuilderWithPublicValues> Air<AB> for PageNodeReadChip
where
    AB::Var: Clone,
    AB::M: Clone,
{
    // mult_p = mult * is_present
    // mult_b = mult * is_present * (1-is_terminal)
    // mult_t = mult * is_present * is_terminal
    fn eval(&self, builder: &mut AB) {
        // Choosing the second partition of the trace, which looks like (index, mult)
        let main: &<AB as AirBuilder>::M = &builder.partitioned_main()[1].clone();
        let data: &<AB as AirBuilder>::M = &builder.partitioned_main()[0].clone();

        let local = main.row_slice(0);
        let local_data = data.row_slice(0);
        let local: &[AB::Var] = (*local).borrow();
        let local_data: &[AB::Var] = (*local_data).borrow();
        let pis = builder.public_values().to_vec();
        for i in 0..MAX_COMMITMENT_LEN {
            builder.assert_eq(local[i], pis[i]);
        }
        let local_len = MAX_COMMITMENT_LEN + 4;
        builder.assert_bool(local_data[0]);
        builder.assert_bool(local_data[1]);
        builder.assert_eq(local[local_len - 2], local[local_len - 1] * local_data[0]);
        builder.assert_eq(
            local[local_len - 2],
            local[local_len - 3] + local[local_len - 4],
        );
        builder.assert_eq(
            local[local_len - 4],
            local[local_len - 1] * local_data[0] * local_data[1],
        );
    }
}
