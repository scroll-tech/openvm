use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::Field;

use super::PageRequester;

impl<F: Field> BaseAir<F> for PageRequester {
    fn width(&self) -> usize {
        self.idx_len + self.data_len
    }
}

impl<AB> Air<AB> for PageRequester
where
    AB: AirBuilder,
{
    fn eval(&self, _builder: &mut AB) {}
}
