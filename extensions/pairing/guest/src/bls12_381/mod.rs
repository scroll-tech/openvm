use core::ops::Neg;

use openvm_algebra_guest::{Field, IntMod};
use openvm_algebra_moduli_setup::moduli_declare;
use openvm_ecc_guest::{weierstrass::IntrinsicCurve, CyclicGroup, Group};

mod fp12;
mod fp2;
mod pairing;

pub use fp12::*;
pub use fp2::*;
use hex_literal::hex;
#[cfg(not(target_os = "zkvm"))]
use lazy_static::lazy_static;
#[cfg(not(target_os = "zkvm"))]
use num_bigint::BigUint;
use openvm_ecc_sw_setup::sw_declare;

use crate::pairing::PairingIntrinsics;

#[cfg(all(test, feature = "halo2curves", not(target_os = "zkvm")))]
mod tests;

#[cfg(not(target_os = "zkvm"))]
lazy_static! {
    pub static ref BLS12_381_MODULUS: BigUint = BigUint::from_bytes_be(&hex!(
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab"
    ));
    pub static ref BLS12_381_ORDER: BigUint = BigUint::from_bytes_be(&hex!(
        "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001"
    ));
}

pub const BLS12_381_XI_ISIZE: [isize; 2] = [1, 1];
pub const BLS12_381_NUM_LIMBS: usize = 48;
pub const BLS12_381_LIMB_BITS: usize = 8;
pub const BLS12_381_BLOCK_SIZE: usize = 16;

pub const BLS12_381_SEED_ABS: u64 = 0xd201000000010000;
pub const BLS12_381_PSEUDO_BINARY_ENCODING: [i8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1,
];

moduli_declare! {
    Bls12_381Fp { modulus = "0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab" },
    Bls12_381Scalar { modulus = "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001" },
}

const CURVE_B: Bls12_381Fp = Bls12_381Fp::from_const_u8(4);

sw_declare! {
    Bls12_381G1Affine { mod_type = Bls12_381Fp, b = CURVE_B },
}

pub type Fp = Bls12_381Fp;
pub type Scalar = Bls12_381Scalar;
/// Affine point representation of `Fp` points of BLS12-381.
/// **Note**: an instance of this type may be constructed that lies
/// on the curve but not necessarily in the prime order subgroup
/// because the group has cofactors.
pub type G1Affine = Bls12_381G1Affine;
pub use g2::G2Affine;

impl Field for Fp {
    type SelfRef<'a> = &'a Self;
    const ZERO: Self = <Self as IntMod>::ZERO;
    const ONE: Self = <Self as IntMod>::ONE;

    fn double_assign(&mut self) {
        IntMod::double_assign(self);
    }

    fn square_assign(&mut self) {
        IntMod::square_assign(self);
    }
}

impl Field for Scalar {
    type SelfRef<'a> = &'a Self;
    const ZERO: Self = <Self as IntMod>::ZERO;
    const ONE: Self = <Self as IntMod>::ONE;

    fn double_assign(&mut self) {
        IntMod::double_assign(self);
    }

    fn square_assign(&mut self) {
        IntMod::square_assign(self);
    }
}

// https://hackmd.io/@benjaminion/bls12-381#Cofactor
// BLS12-381: The from_xy function will allow constructing elements that lie on the curve
// but aren't actually in the cyclic subgroup of prime order that is usually called G1.
impl CyclicGroup for G1Affine {
    // https://github.com/zcash/librustzcash/blob/6e0364cd42a2b3d2b958a54771ef51a8db79dd29/pairing/src/bls12_381/README.md#generators
    const GENERATOR: Self = G1Affine {
        x: Bls12_381Fp::from_const_bytes(hex!(
            "BBC622DB0AF03AFBEF1A7AF93FE8556C58AC1B173F3A4EA105B974974F8C68C30FACA94F8C63952694D79731A7D3F117"
        )),
        y: Bls12_381Fp::from_const_bytes(hex!(
            "E1E7C5462923AA0CE48A88A244C73CD0EDB3042CCB18DB00F60AD0D595E0F5FCE48A1D74ED309EA0F1A0AAE381F4B308"
        )),
    };
    const NEG_GENERATOR: Self = G1Affine {
        x: Bls12_381Fp::from_const_bytes(hex!(
            "BBC622DB0AF03AFBEF1A7AF93FE8556C58AC1B173F3A4EA105B974974F8C68C30FACA94F8C63952694D79731A7D3F117"
        )),
        y: Bls12_381Fp::from_const_bytes(hex!(
            "CAC239B9D6DC54AD1B75CB0EBA386F4E3642ACCAD5B95566C907B51DEF6A8167F2212ECFC8767DAAA845D555681D4D11"
        )),
    };
}

