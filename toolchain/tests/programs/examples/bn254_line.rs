#![cfg_attr(target_os = "zkvm", no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

use axvm::io::read_vec;
use axvm_algebra::IntMod;
use axvm_ecc::{
    field::FieldExtension,
    pairing::{
        bn254::{Bn254Fp, Bn254Fp12, Bn254Fp2, Bn254Intrinsic},
        EvaluatedLine, LineMulDType,
    },
};

axvm::entry!(main);

fn test_mul_013_by_013(io: &[u8]) {
    let l0 = &io[..32 * 4];
    let l1 = &io[32 * 4..32 * 8];
    let expected = &io[32 * 8..32 * 18];

    let l0_cast = unsafe { &*(l0.as_ptr() as *const EvaluatedLine<Bn254Fp, Bn254Fp2>) };
    let l1_cast = unsafe { &*(l1.as_ptr() as *const EvaluatedLine<Bn254Fp, Bn254Fp2>) };

    let r = Bn254Intrinsic::mul_013_by_013(l0_cast.clone(), l1_cast.clone());
    let mut r_bytes = [0u8; 32 * 10];
    let mut i = 0;
    for x in r {
        r_bytes[i..i + 32].copy_from_slice(&x.c0.as_le_bytes());
        r_bytes[i + 32..i + 64].copy_from_slice(&x.c1.as_le_bytes());
        i += 64;
    }
    assert_eq!(r_bytes, expected);
}

fn test_mul_by_01234(io: &[u8]) {
    let f = &io[32 * 18..32 * 30];
    let x = &io[32 * 30..32 * 40];
    let expected = &io[32 * 40..32 * 52];

    let f_cast = unsafe { &*(f.as_ptr() as *const Bn254Fp12) };
    let x_cast = unsafe { &*(x.as_ptr() as *const [Bn254Fp2; 5]) };

    let r = Bn254Intrinsic::mul_by_01234(f_cast.clone(), x_cast.clone());
    let mut r_bytes = [0u8; 32 * 12];
    let mut i = 0;
    for x in r.to_coeffs() {
        r_bytes[i..i + 32].copy_from_slice(&x.c0.as_le_bytes());
        r_bytes[i + 32..i + 64].copy_from_slice(&x.c1.as_le_bytes());
        i += 64;
    }
    assert_eq!(r_bytes, expected);
    // assert_eq!(r_bytes[0..64], expected[0..64]);
    // assert_eq!(r_bytes[64..128], expected[64..128]);
    // assert_eq!(r_bytes[128..192], expected[128..192]);
    // assert_eq!(r_bytes[192..256], expected[192..256]);
    // assert_eq!(r_bytes[256..320], expected[256..320]);
    // assert_eq!(r_bytes[320..384], expected[320..384]);
}

pub fn main() {
    let io = read_vec();
    assert_eq!(io.len(), 32 * 52);

    test_mul_013_by_013(&io);
    test_mul_by_01234(&io);
}
