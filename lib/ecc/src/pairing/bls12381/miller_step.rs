use axvm::intrinsics::{Fp2, Fp2Bls12381};

pub trait MillerStepBls12381 {
    fn miller_double_step(s: [Fp2Bls12381; 2]) -> ([Fp2Bls12381; 2], [Fp2Bls12381; 2]);

    fn miller_double_and_add_step(
        s: [Fp2Bls12381; 2],
        q: [Fp2Bls12381; 2],
    ) -> ([Fp2Bls12381; 2], [Fp2Bls12381; 2], [Fp2Bls12381; 2]);
}

impl MillerStepBls12381 for Fp2Bls12381 {
    fn miller_double_step(s: [Fp2Bls12381; 2]) -> ([Fp2Bls12381; 2], [Fp2Bls12381; 2]) {
        #[cfg(not(target_os = "zkvm"))]
        {
            todo!()
        }
        #[cfg(target_os = "zkvm")]
        {
            todo!()
        }
    }

    fn miller_double_and_add_step(
        s: [Fp2Bls12381; 2],
        q: [Fp2Bls12381; 2],
    ) -> ([Fp2Bls12381; 2], [Fp2Bls12381; 2], [Fp2Bls12381; 2]) {
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
