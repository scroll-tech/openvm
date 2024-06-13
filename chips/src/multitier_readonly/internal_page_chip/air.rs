use std::borrow::Borrow;

use afs_stark_backend::air_builders::PartitionedAirBuilder;
use p3_air::{Air, AirBuilder, AirBuilderWithPublicValues, BaseAir};
use p3_field::Field;
use p3_matrix::Matrix;

use crate::sub_chip::AirConfig;

use super::{
    columns::{InternalDataCols, InternalPageCols, InternalPageMetadataCols},
    InternalPageChip,
};

impl<F: Field, const COMMITMENT_LEN: usize> BaseAir<F> for InternalPageChip<COMMITMENT_LEN> {
    fn width(&self) -> usize {
        self.air_width()
    }
}
impl<const COMMITMENT_LEN: usize> AirConfig for InternalPageChip<COMMITMENT_LEN> {
    type Cols<T> = InternalPageCols<T>;
}

impl<AB: PartitionedAirBuilder + AirBuilderWithPublicValues, const COMMITMENT_LEN: usize> Air<AB>
    for InternalPageChip<COMMITMENT_LEN>
where
    AB::Var: Clone,
    AB::M: Clone,
{
    fn eval(&self, builder: &mut AB) {
        // Choosing the second partition of the trace, which looks like (index, mult)
        let main: &<AB as AirBuilder>::M = &builder.partitioned_main()[1].clone();
        let data: &<AB as AirBuilder>::M = &builder.partitioned_main()[0].clone();

        let local = main.row_slice(0);
        let local_data = data.row_slice(0);
        let local: &[AB::Var] = (*local).borrow();
        let local_data: &[AB::Var] = (*local_data).borrow();
        let pis = builder.public_values().to_vec();
        let metadata = InternalPageMetadataCols::from_slice(local, COMMITMENT_LEN);
        let page_data = InternalDataCols::from_slice(local_data, self.idx_len, COMMITMENT_LEN);
        // assert second to last thing is the pi commitment of this
        for i in 0..COMMITMENT_LEN {
            builder.assert_eq(metadata.commitment[i], pis[i]);
        }
        builder.assert_zero(page_data.is_leaf);
        builder.assert_eq(metadata.mult_alloc, metadata.mult * page_data.is_alloc);
    }
}
