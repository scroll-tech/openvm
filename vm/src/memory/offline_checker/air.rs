use afs_primitives::{
    offline_checker::columns::OfflineCheckerCols,
    sub_chip::{AirConfig, SubAir},
};
use afs_stark_backend::{air_builders::PartitionedAirBuilder, interaction::InteractionBuilder};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, Field, PrimeField32};
use p3_matrix::Matrix;

use super::{columns::MemoryOfflineCheckerCols, MemoryChip, MemoryOfflineChecker};

impl<const WORD_SIZE: usize, F: PrimeField32> AirConfig for MemoryChip<WORD_SIZE, F> {
    type Cols<T> = OfflineCheckerCols<T>;
}

impl<F: Field> BaseAir<F> for MemoryOfflineChecker {
    fn width(&self) -> usize {
        self.air_width()
    }
}

impl<AB: PartitionedAirBuilder + InteractionBuilder> Air<AB> for MemoryOfflineChecker {
    /// This constrains extra rows to be at the bottom and the following on non-extra rows:
    /// same_addr_space, same_pointer, same_data, lt_bit is correct (see definition in columns.rs)
    /// A read must be preceded by a write with the same address space, pointer, and data
    fn eval(&self, builder: &mut AB) {
        let main = &builder.main();

        let local_cols = MemoryOfflineCheckerCols::from_slice(&main.row_slice(0), self);
        let next_cols = MemoryOfflineCheckerCols::from_slice(&main.row_slice(1), self);

        builder.assert_bool(local_cols.offline_checker_cols.op_type);

        // loop over data_len
        // is_valid * (1 - op_type) * same_idx * (x[i] - y[i])
        for i in 0..self.offline_checker.data_len {
            // NOTE: constraint degree is 4
            builder.when_transition().assert_zero(
                next_cols.offline_checker_cols.is_valid.into()
                    * (AB::Expr::one() - next_cols.offline_checker_cols.op_type.into())
                    * next_cols.offline_checker_cols.same_idx.into()
                    * (local_cols.offline_checker_cols.data[i]
                        - next_cols.offline_checker_cols.data[i]),
            );
        }

        // constrain that is_final_access is 1 when the index changes
        builder.assert_eq(
            AB::Expr::one() - next_cols.offline_checker_cols.same_idx,
            local_cols.is_final_access,
        );

        self.eval_interactions(builder, &local_cols);
        SubAir::eval(
            &self.offline_checker,
            builder,
            (
                local_cols.offline_checker_cols,
                next_cols.offline_checker_cols,
            ),
            (),
        );
    }
}
