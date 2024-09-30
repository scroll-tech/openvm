use halo2curves_axiom::ff::Field;

use crate::common::{FieldExtension, Fp12Constructor};

pub fn conv_013_to_fp12<Fp, Fp2, Fp12>(line: [Fp2; 2]) -> Fp12
where
    Fp: Field,
    Fp2: FieldExtension<2, BaseField = Fp>,
    Fp12: FieldExtension<6, BaseField = Fp2> + Fp12Constructor<Fp2>,
{
    // x0 + x1*w + x2*w^2 + x3*w^3 + x4*w^4 + x5*w^5
    // (x0 + x2*w^2 + x4*w^4) + (x1 + x3*w^2 + x5*w^4)*w
    let x0 = Fp2::ONE;
    let x1 = line[0];
    let x2 = Fp2::ZERO;
    let x3 = line[1];
    let x4 = Fp2::ZERO;
    let x5 = Fp2::ZERO;

    Fp12::new(x0, x2, x4, x1, x3, x5)
}

pub fn conv_fp2_coeffs_to_fp12<Fp, Fp2, Fp12>(fp2_coeffs: &[Fp2]) -> Fp12
where
    Fp: Field,
    Fp2: FieldExtension<2, BaseField = Fp>,
    Fp12: FieldExtension<6, BaseField = Fp2> + Fp12Constructor<Fp2>,
{
    assert!(
        fp2_coeffs.len() <= 6,
        "fp2_coeffs must have at most 6 elements"
    );
    let mut coeffs = fp2_coeffs.to_vec();
    coeffs.resize(6, Fp2::ZERO);

    let x0 = coeffs[0];
    let x1 = coeffs[1];
    let x2 = coeffs[2];
    let x3 = coeffs[3];
    let x4 = coeffs[4];
    let x5 = coeffs[5];

    Fp12::new(x0, x2, x4, x1, x3, x5)
}
