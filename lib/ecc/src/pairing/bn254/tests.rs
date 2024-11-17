use axvm_algebra::IntMod;
use halo2curves_axiom::{
    bn256::{Fq, Fq12, Fq2},
    ff::Field,
};
use rand::{rngs::StdRng, SeedableRng};

use super::Bn254Fp;
use crate::{
    field::{FieldExtension, SexticExtField},
    pairing::bn254::{Bn254Fp12, Bn254Fp2},
};

fn convert_fq_to_bn254fp(x: Fq) -> Bn254Fp {
    let bytes = x.0.map(|y| y.to_le_bytes()).concat();
    Bn254Fp::from_le_bytes(&bytes)
}

fn convert_fq2_to_bn254fp2(x: Fq2) -> Bn254Fp2 {
    Bn254Fp2::new(convert_fq_to_bn254fp(x.c0), convert_fq_to_bn254fp(x.c1))
}

fn convert_fq12_to_bn254fp12(x: Fq12) -> Bn254Fp12 {
    Bn254Fp12 {
        c: [
            convert_fq2_to_bn254fp2(x.c0.c0),
            convert_fq2_to_bn254fp2(x.c0.c1),
            convert_fq2_to_bn254fp2(x.c0.c2),
            convert_fq2_to_bn254fp2(x.c1.c0),
            convert_fq2_to_bn254fp2(x.c1.c1),
            convert_fq2_to_bn254fp2(x.c1.c2),
        ],
    }
}

#[test]
fn test_bn254_frobenius() {
    let mut rng = StdRng::seed_from_u64(15);
    for pow in 0..4 {
        let a = Fq12::random(&mut rng);
        let b = a.frobenius_map(pow);

        let f = convert_fq12_to_bn254fp12(a);
        let ff = SexticExtField::frobenius_map(&f, pow);

        let cmp = convert_fq12_to_bn254fp12(b);

        assert_eq!(ff, cmp);
    }
}
