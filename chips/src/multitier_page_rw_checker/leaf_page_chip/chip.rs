use afs_stark_backend::interaction::{Chip, Interaction};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

use super::columns::LeafPageCols;
use super::LeafPageChip;
use crate::is_less_than_tuple::columns::{
    IsLessThanTupleAuxCols, IsLessThanTupleCols, IsLessThanTupleIOCols,
};
use crate::sub_chip::SubAirWithInteractions;

impl<const COMMITMENT_LEN: usize> LeafPageChip<COMMITMENT_LEN> {
    fn custom_receives_path<F: PrimeField64>(
        &self,
        col_indices: LeafPageCols<usize>,
    ) -> Vec<Interaction<F>> {
        // Sending the path
        if self.is_init {
            let virtual_cols = (col_indices.metadata.own_commitment)
                .into_iter()
                .map(VirtualPairCol::single_main)
                .collect::<Vec<_>>();

            vec![Interaction {
                fields: virtual_cols,
                count: VirtualPairCol::single_main(col_indices.cache_cols.is_alloc),
                argument_index: *self.path_bus_index(),
            }]
        } else {
            let range_inclusion_cols = col_indices.metadata.range_inclusion_cols.unwrap();
            let virtual_cols = range_inclusion_cols
                .start
                .into_iter()
                .chain(range_inclusion_cols.end)
                .chain(col_indices.metadata.own_commitment)
                .map(VirtualPairCol::single_main)
                .collect::<Vec<_>>();

            vec![Interaction {
                fields: virtual_cols,
                count: VirtualPairCol::single_main(col_indices.cache_cols.is_alloc),
                argument_index: *self.path_bus_index(),
            }]
        }
    }
}

impl<F: PrimeField64, const COMMITMENT_LEN: usize> SubAirWithInteractions<F>
    for LeafPageChip<COMMITMENT_LEN>
{
    fn receives(&self, col_indices: LeafPageCols<usize>) -> Vec<Interaction<F>> {
        let mut interactions = vec![];
        interactions.extend(SubAirWithInteractions::receives(
            self.page_chip(),
            col_indices.cache_cols.clone(),
        ));
        interactions.extend(self.custom_receives_path(col_indices.clone()));

        if !self.is_init {
            let subairs = self.is_less_than_tuple_air.clone().unwrap();
            let range_inclusion = col_indices.metadata.range_inclusion_cols.clone().unwrap();
            let subair_aux = col_indices.metadata.subchip_aux_cols.clone().unwrap();
            interactions.extend(SubAirWithInteractions::receives(
                &subairs.key_start,
                IsLessThanTupleCols {
                    io: IsLessThanTupleIOCols {
                        x: col_indices.cache_cols.idx.clone(),
                        y: range_inclusion.start.clone(),
                        tuple_less_than: range_inclusion.less_than_start.clone(),
                    },
                    aux: subair_aux.key_start.clone(),
                },
            ));
            interactions.extend(SubAirWithInteractions::receives(
                &subairs.end_key,
                IsLessThanTupleCols {
                    io: IsLessThanTupleIOCols {
                        x: range_inclusion.end.clone(),
                        y: col_indices.cache_cols.idx.clone(),
                        tuple_less_than: range_inclusion.greater_than_end.clone(),
                    },
                    aux: subair_aux.end_key.clone(),
                },
            ));
        }
        interactions
    }

    fn sends(&self, col_indices: LeafPageCols<usize>) -> Vec<Interaction<F>> {
        let mut interactions = vec![];
        interactions.extend(SubAirWithInteractions::sends(
            self.page_chip(),
            col_indices.cache_cols.clone(),
        ));

        if !self.is_init {
            let subairs = self.is_less_than_tuple_air.clone().unwrap();
            let range_inclusion = col_indices.metadata.range_inclusion_cols.clone().unwrap();
            let subair_aux = col_indices.metadata.subchip_aux_cols.clone().unwrap();
            interactions.extend(SubAirWithInteractions::sends(
                &subairs.key_start,
                IsLessThanTupleCols {
                    io: IsLessThanTupleIOCols {
                        x: col_indices.cache_cols.idx.clone(),
                        y: range_inclusion.start.clone(),
                        tuple_less_than: range_inclusion.less_than_start.clone(),
                    },
                    aux: subair_aux.key_start.clone(),
                },
            ));
            interactions.extend(SubAirWithInteractions::sends(
                &subairs.end_key,
                IsLessThanTupleCols {
                    io: IsLessThanTupleIOCols {
                        x: range_inclusion.end.clone(),
                        y: col_indices.cache_cols.idx.clone(),
                        tuple_less_than: range_inclusion.greater_than_end.clone(),
                    },
                    aux: subair_aux.end_key.clone(),
                },
            ));
        }
        interactions
    }
}

impl<F: PrimeField64, const COMMITMENT_LEN: usize> Chip<F> for LeafPageChip<COMMITMENT_LEN> {
    fn receives(&self) -> Vec<Interaction<F>> {
        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_to_receive = LeafPageCols::<usize>::from_slice(
            &all_cols,
            self.idx_len,
            self.data_len,
            COMMITMENT_LEN,
            self.is_init,
            self.is_less_than_tuple_param.clone(),
        );
        SubAirWithInteractions::receives(self, cols_to_receive)
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_to_receive = LeafPageCols::<usize>::from_slice(
            &all_cols,
            self.idx_len,
            self.data_len,
            COMMITMENT_LEN,
            self.is_init,
            self.is_less_than_tuple_param.clone(),
        );
        SubAirWithInteractions::sends(self, cols_to_receive)
    }
}
