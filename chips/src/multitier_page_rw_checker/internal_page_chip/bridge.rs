use std::iter;

use afs_stark_backend::interaction::{AirBridge, Interaction};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

use super::columns::InternalPageCols;
use super::InternalPageChip;
use crate::is_less_than_tuple::columns::{IsLessThanTupleCols, IsLessThanTupleIOCols};
use crate::sub_chip::SubAirBridge;

impl<const COMMITMENT_LEN: usize> InternalPageChip<COMMITMENT_LEN> {
    fn custom_receives_path<F: PrimeField64>(
        &self,
        col_indices: InternalPageCols<usize>,
    ) -> Vec<Interaction<F>> {
        // Sending the path
        if self.is_init {
            let virtual_cols = (col_indices.metadata.own_commitment)
                .into_iter()
                .map(VirtualPairCol::single_main)
                .collect::<Vec<_>>();

            vec![Interaction {
                fields: virtual_cols,
                count: VirtualPairCol::single_main(col_indices.metadata.mult_alloc),
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
                count: VirtualPairCol::single_main(col_indices.metadata.mult_alloc),
                argument_index: *self.path_bus_index(),
            }]
        }
    }

    fn custom_sends_or_receives<F: PrimeField64>(
        &self,
        col_indices: InternalPageCols<usize>,
    ) -> Vec<Interaction<F>> {
        let virtual_cols = iter::once(col_indices.cache_cols.is_alloc)
            .chain(col_indices.cache_cols.start)
            .chain(col_indices.cache_cols.end)
            .chain(col_indices.cache_cols.commitment)
            .map(VirtualPairCol::single_main)
            .collect::<Vec<_>>();

        vec![Interaction {
            fields: virtual_cols,
            count: VirtualPairCol::single_main(col_indices.metadata.mult_alloc_is_1),
            argument_index: *self.data_bus_index(),
        }]
    }

    fn custom_sends_path<F: PrimeField64>(
        &self,
        col_indices: InternalPageCols<usize>,
    ) -> Vec<Interaction<F>> {
        // Sending the path
        if self.is_init {
            let virtual_cols = (col_indices.cache_cols.commitment)
                .into_iter()
                .map(VirtualPairCol::single_main)
                .collect::<Vec<_>>();

            vec![Interaction {
                fields: virtual_cols,
                count: VirtualPairCol::single_main(col_indices.metadata.mult_minus_one_alloc),
                argument_index: *self.path_bus_index(),
            }]
        } else {
            let virtual_cols = col_indices
                .cache_cols
                .start
                .into_iter()
                .chain(col_indices.cache_cols.end)
                .chain(col_indices.cache_cols.commitment)
                .map(VirtualPairCol::single_main)
                .collect::<Vec<_>>();

            vec![Interaction {
                fields: virtual_cols,
                count: VirtualPairCol::single_main(col_indices.metadata.mult_minus_one_alloc),
                argument_index: *self.path_bus_index(),
            }]
        }
    }
}

impl<F: PrimeField64, const COMMITMENT_LEN: usize> SubAirBridge<F>
    for InternalPageChip<COMMITMENT_LEN>
{
    fn receives(&self, col_indices: InternalPageCols<usize>) -> Vec<Interaction<F>> {
        let mut interactions = vec![];
        interactions.extend(self.custom_receives_path(col_indices.clone()));
        interactions
    }

    fn sends(&self, col_indices: InternalPageCols<usize>) -> Vec<Interaction<F>> {
        let mut interactions = vec![];
        interactions.extend(self.custom_sends_path(col_indices.clone()));
        if self.is_init {
            interactions.extend(self.custom_sends_or_receives(col_indices.clone()));
        }
        if !self.is_init {
            let subairs = self.is_less_than_tuple_air.clone().unwrap();
            let subair_aux = col_indices.metadata.subchip_aux_cols.clone().unwrap();
            let range_inclusion = col_indices.metadata.range_inclusion_cols.clone().unwrap();
            let prove_sort = col_indices.metadata.prove_sort_cols.clone().unwrap();
            interactions.extend(SubAirBridge::sends(
                &subairs.key1_start,
                IsLessThanTupleCols {
                    io: IsLessThanTupleIOCols {
                        x: col_indices.cache_cols.start.clone(),
                        y: range_inclusion.start.clone(),
                        tuple_less_than: range_inclusion.less_than_start.0,
                    },
                    aux: subair_aux.key1_start,
                },
            ));
            interactions.extend(SubAirBridge::sends(
                &subairs.end_key1,
                IsLessThanTupleCols {
                    io: IsLessThanTupleIOCols {
                        x: range_inclusion.end.clone(),
                        y: col_indices.cache_cols.start.clone(),
                        tuple_less_than: range_inclusion.greater_than_end.0,
                    },
                    aux: subair_aux.end_key1,
                },
            ));
            interactions.extend(SubAirBridge::sends(
                &subairs.key2_start,
                IsLessThanTupleCols {
                    io: IsLessThanTupleIOCols {
                        x: col_indices.cache_cols.end.clone(),
                        y: range_inclusion.start.clone(),
                        tuple_less_than: range_inclusion.less_than_start.1,
                    },
                    aux: subair_aux.key2_start,
                },
            ));
            interactions.extend(SubAirBridge::sends(
                &subairs.end_key2,
                IsLessThanTupleCols {
                    io: IsLessThanTupleIOCols {
                        x: range_inclusion.end.clone(),
                        y: col_indices.cache_cols.end.clone(),
                        tuple_less_than: range_inclusion.greater_than_end.1,
                    },
                    aux: subair_aux.end_key2,
                },
            ));
            interactions.extend(SubAirBridge::sends(
                &subairs.end_next,
                IsLessThanTupleCols {
                    io: IsLessThanTupleIOCols {
                        x: col_indices.cache_cols.end.clone(),
                        y: prove_sort.next_key.clone(),
                        tuple_less_than: prove_sort.end_less_than_next,
                    },
                    aux: subair_aux.end_next,
                },
            ));
            interactions.extend(SubAirBridge::sends(
                &subairs.end_start,
                IsLessThanTupleCols {
                    io: IsLessThanTupleIOCols {
                        x: col_indices.cache_cols.end.clone(),
                        y: col_indices.cache_cols.start.clone(),
                        tuple_less_than: prove_sort.end_less_than_start,
                    },
                    aux: subair_aux.end_start,
                },
            ));
        }
        interactions
    }
}

impl<F: PrimeField64, const COMMITMENT_LEN: usize> AirBridge<F>
    for InternalPageChip<COMMITMENT_LEN>
{
    fn receives(&self) -> Vec<Interaction<F>> {
        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_to_receive = InternalPageCols::<usize>::from_slice(
            &all_cols,
            self.idx_len,
            COMMITMENT_LEN,
            self.is_init,
            self.is_less_than_tuple_param.clone(),
        );
        SubAirBridge::receives(self, cols_to_receive)
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_to_receive = InternalPageCols::<usize>::from_slice(
            &all_cols,
            self.idx_len,
            COMMITMENT_LEN,
            self.is_init,
            self.is_less_than_tuple_param.clone(),
        );
        SubAirBridge::sends(self, cols_to_receive)
    }
}
