use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::Field;

use crate::MAX_COMMITMENT_LEN;

use super::Page2Requester;

impl<F: Field> BaseAir<F> for Page2Requester {
    fn width(&self) -> usize {
        self.key_len + self.val_len + MAX_COMMITMENT_LEN
    }
}

impl<AB> Air<AB> for Page2Requester
where
    AB: AirBuilder,
{
    fn eval(&self, _builder: &mut AB) {}
}
