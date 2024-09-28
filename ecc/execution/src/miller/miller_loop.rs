use halo2curves_axiom::ff::Field;
use itertools::{izip, Itertools};

use super::{miller_double_and_add, miller_double_step, q_signed};
use crate::{
    common::{EcPoint, FieldExtension},
    miller::miller_add_step,
    operations::{
        evaluate_line, fp12_multiply, fp12_square, mul_013_by_013, mul_by_01234, mul_by_013,
    },
};

#[allow(non_snake_case)]
pub fn multi_miller_loop<Fp, Fp2, Fp12>(
    P: &[EcPoint<Fp>],
    Q: &[EcPoint<Fp2>],
    pseudo_binary_encoding: &[i32],
    xi: Fp2,
) -> Fp12
where
    Fp: Field,
    Fp2: FieldExtension<2, BaseField = Fp>,
    Fp12: FieldExtension<6, BaseField = Fp2>,
{
    multi_miller_loop_embedded_exp::<Fp, Fp2, Fp12>(P, Q, None, pseudo_binary_encoding, xi)
}

#[allow(non_snake_case)]
pub fn multi_miller_loop_embedded_exp<Fp, Fp2, Fp12>(
    P: &[EcPoint<Fp>],
    Q: &[EcPoint<Fp2>],
    c: Option<Fp12>,
    pseudo_binary_encoding: &[i32],
    xi: Fp2,
) -> Fp12
where
    Fp: Field,
    Fp2: FieldExtension<2, BaseField = Fp>,
    Fp12: FieldExtension<6, BaseField = Fp2>,
{
    assert!(P.len() > 0);
    assert_eq!(P.len(), Q.len());

    let y_invs = P.iter().map(|P| P.y.invert().unwrap()).collect::<Vec<Fp>>();
    let x_over_ys = P
        .iter()
        .zip(y_invs.iter())
        .map(|(P, y_inv)| P.x * y_inv)
        .collect::<Vec<Fp>>();
    let c_inv = if let Some(c) = c {
        c.invert().unwrap()
    } else {
        Fp12::ONE
    };
    let mut f = Fp12::ONE;
    let mut Q_acc = Q.to_vec();

    let mut total_double = 0;
    let mut total_double_add = 0;

    for i in (0..pseudo_binary_encoding.len() - 1).rev() {
        println!(
            "miller i: {} = {}; Q_acc.x: {:?}",
            i, pseudo_binary_encoding[i], Q_acc[0].x
        );
        let mut lines = Vec::<[Fp2; 2]>::new();

        // First iteration is at pseudo_binary_encoding[len-2]
        if i == pseudo_binary_encoding.len() - 2 {
            println!("miller first iter special case");
            // special case first step of pseudo-binary encoding as 1
            // this means that the first step is a double and add, but we need to separate the two steps since the optimized
            // `miller_double_and_add_step` will fail because Q_acc is equal to Q_signed on the first iteration
            let (Q_out_double, lines_2S) = Q_acc
                .into_iter()
                .map(miller_double_step::<Fp, Fp2>)
                .unzip::<_, _, Vec<_>, Vec<_>>();
            Q_acc = Q_out_double;

            let (Q_out_add, lines_S_plus_Q) = Q_acc
                .iter()
                .zip(Q.iter())
                .map(|(Q_acc, Q)| miller_add_step::<Fp, Fp2>(Q_acc.clone(), Q.clone()))
                .unzip::<_, _, Vec<_>, Vec<_>>();
            Q_acc = Q_out_add;

            let lines_iter = izip!(
                lines_2S.iter(),
                lines_S_plus_Q.iter(),
                x_over_ys.iter(),
                y_invs.iter()
            );
            let mut lines0 = Vec::<[Fp2; 2]>::new();
            let mut lines1 = Vec::<[Fp2; 2]>::new();
            for (line0, line1, x_over_y, y_inv) in lines_iter {
                let line0 = &evaluate_line::<Fp, Fp2>(*line0, *x_over_y, *y_inv);
                let line1 = &evaluate_line::<Fp, Fp2>(*line1, *x_over_y, *y_inv);
                lines0.push(*line0);
                lines1.push(*line1);
                // lines.push(*line0);
                // lines.push(*line1);
            }
            let lines_concat = [lines0, lines1].concat();
            lines.extend(lines_concat);

            // Debug counter
            total_double_add += 1;

            // // Gnark implementation
            // for chunk in lines.chunks(2) {
            //     if let [line0, line1] = chunk {
            //         let prod = mul_013_by_013(*line0, *line1, xi);
            //         f = mul_by_01234(f, prod);
            //     } else {
            //         panic!("lines.len() % 2 should be 0 at this point");
            //     }
            // }
        } else {
            f = fp12_square::<Fp12>(f);

            if pseudo_binary_encoding[i] == 0 {
                // Run miller double step if \sigma_i == 0
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

                // Debug counter
                total_double += 1;

                // // Gnark implementation
                // for line in lines.iter() {
                //     f = mul_by_013::<Fp, Fp2, Fp12>(f, *line);
                // }
            } else {
                // use embedded exponent technique if c is provided
                f = if let Some(c) = c {
                    match pseudo_binary_encoding[i] {
                        1 => fp12_multiply(f, c),
                        -1 => fp12_multiply(f, c_inv),
                        _ => panic!("Invalid sigma_i"),
                    }
                } else {
                    f
                };

                // Run miller double and add if \sigma_i != 0
                let Q_signed = q_signed(Q, pseudo_binary_encoding[i]);
                let (Q_out, lines_S_plus_Q, lines_S_plus_Q_plus_S): (Vec<_>, Vec<_>, Vec<_>) =
                    Q_acc
                        .iter()
                        .zip(Q_signed.iter())
                        .map(|(Q_acc, Q_signed)| {
                            miller_double_and_add::<Fp, Fp2>(Q_acc.clone(), Q_signed.clone())
                        })
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
                    // lines.push(*line0);
                    // lines.push(*line1);
                }
                let lines_concat = [lines0, lines1].concat();
                lines.extend(lines_concat);

                // Debug counter
                total_double_add += 1;

                // // Gnark implementation
                // for chunk in lines.chunks(2) {
                //     if let [line0, line1] = chunk {
                //         let prod = mul_013_by_013(*line0, *line1, xi);
                //         f = mul_by_01234(f, prod);
                //     } else {
                //         panic!("lines.len() % 2 should be 0 at this point");
                //     }
                // }
            };
        }

        if lines.len() % 2 == 1 {
            f = mul_by_013::<Fp, Fp2, Fp12>(f, lines.pop().unwrap());
        }
        for chunk in lines.chunks(2) {
            if let [line0, line1] = chunk {
                let prod = mul_013_by_013(*line0, *line1, xi);
                f = mul_by_01234(f, prod);
            } else {
                panic!("lines.len() % 2 should be 0 at this point");
            }
        }
    }
    println!("miller: total double: {total_double}, total double&add: {total_double_add}");

    // NOTE: match gnark implementation
    // f.conjugate();

    f
}

