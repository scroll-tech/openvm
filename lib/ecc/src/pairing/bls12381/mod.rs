mod line;
use axvm::intrinsics::{Fp2Bls12381, BLS12_381_LIMBS};
pub use line::*;

mod miller_step;
pub use miller_step::*;

use crate::field::Field;

#[derive(Clone, Copy, Debug)]
pub struct FpBls12381([u8; BLS12_381_LIMBS]);
