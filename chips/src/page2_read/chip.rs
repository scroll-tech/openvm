use afs_stark_backend::interaction::{Chip, Interaction};
use p3_field::PrimeField64;

use super::columns::Page2ReadCols;
use super::Page2ReadChip;

impl<F: PrimeField64> Chip<F> for Page2ReadChip {
    // fn sends(&self) -> Vec<Interaction<F>> {
    //     let num_cols = XorLimbsCols::<N, M, F>::get_width();
    //     let all_cols = (0..num_cols).collect::<Vec<usize>>();

    //     let cols_numbered = XorLimbsCols::<N, M, F>::cols_numbered(&all_cols);
    //     self.sends_custom(cols_numbered)
    // }

    fn receives(&self) -> Vec<Interaction<F>> {
        let num_cols = self.get_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_numbered = Page2ReadCols::<F>::cols_numbered(self.key_len, &all_cols);
        self.receives_custom(cols_numbered)
    }

    fn sends(&self) -> Vec<Interaction<F>> {
        let num_cols = self.get_width();
        let all_cols = (0..num_cols).collect::<Vec<usize>>();

        let cols_numbered = Page2ReadCols::<F>::cols_numbered(self.key_len, &all_cols);
        self.sends_custom(cols_numbered)
    }
}
