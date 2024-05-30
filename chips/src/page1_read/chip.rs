use afs_stark_backend::interaction::{Chip, Interaction};
use p3_field::PrimeField64;

use super::columns::Page1ReadCols;
use super::Page1ReadChip;

impl<F: PrimeField64> Chip<F> for Page1ReadChip {
    fn receives(&self) -> Vec<Interaction<F>> {
        let num_cols = self.get_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_numbered =
            Page1ReadCols::<F>::cols_numbered(self.key_len, self.val_len, &all_cols);
        self.receives_custom(cols_numbered)
    }
}