#[allow(non_snake_case)]
pub fn multi_miller_loop_separate_double_plus_add<Fp, Fp2, Fp12>(
    P: &[EcPoint<Fp>],
    Q: &[EcPoint<Fp2>],
    _pseudo_binary_encoding: &[i32],
    xi: Fp2,
) -> Fp12
where
    Fp: Field,
    Fp2: FieldExtension<2, BaseField = Fp>,
    Fp12: FieldExtension<6, BaseField = Fp2>,
{
    pub const BLS_X: u64 = 0xd201_0000_0001_0000;
    assert!(P.len() > 0);
    assert_eq!(P.len(), Q.len());

    let y_invs = P.iter().map(|P| P.y.invert().unwrap()).collect::<Vec<Fp>>();
    let x_over_ys = P
        .iter()
        .zip(y_invs.iter())
        .map(|(P, y_inv)| P.x * y_inv)
        .collect::<Vec<Fp>>();

    let mut f = Fp12::ONE;
    let mut Q_acc = Q.to_vec();

    let mut total_double = 0;
    let mut total_double_add = 0;

    let mut found_one = false;
    for i in (0..64).rev().map(|b| (((BLS_X >> 1) >> b) & 1) == 1) {
        if !found_one {
            found_one = i;
            continue;
        }
        let i_binary = i as u8;
        println!("miller i: {} = {}; Q_acc.x: {:?}", i, i_binary, Q_acc[0].x);
        let mut lines = Vec::<[Fp2; 2]>::new();

        // Do double step
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

        // Gnark implementation
        // for line in lines.iter() {
        //     f = mul_by_013::<Fp, Fp2, Fp12>(f, *line);
        // }
        // lines = Vec::new();

        if i {
            // Do add step
            let (Q_out_add, lines_S_plus_Q) = Q_acc
                .iter()
                .zip(Q.iter())
                .map(|(Q_acc, Q)| miller_add_step::<Fp, Fp2>(Q_acc.clone(), Q.clone()))
                .unzip::<_, _, Vec<_>, Vec<_>>();
            Q_acc = Q_out_add;

            let lines_iter = izip!(
                lines_2S.iter(),
                lines_S_plus_Q.iter(),
                x_over_ys.iter(),
                y_invs.iter()
            );
            let mut lines0 = Vec::<[Fp2; 2]>::new();
            let mut lines1 = Vec::<[Fp2; 2]>::new();
            for (line0, line1, x_over_y, y_inv) in lines_iter {
                let line0 = &evaluate_line::<Fp, Fp2>(*line0, *x_over_y, *y_inv);
                let line1 = &evaluate_line::<Fp, Fp2>(*line1, *x_over_y, *y_inv);
                lines0.push(*line0);
                lines1.push(*line1);
                // lines.push(*line0);
                // lines.push(*line1);
            }
            let lines_concat = [lines0, lines1].concat();
            lines.extend(lines_concat);

            // Debug counter
            total_double_add += 1;

            // // Gnark implementation
            // for chunk in lines.chunks(2) {
            //     if let [line0, line1] = chunk {
            //         let prod = mul_013_by_013(*line0, *line1, xi);
            //         f = mul_by_01234(f, prod);
            //     } else {
            //         panic!("lines.len() % 2 should be 0 at this point");
            //     }
            // }
        } else {
            // Debug counter
            total_double += 1;
        }

        if lines.len() % 2 == 1 {
            f = mul_by_013::<Fp, Fp2, Fp12>(f, lines.pop().unwrap());
        }
        for chunk in lines.chunks(2) {
            if let [line0, line1] = chunk {
                let prod = mul_013_by_013(*line0, *line1, xi);
                f = mul_by_01234(f, prod);
            } else {
                panic!("lines.len() % 2 should be 0 at this point");
            }
        }

        f = fp12_square::<Fp12>(f);
    }
    let mut lines = Vec::<[Fp2; 2]>::new();

    // Do double step
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

    // Gnark implementation
    for line in lines.iter() {
        f = mul_by_013::<Fp, Fp2, Fp12>(f, *line);
    }

    // Debug counter
    total_double += 1;

    println!("miller: total double: {total_double}, total double&add: {total_double_add}");

    f.conjugate();

    f
}
