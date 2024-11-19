#[cfg(target_os = "zkvm")]
use core::hint::black_box;
use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use axvm_algebra::{DivUnsafe, IntMod};
#[cfg(target_os = "zkvm")]
use {
    axvm_platform::constants::{Custom1Funct3, ModArithBaseFunct7, SwBaseFunct7, CUSTOM_1},
    axvm_platform::custom_insn_r,
    core::mem::MaybeUninit,
};

use super::group::Group;

axvm::moduli_setup! {
    Secp256k1Coord = "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F";
    Secp256k1Scalar = "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141";
}

// TODO[jpw] rename to Secp256k1
axvm::sw_setup! {
    Secp256k1Point = Secp256k1Coord;
}

pub fn setup_moduli() {
    #[cfg(target_os = "zkvm")]
    {
        black_box(AXIOM_SERIALIZED_MODULUS_Secp256k1Coord);
        black_box(AXIOM_SERIALIZED_MODULUS_Secp256k1Scalar);
    }
}
