#![cfg_attr(target_os = "zkvm", no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

use axvm::io::read_vec;
use axvm_algebra::IntMod;
use axvm_ecc::{
    field::FieldExtension,
    pairing::{
        bls12381::{Bls12381Fp, Bls12381Fp12, Bls12381Fp2, Bls12381Intrinsic},
        EvaluatedLine, LineMulMType,
    },
};

axvm::entry!(main);

fn test_mul_023_by_023(io: &[u8]) {
    let l0 = &io[..(3 * 16) * 4];
    let l1 = &io[(3 * 16) * 4..(3 * 16) * 8];
    let expected = &io[(3 * 16) * 8..(3 * 16) * 18];

    let l0_cast = unsafe { &*(l0.as_ptr() as *const EvaluatedLine<Bls12381Fp, Bls12381Fp2>) };
    let l1_cast = unsafe { &*(l1.as_ptr() as *const EvaluatedLine<Bls12381Fp, Bls12381Fp2>) };

    let r = Bls12381Intrinsic::mul_023_by_023(l0_cast.clone(), l1_cast.clone());
    let mut r_bytes = [0u8; (3 * 16) * 10];
    let mut i = 0;
    for x in r {
        r_bytes[i..i + (3 * 16)].copy_from_slice(&x.c0.as_le_bytes());
        r_bytes[i + (3 * 16)..i + 96].copy_from_slice(&x.c1.as_le_bytes());
        i += 96;
    }
    assert_eq!(r_bytes, expected);
}

fn test_mul_by_02345(io: &[u8]) {
    let f = &io[(3 * 16) * 18..(3 * 16) * 30];
    let x = &io[(3 * 16) * 30..(3 * 16) * 40];
    let expected = &io[(3 * 16) * 40..(3 * 16) * 52];

    let f_cast = unsafe { &*(f.as_ptr() as *const Bls12381Fp12) };
    let x_cast = unsafe { &*(x.as_ptr() as *const [Bls12381Fp2; 5]) };

    let r = Bls12381Intrinsic::mul_by_02345(f_cast.clone(), x_cast.clone());
    let mut r_bytes = [0u8; (3 * 16) * 12];
    let mut i = 0;
    for x in r.to_coeffs() {
        r_bytes[i..i + (3 * 16)].copy_from_slice(&x.c0.as_le_bytes());
        r_bytes[i + (3 * 16)..i + 96].copy_from_slice(&x.c1.as_le_bytes());
        i += 96;
    }
    assert_eq!(r_bytes, expected);
}

pub fn main() {
    let io = read_vec();
    assert_eq!(io.len(), (3 * 16) * 52);

    test_mul_023_by_023(&io);
    test_mul_by_02345(&io);
}
