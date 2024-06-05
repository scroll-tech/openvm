use std::borrow::Borrow;

use afs_stark_backend::air_builders::PartitionedAirBuilder;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use super::{columns::OfflineCheckerCols, OfflineChecker};
use crate::{
    is_equal_vec::{columns::IsEqualVecCols, IsEqualVecChip},
    sub_chip::{AirConfig, SubAir},
};

impl AirConfig for OfflineChecker {
    type Cols<T> = OfflineCheckerCols<T>;
}

impl<F: Field> BaseAir<F> for OfflineChecker {
    fn width(&self) -> usize {
        self.air_width()
    }
}

/// This imposes constraints to make sure rows follow the format that we want (outlined in trace.rs)
impl<AB: PartitionedAirBuilder> Air<AB> for OfflineChecker
where
    AB::M: Clone,
{
    fn eval(&self, builder: &mut AB) {
        let main = &builder.partitioned_main()[0].clone();

        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let local: &[AB::Var] = (*local).borrow();
        let next: &[AB::Var] = (*next).borrow();

        let local_cols =
            OfflineCheckerCols::from_slice(local, self.page_width(), self.idx_len, self.data_len);
        let next_cols =
            OfflineCheckerCols::from_slice(next, self.page_width(), self.idx_len, self.data_len);

        // Making sure bits are bools
        builder.assert_bool(local_cols.is_initial);
        builder.assert_bool(local_cols.is_final);
        builder.assert_bool(local_cols.op_type);
        builder.assert_bool(local_cols.same_idx);
        builder.assert_bool(local_cols.same_data);
        builder.assert_bool(local_cols.is_extra);

        // Making sure first row starts with same_idx, same_data being false
        builder.when_first_row().assert_zero(local_cols.same_idx);
        builder.when_first_row().assert_zero(local_cols.same_data);

        // Making sure same_idx is correct across rows
        let is_equal_idx_vec = local_cols.page_row[1..self.idx_len + 1]
            .iter()
            .copied()
            .chain(next_cols.page_row[1..self.idx_len + 1].to_vec())
            .chain(next_cols.is_equal_idx_aux.flatten())
            .collect::<Vec<AB::Var>>();
        let is_equal_idx = IsEqualVecCols::from_slice(&is_equal_idx_vec, self.idx_len);
        let is_equal_idx_chip = IsEqualVecChip::new(self.idx_len);

        SubAir::eval(
            &is_equal_idx_chip,
            &mut builder.when_transition(),
            is_equal_idx.io,
            is_equal_idx.aux,
        );

        // Making sure same_data is correct across rows
        let is_equal_data_vec = local_cols.page_row[self.idx_len + 1..]
            .iter()
            .copied()
            .chain(next_cols.page_row[self.idx_len + 1..].to_vec())
            .chain(next_cols.is_equal_data_aux.flatten())
            .collect::<Vec<AB::Var>>();
        let is_equal_data = IsEqualVecCols::from_slice(&is_equal_data_vec, self.data_len);
        let is_equal_data_chip = IsEqualVecChip::new(self.data_len);

        SubAir::eval(
            &is_equal_data_chip,
            &mut builder.when_transition(),
            is_equal_data.io,
            is_equal_data.aux,
        );

        // TODO: make sure all rows are sorted

        // Some helpers
        let and = |a: AB::Expr, b: AB::Expr| a * b;
        let or = |a: AB::Expr, b: AB::Expr| a.clone() + b.clone() - a * b;
        let implies = |a: AB::Expr, b: AB::Expr| or(AB::Expr::one() - a, b);

        // Making sure every idx block starts with a write
        // not same_idx => write
        builder.assert_one(or(
            local_cols.is_extra.into(),
            or(local_cols.same_idx.into(), local_cols.op_type.into()),
        ));

        // Making sure every idx block ends with a is_final
        builder.when_transition().assert_one(or(
            local_cols.is_extra.into(),
            or(next_cols.same_idx.into(), local_cols.is_final.into()),
        ));
        builder.when_transition().assert_one(implies(
            and(
                AB::Expr::one() - local_cols.is_extra.into(),
                next_cols.is_extra.into(),
            ),
            local_cols.is_final.into(),
        ));
        builder.when_last_row().assert_one(implies(
            AB::Expr::one() - local_cols.is_extra,
            local_cols.is_final.into(),
        ));

        // Making sure that is_initial rows only appear at the start of blocks
        // is_initial => not same_idx
        builder.assert_one(implies(
            local_cols.is_initial.into(),
            AB::Expr::one() - local_cols.same_idx,
        ));

        // Making sure that every read uses the same data as the last operation
        // read => same_data
        builder.assert_one(or(
            local_cols.is_extra.into(),
            or(local_cols.op_type.into(), local_cols.same_data.into()),
        ));

        // is_final => read
        builder.assert_one(or(
            local_cols.is_extra.into(),
            implies(
                local_cols.is_final.into(),
                AB::Expr::one() - local_cols.op_type.into(),
            ),
        ));

        // Making sure is_extra rows are at the bottom
        builder.when_transition().assert_one(implies(
            AB::Expr::one() - next_cols.is_extra,
            AB::Expr::one() - local_cols.is_extra,
        ));

        // Note that the following is implied:
        // - for every row: (is_initial => write) because is_initial => not same_idx => write
        // - for every row: (is_initial => not is_final) because is_final => read and is_initial => not same_idx => write
        // - there is at most 1 is_initial per index block because every row is sent at most once from the inital page chip
        // - there is exactly 1 is_final per index block because every row is received at most once from the final page chip
        //   and we make sure that is_final is the last row in the block
    }
}
