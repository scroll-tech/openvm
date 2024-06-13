use afs_stark_backend::interaction::{AirBridge, Interaction};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

use crate::sub_chip::SubAirBridge;

use super::columns::LeafPageCols;
use super::LeafPageChip;

impl<F: PrimeField64, const COMMITMENT_LEN: usize> AirBridge<F> for LeafPageChip<COMMITMENT_LEN> {
    fn receives(&self) -> Vec<Interaction<F>> {
        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_numbered = LeafPageCols::<usize>::from_slice(
            &all_cols,
            self.idx_len,
            self.data_len,
            COMMITMENT_LEN,
        );
        SubAirBridge::receives(self, cols_numbered)
    }
    fn sends(&self) -> Vec<Interaction<F>> {
        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_numbered = LeafPageCols::<usize>::from_slice(
            &all_cols,
            self.idx_len,
            self.data_len,
            COMMITMENT_LEN,
        );
        SubAirBridge::sends(self, cols_numbered)
    }
}

impl<F: PrimeField64, const COMMITMENT_LEN: usize> SubAirBridge<F>
    for LeafPageChip<COMMITMENT_LEN>
{
    fn receives(&self, col_indices: Self::Cols<usize>) -> Vec<Interaction<F>> {
        let virtual_cols = (col_indices.metadata.commitment)
            .into_iter()
            .map(VirtualPairCol::single_main)
            .collect::<Vec<_>>();
        let data_cols = (col_indices.cache_cols.idx)
            .into_iter()
            .chain(col_indices.cache_cols.data)
            .map(VirtualPairCol::single_main)
            .collect::<Vec<_>>();

        vec![
            Interaction {
                fields: virtual_cols,
                count: VirtualPairCol::single_main(col_indices.metadata.mult_alloc),
                argument_index: *self.path_bus_index(),
            },
            Interaction {
                fields: data_cols,
                count: VirtualPairCol::single_main(col_indices.metadata.mult_alloc),
                argument_index: *self.data_bus_index(),
            },
        ]
    }
}
