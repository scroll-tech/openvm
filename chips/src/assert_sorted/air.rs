use std::borrow::Borrow;

use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use crate::is_less_than_tuple::columns::IsLessThanTupleCols;
use crate::sub_chip::SubAir;

use super::columns::AssertSortedCols;
use super::AssertSortedChip;

impl<F: Field> BaseAir<F> for AssertSortedChip {
    fn width(&self) -> usize {
        AssertSortedCols::<F>::get_width(
            self.air.limb_bits().clone(),
            *self.air.decomp(),
            *self.air.key_vec_len(),
        )
    }
}

impl<AB: AirBuilder> Air<AB> for AssertSortedChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();

        // get the current row and the next row
        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let local: &[AB::Var] = (*local).borrow();
        let next: &[AB::Var] = (*next).borrow();

        let local_cols = AssertSortedCols::<AB::Var>::from_slice(
            local,
            self.air.limb_bits().clone(),
            *self.air.decomp(),
            *self.air.key_vec_len(),
        );

        let next_cols = AssertSortedCols::<AB::Var>::from_slice(
            next,
            self.air.limb_bits().clone(),
            *self.air.decomp(),
            *self.air.key_vec_len(),
        );

        let key_len = *self.air.key_vec_len();

        for i in 0..key_len {
            let mut key_from_limbs: AB::Expr = AB::Expr::zero();

            // num_limbs is the number of sublimbs the current limb should be decomposed into
            let num_limbs = (self.air.limb_bits()[i] + self.air.decomp() - 1) / self.air.decomp();
            // to range check the last sublimb, we need to shift it
            let last_limb_shift = (self.air.decomp()
                - (self.air.limb_bits()[i] % self.air.decomp()))
                % self.air.decomp();

            // constrain that the decomposition is correct
            for j in 0..num_limbs {
                key_from_limbs += local_cols.keys_decomp[i][j]
                    * AB::Expr::from_canonical_u64(1 << (j * self.air.decomp()));
            }

            // constrain that the shifted last sublimb is shifted correctly
            let shifted_val = local_cols.keys_decomp[i][num_limbs - 1]
                * AB::Expr::from_canonical_u64(1 << last_limb_shift);

            builder.assert_eq(local_cols.keys_decomp[i][num_limbs], shifted_val);
            builder.assert_eq(key_from_limbs, local_cols.is_less_than_tuple_cols.io.x[i]);

            // constrain that the keys are consistent across rows
            builder.when_transition().assert_eq(
                local_cols.is_less_than_tuple_cols.io.y[i],
                next_cols.is_less_than_tuple_cols.io.x[i],
            );
        }

        // constrain that the current key is less than the next
        builder
            .when_transition()
            .assert_one(local_cols.is_less_than_tuple_cols.io.tuple_less_than);

        // generate IsLessThanTupleCols struct for current row and next row
        let mut curr_start_idx = 0;
        let mut curr_end_idx = 2 * key_len;
        // get the current key and next key
        let mut local_slice: Vec<AB::Var> = local[curr_start_idx..curr_end_idx].to_vec();

        // skip the key decomposition
        for i in 0..key_len {
            let num_limbs = (self.air.limb_bits()[i] + self.air.decomp() - 1) / self.air.decomp();
            curr_end_idx += num_limbs + 1;
        }

        // get the rest of the columns
        curr_start_idx = curr_end_idx;

        local_slice.extend_from_slice(&local[curr_start_idx..]);

        let local_cols = IsLessThanTupleCols::<AB::Var>::from_slice(
            &local_slice,
            self.air.limb_bits().clone(),
            *self.air.decomp(),
            *self.air.key_vec_len(),
        );

        // constrain the indicator that we used to check whether the current key < next key is correct
        SubAir::eval(
            &self.is_less_than_tuple_chip.air,
            &mut builder.when_transition(),
            local_cols.io,
            local_cols.aux,
        );
    }
}
