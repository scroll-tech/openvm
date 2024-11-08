mod line;
use core::ops::Neg;

use axvm::intrinsics::{Fp2, Fp2Bls12381, BLS12_381_LIMBS};
pub use line::*;

mod miller_step;
pub use miller_step::*;

use crate::field::Field;

pub struct Bls12381;

impl Field for Fp2Bls12381 {
    type SelfRef<'a> = Fp2Bls12381;

    const ZERO: Self = Fp2Bls12381::ZERO;
    const ONE: Self = Fp2Bls12381::ONE;

    fn invert(&self) -> Option<Self> {
        unimplemented!()
    }
}
