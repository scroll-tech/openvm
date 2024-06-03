use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

use crate::sub_chip::LocalTraceInstructions;

use super::{columns::AssertSortedCols, AssertSortedChip};

impl<const MAX: u32> AssertSortedChip<MAX> {
    pub fn generate_trace<F: PrimeField64>(&self) -> RowMajorMatrix<F> {
        let num_cols: usize = AssertSortedCols::<F>::get_width(
            *self.less_than_chip.air.limb_bits(),
            *self.less_than_chip.air.decomp(),
            *self.less_than_chip.air.key_vec_len(),
        );

        let num_limbs = (*self.less_than_chip.air.limb_bits() + *self.less_than_chip.air.decomp()
            - 1)
            / *self.less_than_chip.air.decomp();

        // to range check the last sublimb of the decomposed limb, we need to shift it to make sure it is in
        // the correct range
        let last_limb_shift = (self.less_than_chip.air.decomp()
            - (self.less_than_chip.air.limb_bits() % self.less_than_chip.air.decomp()))
            % self.less_than_chip.air.decomp();

        let mut rows: Vec<F> = vec![];
        for i in 0..*self.less_than_chip.air.key_vec_len() {
            let key = self.less_than_chip.air.keys()[i].clone();
            let next_key: Vec<u32> = if i == *self.less_than_chip.air.key_vec_len() - 1 {
                vec![0; *self.less_than_chip.air.key_vec_len()]
            } else {
                self.less_than_chip.air.keys()[i + 1].clone()
            };

            let less_than_trace = LocalTraceInstructions::generate_trace_row(
                &self.less_than_chip,
                (key.clone(), next_key.clone()),
            )
            .flatten();

            let mut key_decomp_trace: Vec<F> = vec![];
            // decompose each limb into sublimbs of size self.decomp() bits
            for &val in key.iter() {
                for i in 0..num_limbs {
                    let bits = (val >> (i * self.less_than_chip.air.decomp()))
                        & ((1 << self.less_than_chip.air.decomp()) - 1);
                    key_decomp_trace.push(F::from_canonical_u32(bits));
                    self.less_than_chip.range_checker_gate.add_count(bits);
                }
                // the last sublimb should be of size self.limb_bits() % self.decomp() bits,
                // so we need to shift it to constrain this
                let bits = (val >> ((num_limbs - 1) * self.less_than_chip.air.decomp()))
                    & ((1 << self.less_than_chip.air.decomp()) - 1);
                if (bits << last_limb_shift) < MAX {
                    self.less_than_chip
                        .range_checker_gate
                        .add_count(bits << last_limb_shift);
                }
                key_decomp_trace.push(F::from_canonical_u32(bits << last_limb_shift));
            }

            let mut row: Vec<F> =
                less_than_trace[0..*self.less_than_chip.air.key_vec_len()].to_vec();
            row.extend_from_slice(&key_decomp_trace);
            row.extend_from_slice(&less_than_trace[*self.less_than_chip.air.key_vec_len()..]);

            rows.extend_from_slice(&row);
        }

        RowMajorMatrix::new(rows, num_cols)
    }
}
