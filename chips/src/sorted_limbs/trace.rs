use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

use super::{columns::SortedLimbsCols, SortedLimbsChip};

impl<const MAX: u32> SortedLimbsChip<MAX> {
    pub fn generate_trace<F: PrimeField64>(&self) -> RowMajorMatrix<F> {
        let num_cols: usize =
            SortedLimbsCols::<F>::get_width(self.limb_bits(), self.decomp(), self.key_vec_len());

        let keys = self.keys();

        let num_limbs = (self.limb_bits() + self.decomp() - 1) / self.decomp();

        // to range check the last sublimb of the decomposed limb, we need to shift it to make sure it is in
        // the correct range
        let last_limb_shift = (self.decomp() - (self.limb_bits() % self.decomp())) % self.decomp();

        let rows = keys
            .iter()
            .enumerate()
            .map(|(i, key)| {
                // put the key itself into the trace
                let mut row = vec![];
                for &item in key.iter() {
                    row.push(F::from_canonical_u32(item));
                }

                // decompose each limb into sublimbs of size self.decomp() bits
                for &val in key.iter() {
                    for j in 0..num_limbs {
                        let bits = (val >> (j * self.decomp())) & ((1 << self.decomp()) - 1);
                        row.push(F::from_canonical_u32(bits));
                        self.range_checker_gate.add_count(bits)
                    }
                    // the last sublimb should be of size self.limb_bits() % self.decomp() bits,
                    // so we need to shift it to constrain this
                    let bits =
                        (val >> ((num_limbs - 1) * self.decomp())) & ((1 << self.decomp()) - 1);
                    if (bits << last_limb_shift) < MAX {
                        self.range_checker_gate.add_count(bits << last_limb_shift);
                    }
                    row.push(F::from_canonical_u32(bits << last_limb_shift));
                }

                // this will contain 2^limb_bits + b - a
                let mut checks: Vec<F> = vec![];
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
                if i + 1 < keys.len() {
                    let next_key = &keys[i + 1];
                    for (j, &val) in key.iter().enumerate() {
                        let next_val = next_key[j];
                        // compute 2^limb_bits + next_val - val - 1
                        let check_less_than = (1 << self.limb_bits()) + next_val - val - 1;
                        checks.push(F::from_canonical_u32(check_less_than));
                        // the lower limb_bits bits of the check value
                        lower_bits.push(F::from_canonical_u32(
                            check_less_than & ((1 << self.limb_bits()) - 1),
                        ));
                        // we also need the u32 value to compute the decomposition later
                        lower_bits_u32.push(check_less_than & ((1 << self.limb_bits()) - 1));
                        // the (n + 1)st bit of the check value, will be 1 if a < b
                        upper_bit.push(F::from_canonical_u32(check_less_than >> self.limb_bits()));

                        // the difference between the two limbs
                        let curr_diff =
                            F::from_canonical_u32(next_val) - F::from_canonical_u32(val);
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
                } else {
                    for _ in 0..self.key_vec_len() {
                        checks.push(F::zero());
                        lower_bits.push(F::zero());
                        lower_bits_u32.push(0);
                        upper_bit.push(F::zero());
                        diff.push(F::zero());
                        is_zero.push(F::zero());
                        inverses.push(F::zero());
                    }
                }

                row.extend(checks);
                row.extend(lower_bits);
                row.extend(upper_bit);

                // decompose each element of lower_bits so we can range check that the element
                // has at most limb_bits bits
                for val in lower_bits_u32 {
                    for j in 0..num_limbs {
                        let bits = (val >> (j * self.decomp())) & ((1 << self.decomp()) - 1);
                        row.push(F::from_canonical_u32(bits));
                        self.range_checker_gate.add_count(bits);
                    }
                    let bits =
                        (val >> ((num_limbs - 1) * self.decomp())) & ((1 << self.decomp()) - 1);
                    if (bits << last_limb_shift) < MAX {
                        self.range_checker_gate.add_count(bits << last_limb_shift);
                    }
                    row.push(F::from_canonical_u32(bits << last_limb_shift));
                }

                row.extend(diff);
                row.extend(is_zero);
                row.extend(inverses);

                row
            })
            .collect::<Vec<_>>();

        RowMajorMatrix::new(rows.concat(), num_cols)
    }
}
