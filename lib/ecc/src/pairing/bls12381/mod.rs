use lazy_static::lazy_static;

mod line;
pub use line::*;

mod miller_step;
pub use miller_step::*;

use crate::field::Field;

pub struct Bls12381;

lazy_static! {
    pub static ref BLS12381_XI: Fp2Bls12381 = Fp2Bls12381::from_u32((1, 1));
}
