use axvm::moduli_setup;
use axvm_algebra::{DivUnsafe, IntMod};

use crate::field::Field;

mod fp2;
pub use fp2::*;

mod fp12;
pub use fp12::*;

use super::LineMulDType;

pub struct Bn254Intrinsic;

moduli_setup! {
    Bn254Fp = "21888242871839275222246405745257275088696311157297823662689037894645226208583";
}

impl Field for Bn254Fp {
    type SelfRef<'a> = &'a Self;

    fn zero() -> Self {
        <Self as IntMod>::ZERO
    }

    fn one() -> Self {
        <Self as IntMod>::ONE
    }

    fn invert(&self) -> Option<Self> {
        Some(<Bn254Fp as IntMod>::ONE.div_unsafe(self))
    }
}

impl LineMulDType<Bn254Fp, Bn254Fp2, Bn254Fp12> for Bn254Intrinsic {}
