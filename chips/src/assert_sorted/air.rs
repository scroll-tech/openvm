use std::borrow::Borrow;

use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use crate::less_than::columns::LessThanCols;
use crate::sub_chip::SubAir;

use super::columns::AssertSortedCols;
use super::AssertSortedChip;

impl<F: Field> BaseAir<F> for AssertSortedChip {
    fn width(&self) -> usize {
        AssertSortedCols::<F>::get_width(
            *self.less_than_chip.air.limb_bits(),
            *self.less_than_chip.air.decomp(),
            *self.less_than_chip.air.key_vec_len(),
        )
    }
}

impl<AB: AirBuilder> Air<AB> for AssertSortedChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();

        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let local: &[AB::Var] = (*local).borrow();

        let local_cols = AssertSortedCols::<AB::Var>::from_slice(
            local,
            *self.less_than_chip.air.limb_bits(),
            *self.less_than_chip.air.decomp(),
            *self.less_than_chip.air.key_vec_len(),
        );

        let num_limbs = (*self.less_than_chip.air.limb_bits() + *self.less_than_chip.air.decomp()
            - 1)
            / *self.less_than_chip.air.decomp();
        let key_len = *self.less_than_chip.air.key_vec_len();

        // to range check the last sublimb of the decomposed limb, we need to shift it to make sure it is in
        // the correct range
        let last_limb_shift = (*self.less_than_chip.air.decomp()
            - (*self.less_than_chip.air.limb_bits() % *self.less_than_chip.air.decomp()))
            % *self.less_than_chip.air.decomp();

        for i in 0..key_len {
            let mut key_from_limbs: AB::Expr = AB::Expr::zero();
            // constrain that the decomposition is correct
            for j in 0..num_limbs {
                key_from_limbs += local_cols.keys_decomp[i][j]
                    * AB::Expr::from_canonical_u64(1 << (j * self.less_than_chip.air.decomp()));
            }

            // constrain that the shifted last sublimb is shifted correctly
            let shifted_val = local_cols.keys_decomp[i][num_limbs - 1]
                * AB::Expr::from_canonical_u64(1 << last_limb_shift);

            builder.assert_eq(local_cols.keys_decomp[i][num_limbs], shifted_val);
            builder.assert_eq(key_from_limbs, local_cols.less_than_cols.io.key[i]);
        }

        // generate LessThanCols struct for current row and next row
        let mut local_slice: Vec<AB::Var> = local[0..key_len].to_vec();
        local_slice.extend_from_slice(&local[key_len * (num_limbs + 2)..]);

        let mut next_slice: Vec<AB::Var> = next[0..key_len].to_vec();
        next_slice
            .extend_from_slice(&next[(self.less_than_chip.air.key_vec_len() * (num_limbs + 2))..]);

        let local_cols = LessThanCols::<AB::Var>::from_slice(
            &local_slice,
            *self.less_than_chip.air.limb_bits(),
            *self.less_than_chip.air.decomp(),
            *self.less_than_chip.air.key_vec_len(),
        );

        let next_cols = LessThanCols::<AB::Var>::from_slice(
            &next_slice,
            *self.less_than_chip.air.limb_bits(),
            *self.less_than_chip.air.decomp(),
            *self.less_than_chip.air.key_vec_len(),
        );

        // constrain the current row is less than the next row
        SubAir::eval(
            &self.less_than_chip.air,
            builder,
            [local_cols.io, next_cols.io],
            local_cols.aux,
        );
    }
}
