use halo2curves_axiom::ff::Field;

use crate::common::field::FieldExtension;

pub fn fp12_square<Fp, Fp2, Fp6, Fp12>(x: Fp12) -> Fp12
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp>,
    Fp6: FieldExtension<BaseField = Fp2>,
    Fp12: FieldExtension<BaseField = Fp6>,
{
    fp12_multiply(x, x)
}

pub fn fp12_multiply<Fp, Fp2, Fp6, Fp12>(x: Fp12, y: Fp12) -> Fp12
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp>,
    Fp6: FieldExtension<BaseField = Fp2>,
    Fp12: FieldExtension<BaseField = Fp6>,
{
    x * y
}
