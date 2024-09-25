use halo2curves_axiom::{
    bls12_381::{Fq, Fq12, Fq2, Fq6},
    ff::Field,
};

use crate::common::field::FieldExtension;
const UNUSED_POWER_ARG: usize = 0;

/// FieldExtension for Fq2 with Fq as base field
impl FieldExtension for Fq2 {
    type BaseField = Fq;

    fn lift(base: &Self::BaseField) -> Self {
        Fq2 {
            c0: *base,
            c1: Fq::ZERO,
        }
    }

    fn mul_base(self, rhs: &Self::BaseField) -> Self {
        Fq2 {
            c0: self.c0 * rhs,
            c1: self.c1 * rhs,
        }
    }

    fn frobenius_map(&mut self, _power: usize) {
        Fq2::frobenius_map(self);
    }
}

///
/// Note that halo2curves does not implement `Field` for Fq6, so we need to implement the intermediate points manually.
///

/// FieldExtension for Fq12 with Fq2 as base field since halo2curves does not implement `Field` for Fq6.
impl FieldExtension for Fq12 {
    type BaseField = Fq2;

    fn lift(base: &Self::BaseField) -> Self {
        let fq6_pt = Fq6 {
            c0: *base,
            c1: Fq2::zero(),
            c2: Fq2::zero(),
        };
        Fq12 {
            c0: fq6_pt,
            c1: Fq6::zero(),
        }
    }

    fn frobenius_map(&mut self, _power: usize) {
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
