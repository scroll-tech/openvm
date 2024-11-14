use super::Bn254Fp2;
use crate::field::{Field, FieldExtension, SexticExtField};

pub type Bn254Fp12 = SexticExtField<Bn254Fp2>;

impl FieldExtension for Bn254Fp12 {
    type BaseField = Bn254Fp2;
    type Coeffs = [Bn254Fp2; 6];
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
