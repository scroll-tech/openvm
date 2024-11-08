use super::{Bn254, Fp, Fp12, Fp2};
use crate::pairing::MultiMillerLoop;

impl MultiMillerLoop<Fp, Fp2, Fp12> for Bn254 {}
