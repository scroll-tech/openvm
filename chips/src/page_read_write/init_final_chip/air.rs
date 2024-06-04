use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::Field;

use super::{columns::InitFinalCols, InitFinalChip};
use crate::sub_chip::AirConfig;

impl<F: Field> BaseAir<F> for InitFinalChip {
    fn width(&self) -> usize {
        self.air_width()
    }
}

impl AirConfig for InitFinalChip {
    type Cols<T> = InitFinalCols<T>;
}

impl<AB: AirBuilder> Air<AB> for InitFinalChip {
    fn eval(&self, _builder: &mut AB) {}
}
