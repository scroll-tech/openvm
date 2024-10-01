use halo2curves_axiom::ff::Field;

use crate::common::FieldExtension;

pub fn evaluate_line<Fp, Fp2>(line: [Fp2; 2], x_over_y: Fp, y_inv: Fp) -> [Fp2; 2]
where
    Fp: Field,
    Fp2: FieldExtension<2, BaseField = Fp>,
{
    let b_prime = line[0];
    let c_prime = line[1];
    let b = b_prime.mul_base(&x_over_y);
    let c = c_prime.mul_base(&y_inv);
    [b, c]
}

/// Multiplies two elements in 013 form and outputs the product in 012345 form
pub fn mul_023_by_023<Fp, Fp2>(
    line_0: [Fp2; 2],
    line_1: [Fp2; 2],
    // TODO[yj]: once this function is moved into a chip, we can use the xi property instead of passing in this argument
    xi: Fp2,
) -> [Fp2; 6]
where
    Fp: Field,
    Fp2: FieldExtension<2, BaseField = Fp>,
{
    let b0 = line_0[0];
    let c0 = line_0[1];
    let b1 = line_1[0];
    let c1 = line_1[1];

    // where w⁶ = xi
    // l0 * l1 = b0b1 + (b0c1 + b1c0)w² + (b0 + b1)w³ + (c0c1)w⁵ + w⁶
    //         = (b0b1 + xi) + (b0c1 + b1c0)w² + (b0 + b1)w³ + (c0c1)w⁵
    let l0 = b0 + b1 + xi;
    let l1 = Fp2::ZERO;
    let l2 = b0 * c1 + b1 * c0;
    let l3 = b0 + b1;
    let l4 = Fp2::ZERO;
    let l5 = c0 * c1;

    [l0, l1, l2, l3, l4, l5]
}

pub fn mul_by_023<Fp, Fp2, Fp12>(f: Fp12, line: [Fp2; 2]) -> Fp12
where
    Fp: Field,
    Fp2: FieldExtension<2, BaseField = Fp>,
    Fp12: FieldExtension<6, BaseField = Fp2>,
{
    mul_by_012345(
        f,
        [line[0], Fp2::ZERO, line[1], Fp2::ONE, Fp2::ZERO, Fp2::ZERO],
    )
}

pub fn mul_by_012345<Fp, Fp2, Fp12>(f: Fp12, x: [Fp2; 6]) -> Fp12
where
    Fp: Field,
    Fp2: FieldExtension<2, BaseField = Fp>,
    Fp12: FieldExtension<6, BaseField = Fp2>,
{
    let x_fp12 = Fp12::from_coeffs(x);
    f * x_fp12
}
