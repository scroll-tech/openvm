use std::borrow::Borrow;

use afs_stark_backend::air_builders::PartitionedAirBuilder;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use super::{columns::MiddleChipCols, MiddleChip};
use crate::sub_chip::AirConfig;

impl AirConfig for MiddleChip {
    type Cols<T> = MiddleChipCols<T>;
}

impl<F: Field> BaseAir<F> for MiddleChip {
    fn width(&self) -> usize {
        self.air_width()
    }
}

impl MiddleChip {
    fn implies<AB: AirBuilder>(&self, builder: &mut AB, a: AB::Expr, b: AB::Expr) {
        builder.assert_one((AB::Expr::one() - a.clone()) + b.clone() - (AB::Expr::one() - a) * b);
    }
}

/// Imposes the following constraints:
/// - Rows are sorted by key then by timestamp (clk)
/// - Every key block starts with a write
/// - Every key block ends with an is_final
/// - For every key, every read uses the same value as the last
///   operation with that key
impl<AB: PartitionedAirBuilder> Air<AB> for MiddleChip
where
    AB::M: Clone,
{
    fn eval(&self, builder: &mut AB) {
        let main = &builder.partitioned_main()[2].clone();

        // let main = builder.main();

        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let local: &[AB::Var] = (*local).borrow();
        let next: &[AB::Var] = (*next).borrow();

        println!(
            "Here in the airbuilder eval file, local.len(): {:?}",
            local.len()
        );

        let local_cols = MiddleChipCols::from_slice(local);
        let next_cols = MiddleChipCols::from_slice(next);

        // TODO: make sure all the relations between is_initial, is_final, op_type are followed

        // Making sure op_type is always bool (read or write)
        builder.assert_bool(local_cols.op_type);

        // Making sure first row starts with same_key, same_value being false
        builder.when_first_row().assert_zero(local_cols.same_key);
        builder.when_first_row().assert_zero(local_cols.same_val);

        // TODO: make sure same_key and same_val are correct for the rest of the rows

        // Making sure every key block starts with a write
        builder.assert_one(
            local_cols.same_key + local_cols.op_type - local_cols.same_key * local_cols.op_type,
        );

        // Making sure every key block ends with a is_final
        builder.assert_one(
            (AB::Expr::one() - next_cols.same_key) + local_cols.is_final
                - (AB::Expr::one() - next_cols.same_key) * local_cols.is_final,
        );
        builder.when_last_row().assert_one(local_cols.is_final);

        // Making sure that is_initial rows only appear at the start of blocks
        // is_initial => not same_key
        builder.assert_one(
            (AB::Expr::one() - local_cols.is_initial) + (AB::Expr::one() - local_cols.same_key)
                - (AB::Expr::one() - local_cols.is_initial)
                    * (AB::Expr::one() - local_cols.same_key),
        );

        // Making sure that every read uses the same value as the last operation
        // read => same_val
        builder.assert_one(
            next_cols.op_type + next_cols.same_val - next_cols.op_type * next_cols.same_val,
        );

        // Making sure that is_final implies a read
        builder.assert_one(
            (AB::Expr::one() - local_cols.is_final) + (AB::Expr::one() - local_cols.op_type)
                - (AB::Expr::one() - local_cols.is_final) * (AB::Expr::one() - local_cols.op_type),
        );
    }
}
