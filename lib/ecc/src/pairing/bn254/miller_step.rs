use alloc::vec::Vec;

use super::{Bn254, Fp, Fp12, Fp2};
use crate::{
    pairing::{EvaluatedLine, MillerStep, MultiMillerLoop},
    point::EcPoint,
};

impl MillerStep for Bn254 {
    type Fp = Fp;
    type Fp2 = Fp2;
}

impl MultiMillerLoop for Bn254 {
    type Fp12 = Fp12;

    fn xi() -> Fp2 {
        #[cfg(not(target_os = "zkvm"))]
        {
            todo!()
        }
        #[cfg(target_os = "zkvm")]
        {
            todo!()
        }
    }

    fn seed() -> u64 {
        #[cfg(not(target_os = "zkvm"))]
        {
            todo!()
        }
        #[cfg(target_os = "zkvm")]
        {
            todo!()
        }
    }

    fn pseudo_binary_encoding() -> Vec<i8> {
        #[cfg(not(target_os = "zkvm"))]
        {
            todo!()
        }
        #[cfg(target_os = "zkvm")]
        {
            todo!()
        }
    }

    fn evaluate_lines_vec(&self, f: Fp12, lines: Vec<EvaluatedLine<Fp, Fp2>>) -> Fp12 {
        #[cfg(not(target_os = "zkvm"))]
        {
            todo!()
        }
        #[cfg(target_os = "zkvm")]
        {
            todo!()
        }
    }

    fn pre_loop(
        &self,
        f: &Fp12,
        Q_acc: Vec<EcPoint<Fp2>>,
        Q: &[EcPoint<Fp2>],
        c: Option<Fp12>,
        x_over_ys: Vec<Fp>,
        y_invs: Vec<Fp>,
    ) -> (Fp12, Vec<EcPoint<Fp2>>) {
        #[cfg(not(target_os = "zkvm"))]
        {
            todo!()
        }
        #[cfg(target_os = "zkvm")]
        {
            todo!()
        }
    }

    fn post_loop(
        &self,
        f: &Fp12,
        Q_acc: Vec<EcPoint<Fp2>>,
        Q: &[EcPoint<Fp2>],
        c: Option<Fp12>,
        x_over_ys: Vec<Fp>,
        y_invs: Vec<Fp>,
    ) -> (Fp12, Vec<EcPoint<Fp2>>) {
        #[cfg(not(target_os = "zkvm"))]
        {
            todo!()
        }
        #[cfg(target_os = "zkvm")]
        {
            todo!()
        }
    }
}
