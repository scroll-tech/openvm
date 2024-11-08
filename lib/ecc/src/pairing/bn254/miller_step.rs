use axvm::intrinsics::{Fp2, Fp2Bn254, BN256_LIMBS};

use super::Bn254;
use crate::{
    pairing::{
        bn254::{BN254_THREE, BN254_TWO},
        MillerStepOpcode, UnevaluatedLine,
    },
    point::EcPoint,
};

impl MillerStepOpcode<FpBn254, Fp2Bn254> for Bn254 {}
