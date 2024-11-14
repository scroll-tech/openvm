use axvm::{
    intrinsics::{DivUnsafe, IntMod},
    moduli_setup,
};

use crate::field::{Complex, Field, FieldExtension, SexticExtField, Xi};

mod line;

pub struct Bn254;

moduli_setup! {
    Fp = "21888242871839275222246405745257275088696311157297823662689037894645226208583";
}
type Fp2 = Complex<Fp>;
type Fp12 = SexticExtField<Fp2>;

impl Xi for Fp2 {
    const XI: Self = Self::new(Fp::from_const_u8(9), Fp::from_const_u8(1));
}

impl Field for Fp {
    type SelfRef<'a> = &'a Self;

    fn zero() -> Self {
        <Self as IntMod>::ZERO
    }

    fn one() -> Self {
        <Self as IntMod>::ONE
    }

    fn invert(&self) -> Option<Self> {
        Some(<Fp as IntMod>::ONE.div_unsafe(self))
    }
}

impl FieldExtension for Fp2 {
    type BaseField = Fp;
    type Coeffs = [Fp; 2];
    type SelfRef<'a> = &'a Self;

    fn from_coeffs(coeffs: Self::Coeffs) -> Self {
        Self {
            c0: coeffs[0].clone(),
            c1: coeffs[1].clone(),
        }
    }

    fn to_coeffs(self) -> Self::Coeffs {
        [self.c0, self.c1]
    }

    fn embed(base_elem: Self::BaseField) -> Self {
        Self {
            c0: base_elem,
            c1: <Self::BaseField as Field>::zero(),
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
            c0: &self.c0 * &rhs,
            c1: &self.c1 * &rhs,
        }
    }
}

impl FieldExtension for Fp12 {
    type BaseField = Fp2;
    type Coeffs = [Fp2; 6];
    type SelfRef<'a> = &'a Self;

    fn from_coeffs(coeffs: Self::Coeffs) -> Self {
        Self::new([
            coeffs[0].clone(),
            coeffs[1].clone(),
            coeffs[2].clone(),
            coeffs[3].clone(),
            coeffs[4].clone(),
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

    fn frobenius_map(&self, power: Option<usize>) -> Self {
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
