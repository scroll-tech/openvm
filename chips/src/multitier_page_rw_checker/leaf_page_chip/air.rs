use afs_stark_backend::air_builders::PartitionedAirBuilder;
use p3_air::{Air, AirBuilder, AirBuilderWithPublicValues, BaseAir};
use p3_field::Field;
use p3_matrix::Matrix;

use super::{
    columns::{LeafPageCols, LeafPageMetadataCols},
    LeafPageChip,
};
use crate::{
    is_less_than_tuple::columns::IsLessThanTupleIOCols,
    page_rw_checker::page_chip::columns::PageCols,
    sub_chip::{AirConfig, SubAir},
};

impl<F: Field, const COMMITMENT_LEN: usize> BaseAir<F> for LeafPageChip<COMMITMENT_LEN> {
    fn width(&self) -> usize {
        self.air_width()
    }
}

impl<const COMMITMENT_LEN: usize> AirConfig for LeafPageChip<COMMITMENT_LEN> {
    type Cols<T> = LeafPageCols<T>;
}

impl<
        AB: AirBuilder + AirBuilderWithPublicValues + PartitionedAirBuilder,
        const COMMITMENT_LEN: usize,
    > Air<AB> for LeafPageChip<COMMITMENT_LEN>
where
    AB::M: Clone,
{
    fn eval(&self, builder: &mut AB) {
        // only constrain that own_commitment is accurate
        // partition is physical page data vs metadata
        let main: &<AB as AirBuilder>::M = &builder.partitioned_main()[1].clone();
        let local = main.row_slice(0);
        let pi = builder.public_values().to_vec();
        let data: &<AB as AirBuilder>::M = &builder.partitioned_main()[0].clone();
        let cached_data = PageCols::from_slice(&data.row_slice(0), self.idx_len, self.data_len);
        for i in 0..COMMITMENT_LEN {
            builder.assert_eq(pi[i], local[i]);
        }
        if self.is_init {
            return;
        }
        // assert stuff is in range
        else {
            let metadata = LeafPageMetadataCols::from_slice(
                &local,
                self.idx_len,
                COMMITMENT_LEN,
                false,
                self.is_less_than_tuple_param.clone(),
            );
            let range_inclusion_cols = metadata.range_inclusion_cols.unwrap();
            let less_than_start = range_inclusion_cols.less_than_start;
            let greater_than_end = range_inclusion_cols.greater_than_end;
            builder.assert_zero(cached_data.is_alloc * (less_than_start + greater_than_end));
            let subair_aux_cols = metadata.subchip_aux_cols.unwrap();
            let subairs = self.is_less_than_tuple_air.clone().unwrap();
            {
                let io = IsLessThanTupleIOCols {
                    x: cached_data.idx.clone(),
                    y: range_inclusion_cols.start.clone(),
                    tuple_less_than: range_inclusion_cols.less_than_start.clone(),
                };
                let aux = subair_aux_cols.key_start.clone();
                SubAir::eval(&subairs.key_start, builder, io, aux);
            }
            {
                let io = IsLessThanTupleIOCols {
                    x: range_inclusion_cols.end.clone(),
                    y: cached_data.idx.clone(),
                    tuple_less_than: range_inclusion_cols.greater_than_end.clone(),
                };
                let aux = subair_aux_cols.end_key.clone();
                SubAir::eval(&subairs.end_key, builder, io, aux);
            }
            // builder.assert_bool(cached_data.is_alloc);
        }
        SubAir::eval(self.page_chip(), builder, cached_data, ());
    }
}
