use halo2curves_axiom::bn256::{Fq, Fq12, Fq2, Fq6, G1Affine, G2Affine};

use crate::{common::point::EcPoint, miller::miller_double_step};

#[test]
fn test_miller_double_step() {
    let s = EcPoint {
        x: Fq2::one(),
        y: Fq2::one(),
    };
    let (q_acc, line) = miller_double_step::<Fq, Fq2>(s);
}

#[test]
fn test_miller_double_and_add() {}
