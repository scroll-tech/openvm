use super::{Bls12381, Fp, Fp12, Fp2};
use crate::pairing::LineMulMType;

impl LineMulMType<Fp, Fp2, Fp12> for Bls12381 {}
