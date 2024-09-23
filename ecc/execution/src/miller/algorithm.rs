use halo2curves_axiom::ff::Field;
use itertools::{izip, Itertools};

use super::{miller_double_and_add, miller_double_step, q_signed};
use crate::{
    common::{field::FieldExtension, point::EcPoint},
    operations::{evaluate_line, fp12_square, mul_013_by_013, mul_by_01234, mul_by_013},
};

#[allow(non_snake_case)]
pub fn multi_miller_loop<Fp, Fp2, Fp6, Fp12>(
    P: &[EcPoint<Fp>],
    Q: &[EcPoint<Fp2>],
    pseudo_binary_encoding: &[i32],
    xi_0: Fp2,
) -> Fp12
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp>,
    Fp6: FieldExtension<BaseField = Fp2>,
    Fp12: FieldExtension<BaseField = Fp6>,
{
    let x_over_ys = P
        .iter()
        .map(|P| P.x * P.y.invert().unwrap())
        .collect::<Vec<Fp>>();
    let y_invs = P.iter().map(|P| P.y.invert().unwrap()).collect::<Vec<Fp>>();
    let mut f = Fp12::ONE;
    let mut Q_acc = Q.to_vec();

    for i in pseudo_binary_encoding.len() - 2..=0 {
        f = fp12_square::<Fp, Fp2, Fp6, Fp12>(f);
        let mut lines = Vec::<[Fp2; 2]>::new();
        if pseudo_binary_encoding[i] == 0 {
            // let line_out = Vec::<[Fp2; 2]>::with_capacity(Q_acc.len());
            let (Q_out, lines_2S) = Q_acc
                .into_iter()
                .map(miller_double_step::<Fp, Fp2>)
                .unzip::<_, _, Vec<_>, Vec<_>>();
            Q_acc = Q_out;

            let lines_iter = izip!(lines_2S.iter(), x_over_ys.iter(), y_invs.iter());
            for (line_2S, x_over_y, y_inv) in lines_iter {
                let line = &evaluate_line::<Fp, Fp2>(*line_2S, *x_over_y, *y_inv);
                lines.push(*line);
            }
        } else {
            let Q_signed = q_signed(&Q_acc[i], pseudo_binary_encoding[i]);
            let (Q_out, lines_S_plus_Q, lines_S_plus_Q_plus_S): (Vec<_>, Vec<_>, Vec<_>) = Q_acc
                .iter()
                .map(|Q| miller_double_and_add::<Fp, Fp2>(Q.clone(), Q_signed.clone()))
                .multiunzip();
            Q_acc = Q_out;

            let lines_iter = izip!(
                lines_S_plus_Q.iter(),
                lines_S_plus_Q_plus_S.iter(),
                x_over_ys.iter(),
                y_invs.iter()
            );
            let mut lines0 = Vec::<[Fp2; 2]>::new();
            let mut lines1 = Vec::<[Fp2; 2]>::new();
            for (line_S_plus_Q, line_S_plus_Q_plus_S, x_over_y, y_inv) in lines_iter {
                let line0 = &evaluate_line::<Fp, Fp2>(*line_S_plus_Q, *x_over_y, *y_inv);
                let line1 = &evaluate_line::<Fp, Fp2>(*line_S_plus_Q_plus_S, *x_over_y, *y_inv);
                lines0.push(*line0);
                lines1.push(*line1);
            }
            let lines_concat = [lines0, lines1].concat();
            lines.extend(lines_concat);
        }

        if lines.len() % 2 == 1 {
            f = mul_by_013::<Fp, Fp2, Fp6, Fp12>(f, lines.pop().unwrap());
        }
        for chunk in lines.chunks(2) {
            if let [line0, line1] = chunk {
                let prod = mul_013_by_013(*line0, *line1, xi_0);
                f = mul_by_01234(f, prod);
            } else {
                panic!("lines.len() % 2 should be 0 at this point");
            }
        }
    }
    f
}

pub fn final_exponentiation<Fp, Fp2, Fp6, Fp12>(x: Fp12) -> Fp12
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp>,
    Fp6: FieldExtension<BaseField = Fp2>,
    Fp12: FieldExtension<BaseField = Fp6>,
{
    x * x
}
