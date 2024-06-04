use afs_stark_backend::interaction::{Chip, Interaction};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

use crate::sub_chip::SubAirWithInteractions;

use super::columns::MiddleChipCols;
use super::MiddleChip;

impl<F: PrimeField64> SubAirWithInteractions<F> for MiddleChip {
    fn receives(&self, col_indices: MiddleChipCols<usize>) -> Vec<Interaction<F>> {
        let virtual_cols = col_indices
            .page_row
            .iter()
            .map(|col| VirtualPairCol::single_main(*col))
            .collect::<Vec<_>>();

        vec![Interaction {
            fields: virtual_cols,
            count: VirtualPairCol::single_main(col_indices.is_initial),
            argument_index: self.bus_index(),
        }]
    }

    fn sends(&self, col_indices: MiddleChipCols<usize>) -> Vec<Interaction<F>> {
        let virtual_cols = col_indices
            .page_row
            .iter()
            .map(|col| VirtualPairCol::single_main(*col))
            .collect::<Vec<_>>();

        vec![Interaction {
            fields: virtual_cols,
            count: VirtualPairCol::single_main(col_indices.is_final),
            argument_index: self.bus_index(),
        }]
    }
}

impl<F: PrimeField64> Chip<F> for MiddleChip {
    fn receives(&self) -> Vec<Interaction<F>> {
        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_to_receive = MiddleChipCols::<usize>::from_slice(
            &all_cols,
            self.page_width(),
            self.key_len,
            self.val_len,
        );
        SubAirWithInteractions::receives(self, cols_to_receive)
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_to_send = MiddleChipCols::<usize>::from_slice(
            &all_cols,
            self.page_width(),
            self.key_len,
            self.val_len,
        );
        SubAirWithInteractions::sends(self, cols_to_send)
    }
}
