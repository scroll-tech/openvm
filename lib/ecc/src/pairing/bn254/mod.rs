use core::ops::{Add, Neg};

use lazy_static::lazy_static;

mod line;
pub use line::*;

mod miller_step;
pub use miller_step::*;

use crate::field::{Complex, Field, FieldExt, SexticExtField};

pub struct Bn254;

lazy_static! {
    pub static ref BN254_XI: Fp2 = Fp2::from_u32((9, 1));
    pub static ref BN254_TWO: Fp2 = Fp2::from_u32((2, 0));
    pub static ref BN254_THREE: Fp2 = Fp2::from_u32((3, 0));
}

#[derive(Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Fp([u8; 32]);

impl Field for Fp {
    type SelfRef<'a> = &'a Self;

    const ZERO: Self = Self([0u8; 32]);
    const ONE: Self = Self([1u8; 32]);

    fn invert(&self) -> Option<Self> {
        panic!("Not implemented");
    }
}

impl Neg for Fp {
    type Output = Self;

    fn neg(self) -> Self {
        -1 * self
    }
}

impl Add for Fp {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

pub type Fp2 = Complex<Fp>;

impl Field for Fp2 {
    const ZERO: Self = Self::from_coeffs([Fp::ZERO, Fp::ZERO]);
    const ONE: Self = Self::from_coeffs([Fp::ONE, Fp::ZERO]);

    fn invert(&self) -> Option<Self> {
        todo!()
    }
}

impl FieldExt for Fp2 {
    type BaseField = Fp;
    type Coeffs = [Fp; 2];
    type SelfRef<'a> = &'a Self;

    fn from_coeffs(coeffs: Self::Coeffs) -> Self {
        Self {
            c0: coeffs[0].clone(),
            c1: coeffs[1].clone(),
        }
    }

    fn embed(base_elem: Self::BaseField) -> Self {
        Self {
            c0: base_elem.clone(),
            c1: Fp::ZERO,
        }
    }

    fn conjugate(&self) -> Self {
        Self {
            c0: self.c0.clone(),
            c1: -self.c1.clone(),
        }
    }

    fn frobenius_map(&self, power: Option<usize>) -> Self {
        todo!()
    }

    fn mul_base(&self, rhs: Self::BaseField) -> Self {
        Self {
            c0: self.c0.mul_base(rhs),
            c1: self.c1.mul_base(rhs),
        }
    }
}

impl Neg for Fp2 {
    type Output = Self;

    fn neg(self) -> Self {
        -1 * self
    }
}

pub type Fp12 = SexticExtField<Fp2>;
