use std::{borrow::Borrow, sync::Arc};

use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use crate::{
    is_less_than::{columns::IsLessThanCols, IsLessThanChip},
    range_gate::RangeCheckerGateChip,
    sub_chip::{AirConfig, SubAir},
};

use super::{
    columns::{IsLessThanTupleAuxCols, IsLessThanTupleCols, IsLessThanTupleIOCols},
    IsLessThanTupleAir, IsLessThanTupleChip,
};

impl AirConfig for IsLessThanTupleChip {
    type Cols<T> = IsLessThanTupleCols<T>;
}

impl<F: Field> BaseAir<F> for IsLessThanTupleChip {
    fn width(&self) -> usize {
        IsLessThanTupleCols::<F>::get_width(
            self.air.limb_bits().clone(),
            *self.air.decomp(),
            self.air.tuple_len(),
        )
    }
}

impl<AB: AirBuilder> Air<AB> for IsLessThanTupleChip {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();

        let local = main.row_slice(0);
        let local: &[AB::Var] = (*local).borrow();

        let local_cols = IsLessThanTupleCols::<AB::Var>::from_slice(
            local,
            self.air.limb_bits().clone(),
            *self.air.decomp(),
            self.air.tuple_len(),
        );

        SubAir::eval(&self.air, builder, local_cols.io, local_cols.aux);
    }
}

// sub-chip with constraints to check whether one tuple is less than the another
impl<AB: AirBuilder> SubAir<AB> for IsLessThanTupleAir {
    type IoView = IsLessThanTupleIOCols<AB::Var>;
    type AuxView = IsLessThanTupleAuxCols<AB::Var>;

    // constrain that x < y lexicographically
    fn eval(&self, builder: &mut AB, io: Self::IoView, aux: Self::AuxView) {
        let x = io.x.clone();
        let y = io.y.clone();

        for i in 0..x.len() {
            let x_val = x[i];
            let y_val = y[i];

            let range_checker_dummy = Arc::new(RangeCheckerGateChip::new(
                *self.bus_index(),
                *self.range_max(),
            ));

            let is_less_than_chip_dummy = IsLessThanChip::new(
                *self.bus_index(),
                *self.range_max(),
                self.limb_bits()[i],
                *self.decomp(),
                range_checker_dummy,
            );

            // here we constrain that less_than[i] indicates whether x[i] < y[i] using the IsLessThan subchip
            let mut is_less_than_slice = vec![x_val, y_val];
            is_less_than_slice.push(aux.less_than[i]);
            is_less_than_slice.push(aux.lower_bits[i]);
            is_less_than_slice.extend_from_slice(&aux.lower_bits_decomp[i]);

            let is_less_than_cols = IsLessThanCols::<AB::Var>::from_slice(
                &is_less_than_slice,
                self.limb_bits()[i],
                *self.decomp(),
            );

            SubAir::eval(
                &is_less_than_chip_dummy.air,
                builder,
                is_less_than_cols.io,
                is_less_than_cols.aux,
            );
        }

        for i in 0..x.len() {
            // constrain that diff is the difference between the two elements of consecutive rows
            let diff = y[i] - x[i];
            builder.assert_eq(diff, aux.diff[i]);
        }

        // together, these constrain that is_equal is the indicator for whether diff == 0, i.e. x[i] = y[i]
        for i in 0..self.tuple_len() {
            let diff = aux.diff[i];
            let is_equal = aux.is_zero[i];
            let inverse = aux.inverses[i];

            // check that diff * is_equal = 0
            builder.assert_zero(diff * is_equal);
            // check that is_equal is boolean
            builder.assert_zero(is_equal * (AB::Expr::one() - is_equal));
            // check that inverse * (diff + is_equal) = 1
            builder.assert_one(inverse * (diff + is_equal));
        }

        // to check whether one row is less than another, we can use the indicators to generate a boolean
        // expression; the idea is that, starting at the most significant limb, a row is less than the next
        // if all the limbs more significant are equal and the current limb is less than the corresponding
        // limb in the next row
        let mut check_less_than: AB::Expr = AB::Expr::zero();
        let less_than = aux.less_than.clone();

        for (i, &less_than_value) in less_than.iter().enumerate() {
            let mut curr_expr: AB::Expr = less_than_value.into();
            for &is_zero_value in &aux.is_zero[i + 1..] {
                curr_expr *= is_zero_value.into();
            }
            check_less_than += curr_expr;
        }

        builder.assert_eq(io.tuple_less_than, check_less_than);
    }
}
