use halo2curves_axiom::ff::Field;

use crate::common::{field::FieldExtension, point::EcPoint};

#[allow(non_snake_case)]
pub fn q_signed<Fp, Fp2>(Q: &EcPoint<Fp2>, sigma_i: i32) -> EcPoint<Fp2>
where
    Fp: Field,
    Fp2: FieldExtension<2, BaseField = Fp>,
{
    match sigma_i {
        1 => Q.clone(),
        -1 => Q.neg(),
        _ => panic!("Invalid sigma_i"),
    }
}
