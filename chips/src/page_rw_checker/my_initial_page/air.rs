use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::Field;

use super::MyInitialPageAir;
use crate::common::page_cols::PageCols;
use crate::sub_chip::{AirConfig, SubAir};

impl<F: Field> BaseAir<F> for MyInitialPageAir {
    fn width(&self) -> usize {
        self.air_width()
    }
}

impl AirConfig for MyInitialPageAir {
    type Cols<T> = PageCols<T>;
}

impl<AB: AirBuilder> Air<AB> for MyInitialPageAir {
    fn eval(&self, _builder: &mut AB) {}
}

impl<AB: AirBuilder> SubAir<AB> for MyInitialPageAir
where
    AB::M: Clone,
{
    type IoView = PageCols<AB::Var>;
    type AuxView = ();

    fn eval(&self, _builder: &mut AB, _io: Self::IoView, _aux: Self::AuxView) {}
}
