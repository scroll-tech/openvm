use std::{borrow::Borrow, fmt::Debug};

use afs_stark_backend::air_builders::PartitionedAirBuilder;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use super::{columns::MiddleChipCols, MiddleChip};
use crate::{
    is_equal_vec::{columns::IsEqualVecCols, IsEqualVecChip},
    sub_chip::{AirConfig, SubAir},
};

impl AirConfig for MiddleChip {
    type Cols<T> = MiddleChipCols<T>;
}

impl<F: Field> BaseAir<F> for MiddleChip {
    fn width(&self) -> usize {
        self.air_width()
    }
}

// TODO: look at imports in all files and make sure they're nice

/// Imposes the following constraints:
/// - Rows are sorted by key then by timestamp (clk)
/// - Every key block starts with a write
/// - Every key block ends with an is_final
/// - For every key, every read uses the same value as the last
///   operation with that key
impl<AB: PartitionedAirBuilder> Air<AB> for MiddleChip
where
    AB::M: Clone,
    AB::Var: Debug, // TODO: remove
{
    fn eval(&self, builder: &mut AB) {
        let main = &builder.partitioned_main()[0].clone();

        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let local: &[AB::Var] = (*local).borrow();
        let next: &[AB::Var] = (*next).borrow();

        println!(
            "Here in the airbuilder eval file, local.len(): {:?}",
            local.len()
        );

        let local_cols =
            MiddleChipCols::from_slice(local, self.page_width(), self.key_len, self.val_len);
        let next_cols =
            MiddleChipCols::from_slice(next, self.page_width(), self.key_len, self.val_len);

        // TODO: make sure all the relations between is_initial, is_final, op_type are followed

        // Making sure bits are bools
        builder.assert_bool(local_cols.is_initial);
        builder.assert_bool(local_cols.is_final);
        builder.assert_bool(local_cols.op_type);
        builder.assert_bool(local_cols.same_key);
        builder.assert_bool(local_cols.same_val);
        builder.assert_bool(local_cols.is_extra);

        // Making sure first row starts with same_key, same_value being false
        builder.when_first_row().assert_zero(local_cols.same_key);
        builder.when_first_row().assert_zero(local_cols.same_val);

        // Making sure same_key is correct across rows
        let is_equal_keys_vec = local_cols.page_row[1..self.key_len + 1]
            .to_vec()
            .into_iter()
            .chain(next_cols.page_row[1..self.key_len + 1].to_vec())
            .chain(next_cols.is_equal_key_aux.flatten())
            .collect::<Vec<AB::Var>>();
        let is_equal_keys = IsEqualVecCols::from_slice(&is_equal_keys_vec, self.key_len);
        let is_equal_keys_chip = IsEqualVecChip::new(self.key_len);

        SubAir::eval(
            &is_equal_keys_chip,
            &mut builder.when_transition(),
            is_equal_keys.io,
            is_equal_keys.aux,
        );

        // Making sure same_val is correct across rows
        let is_equal_vals_vec = local_cols.page_row[self.key_len + 1..]
            .to_vec()
            .into_iter()
            .chain(next_cols.page_row[self.key_len + 1..].to_vec())
            .chain(next_cols.is_equal_val_aux.flatten())
            .collect::<Vec<AB::Var>>();
        let is_equal_vals = IsEqualVecCols::from_slice(&is_equal_vals_vec, self.val_len);
        let is_equal_vals_chip = IsEqualVecChip::new(self.val_len);

        SubAir::eval(
            &is_equal_vals_chip,
            &mut builder.when_transition(),
            is_equal_vals.io,
            is_equal_vals.aux,
        );

        // TODO: make sure all rows are sorted

        // Some helpers
        let and = |a: AB::Expr, b: AB::Expr| a * b;
        let or = |a: AB::Expr, b: AB::Expr| a.clone() + b.clone() - a * b;
        let implies = |a: AB::Expr, b: AB::Expr| or(AB::Expr::one() - a, b);

        // Making sure every key block starts with a write
        // not same_key => write
        builder.assert_one(or(
            local_cols.is_extra.into(),
            or(local_cols.same_key.into(), local_cols.op_type.into()),
        ));

        // Making sure every key block ends with a is_final
        builder.when_transition().assert_one(or(
            local_cols.is_extra.into(),
            or(next_cols.same_key.into(), local_cols.is_final.into()),
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
        // is_initial => not same_key
        builder.assert_one(implies(
            local_cols.is_initial.into(),
            AB::Expr::one() - local_cols.same_key,
        ));

        // Making sure that every read uses the same value as the last operation
        // read => same_val
        builder.assert_one(or(
            local_cols.is_extra.into(),
            or(local_cols.op_type.into(), local_cols.same_val.into()),
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
        // - for every row: (is_initial => write) because is_initial => not same_key => write
        // - for every row: (is_initial => not is_final) because is_final => read => same_val and is_initial => not same_key
        // - there is at most 1 is_initial per key block because every row is sent at most once from the inital page chip
        // - there is exactly 1 is_final per key block because every row is received at most once from the final page chip
        //   and we make sure that is_final is the last row in the block
    }
}
