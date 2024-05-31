use afs_stark_backend::interaction::{Chip, Interaction};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

use super::columns::InitFinalCols;
use super::InitFinalChip;

impl InitFinalChip {
    pub fn sends_receives_custom<F: PrimeField64>(
        &self,
        cols: InitFinalCols<usize>,
    ) -> Vec<Interaction<F>> {
        let virtual_cols = cols
            .page_row
            .iter()
            .map(|col| VirtualPairCol::single_main(*col))
            .collect::<Vec<_>>();

        vec![Interaction {
            fields: virtual_cols,
            count: VirtualPairCol::constant(F::one()),
            argument_index: self.bus_index(),
        }]
    }
}

impl<F: PrimeField64> Chip<F> for InitFinalChip {
    fn receives(&self) -> Vec<Interaction<F>> {
        if self.is_send {
            return vec![];
        }

        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_to_receive = InitFinalCols::<F>::cols_numbered(&all_cols);
        self.sends_receives_custom(cols_to_receive)
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        if !self.is_send {
            return vec![];
        }

        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_to_receive = InitFinalCols::<F>::cols_numbered(&all_cols);
        self.sends_receives_custom(cols_to_receive)
    }
}
