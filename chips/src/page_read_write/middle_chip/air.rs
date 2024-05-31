use std::borrow::Borrow;

use afs_stark_backend::air_builders::PartitionedAirBuilder;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use super::{columns::MiddleChipCols, MiddleChip};

impl<F: Field> BaseAir<F> for MiddleChip {
    fn width(&self) -> usize {
        self.air_width()
    }
}

impl<AB: AirBuilder> Air<AB> for MiddleChip
where
    AB::Var: Clone,
{
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();

        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let local: &[AB::Var] = (*local).borrow();
        let next: &[AB::Var] = (*next).borrow();

        let local_cols = MiddleChipCols::from_slice(local);
        let next_cols = MiddleChipCols::from_slice(next);
    }
}
