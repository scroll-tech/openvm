use afs_stark_backend::interaction::{Chip, Interaction};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

use crate::sub_chip::SubAirWithInteractions;

use super::{columns::RootSignalCols, RootSignalChip};

impl<F: PrimeField64, const COMMITMENT_LEN: usize> Chip<F> for RootSignalChip<COMMITMENT_LEN> {
    fn receives(&self) -> Vec<Interaction<F>> {
        vec![]
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        let num_cols = self.air_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols = RootSignalCols::<usize>::from_slice(
            &all_cols,
            self.idx_len,
            COMMITMENT_LEN,
            self.is_init,
        );
        if self.is_init {
            let virtual_cols = (cols.root_commitment)
                .into_iter()
                .map(VirtualPairCol::single_main)
                .collect::<Vec<_>>();

            vec![Interaction {
                fields: virtual_cols,
                count: VirtualPairCol::single_main(cols.mult),
                argument_index: *self.bus_index(),
            }]
        } else {
            let virtual_cols = (cols.root_commitment)
                .into_iter()
                .chain(cols.range.clone().unwrap().0)
                .chain(cols.range.clone().unwrap().1)
                .map(VirtualPairCol::single_main)
                .collect::<Vec<_>>();

            vec![Interaction {
                fields: virtual_cols,
                count: VirtualPairCol::single_main(cols.mult),
                argument_index: *self.bus_index(),
            }]
        }
    }
}
