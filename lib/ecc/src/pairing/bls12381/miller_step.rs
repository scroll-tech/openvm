use axvm::intrinsics::{Fp2, Fp2Bls12381, BLS12_381_LIMBS};

use super::Bls12381;
use crate::pairing::{MillerStepOpcode, MultiMillerLoop};

impl MillerStepOpcode<FpBls12381, Fp2Bls12381> for Bls12381 {}
