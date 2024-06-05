use std::{borrow::Borrow, sync::Arc};

use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use crate::{
    is_equal::{
        columns::{IsEqualAuxCols, IsEqualCols, IsEqualIOCols},
        IsEqualChip,
    },
    is_less_than::{
        columns::{IsLessThanAuxCols, IsLessThanCols, IsLessThanIOCols},
        IsLessThanChip,
    },
    range_gate::RangeCheckerGateChip,
    sub_chip::{AirConfig, SubAir},
};

use super::{
    columns::{IsLessThanTupleAuxCols, IsLessThanTupleCols, IsLessThanTupleIOCols},
    IsLessThanTupleAir, IsLessThanTupleChip,
};

impl AirConfig for IsLessThanTupleAir {
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

            let is_less_than_chip_dummy = IsLessThanChip {
                air: self.is_lt_airs[i].clone(),
                range_checker: range_checker_dummy,
            };

            // here we constrain that less_than[i] indicates whether x[i] < y[i] using the IsLessThan subchip
            let is_less_than_cols = IsLessThanCols {
                io: IsLessThanIOCols {
                    x: x_val,
                    y: y_val,
                    less_than: aux.less_than[i],
                },
                aux: IsLessThanAuxCols {
                    lower: aux.less_than_cols[i].lower,
                    lower_decomp: aux.less_than_cols[i].lower_decomp.clone(),
                },
            };

            SubAir::eval(
                &is_less_than_chip_dummy.air,
                builder,
                is_less_than_cols.io,
                is_less_than_cols.aux,
            );
        }

        // together, these constrain that is_equal is the indicator for whether diff == 0, i.e. x[i] = y[i]
        for i in 0..x.len() {
            let is_equal = aux.is_equal[i];
            let inv = aux.is_equal_cols[i].inv;

            let is_equal_chip = IsEqualChip {};
            let is_equal_cols = IsEqualCols {
                io: IsEqualIOCols {
                    x: x[i],
                    y: y[i],
                    is_equal,
                },
                aux: IsEqualAuxCols { inv },
            };

            SubAir::eval(&is_equal_chip, builder, is_equal_cols.io, is_equal_cols.aux);
        }

        // to check whether one row is less than another, we can use the indicators to generate a boolean
        // expression; the idea is that, starting at the most significant limb, a row is less than the next
        // if all the limbs more significant are equal and the current limb is less than the corresponding
        // limb in the next row
        let mut check_less_than: AB::Expr = AB::Expr::zero();
        let less_than = aux.less_than.clone();

        for (i, &less_than_value) in less_than.iter().enumerate() {
            let mut curr_expr: AB::Expr = less_than_value.into();
            for &is_equal_value in &aux.is_equal[i + 1..] {
                curr_expr *= is_equal_value.into();
            }
            check_less_than += curr_expr;
        }

        builder.assert_eq(io.tuple_less_than, check_less_than);
    }
}
