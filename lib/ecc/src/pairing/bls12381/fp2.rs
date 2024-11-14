use axvm_algebra::IntMod;

use super::Bls12381Fp;
use crate::field::{Complex, Field, FieldExtension, SexticExtField, Xi};

pub type Bls12381Fp2 = Complex<Bls12381Fp>;

impl Xi for Bls12381Fp2 {
    const XI: Self = Self::new(Bls12381Fp::ONE, Bls12381Fp::ONE);
}

impl FieldExtension for Bls12381Fp2 {
    type BaseField = Bls12381Fp;
    type Coeffs = [Bls12381Fp; 2];
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

    fn frobenius_map(&self, _power: Option<usize>) -> Self {
        todo!()
    }

    fn mul_base(&self, rhs: Self::BaseField) -> Self {
        Self {
            c0: &self.c0 * &rhs,
            c1: &self.c1 * &rhs,
        }
    }
}
