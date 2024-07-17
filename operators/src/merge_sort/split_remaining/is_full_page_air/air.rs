use afs_chips::{
    common::page_cols::PageCols,
    is_less_than_tuple::columns::IsLessThanTupleIOCols,
    sub_chip::{AirConfig, SubAir},
};
use afs_stark_backend::air_builders::PartitionedAirBuilder;
use p3_air::{Air, AirBuilder, AirBuilderWithPublicValues, BaseAir};
use p3_field::Field;
use p3_matrix::Matrix;

use super::{
    columns::{IsFullIndexedOutputPageCols, IsFullIndexedOutputPageMetadataCols},
    IsFullIndexedOutputPageAir,
};

impl<F: Field> BaseAir<F> for IsFullIndexedOutputPageAir {
    fn width(&self) -> usize {
        self.air_width()
    }
}

impl AirConfig for IsFullIndexedOutputPageAir {
    type Cols<T> = IsFullIndexedOutputPageCols<T>;
}

impl<AB: AirBuilder + AirBuilderWithPublicValues + PartitionedAirBuilder> Air<AB>
    for IsFullIndexedOutputPageAir
where
    AB::M: Clone,
{
    fn eval(&self, builder: &mut AB) {
        // partition is physical page data vs metadata
        let main: &<AB as AirBuilder>::M = &builder.partitioned_main()[1].clone();
        let local = main.row_slice(0);
        let pi = builder.public_values().to_vec();
        let data: &<AB as AirBuilder>::M = &builder.partitioned_main()[0].clone();
        let cached_data = PageCols::from_slice(&data.row_slice(0), self.idx_len, self.data_len);
        let next_data = PageCols::from_slice(&data.row_slice(1), self.idx_len, self.data_len);
        let metadata = IsFullIndexedOutputPageMetadataCols::from_slice(
            &local,
            self.idx_len,
            self.is_less_than_tuple_param.clone(),
        );
        let next_aux = IsFullIndexedOutputPageMetadataCols::from_slice(
            &main.row_slice(1),
            self.idx_len,
            self.is_less_than_tuple_param.clone(),
        )
        .subchip_aux_cols
        .final_page_aux;
        let range_inclusion_cols = metadata.range_inclusion_cols;
        for (i, c) in range_inclusion_cols.start.iter().enumerate() {
            builder.assert_eq(pi[i + 1], *c);
        }
        for (i, c) in range_inclusion_cols.end.iter().enumerate() {
            builder.assert_eq(pi[i + self.idx_len + 1], *c);
        }
        let is_full = pi[0];
        let less_than_start = range_inclusion_cols.less_than_start;
        let greater_than_end = range_inclusion_cols.greater_than_end;
        builder.assert_zero(cached_data.is_alloc * (less_than_start + greater_than_end));
        builder.assert_zero(is_full.into() * cached_data.is_alloc - is_full.into());
        let subair_aux_cols = metadata.subchip_aux_cols;
        let subairs = self.is_less_than_tuple_air.clone();
        {
            let io = IsLessThanTupleIOCols {
                x: cached_data.idx.clone(),
                y: range_inclusion_cols.start.clone(),
                tuple_less_than: range_inclusion_cols.less_than_start,
            };
            let aux = subair_aux_cols.idx_start.clone();
            SubAir::eval(&subairs.idx_start, builder, io, aux);
        }
        {
            let io = IsLessThanTupleIOCols {
                x: range_inclusion_cols.end.clone(),
                y: cached_data.idx.clone(),
                tuple_less_than: range_inclusion_cols.greater_than_end,
            };
            let aux = subair_aux_cols.end_idx.clone();
            SubAir::eval(&subairs.end_idx, builder, io, aux);
        }
        SubAir::eval(&self.page_chip, builder, [cached_data, next_data], next_aux);
    }
}
