#![cfg_attr(target_os = "zkvm", no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

use core::hint::black_box;

use axvm::io::read_vec;
use axvm_algebra::IntMod;
use axvm_ecc::{
    field::Field,
    pairing::{
        bn254::{Bn254, Bn254Fp, Bn254Fp2},
        EvaluatedLine, LineMulDType,
    },
};

axvm::entry!(main);

pub fn main() {
    let l0 = black_box(EvaluatedLine::<Bn254Fp, Bn254Fp2> {
        b: Bn254Fp2::zero(),
        c: Bn254Fp2::zero(),
    });
    let l1 = black_box(EvaluatedLine::<Bn254Fp, Bn254Fp2> {
        b: Bn254Fp2::zero(),
        c: Bn254Fp2::zero(),
    });
    let r = Bn254::mul_013_by_013(l0, l1);
    assert_eq!(&r[0].c0, &Bn254Fp::ZERO);
    assert_eq!(&r[0].c1, &Bn254Fp::ZERO);
}
