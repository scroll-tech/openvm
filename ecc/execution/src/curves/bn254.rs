use halo2curves_axiom::{
    bn256::{Fq, Fq12, Fq2, Fq6},
    ff::Field,
};

use crate::common::field::FieldExtension;

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
}

impl FieldExtension for Fq6 {
    type BaseField = Fq2;

    fn lift(base: &Self::BaseField) -> Self {
        Fq6 {
            c0: *base,
            c1: Fq2::ZERO,
            c2: Fq2::ZERO,
        }
    }

    fn mul_base(self, rhs: &Self::BaseField) -> Self {
        Fq6 {
            c0: self.c0 * rhs,
            c1: self.c1 * rhs,
            c2: self.c2 * rhs,
        }
    }
}

impl FieldExtension for Fq12 {
    type BaseField = Fq6;

    fn lift(base: &Self::BaseField) -> Self {
        Fq12 {
            c0: *base,
            c1: Fq6::ZERO,
        }
    }

    fn mul_base(self, rhs: &Self::BaseField) -> Self {
        Fq12 {
            c0: self.c0 * rhs,
            c1: self.c1 * rhs,
        }
    }
}
