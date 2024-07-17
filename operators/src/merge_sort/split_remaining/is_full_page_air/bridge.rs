use afs_chips::{
    indexed_output_page_air::columns::IndexedOutputPageCols,
    is_less_than_tuple::columns::{IsLessThanTupleCols, IsLessThanTupleIOCols},
    sub_chip::SubAirBridge,
    utils::to_vcols,
};
use afs_stark_backend::interaction::{AirBridge, Interaction};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

use super::{columns::IsFullIndexedOutputPageCols, IsFullIndexedOutputPageAir};

impl<F: PrimeField64> SubAirBridge<F> for IsFullIndexedOutputPageAir {
    fn receives(&self, col_indices: IsFullIndexedOutputPageCols<usize>) -> Vec<Interaction<F>> {
        let mut interactions = vec![];
        interactions.extend(SubAirBridge::receives(
            &self.page_chip,
            IndexedOutputPageCols {
                page_cols: col_indices.cache_cols.clone(),
                aux_cols: col_indices.metadata.subchip_aux_cols.final_page_aux,
            },
        ));
        let page_cols = col_indices.cache_cols.clone();
        let page_cols = to_vcols(&[page_cols.idx, page_cols.data].concat());
        interactions.push(Interaction {
            fields: page_cols,
            count: VirtualPairCol::single_main(col_indices.cache_cols.is_alloc),
            argument_index: self.page_bus_index,
        });
        interactions
    }

    fn sends(&self, col_indices: IsFullIndexedOutputPageCols<usize>) -> Vec<Interaction<F>> {
        let mut interactions = vec![];

        interactions.extend(SubAirBridge::sends(
            &self.page_chip,
            IndexedOutputPageCols {
                page_cols: col_indices.cache_cols.clone(),
                aux_cols: col_indices.metadata.subchip_aux_cols.final_page_aux.clone(),
            },
        ));

        let subairs = self.is_less_than_tuple_air.clone();
        let range_inclusion = col_indices.metadata.range_inclusion_cols.clone();
        let subair_aux = col_indices.metadata.subchip_aux_cols.clone();
        interactions.extend(SubAirBridge::sends(
            &subairs.idx_start,
            IsLessThanTupleCols {
                io: IsLessThanTupleIOCols {
                    x: col_indices.cache_cols.idx.clone(),
                    y: range_inclusion.start.clone(),
                    tuple_less_than: range_inclusion.less_than_start,
                },
                aux: subair_aux.idx_start.clone(),
            },
        ));
        interactions.extend(SubAirBridge::sends(
            &subairs.end_idx,
            IsLessThanTupleCols {
                io: IsLessThanTupleIOCols {
                    x: range_inclusion.end.clone(),
                    y: col_indices.cache_cols.idx.clone(),
                    tuple_less_than: range_inclusion.greater_than_end,
                },
                aux: subair_aux.end_idx.clone(),
            },
        ));
        interactions
    }
}

impl<F: PrimeField64> AirBridge<F> for IsFullIndexedOutputPageAir {
    fn receives(&self) -> Vec<Interaction<F>> {
        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_to_receive = IsFullIndexedOutputPageCols::<usize>::from_slice(
            &all_cols,
            self.idx_len,
            self.data_len,
            self.is_less_than_tuple_param.clone(),
        );
        SubAirBridge::receives(self, cols_to_receive)
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_to_receive = IsFullIndexedOutputPageCols::<usize>::from_slice(
            &all_cols,
            self.idx_len,
            self.data_len,
            self.is_less_than_tuple_param.clone(),
        );
        SubAirBridge::sends(self, cols_to_receive)
    }
}
