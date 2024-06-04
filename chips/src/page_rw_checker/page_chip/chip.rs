use std::iter;

use afs_stark_backend::interaction::{Chip, Interaction};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

use super::columns::PageCols;
use super::PageChip;
use crate::sub_chip::SubAirWithInteractions;

impl PageChip {
    fn custom_sends_or_receives<F: PrimeField64>(
        &self,
        col_indices: PageCols<usize>,
    ) -> Vec<Interaction<F>> {
        let virtual_cols = iter::once(col_indices.is_alloc)
            .chain(col_indices.page_row)
            .map(VirtualPairCol::single_main)
            .collect::<Vec<_>>();

        vec![Interaction {
            fields: virtual_cols,
            count: VirtualPairCol::single_main(col_indices.is_alloc),
            argument_index: self.bus_index(),
        }]
    }
}

impl<F: PrimeField64> SubAirWithInteractions<F> for PageChip {
    fn receives(&self, col_indices: PageCols<usize>) -> Vec<Interaction<F>> {
        self.custom_sends_or_receives(col_indices)
    }

    fn sends(&self, col_indices: PageCols<usize>) -> Vec<Interaction<F>> {
        self.custom_sends_or_receives(col_indices)
    }
}

impl<F: PrimeField64> Chip<F> for PageChip {
    fn receives(&self) -> Vec<Interaction<F>> {
        if self.is_send {
            return vec![];
        }

        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_to_receive = PageCols::<F>::cols_numbered(&all_cols);
        SubAirWithInteractions::receives(self, cols_to_receive)
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        if !self.is_send {
            return vec![];
        }

        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_to_send = PageCols::<F>::cols_numbered(&all_cols);
        SubAirWithInteractions::sends(self, cols_to_send)
    }
}
