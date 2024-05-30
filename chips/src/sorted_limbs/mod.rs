use crate::range_gate::RangeCheckerGateChip;

use afs_stark_backend::interaction::Interaction;
use columns::SortedLimbsCols;
use p3_air::{AirBuilder, VirtualPairCol};
use p3_field::{AbstractField, PrimeField64};

#[cfg(test)]
pub mod tests;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

/**
 * This Chip constrains that consecutive rows are sorted lexicographically.
 *
 * Each row consists of a key decomposed into limbs, and the chip constrains
 * each limb has at most limb_bits bits, where limb_bits is at most 31. It
 * does this by interacting with a RangeCheckerGateChip. Because the range checker
 * gate can take MAX up to 2^20, we further decompose each limb into sublimbs
 * of size decomp bits.
 */
#[derive(Default)]
pub struct SortedLimbsChip<const MAX: u32> {
    bus_index: usize,
    limb_bits: usize,
    decomp: usize,
    key_vec_len: usize,
    keys: Vec<Vec<u32>>,

    pub range_checker_gate: RangeCheckerGateChip<MAX>,
}

impl<const MAX: u32> SortedLimbsChip<MAX> {
    pub fn new(
        bus_index: usize,
        limb_bits: usize,
        decomp: usize,
        key_vec_len: usize,
        keys: Vec<Vec<u32>>,
    ) -> Self {
        Self {
            bus_index,
            limb_bits,
            decomp,
            key_vec_len,
            keys,
            range_checker_gate: RangeCheckerGateChip::<MAX>::new(bus_index),
        }
    }

    pub fn bus_index(&self) -> usize {
        self.bus_index
    }

    pub fn limb_bits(&self) -> usize {
        self.limb_bits
    }

    pub fn decomp(&self) -> usize {
        self.decomp
    }

    pub fn key_vec_len(&self) -> usize {
        self.key_vec_len
    }

    pub fn keys(&self) -> Vec<Vec<u32>> {
        self.keys.clone()
    }

    pub fn sends_custom<F: PrimeField64>(
        &self,
        cols: SortedLimbsCols<usize>,
    ) -> Vec<Interaction<F>> {
        // num_limbs is the number of sublimbs per limb of key, not including the
        // shifted last sublimb
        let num_limbs = (self.limb_bits() + self.decomp() - 1) / self.decomp();
        let num_keys = self.key_vec_len();

        let mut interactions = vec![];

        // we will range check the decomposed limbs of the key
        for i in 0..num_keys {
            // add 1 to account for the shifted last sublimb
            for j in 0..(num_limbs + 1) {
                interactions.push(Interaction {
                    fields: vec![VirtualPairCol::single_main(cols.keys_decomp[i][j])],
                    count: VirtualPairCol::constant(F::one()),
                    argument_index: self.bus_index(),
                });
            }
        }

        // we also range check the limbs of the lower_bits so that we know each element
        // of lower_bits has at most limb_bits bits
        for i in 0..num_keys {
            for j in 0..(num_limbs + 1) {
                interactions.push(Interaction {
                    fields: vec![VirtualPairCol::single_main(cols.lower_bits_decomp[i][j])],
                    count: VirtualPairCol::constant(F::one()),
                    argument_index: self.bus_index(),
                });
            }
        }

        interactions
    }

    // sub-chip with constraints to check whether one key is less than the next (row-wise)
    pub fn is_less_than<AB: AirBuilder>(
        &self,
        builder: &mut AB,
        local_cols: SortedLimbsCols<AB::Var>,
        next_cols: SortedLimbsCols<AB::Var>,
    ) where
        AB::Var: Clone,
    {
        // num_limbs is the number of sublimbs per limb, not including the shifted last sublimb
        let num_limbs = (self.limb_bits() + self.decomp() - 1) / self.decomp();

        let intermed_sum = local_cols.intermed_sum;
        let lower_bits = local_cols.lower_bits;
        let upper_bit = local_cols.upper_bit;
        let lower_bits_decomp = local_cols.lower_bits_decomp;

        // we want to check these constraints for each row except the last one
        let mut when_transition = builder.when_transition();

        // to range check the last sublimb of the decomposed limb, we need to shift it to make sure it is in
        // the correct range
        let last_limb_shift = (self.decomp() - (self.limb_bits() % self.decomp())) % self.decomp();

        for (i, (key_local, key_next)) in
            local_cols.key.iter().zip(next_cols.key.iter()).enumerate()
        {
            // this is the desired intermediate value (i.e. 2^limb_bits + b - a - 1)
            let intermed_val = *key_next - *key_local
                + AB::Expr::from_canonical_u64(1 << self.limb_bits())
                - AB::Expr::one();

            // constrain that the intermed val (2^limb_bits + key_next - key_local) is correct
            when_transition.assert_eq(intermed_sum[i], intermed_val);

            // constrain that lower_bits[i] + upper_bit[i] * 2^limb_bits is the correct intermediate sum
            let check_val =
                lower_bits[i] + upper_bit[i] * AB::Expr::from_canonical_u64(1 << self.limb_bits());
            when_transition.assert_eq(intermed_sum[i], check_val);

            // constrain that diff is the difference between the two elements of consecutive rows
            let diff = *key_next - *key_local;
            //when_transition.assert_zero(local_cols.diff[i]);
            when_transition.assert_eq(diff, local_cols.diff[i]);
        }

        for i in 0..self.key_vec_len() {
            let mut lower_bits_from_decomp: AB::Expr = AB::Expr::zero();
            // constrain that the decomposition of each lower_bits element is correct
            for j in 0..num_limbs {
                lower_bits_from_decomp += lower_bits_decomp[i][j]
                    * AB::Expr::from_canonical_u64(1 << (j * self.decomp()));
            }

            // constrain that the shifted last limb is shifted correctly
            let shifted_val = lower_bits_decomp[i][num_limbs - 1]
                * AB::Expr::from_canonical_u64(1 << last_limb_shift);

            when_transition.assert_eq(lower_bits_decomp[i][num_limbs], shifted_val);
            when_transition.assert_eq(lower_bits_from_decomp, lower_bits[i]);
        }

        for upper_bit_value in &upper_bit {
            // constrain that each element in upper_bit is a boolean
            let is_bool = *upper_bit_value * (AB::Expr::one() - *upper_bit_value);
            when_transition.assert_zero(is_bool);
        }

        for i in 0..self.key_vec_len() {
            let diff = local_cols.diff[i];
            let is_equal = local_cols.is_zero[i];
            let inverse = local_cols.inverses[i];

            // check that diff * is_equal = 0
            when_transition.assert_zero(diff * is_equal);
            // check that is_equal is boolean
            when_transition.assert_zero(is_equal * (AB::Expr::one() - is_equal));
            // check that inverse * (diff + is_equal) = 1
            when_transition.assert_one(inverse * (diff + is_equal));
        }

        // to check whether one row is less than another, we can use the indicators to generate a boolean
        // expression; the idea is that, starting at the most significant limb, a row is less than the next
        // if all the limbs more significant are equal and the current limb is less than the corresponding
        // limb in the next row
        let mut check_less_than: AB::Expr = AB::Expr::zero();

        for (i, &upper_bit_value) in upper_bit.iter().enumerate() {
            let mut curr_expr: AB::Expr = upper_bit_value.into();
            for &is_zero_value in &local_cols.is_zero[i + 1..] {
                curr_expr *= is_zero_value.into();
            }
            check_less_than += curr_expr;
        }
        when_transition.assert_one(check_less_than);
    }
}
