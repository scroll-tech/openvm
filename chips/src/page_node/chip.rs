use afs_stark_backend::interaction::{Chip, Interaction};
use p3_field::PrimeField64;

use super::columns::PageNodeReadCols;
use super::PageNodeReadChip;

impl<F: PrimeField64> Chip<F> for PageNodeReadChip {
    fn sends(&self) -> Vec<Interaction<F>> {
        let num_cols = self.get_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();
        self.sends_branch(PageNodeReadCols::<F>::as_branch(self.key_len, &all_cols))
    }
    fn receives(&self) -> Vec<Interaction<F>> {
        let num_cols = self.get_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let mut interactions =
            self.receives_branch(PageNodeReadCols::<F>::as_branch(self.key_len, &all_cols));
        interactions.append(
            &mut self.receives_terminal(PageNodeReadCols::<F>::as_terminal(
                self.key_len,
                self.val_len,
                &all_cols,
            )),
        );
        interactions
    }
}
