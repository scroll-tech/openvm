use halo2curves_axiom::ff::Field;

use crate::common::{EcPoint, FieldExtension};

#[allow(non_snake_case)]
pub fn q_signed<Fp, Fp2>(Q: &[EcPoint<Fp2>], sigma_i: i32) -> Vec<EcPoint<Fp2>>
where
    Fp: Field,
    Fp2: FieldExtension<2, BaseField = Fp>,
{
    Q.iter()
        .map(|q| match sigma_i {
            1 => q.clone(),
            -1 => q.neg(),
            _ => panic!("Invalid sigma_i"),
        })
        .collect()
}
