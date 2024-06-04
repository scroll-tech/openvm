use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

use crate::sub_chip::LocalTraceInstructions;

use super::{
    columns::{LessThanAuxCols, LessThanCols, LessThanIOCols},
    LessThanChip,
};

impl LessThanChip {
    pub fn generate_trace<F: PrimeField64>(&self) -> RowMajorMatrix<F> {
        let num_cols: usize = LessThanCols::<F>::get_width(
            *self.air.limb_bits(),
            *self.air.decomp(),
            *self.air.key_vec_len(),
        );

        let mut rows: Vec<F> = vec![];
        for i in 0..*self.air.key_vec_len() {
            let key = self.air.keys[i].clone();
            let next_key: Vec<u32> = if i == *self.air.key_vec_len() - 1 {
                vec![0; *self.air.key_vec_len()]
            } else {
                self.air.keys[i + 1].clone()
            };
            let row = self.generate_trace_row((key, next_key)).flatten();
            rows.extend_from_slice(&row);
        }

        RowMajorMatrix::new(rows, num_cols)
    }
}

impl<F: PrimeField64> LocalTraceInstructions<F> for LessThanChip {
    type LocalInput = (Vec<u32>, Vec<u32>);

    fn generate_trace_row(&self, consecutive_keys: (Vec<u32>, Vec<u32>)) -> Self::Cols<F> {
        let (key, next_key) = consecutive_keys;
        let num_limbs = (self.air.limb_bits() + self.air.decomp() - 1) / self.air.decomp();
        let last_limb_shift =
            (self.air.decomp() - (self.air.limb_bits() % self.air.decomp())) % self.air.decomp();

        // the lower limb_bits bits of the corresponding check value
        let mut lower_bits: Vec<F> = vec![];
        let mut lower_bits_u32: Vec<u32> = vec![];
        // the (n + 1)st bits of the corresponding check value, will be 1 if a < b
        let mut upper_bit: Vec<F> = vec![];

        // contains the difference between consecutive rows
        let mut diff: Vec<F> = vec![];
        // contains indicator whether difference is zero
        let mut is_zero: Vec<F> = vec![];
        // contains y such that y * (i + x) = 1
        let mut inverses: Vec<F> = vec![];

        // we compute the indicators, which only matter if the row is not the last
        for (j, &val) in key.iter().enumerate() {
            let next_val = next_key[j];
            // compute 2^limb_bits + next_val - val - 1
            let check_less_than = (1 << self.air.limb_bits()) + next_val - val - 1;

            // the lower limb_bits bits of the check value
            lower_bits.push(F::from_canonical_u32(
                check_less_than & ((1 << self.air.limb_bits()) - 1),
            ));
            // we also need the u32 value to compute the decomposition later
            lower_bits_u32.push(check_less_than & ((1 << self.air.limb_bits()) - 1));
            // the (n + 1)st bit of the check value, will be 1 if a < b
            upper_bit.push(F::from_canonical_u32(
                check_less_than >> self.air.limb_bits(),
            ));

            // the difference between the two limbs
            let curr_diff = F::from_canonical_u32(next_val) - F::from_canonical_u32(val);
            diff.push(curr_diff);

            // compute the equal indicator and inverses
            if next_val == val {
                is_zero.push(F::one());
                inverses.push((curr_diff + F::one()).inverse());
            } else {
                is_zero.push(F::zero());
                inverses.push(curr_diff.inverse());
            }
        }

        let mut lower_bits_decomp: Vec<Vec<F>> = vec![];

        // decompose each element of lower_bits so we can range check that the element
        // has at most limb_bits bits
        for i in 0..lower_bits_u32.len() {
            let val = lower_bits_u32[i];
            if i != lower_bits_u32.len() {
                let mut curr_decomp: Vec<F> = vec![];
                for j in 0..num_limbs {
                    let bits = (val >> (j * self.air.decomp())) & ((1 << self.air.decomp()) - 1);
                    curr_decomp.push(F::from_canonical_u32(bits));
                    self.range_checker_gate.add_count(bits);
                }
                let bits =
                    (val >> ((num_limbs - 1) * self.air.decomp())) & ((1 << self.air.decomp()) - 1);
                if (bits << last_limb_shift) < *self.air.range_max() {
                    self.range_checker_gate.add_count(bits << last_limb_shift);
                }
                curr_decomp.push(F::from_canonical_u32(bits << last_limb_shift));
                lower_bits_decomp.push(curr_decomp);
            } else {
                lower_bits_decomp.push(vec![F::zero(); num_limbs + 1]);
            }
        }

        let io = LessThanIOCols {
            key: key.into_iter().map(F::from_canonical_u32).collect(),
        };
        let aux = LessThanAuxCols {
            lower_bits,
            upper_bit,
            lower_bits_decomp,
            diff,
            is_zero,
            inverses,
        };

        LessThanCols { io, aux }
    }
}
