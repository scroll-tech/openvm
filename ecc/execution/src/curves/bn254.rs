use halo2curves_axiom::{
    bn256::{Fq, Fq12, Fq2, Fq6},
    ff::Field,
};

use crate::common::FieldExtension;

pub const BN254_XI: Fq2 = Fq2 {
    c0: Fq::from_raw([
        0x0000000000000009,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
    ]),
    c1: Fq::ONE,
};

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

    fn frobenius_map(&mut self, power: Option<usize>) {
        self.frobenius_map(power.unwrap());
    }

    fn mul_base(self, rhs: &Self::BaseField) -> Self {
        Fq2 {
            c0: self.c0 * rhs,
            c1: self.c1 * rhs,
        }
    }
}

/// FieldExtension for Fq6 with Fq2 as base field
impl FieldExtension<3> for Fq6 {
    type BaseField = Fq2;

    fn from_coeffs(coeffs: [Self::BaseField; 3]) -> Self {
        Fq6 {
            c0: coeffs[0],
            c1: coeffs[1],
            c2: coeffs[2],
        }
    }

    fn embed(base_elem: &Self::BaseField) -> Self {
        Fq6 {
            c0: *base_elem,
            c1: Fq2::ZERO,
            c2: Fq2::ZERO,
        }
    }

    fn frobenius_map(&mut self, power: Option<usize>) {
        self.frobenius_map(power.unwrap());
    }

    fn mul_base(self, rhs: &Self::BaseField) -> Self {
        Fq6 {
            c0: self.c0 * rhs,
            c1: self.c1 * rhs,
            c2: self.c2 * rhs,
        }
    }
}

/// FieldExtension for Fq12 with Fq6 as base field since halo2curves does not implement `Field` for Fq6.
impl FieldExtension<2> for Fq12 {
    type BaseField = Fq6;

    fn from_coeffs(coeffs: [Self::BaseField; 2]) -> Self {
        Fq12 {
            c0: coeffs[0],
            c1: coeffs[1],
        }
    }

    fn embed(base_elem: &Self::BaseField) -> Self {
        Fq12 {
            c0: *base_elem,
            c1: Fq6::ZERO,
        }
    }

    fn frobenius_map(&mut self, power: Option<usize>) {
        self.frobenius_map(power.unwrap());
    }

    fn mul_base(self, rhs: &Self::BaseField) -> Self {
        Fq12 {
            c0: self.c0 * rhs,
            c1: self.c1 * rhs,
        }
    }
}

pub fn conv_013_to_fq12(line: [Fq2; 2]) -> Fq12 {
    let x0 = Fq2::ONE;
    let x1 = line[0];
    let x3 = line[1];
    // x0 + x1*w + x2*w^2 + x3*w^3 + x4*w^4 + x5*w^5
    // (x0 + x2*w^2 + x4*w^4) + (x1 + x3*w^2 + x5*w^4)*w
    Fq12 {
        c0: Fq6 {
            c0: x0,        // x0
            c1: Fq2::ZERO, // x2
            c2: Fq2::ZERO, // x4
        },
        c1: Fq6 {
            c0: x1,        // x1
            c1: x3,        // x3
            c2: Fq2::ZERO, // x5
        },
    }
}

pub fn conv_fp2_coeffs_to_fq12(fp2_coeffs: &[Fq2]) -> Fq12 {
    assert!(
        fp2_coeffs.len() <= 6,
        "fp2_coeffs must have at most 6 elements"
    );
    let mut coeffs = fp2_coeffs.to_vec();
    coeffs.resize(6, Fq2::ZERO);

    let x0 = coeffs[0];
    let x1 = coeffs[1];
    let x2 = coeffs[2];
    let x3 = coeffs[3];
    let x4 = coeffs[4];
    let x5 = coeffs[5];
    Fq12 {
        c0: Fq6 {
            c0: x0,
            c1: x2,
            c2: x4,
        },
        c1: Fq6 {
            c0: x1,
            c1: x3,
            c2: x5,
        },
    }
}
