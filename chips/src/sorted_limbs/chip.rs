use super::columns::SortedLimbsCols;
use afs_stark_backend::interaction::{Chip, Interaction};
use p3_field::PrimeField64;

use super::SortedLimbsChip;

impl<F: PrimeField64, const MAX: u32> Chip<F> for SortedLimbsChip<MAX> {
    fn sends(&self) -> Vec<Interaction<F>> {
        let num_cols =
            SortedLimbsCols::<F>::get_width(self.limb_bits(), self.decomp(), self.key_vec_len());
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_numbered = SortedLimbsCols::<F>::cols_numbered(
            &all_cols,
            self.limb_bits(),
            self.decomp(),
            self.key_vec_len(),
        );

        self.sends_custom(cols_numbered)
    }
}
