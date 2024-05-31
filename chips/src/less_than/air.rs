use std::borrow::Borrow;

use p3_air::{Air, AirBuilder, AirBuilderWithPublicValues, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use crate::sub_chip::{AirConfig, SubAir};

use super::{columns::LessThanCols, LessThanChip};

impl<F: Field, const MAX: u32> BaseAir<F> for LessThanChip<MAX> {
    fn width(&self) -> usize {
        LessThanCols::<F>::get_width(self.limb_bits(), self.decomp(), self.key_vec_len())
    }
}

impl<AB: AirBuilderWithPublicValues, const MAX: u32> Air<AB> for LessThanChip<MAX>
where
    AB: AirBuilder,
    AB::Var: Clone,
{
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let _pis = builder.public_values();

        let (local, _next) = (main.row_slice(0), main.row_slice(1));
        let local: &[AB::Var] = (*local).borrow();

        let local_cols = LessThanCols::<AB::Var>::from_slice(
            local,
            self.limb_bits(),
            self.decomp(),
            self.key_vec_len(),
        );

        SubAir::eval(self, builder, vec![local_cols]);
    }
}

impl<const MAX: u32> AirConfig for LessThanChip<MAX> {
    type Cols<T> = LessThanCols<T>;
}

// sub-chip with constraints to check whether one key is less than the next (row-wise)
impl<const MAX: u32, AB: AirBuilder> SubAir<AB> for LessThanChip<MAX> {
    type ColsPassed = Vec<Self::Cols<AB::Var>>;

    fn eval(&self, builder: &mut AB, cols: Self::ColsPassed) {
        let local_cols = &cols[0];
        let next_cols = &cols[1];

        // num_limbs is the number of sublimbs per limb, not including the shifted last sublimb
        let num_limbs = (self.limb_bits() + self.decomp() - 1) / self.decomp();

        let intermed_sum = local_cols.intermed_sum.clone();
        let lower_bits = local_cols.lower_bits.clone();
        let upper_bit = local_cols.upper_bit.clone();
        let lower_bits_decomp = local_cols.lower_bits_decomp.clone();

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