pub struct Bls12_381;

impl IntrinsicCurve for Bls12_381 {
    type Scalar = Scalar;
    type Point = G1Affine;

    fn msm(coeffs: &[Self::Scalar], bases: &[Self::Point]) -> Self::Point {
        // TODO: msm optimization
        openvm_ecc_guest::msm(coeffs, bases)
    }
}

// Define a G2Affine struct that implements curve operations using `Fp2` intrinsics
// but not special E(Fp2) intrinsics.
mod g2 {
    use openvm_algebra_guest::Field;
    use openvm_ecc_guest::{
        impl_sw_affine, impl_sw_group_ops, weierstrass::WeierstrassPoint, AffinePoint, Group,
    };

    use super::{Fp, Fp2};

    const THREE: Fp2 = Fp2::new(Fp::from_const_u8(3), Fp::ZERO);
    const B: Fp2 = Fp2::new(Fp::from_const_u8(4), Fp::from_const_u8(4));
    impl_sw_affine!(G2Affine, Fp2, THREE, B);
    impl_sw_group_ops!(G2Affine, Fp2);
}

impl PairingIntrinsics for Bls12_381 {
    type Fp = Fp;
    type Fp2 = Fp2;
    type Fp12 = Fp12;

    const PAIRING_IDX: usize = 1;
    const XI: Fp2 = Fp2::new(Fp::from_const_u8(1), Fp::from_const_u8(1));
    const FROBENIUS_COEFFS: [[Self::Fp2; 5]; 12] = [
        [
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
                    ")),
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
                    ")),
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
                    ")),
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
                    ")),
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
                    ")),
            },
        ],
        [
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "b85f2392ed75078d3d81e7633da57ef6c4b9ba84d743247b4f5fbd3cfd03d60f1f0d2c20b4be31c26706bb02bfd30419"
                )),
                c1: Bls12_381Fp(hex!(
                    "f34adc6d128af72cc27e6c4dc15a2d285f3cf671c98e0cec6fb3c7b68747a154b89f1f2302e9e98832e0c4362b3efc00"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "acaa00000000fd8bfdff494feb2794409b5fb80f65297d89d49a75897d850daa85ded463864002ec99e67f39ea11011a"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "09cce3edfb8410c8f405ec722f9967eec5419200176ef7775e43d3c2ab5d3948fe7fd16b6de331680b40ff37040eaf06"
                )),
                c1: Bls12_381Fp(hex!(
                    "09cce3edfb8410c8f405ec722f9967eec5419200176ef7775e43d3c2ab5d3948fe7fd16b6de331680b40ff37040eaf06"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "adaa00000000fd8bfdff494feb2794409b5fb80f65297d89d49a75897d850daa85ded463864002ec99e67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "16810780e9fa189b32877f256e3e3ac666059c8e4ddfea8bee8f0b0c241698f345e0b1486bfa47dfd85f3a01d9cfb205"
                )),
                c1: Bls12_381Fp(hex!(
                    "9529f87f1605e61ecd78d48b90c17158bdf0146853f345dbd08279e76035df7091cc99fa4aadd36bc186453811424e14"
                ))
            },
        ],
        [
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "fffffeffffff012e02000a6213d817de8896f8e63ba9b3ddea770f6a07c669ba51ce76df2f67195f0000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "fefffeffffff012e02000a6213d817de8896f8e63ba9b3ddea770f6a07c669ba51ce76df2f67195f0000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "aaaafffffffffeb9ffff53b1feffab1e24f6b0f6a0d23067bf1285f3844b7764d7ac4b43b6a71b4b9ae67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "acaa00000000fd8bfdff494feb2794409b5fb80f65297d89d49a75897d850daa85ded463864002ec99e67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "adaa00000000fd8bfdff494feb2794409b5fb80f65297d89d49a75897d850daa85ded463864002ec99e67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
        ],
        [
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "a2de1b12047beef10afa673ecf6644305eb41ef6896439ef60cfb130d9ed3d1cd92c7ad748c4e9e28ea68001e6035213"
                )),
                c1: Bls12_381Fp(hex!(
                    "09cce3edfb8410c8f405ec722f9967eec5419200176ef7775e43d3c2ab5d3948fe7fd16b6de331680b40ff37040eaf06"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "a2de1b12047beef10afa673ecf6644305eb41ef6896439ef60cfb130d9ed3d1cd92c7ad748c4e9e28ea68001e6035213"
                )),
                c1: Bls12_381Fp(hex!(
                    "a2de1b12047beef10afa673ecf6644305eb41ef6896439ef60cfb130d9ed3d1cd92c7ad748c4e9e28ea68001e6035213"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "aaaafffffffffeb9ffff53b1feffab1e24f6b0f6a0d23067bf1285f3844b7764d7ac4b43b6a71b4b9ae67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "09cce3edfb8410c8f405ec722f9967eec5419200176ef7775e43d3c2ab5d3948fe7fd16b6de331680b40ff37040eaf06"
                )),
                c1: Bls12_381Fp(hex!(
                    "a2de1b12047beef10afa673ecf6644305eb41ef6896439ef60cfb130d9ed3d1cd92c7ad748c4e9e28ea68001e6035213"
                ))
            },
        ],
        [
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "fefffeffffff012e02000a6213d817de8896f8e63ba9b3ddea770f6a07c669ba51ce76df2f67195f0000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "acaa00000000fd8bfdff494feb2794409b5fb80f65297d89d49a75897d850daa85ded463864002ec99e67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "fefffeffffff012e02000a6213d817de8896f8e63ba9b3ddea770f6a07c669ba51ce76df2f67195f0000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "acaa00000000fd8bfdff494feb2794409b5fb80f65297d89d49a75897d850daa85ded463864002ec99e67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
        ],
        [
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "9529f87f1605e61ecd78d48b90c17158bdf0146853f345dbd08279e76035df7091cc99fa4aadd36bc186453811424e14"
                )),
                c1: Bls12_381Fp(hex!(
                    "16810780e9fa189b32877f256e3e3ac666059c8e4ddfea8bee8f0b0c241698f345e0b1486bfa47dfd85f3a01d9cfb205"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "fefffeffffff012e02000a6213d817de8896f8e63ba9b3ddea770f6a07c669ba51ce76df2f67195f0000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "09cce3edfb8410c8f405ec722f9967eec5419200176ef7775e43d3c2ab5d3948fe7fd16b6de331680b40ff37040eaf06"
                )),
                c1: Bls12_381Fp(hex!(
                    "09cce3edfb8410c8f405ec722f9967eec5419200176ef7775e43d3c2ab5d3948fe7fd16b6de331680b40ff37040eaf06"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "fffffeffffff012e02000a6213d817de8896f8e63ba9b3ddea770f6a07c669ba51ce76df2f67195f0000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "f34adc6d128af72cc27e6c4dc15a2d285f3cf671c98e0cec6fb3c7b68747a154b89f1f2302e9e98832e0c4362b3efc00"
                )),
                c1: Bls12_381Fp(hex!(
                    "b85f2392ed75078d3d81e7633da57ef6c4b9ba84d743247b4f5fbd3cfd03d60f1f0d2c20b4be31c26706bb02bfd30419"
                ))
            },
        ],
        [
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "aaaafffffffffeb9ffff53b1feffab1e24f6b0f6a0d23067bf1285f3844b7764d7ac4b43b6a71b4b9ae67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "aaaafffffffffeb9ffff53b1feffab1e24f6b0f6a0d23067bf1285f3844b7764d7ac4b43b6a71b4b9ae67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "aaaafffffffffeb9ffff53b1feffab1e24f6b0f6a0d23067bf1285f3844b7764d7ac4b43b6a71b4b9ae67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
        ],
        [
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "f34adc6d128af72cc27e6c4dc15a2d285f3cf671c98e0cec6fb3c7b68747a154b89f1f2302e9e98832e0c4362b3efc00"
                )),
                c1: Bls12_381Fp(hex!(
                    "b85f2392ed75078d3d81e7633da57ef6c4b9ba84d743247b4f5fbd3cfd03d60f1f0d2c20b4be31c26706bb02bfd30419"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "acaa00000000fd8bfdff494feb2794409b5fb80f65297d89d49a75897d850daa85ded463864002ec99e67f39ea11011a"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "a2de1b12047beef10afa673ecf6644305eb41ef6896439ef60cfb130d9ed3d1cd92c7ad748c4e9e28ea68001e6035213"
                )),
                c1: Bls12_381Fp(hex!(
                    "a2de1b12047beef10afa673ecf6644305eb41ef6896439ef60cfb130d9ed3d1cd92c7ad748c4e9e28ea68001e6035213"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "adaa00000000fd8bfdff494feb2794409b5fb80f65297d89d49a75897d850daa85ded463864002ec99e67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "9529f87f1605e61ecd78d48b90c17158bdf0146853f345dbd08279e76035df7091cc99fa4aadd36bc186453811424e14"
                )),
                c1: Bls12_381Fp(hex!(
                    "16810780e9fa189b32877f256e3e3ac666059c8e4ddfea8bee8f0b0c241698f345e0b1486bfa47dfd85f3a01d9cfb205"
                ))
            },
        ],
        [
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "acaa00000000fd8bfdff494feb2794409b5fb80f65297d89d49a75897d850daa85ded463864002ec99e67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "fefffeffffff012e02000a6213d817de8896f8e63ba9b3ddea770f6a07c669ba51ce76df2f67195f0000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "acaa00000000fd8bfdff494feb2794409b5fb80f65297d89d49a75897d850daa85ded463864002ec99e67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "fefffeffffff012e02000a6213d817de8896f8e63ba9b3ddea770f6a07c669ba51ce76df2f67195f0000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
        ],
        [
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "09cce3edfb8410c8f405ec722f9967eec5419200176ef7775e43d3c2ab5d3948fe7fd16b6de331680b40ff37040eaf06"
                )),
                c1: Bls12_381Fp(hex!(
                    "a2de1b12047beef10afa673ecf6644305eb41ef6896439ef60cfb130d9ed3d1cd92c7ad748c4e9e28ea68001e6035213"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "09cce3edfb8410c8f405ec722f9967eec5419200176ef7775e43d3c2ab5d3948fe7fd16b6de331680b40ff37040eaf06"
                )),
                c1: Bls12_381Fp(hex!(
                    "09cce3edfb8410c8f405ec722f9967eec5419200176ef7775e43d3c2ab5d3948fe7fd16b6de331680b40ff37040eaf06"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "aaaafffffffffeb9ffff53b1feffab1e24f6b0f6a0d23067bf1285f3844b7764d7ac4b43b6a71b4b9ae67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "a2de1b12047beef10afa673ecf6644305eb41ef6896439ef60cfb130d9ed3d1cd92c7ad748c4e9e28ea68001e6035213"
                )),
                c1: Bls12_381Fp(hex!(
                    "09cce3edfb8410c8f405ec722f9967eec5419200176ef7775e43d3c2ab5d3948fe7fd16b6de331680b40ff37040eaf06"
                ))
            },
        ],
        [
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "adaa00000000fd8bfdff494feb2794409b5fb80f65297d89d49a75897d850daa85ded463864002ec99e67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "acaa00000000fd8bfdff494feb2794409b5fb80f65297d89d49a75897d850daa85ded463864002ec99e67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "aaaafffffffffeb9ffff53b1feffab1e24f6b0f6a0d23067bf1285f3844b7764d7ac4b43b6a71b4b9ae67f39ea11011a"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "fefffeffffff012e02000a6213d817de8896f8e63ba9b3ddea770f6a07c669ba51ce76df2f67195f0000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "fffffeffffff012e02000a6213d817de8896f8e63ba9b3ddea770f6a07c669ba51ce76df2f67195f0000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
        ],
        [
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "16810780e9fa189b32877f256e3e3ac666059c8e4ddfea8bee8f0b0c241698f345e0b1486bfa47dfd85f3a01d9cfb205"
                )),
                c1: Bls12_381Fp(hex!(
                    "9529f87f1605e61ecd78d48b90c17158bdf0146853f345dbd08279e76035df7091cc99fa4aadd36bc186453811424e14"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "fefffeffffff012e02000a6213d817de8896f8e63ba9b3ddea770f6a07c669ba51ce76df2f67195f0000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "a2de1b12047beef10afa673ecf6644305eb41ef6896439ef60cfb130d9ed3d1cd92c7ad748c4e9e28ea68001e6035213"
                )),
                c1: Bls12_381Fp(hex!(
                    "a2de1b12047beef10afa673ecf6644305eb41ef6896439ef60cfb130d9ed3d1cd92c7ad748c4e9e28ea68001e6035213"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "fffffeffffff012e02000a6213d817de8896f8e63ba9b3ddea770f6a07c669ba51ce76df2f67195f0000000000000000"
                )),
                c1: Bls12_381Fp(hex!(
                    "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                ))
            },
            Fp2 {
                c0: Bls12_381Fp(hex!(
                    "b85f2392ed75078d3d81e7633da57ef6c4b9ba84d743247b4f5fbd3cfd03d60f1f0d2c20b4be31c26706bb02bfd30419"
                )),
                c1: Bls12_381Fp(hex!(
                    "f34adc6d128af72cc27e6c4dc15a2d285f3cf671c98e0cec6fb3c7b68747a154b89f1f2302e9e98832e0c4362b3efc00"
                ))
            },
        ],
    ];
}
