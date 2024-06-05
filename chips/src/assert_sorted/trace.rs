use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

use crate::sub_chip::LocalTraceInstructions;

use super::{columns::AssertSortedCols, AssertSortedChip};

impl AssertSortedChip {
    pub fn generate_trace<F: PrimeField64>(&self) -> RowMajorMatrix<F> {
        let num_cols: usize = AssertSortedCols::<F>::get_width(
            self.air.limb_bits().clone(),
            *self.air.decomp(),
            *self.air.key_vec_len(),
        );

        let mut rows: Vec<F> = vec![];
        for i in 0..*self.air.key_vec_len() {
            let key = self.air.keys()[i].clone();
            let next_key: Vec<u32> = if i == *self.air.key_vec_len() - 1 {
                vec![0; *self.air.key_vec_len()]
            } else {
                self.air.keys()[i + 1].clone()
            };

            let is_less_than_tuple_trace = LocalTraceInstructions::generate_trace_row(
                &self.is_less_than_tuple_chip,
                (key.clone(), next_key.clone()),
            )
            .flatten();

            let mut key_decomp_trace: Vec<F> = vec![];
            // decompose each limb into sublimbs of size self.decomp() bits
            for (i, &val) in key.iter().enumerate() {
                let num_limbs =
                    (self.air.limb_bits()[i] + self.air.decomp() - 1) / self.air.decomp();
                let last_limb_shift = (self.air.decomp()
                    - (self.air.limb_bits()[i] % self.air.decomp()))
                    % self.air.decomp();

                for i in 0..num_limbs {
                    let bits = (val >> (i * self.air.decomp())) & ((1 << self.air.decomp()) - 1);
                    key_decomp_trace.push(F::from_canonical_u32(bits));
                    self.range_checker.add_count(bits);
                }
                // the last sublimb should be of size self.limb_bits() % self.decomp() bits,
                // so we need to shift it to constrain this
                let bits =
                    (val >> ((num_limbs - 1) * self.air.decomp())) & ((1 << self.air.decomp()) - 1);
                if (bits << last_limb_shift) < *self.air.range_max() {
                    self.range_checker.add_count(bits << last_limb_shift);
                }
                key_decomp_trace.push(F::from_canonical_u32(bits << last_limb_shift));
            }

            let mut row: Vec<F> = is_less_than_tuple_trace[0..2 * *self.air.key_vec_len()].to_vec();
            row.extend_from_slice(&key_decomp_trace);
            row.extend_from_slice(&is_less_than_tuple_trace[2 * *self.air.key_vec_len()..]);

            rows.extend_from_slice(&row);
        }

        RowMajorMatrix::new(rows, num_cols)
    }
}
