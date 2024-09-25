use halo2curves_axiom::ff::Field;

use crate::common::{field::FieldExtension, point::EcPoint};

/// Returns a line function for a tangent line at the point P
#[allow(non_snake_case)]
pub fn point_to_line_function<Fp, Fp2>(P: EcPoint<Fp>) -> [Fp2; 2]
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp>,
{
    let one = Fp2::ONE;
    let two = one + one;
    let three = one + two;
    let x = Fp2::lift(&P.x);
    let y = Fp2::lift(&P.y);

    // λ = (3x^2) / (2y)
    // 1 - λ(x/y)w + (λx - y)(1/y)w^3
    // b = -3x^3 / 2y^2
    // c = 3x^3/2y^2 - 1
    let x_squared = x.square();
    let x_cubed = x_squared * x;
    let y_squared = y.square();
    let three_x_cubed = three * x_cubed;
    let over_two_y_squared = (two * y_squared).invert().unwrap();
    let b = three_x_cubed.neg() * over_two_y_squared;
    let c = three_x_cubed * over_two_y_squared - Fp2::ONE;
    [b, c]
}

#[allow(non_snake_case)]
pub fn line_function_to_fp12<Fp, Fp2, Fp6, Fp12>(line: [Fp2; 2]) -> Fp12
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp>,
    Fp6: FieldExtension<BaseField = Fp2>,
    Fp12: FieldExtension<BaseField = Fp6>,
{
    let b = line[0];
    let c = line[1];
    Fp12::lift(&Fp6::lift(&b)) + Fp12::lift(&Fp6::lift(&c))
}

pub fn frobenius<Fp, Fp2, Fp6, Fp12>(x: Fp12) -> Fp12
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp>,
    Fp6: FieldExtension<BaseField = Fp2>,
    Fp12: FieldExtension<BaseField = Fp6>,
{
    unimplemented!("frobenius is not implemented");
}
