use super::Bn254Fp;
use crate::field::{Complex, Field, FieldExtension, Xi};

pub type Bn254Fp2 = Complex<Bn254Fp>;

impl Xi for Bn254Fp2 {
    const XI: Self = Self::new(Bn254Fp::from_const_u8(9), Bn254Fp::from_const_u8(1));
}

impl FieldExtension for Bn254Fp2 {
    type BaseField = Bn254Fp;
    type Coeffs = [Bn254Fp; 2];
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
