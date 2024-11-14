#![cfg_attr(target_os = "zkvm", no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

use core::hint::black_box;

use axvm::{io::read_vec, process::panic};
use axvm_algebra::IntMod;
use axvm_ecc::{
    field::Field,
    pairing::{
        bn254::{Bn254Fp, Bn254Fp2, Bn254Intrinsic},
        EvaluatedLine, LineMulDType,
    },
};

axvm::entry!(main);

// pub fn test_mul_013_by_013() {
//     let l0 = black_box(EvaluatedLine::<Bn254Fp, Bn254Fp2> {
//         b: Bn254Fp2::zero(),
//         c: Bn254Fp2::zero(),
//     });
//     let l1 = black_box(EvaluatedLine::<Bn254Fp, Bn254Fp2> {
//         b: Bn254Fp2::zero(),
//         c: Bn254Fp2::zero(),
//     });
//     let r = Bn254::mul_013_by_013(l0, l1);
// }

pub fn main() {
    let io = read_vec();
    assert_eq!(io.len(), 32 * (2 * 4 + 10));
    let l0 = &io[..32 * 4];
    let l1 = &io[32 * 4..32 * 8];
    let expected = &io[32 * 8..];

    let l0_cast = unsafe { &*(l0.as_ptr() as *const EvaluatedLine<Bn254Fp, Bn254Fp2>) };
    let l1_cast = unsafe { &*(l1.as_ptr() as *const EvaluatedLine<Bn254Fp, Bn254Fp2>) };

    let r = Bn254Intrinsic::mul_013_by_013(l0_cast.clone(), l1_cast.clone());
    assert_eq!(r[0].c0.as_le_bytes()[0], expected[0]);
    let mut r_bytes = [0u8; 32 * 10];
    let mut i = 0;
    for x in r {
        r_bytes[i..i + 32].copy_from_slice(&x.c0.as_le_bytes());
        r_bytes[i + 32..i + 64].copy_from_slice(&x.c1.as_le_bytes());
        i += 64;
    }
    assert_eq!(r_bytes, expected);
}
