#[cfg(target_os = "zkvm")]
use {
    axvm_platform::constants::{Custom1Funct3, PairingBaseFunct7, CUSTOM_1},
    axvm_platform::custom_insn_r,
    core::mem::MaybeUninit,
};

use super::{Bn254Fp, Bn254Fp2};
use crate::field::{Field, FieldExtension, Fp12Mul, SexticExtField};

pub type Bn254Fp12 = SexticExtField<Bn254Fp2>;

impl FieldExtension for Bn254Fp12 {
    type BaseField = Bn254Fp2;
    type Coeffs = [Bn254Fp2; 6];
    type SelfRef<'a> = &'a Self;

    fn from_coeffs(coeffs: Self::Coeffs) -> Self {
        Self::new([
            coeffs[0].clone(),
            coeffs[2].clone(),
            coeffs[4].clone(),
            coeffs[1].clone(),
            coeffs[3].clone(),
            coeffs[5].clone(),
        ])
    }

    fn to_coeffs(self) -> Self::Coeffs {
        [
            self.c[0].clone(),
            self.c[1].clone(),
            self.c[2].clone(),
            self.c[3].clone(),
            self.c[4].clone(),
            self.c[5].clone(),
        ]
    }

    fn embed(base_elem: Self::BaseField) -> Self {
        Self::new([
            base_elem,
            <Self::BaseField as Field>::zero(),
            <Self::BaseField as Field>::zero(),
            <Self::BaseField as Field>::zero(),
            <Self::BaseField as Field>::zero(),
            <Self::BaseField as Field>::zero(),
        ])
    }

    fn conjugate(&self) -> Self {
        Self::new([
            self.c[0].clone(),
            -self.c[1].clone(),
            self.c[2].clone(),
            -self.c[3].clone(),
            self.c[4].clone(),
            -self.c[5].clone(),
        ])
    }

    fn frobenius_map(&self, _power: Option<usize>) -> Self {
        todo!()
    }

    fn mul_base(&self, rhs: Self::BaseField) -> Self {
        Self::new([
            &self.c[0] * &rhs,
            &self.c[1] * &rhs,
            &self.c[2] * &rhs,
            &self.c[3] * &rhs,
            &self.c[4] * &rhs,
            &self.c[5] * &rhs,
        ])
    }
}

impl Fp12Mul for Bn254Fp12 {
    type Fp = Bn254Fp;
    type Fp2 = Bn254Fp2;

    fn fp12_mul(&mut self, other: &Self, xi: &Self::Fp2) {
        #[cfg(not(target_os = "zkvm"))]
        {
            // The following multiplication is hand-derived for Fp12 * Fp12:
            // c0 = cs0co0 + xi(cs1co2 + cs2co1 + cs3co5 + cs4co4 + cs5co3)
            // c1 = cs0co1 + cs1co0 + cs3co3 + xi(cs2co2 + cs4co5 + cs5co4)
            // c2 = cs0co2 + cs1co1 + cs2co0 + cs3co4 + cs4co3 + xi(cs5co5)
            // c3 = cs0co3 + cs3co0 + xi(cs1co5 + cs2co4 + cs4co2 + cs5co1)
            // c4 = cs0co4 + cs1co3 + cs3co1 + cs4co0 + xi(cs2co5 + cs5co2)
            // c5 = cs0co5 + cs1co4 + cs2co3 + cs3co2 + cs4co1 + cs5co0
            //   where cs*: self.c*, co*: other.c*

            let (s0, s1, s2, s3, s4, s5) = (
                &self.c[0], &self.c[2], &self.c[4], &self.c[1], &self.c[3], &self.c[5],
            );
            let (o0, o1, o2, o3, o4, o5) = (
                &other.c[0],
                &other.c[2],
                &other.c[4],
                &other.c[1],
                &other.c[3],
                &other.c[5],
            );
            *self = Self::new([
                s0 * o0 + xi * &(s1 * o2 + s2 * o1 + s3 * o5 + s4 * o4 + s5 * o3),
                s0 * o1 + s1 * o0 + s3 * o3 + xi * &(s2 * o2 + s4 * o5 + s5 * o4),
                s0 * o2 + s1 * o1 + s2 * o0 + s3 * o4 + s4 * o3 + xi * &(s5 * o5),
                s0 * o3 + s3 * o0 + xi * &(s1 * o5 + s2 * o4 + s4 * o2 + s5 * o1),
                s0 * o4 + s1 * o3 + s3 * o1 + s4 * o0 + xi * &(s2 * o5 + s5 * o2),
                s0 * o5 + s1 * o4 + s2 * o3 + s3 * o2 + s4 * o1 + s5 * o0,
            ]);
        }
        #[cfg(target_os = "zkvm")]
        {
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                PairingBaseFunct7::Fp12Mul as usize,
                self as *mut Self,
                self as *const Self,
                other as *const Self
            )
        }
    }

    fn fp12_mul_refs(&self, other: &Self, xi: &Self::Fp2) -> Self {
        #[cfg(not(target_os = "zkvm"))]
        {
            let mut res = self.clone();
            res.fp12_mul(other, xi);
            res
        }
        #[cfg(target_os = "zkvm")]
        {
            let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                PairingBaseFunct7::Fp12Mul as usize,
                uninit.as_mut_ptr(),
                self as *const Self,
                other as *const Self
            );
            unsafe { uninit.assume_init() }
        }
    }
}
