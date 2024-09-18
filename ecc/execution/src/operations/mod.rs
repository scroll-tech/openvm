use std::ops::Mul;

use ff::Field;

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
pub fn mul_013_by_013<Fp2>(line_0: [Fp2; 2], line_1: [Fp2; 2]) -> [Fp2; 5]
where
    Fp2: Field,
{
    unimplemented!()
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
