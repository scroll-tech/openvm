use std::borrow::Borrow;

use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use crate::sub_chip::{AirConfig, SubAir};

use super::{
    columns::{IsLessThanAuxCols, IsLessThanCols, IsLessThanIOCols},
    IsLessThanAir, IsLessThanChip,
};

impl AirConfig for IsLessThanChip {
    type Cols<T> = IsLessThanCols<T>;
}

impl<F: Field> BaseAir<F> for IsLessThanChip {
    fn width(&self) -> usize {
        IsLessThanCols::<F>::get_width(*self.air.limb_bits(), *self.air.decomp())
    }
}

impl<AB: AirBuilder> Air<AB> for IsLessThanChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();

        let local = main.row_slice(0);
        let local: &[AB::Var] = (*local).borrow();

        let local_cols =
            IsLessThanCols::<AB::Var>::from_slice(local, *self.air.limb_bits(), *self.air.decomp());

        SubAir::eval(&self.air, builder, local_cols.io, local_cols.aux);
    }
}

// sub-chip with constraints to check whether one key is less than the next (row-wise)
impl<AB: AirBuilder> SubAir<AB> for IsLessThanAir {
    type IoView = IsLessThanIOCols<AB::Var>;
    type AuxView = IsLessThanAuxCols<AB::Var>;

    // constrain that local_key < next_key lexicographically
    fn eval(&self, builder: &mut AB, io: Self::IoView, aux: Self::AuxView) {
        let x = io.x;
        let y = io.y;
        let less_than = io.less_than;

        let local_aux = &aux;

        // num_limbs is the number of sublimbs per limb, not including the shifted last sublimb
        let num_limbs = (self.limb_bits() + self.decomp() - 1) / self.decomp();

        let lower_bits = local_aux.lower_bits;
        let upper_bit = local_aux.upper_bit;
        let lower_bits_decomp = local_aux.lower_bits_decomp.clone();

        // to range check the last sublimb of the decomposed limb, we need to shift it to make sure it is in
        // the correct range
        let last_limb_shift = (self.decomp() - (self.limb_bits() % self.decomp())) % self.decomp();

        // this is the desired intermediate value (i.e. 2^limb_bits + y - x - 1)
        let intermed_val =
            y - x + AB::Expr::from_canonical_u64(1 << self.limb_bits()) - AB::Expr::one();

        // constrain that the lower bits + upper bit * 2^limb_bits is the correct intermediate sum
        let check_val =
            lower_bits + upper_bit * AB::Expr::from_canonical_u64(1 << self.limb_bits());

        builder.assert_eq(intermed_val, check_val);

        // constrain that the decomposition of lower_bits is correct
        let lower_bits_from_decomp = lower_bits_decomp
            .iter()
            .enumerate()
            .take(num_limbs)
            .fold(AB::Expr::zero(), |acc, (i, &val)| {
                acc + val * AB::Expr::from_canonical_u64(1 << (i * self.decomp()))
            });

        builder.assert_eq(lower_bits_from_decomp, lower_bits);

        let shifted_val =
            lower_bits_decomp[num_limbs - 1] * AB::Expr::from_canonical_u64(1 << last_limb_shift);

        // constrain that the shifted last limb is shifted correctly
        builder.assert_eq(lower_bits_decomp[num_limbs], shifted_val);

        // constrain that upper_bit is a boolean
        let is_bool = upper_bit * (AB::Expr::one() - upper_bit);
        builder.assert_zero(is_bool);

        builder.assert_eq(less_than, upper_bit);
    }
}
