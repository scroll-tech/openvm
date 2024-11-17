use axvm_algebra::IntMod;
use halo2curves_axiom::{
    bls12_381::{Fq, Fq12, Fq2},
    ff::Field,
};
use rand::{rngs::StdRng, SeedableRng};

use super::Bls12381Fp;
use crate::{
    field::{FieldExtension, SexticExtField},
    pairing::bls12381::{Bls12381Fp12, Bls12381Fp2},
};

fn convert_fq_to_bls12381fp(x: Fq) -> Bls12381Fp {
    let bytes = x.0.map(|y| y.to_le_bytes()).concat();
    Bls12381Fp::from_le_bytes(&bytes)
}

fn convert_fq2_to_bls12381fp2(x: Fq2) -> Bls12381Fp2 {
    Bls12381Fp2::new(
        convert_fq_to_bls12381fp(x.c0),
        convert_fq_to_bls12381fp(x.c1),
    )
}

fn convert_fq12_to_bls12381fp12(x: Fq12) -> Bls12381Fp12 {
    Bls12381Fp12 {
        c: [
            convert_fq2_to_bls12381fp2(x.c0.c0),
            convert_fq2_to_bls12381fp2(x.c0.c1),
            convert_fq2_to_bls12381fp2(x.c0.c2),
            convert_fq2_to_bls12381fp2(x.c1.c0),
            convert_fq2_to_bls12381fp2(x.c1.c1),
            convert_fq2_to_bls12381fp2(x.c1.c2),
        ],
    }
}

#[test]
fn test_bls12381_frobenius() {
    let mut rng = StdRng::seed_from_u64(15);
    let pow = 1;
    let a = Fq12::random(&mut rng);
    let b = a.frobenius_map();

    let f = convert_fq12_to_bls12381fp12(a);
    let ff = SexticExtField::frobenius_map(&f, pow);

    let cmp = convert_fq12_to_bls12381fp12(b);

    assert_eq!(ff, cmp);
}
