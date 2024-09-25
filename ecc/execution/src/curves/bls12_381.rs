use halo2curves_axiom::{
    bls12_381::{Fq, Fq12, Fq2, Fq6},
    ff::Field,
};

use crate::common::FieldExtension;

/// FieldExtension for Fq2 with Fq as base field
impl FieldExtension<2> for Fq2 {
    type BaseField = Fq;

    fn from_coeffs(coeffs: [Self::BaseField; 2]) -> Self {
        Fq2 {
            c0: coeffs[0],
            c1: coeffs[1],
        }
    }

    fn embed(base_elem: &Self::BaseField) -> Self {
        Fq2 {
            c0: *base_elem,
            c1: Fq::ZERO,
        }
    }

    fn mul_base(self, rhs: &Self::BaseField) -> Self {
        Fq2 {
            c0: self.c0 * rhs,
            c1: self.c1 * rhs,
        }
    }

    fn frobenius_map(&mut self, _power: Option<usize>) {
        Fq2::frobenius_map(self);
    }
}

///
/// Note that halo2curves does not implement `Field` for Fq6, so we need to implement the intermediate points manually.
///

/// FieldExtension for Fq12 with Fq2 as base field since halo2curves does not implement `Field` for Fq6.
impl FieldExtension<6> for Fq12 {
    type BaseField = Fq2;

    fn from_coeffs(coeffs: [Self::BaseField; 6]) -> Self {
        Fq12 {
            c0: Fq6 {
                c0: coeffs[0],
                c1: coeffs[2],
                c2: coeffs[4],
            },
            c1: Fq6 {
                c0: coeffs[1],
                c1: coeffs[3],
                c2: coeffs[5],
            },
        }
    }

    fn embed(base_elem: &Self::BaseField) -> Self {
        let fq6_pt = Fq6 {
            c0: *base_elem,
            c1: Fq2::zero(),
            c2: Fq2::zero(),
        };
        Fq12 {
            c0: fq6_pt,
            c1: Fq6::zero(),
        }
    }

    fn frobenius_map(&mut self, _power: Option<usize>) {
        Fq12::frobenius_map(self);
    }

    fn mul_base(self, rhs: &Self::BaseField) -> Self {
        let fq6_pt = Fq6 {
            c0: *rhs,
            c1: Fq2::zero(),
            c2: Fq2::zero(),
        };
        Fq12 {
            c0: self.c0 * fq6_pt,
            c1: self.c1 * fq6_pt,
        }
    }
}
