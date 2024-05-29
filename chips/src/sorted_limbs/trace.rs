use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

use super::{columns::SortedLimbsCols, SortedLimbsChip};

impl<const MAX: u32> SortedLimbsChip<MAX> {
    pub fn generate_trace<F: PrimeField64>(&self) -> RowMajorMatrix<F> {
        let num_xor_cols: usize =
            SortedLimbsCols::<F>::get_width(self.limb_bits(), self.decomp(), self.key_vec_len());

        let keys = self.keys();

        let num_limbs = (self.limb_bits() + self.decomp() - 1) / self.decomp();

        let rows = keys
            .iter()
            .map(|x| {
                let mut row = vec![];
                for &item in x.iter() {
                    row.push(F::from_canonical_u32(item));
                }
                for &val in x.iter() {
                    // decompose each limb into sublimbs of size self.decomp() bits
                    for j in 0..num_limbs {
                        let bits = (val >> (j * self.decomp())) & ((1 << self.decomp()) - 1);
                        row.push(F::from_canonical_u32(bits));
                        self.range_checker_gate.add_count(bits)
                    }
                    // the last sublimb should be of size self.limb_bits() % self.decomp() bits,
                    // so we need to shift it to constrain this
                    let bits =
                        (val >> ((num_limbs - 1) * self.decomp())) & ((1 << self.decomp()) - 1);
                    self.range_checker_gate
                        .add_count(bits << (self.decomp() - (self.limb_bits() % self.decomp())));

                    row.push(F::from_canonical_u32(
                        bits << (self.decomp() - (self.limb_bits() % self.decomp())),
                    ));
                }
                row
            })
            .collect::<Vec<_>>();

        RowMajorMatrix::new(rows.concat(), num_xor_cols)
    }
}
