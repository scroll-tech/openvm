use super::{Bn254, Fp, Fp12, Fp2};
use crate::pairing::LineMulDType;

impl LineMulDType<Fp, Fp2, Fp12> for Bn254 {}
