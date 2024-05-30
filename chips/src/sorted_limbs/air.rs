use std::borrow::Borrow;

use p3_air::{Air, AirBuilder, AirBuilderWithPublicValues, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use super::columns::SortedLimbsCols;
use super::SortedLimbsChip;

impl<F: Field, const MAX: u32> BaseAir<F> for SortedLimbsChip<MAX> {
    fn width(&self) -> usize {
        SortedLimbsCols::<F>::get_width(self.limb_bits(), self.decomp(), self.key_vec_len())
    }
}

impl<AB: AirBuilderWithPublicValues, const MAX: u32> Air<AB> for SortedLimbsChip<MAX>
where
    AB: AirBuilder,
    AB::Var: Clone,
{
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let _pis = builder.public_values();

        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let local: &[AB::Var] = (*local).borrow();
        let next: &[AB::Var] = (*next).borrow();

        let local_cols = SortedLimbsCols::<AB::Var>::from_slice(
            local,
            self.limb_bits(),
            self.decomp(),
            self.key_vec_len(),
        );
        let next_cols = SortedLimbsCols::<AB::Var>::from_slice(
            next,
            self.limb_bits(),
            self.decomp(),
            self.key_vec_len(),
        );

        let num_limbs = (self.limb_bits() + self.decomp() - 1) / self.decomp();
        let key_len = self.key_vec_len();

        // to range check the last sublimb of the decomposed limb, we need to shift it to make sure it is in
        // the correct range
        let last_limb_shift = (self.decomp() - (self.limb_bits() % self.decomp())) % self.decomp();

        for i in 0..key_len {
            let mut key_from_limbs: AB::Expr = AB::Expr::zero();
            // constrain that the decomposition is correct
            for j in 0..num_limbs {
                key_from_limbs += local_cols.keys_decomp[i][j]
                    * AB::Expr::from_canonical_u64(1 << (j * self.decomp()));
            }

            // constrain that the shifted last sublimb is shifted correctly
            let shifted_val = local_cols.keys_decomp[i][num_limbs - 1]
                * AB::Expr::from_canonical_u64(1 << last_limb_shift);

            builder.assert_eq(local_cols.keys_decomp[i][num_limbs], shifted_val);
            builder.assert_eq(key_from_limbs, local_cols.key[i]);
        }

        self.is_less_than(builder, local_cols, next_cols);
    }
}
