use std::ops::Mul;

use ff::Field;

use crate::ec_point::Xi;

pub fn fp12_square<Fp12>(x: Fp12) -> Fp12
where
    Fp12: Field,
{
    x * x
}

pub fn evaluate_line<Fp, Fp2>(line: [Fp2; 2], x_over_y: Fp, y_inv: Fp) -> [Fp2; 2]
where
    Fp: Field,
    Fp2: Field + Mul<Fp, Output = Fp2>,
{
    let b = line[0];
    let c = line[1];
    let b_prime = b * x_over_y;
    let c_prime = c * y_inv;
    [b_prime, c_prime]
}

/// Multiplies two lines in 013 form and outputs the product in 01234 form
pub fn mul_013_by_013<Fp2>(
    line_0: [Fp2; 2],
    line_1: [Fp2; 2],
    // TODO[yj]: once this function is moved into a chip, we can use the xi property instead of passing in this argument
    xi: Xi<Fp2>,
) -> [Fp2; 5]
where
    Fp2: Field,
{
    let b0 = line_0[0];
    let c0 = line_0[1];
    let b1 = line_1[0];
    let c1 = line_1[1];

    // l0 * l1 = (1 + c0c1 * xi.constant + c0c1 * xi.u) + (b0 + b1)w + (b0b1)w² + (c0 + c1)w³ + (b0c1 + c0b1)w⁴
    let l0 = Fp2::ONE + c0 * c1 * xi.constant + c0 * c1 * xi.u;
    let l1 = b0 + b1;
    let l2 = b0 * b1;
    let l3 = c0 + c1;
    let l4 = b0 * c1 + b1 * c0;

    [l0, l1, l2, l3, l4]
}

pub fn mul_by_013<Fp2, Fp12>(f: Fp12, line: [Fp2; 2]) -> Fp12
where
    Fp2: Field,
    Fp12: Field + Mul<Fp2, Output = Fp12>,
{
    mul_by_01234(f, [Fp2::ZERO, line[0], Fp2::ZERO, line[1], Fp2::ZERO])
}

pub fn mul_by_01234<Fp2, Fp12>(f: Fp12, x: [Fp2; 5]) -> Fp12
where
    Fp2: Field,
    Fp12: Field + Mul<Fp2, Output = Fp12>,
{
    unimplemented!()
}
